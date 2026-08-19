//! Generic CSS scrollable overflow code that is shared between all CSS algorithms.
use crate::geometry::{Point, Rect, Size};
use crate::style::{Contain, Overflow};
use crate::util::sys::{f32_max, f32_min};

#[inline(always)]
/// Determine the rectangle that a given node contributes to its parent's scrollable overflow
/// rectangle.
///
/// `location` is the position of the node's border box measured from the parent's scroll origin
/// (logical start edge): callers pass coordinates that are mirrored for RTL. The node's own
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
pub(crate) fn compute_scrollable_overflow_contribution(
    location: Point<f32>,
    size: Size<f32>,
    scrollable_overflow_rect: Rect<f32>,
    overflow: Point<Overflow>,
    contain: Contain,
    parent_is_scroll_container: bool,
) -> Rect<f32> {
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
    let is_wholly_unreachable = contribution.right <= 0.0 || contribution.bottom <= 0.0;
    if parent_is_scroll_container && is_wholly_unreachable {
        Rect::ZERO
    } else {
        contribution
    }
}
