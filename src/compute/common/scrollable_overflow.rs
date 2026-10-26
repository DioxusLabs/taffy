//! Generic CSS scrollable overflow code that is shared between all CSS algorithms.
use crate::geometry::{Point, Rect, Size};
use crate::style::{Contain, Overflow};
use crate::tree::ScrollableOverflow;
use crate::util::sys::{f32_max, f32_min};

/// Container geometry shared by its children's scrollable overflow contributions.
pub(crate) struct ScrollableOverflowContainer {
    /// Whether the parent clips unreachable overflow and includes end padding.
    pub parent_is_scroll_container: bool,
    /// The extent of the parent's padding-box frame, excluding scrollbar gutters.
    pub parent_size: Size<f32>,
    /// The scroll origin in each physical axis.
    pub reversed: Point<bool>,
    /// The parent's resolved padding.
    pub padding: Rect<f32>,
}

#[inline(always)]
/// Determine the rectangle that a given node contributes to its parent's scrollable overflow
/// rectangle.
///
/// `location`, child overflow, and the result use physical axes in the parent's padding-box frame.
/// Child overflow is relative to its inner border edge, so the child's border is added first.
/// `parent_size` is the extent of the parent frame (excluding scrollbar gutters). The node's own
/// `scrollable_overflow_rect` propagates to the parent only where the node's overflow allows it
/// to escape: a scroll container (`Hidden`/`Scroll` in either axis) clips both axes and
/// contributes only its border box, while for non-scroll-containers `Clip` clips just its own
/// axis and `Visible` propagates.
///
/// Boxes positioned wholly in the unreachable scrollable overflow region (entirely before the
/// scroll origin in either axis) must be clipped by a scroll container and are excluded from its
/// scrollable overflow region (<https://www.w3.org/TR/css-overflow-3/#scrollable>). This
/// exclusion only applies when the parent is a scroll container: boxes with `overflow: visible`
/// have no unreachable region of their own, so all of their content contributes.
///
/// A box whose containment contains its scrollable overflow (layout or paint containment)
/// contributes only its border box, regardless of its overflow style.
///
/// End padding extends the in-flow child's margin edge, not its propagated overflow.
/// Union with the border/overflow contribution prevents negative margins from shrinking it.
#[allow(clippy::too_many_arguments)]
pub(crate) fn compute_scrollable_overflow_contribution(
    location: Point<f32>,
    size: Size<f32>,
    scrollable_overflow_rect: Rect<f32>,
    border: Rect<f32>,
    overflow: Point<Overflow>,
    contain: Contain,
    container: &ScrollableOverflowContainer,
    margin: Rect<f32>,
) -> Rect<f32> {
    let ScrollableOverflowContainer { parent_is_scroll_container, parent_size, reversed, padding } = *container;
    let scrollable_overflow_rect = Rect {
        left: scrollable_overflow_rect.left + border.left,
        right: scrollable_overflow_rect.right + border.left,
        top: scrollable_overflow_rect.top + border.top,
        bottom: scrollable_overflow_rect.bottom + border.top,
    };
    let is_scroll_container = overflow.x.is_scroll_container() || overflow.y.is_scroll_container();
    let overflow_is_contained = contain.contains_scrollable_overflow();
    let propagates = Point {
        x: !is_scroll_container && !overflow_is_contained && overflow.x == Overflow::Visible,
        y: !is_scroll_container && !overflow_is_contained && overflow.y == Overflow::Visible,
    };
    let end_extent = Size {
        width: if propagates.x { f32_max(size.width, scrollable_overflow_rect.right) } else { size.width },
        height: if propagates.y { f32_max(size.height, scrollable_overflow_rect.bottom) } else { size.height },
    };
    if end_extent.width <= 0.0 || end_extent.height <= 0.0 {
        return Rect::ZERO;
    }
    let start_extent = Point {
        x: if propagates.x { f32_min(0.0, scrollable_overflow_rect.left) } else { 0.0 },
        y: if propagates.y { f32_min(0.0, scrollable_overflow_rect.top) } else { 0.0 },
    };
    let contribution = Rect {
        left: location.x + start_extent.x,
        right: location.x + end_extent.width,
        top: location.y + start_extent.y,
        bottom: location.y + end_extent.height,
    };
    let is_wholly_unreachable =
        (if reversed.x { contribution.left >= parent_size.width } else { contribution.right <= 0.0 })
            || (if reversed.y { contribution.top >= parent_size.height } else { contribution.bottom <= 0.0 });
    if parent_is_scroll_container && is_wholly_unreachable {
        Rect::ZERO
    } else {
        if parent_is_scroll_container {
            let mut padded = contribution;
            if reversed.x {
                padded.left = f32_min(padded.left, location.x - f32_max(0.0, padding.left + margin.left));
            } else {
                padded.right =
                    f32_max(padded.right, location.x + size.width + f32_max(0.0, padding.right + margin.right));
            }
            if reversed.y {
                padded.top = f32_min(padded.top, location.y - f32_max(0.0, padding.top + margin.top));
            } else {
                padded.bottom =
                    f32_max(padded.bottom, location.y + size.height + f32_max(0.0, padding.bottom + margin.bottom));
            }
            padded
        } else {
            contribution
        }
    }
}

/// Pack an algorithm's physical accumulator and clip unreachable
/// overflow only for scroll containers. All geometry is unpacked until this output boundary.
pub(crate) fn finalize_scrollable_overflow(
    mut rect: Rect<f32>,
    size: Size<f32>,
    reversed: Point<bool>,
    is_scroll_container: bool,
) -> ScrollableOverflow {
    let size = Size { width: f32_max(size.width, 0.0), height: f32_max(size.height, 0.0) };
    let reversed = if is_scroll_container {
        if reversed.x {
            rect.right = f32_min(rect.right, size.width);
        } else {
            rect.left = 0.0;
        }
        if reversed.y {
            rect.bottom = f32_min(rect.bottom, size.height);
        } else {
            rect.top = 0.0;
        }
        reversed
    } else {
        Point { x: false, y: false }
    };
    ScrollableOverflow::new(rect, reversed)
}
