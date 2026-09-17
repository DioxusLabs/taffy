//! Generic CSS scrollable overflow code that is shared between all CSS algorithms.
use crate::geometry::{Point, Rect, Size};
use crate::style::{Contain, Overflow};
use crate::tree::ScrollableOverflowRect;
use crate::util::sys::{f32_max, f32_min};

/// The scroll origin of a scroll container, which determines which of its overflow is reachable
/// by scrolling (<https://www.w3.org/TR/css-overflow-3/#scroll-origin>).
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct ScrollOrigin {
    /// The size of the container's scrollport (padding box less scrollbars)
    scrollport_size: Size<f32>,
    /// Whether the scroll origin is at the end (right/bottom) edge of the scrollport in each axis,
    /// rather than the start (left/top) edge
    pub at_end: Point<bool>,
}

impl ScrollOrigin {
    /// Create a scroll origin. A scrollport size that is negative (scrollbars larger than the
    /// padding box) is clamped to zero.
    #[inline(always)]
    pub(crate) fn new(scrollport_size: Size<f32>, at_end: Point<bool>) -> Self {
        Self { scrollport_size: scrollport_size.map(|v| f32_max(v, 0.0)), at_end }
    }

    /// An empty rectangle located at the scroll origin, suitable as the initial value when
    /// accumulating (unioning) the contributions of a scroll container's contents
    #[inline(always)]
    pub(crate) fn empty_rect(self) -> Rect<f32> {
        let x = if self.at_end.x { self.scrollport_size.width } else { 0.0 };
        let y = if self.at_end.y { self.scrollport_size.height } else { 0.0 };
        Rect { left: x, right: x, top: y, bottom: y }
    }

    /// The initial value when accumulating the contributions of a container's contents: an empty
    /// rectangle at the scroll origin for a scroll container (`Some`), or at the top-left corner
    /// of the padding box otherwise
    #[inline(always)]
    pub(crate) fn initial_rect(origin: Option<Self>) -> Rect<f32> {
        origin.map(Self::empty_rect).unwrap_or(Rect::ZERO)
    }
}

#[inline(always)]
/// Determine the rectangle that a given node contributes to its parent's scrollable overflow
/// rectangle.
///
/// `location` is the physical position of the node's border box measured from the top-left corner
/// of the parent's scrollport (the same coordinate system as [`ScrollableOverflowRect`]). The
/// node's own `scrollable_overflow_rect` propagates to the parent only where the node's overflow
/// allows it to escape: a scroll container (`Hidden`/`Scroll` in either axis) clips both axes and
/// contributes only its border box, while for non-scroll-containers `Clip` clips just its own
/// axis and `Visible` propagates.
///
/// Boxes positioned wholly in the unreachable scrollable overflow region (entirely on the far side
/// of the scroll origin in either axis) must be clipped by a scroll container and are excluded
/// from its scrollable overflow region (<https://www.w3.org/TR/css-overflow-3/#scrollable>). This
/// exclusion only applies when the parent is a scroll container (`parent_scroll_origin` is `Some`):
/// boxes with `overflow: visible` have no unreachable region of their own, so all of their content
/// contributes.
///
/// A box whose containment contains its scrollable overflow (layout or paint containment)
/// contributes only its border box, regardless of its overflow style.
///
/// Boxes that contribute nothing return an empty rectangle at the parent's scroll origin (see
/// [`ScrollOrigin::initial_rect`]), which is neutral when unioned with an accumulator initialised
/// to that same rectangle.
pub(crate) fn compute_scrollable_overflow_contribution(
    location: Point<f32>,
    size: Size<f32>,
    scrollable_overflow_rect: ScrollableOverflowRect,
    overflow: Point<Overflow>,
    contain: Contain,
    parent_scroll_origin: Option<ScrollOrigin>,
) -> Rect<f32> {
    let is_scroll_container = overflow.x.is_scroll_container() || overflow.y.is_scroll_container();
    let overflow_is_contained = contain.contains_scrollable_overflow();
    let propagates = Point {
        x: !is_scroll_container && !overflow_is_contained && overflow.x == Overflow::Visible,
        y: !is_scroll_container && !overflow_is_contained && overflow.y == Overflow::Visible,
    };
    let rect = scrollable_overflow_rect.rect;
    let end_extent = Size {
        width: if propagates.x { f32_max(size.width, rect.right) } else { size.width },
        height: if propagates.y { f32_max(size.height, rect.bottom) } else { size.height },
    };
    if end_extent.width <= 0.0 || end_extent.height <= 0.0 {
        return ScrollOrigin::initial_rect(parent_scroll_origin);
    }
    let start_extent = Point {
        x: if propagates.x { f32_min(0.0, rect.left) } else { 0.0 },
        y: if propagates.y { f32_min(0.0, rect.top) } else { 0.0 },
    };
    let contribution = Rect {
        left: location.x + start_extent.x,
        right: location.x + end_extent.width,
        top: location.y + start_extent.y,
        bottom: location.y + end_extent.height,
    };
    match parent_scroll_origin {
        Some(origin) if is_wholly_unreachable(contribution, origin) => origin.empty_rect(),
        _ => contribution,
    }
}

/// Whether a rectangle (in scrollport coordinates) lies entirely beyond the scroll origin in either
/// axis, and is therefore unreachable by scrolling
#[inline(always)]
fn is_wholly_unreachable(rect: Rect<f32>, origin: ScrollOrigin) -> bool {
    let unreachable_x = if origin.at_end.x { rect.left >= origin.scrollport_size.width } else { rect.right <= 0.0 };
    let unreachable_y = if origin.at_end.y { rect.top >= origin.scrollport_size.height } else { rect.bottom <= 0.0 };
    unreachable_x || unreachable_y
}

/// Finalize the scrollable overflow rectangle of a scroll container:
///
/// - Extend it by the container's own padding at the end of its content (the side opposite the
///   scroll origin in each axis), which is part of its scrollable overflow region. Boxes that are
///   not scroll containers do not extend their overflow region by their own padding.
/// - Clip away the unreachable region on the far side of the scroll origin, so that in each axis
///   the rectangle only extends away from the origin.
///
/// This must only be called for scroll containers.
#[inline(always)]
pub(crate) fn finalize_scroll_container_overflow(rect: &mut Rect<f32>, padding: Rect<f32>, origin: ScrollOrigin) {
    if origin.at_end.x {
        rect.left -= padding.left;
    } else {
        rect.right += padding.right;
    }
    if origin.at_end.y {
        rect.top -= padding.top;
    } else {
        rect.bottom += padding.bottom;
    }
    clip_unreachable_overflow(rect, origin);
}

/// Clip away the unreachable region of a scroll container's scrollable overflow rectangle (the
/// region on the far side of the scroll origin), so that in each axis the rectangle only extends
/// away from the origin.
#[inline(always)]
pub(crate) fn clip_unreachable_overflow(rect: &mut Rect<f32>, origin: ScrollOrigin) {
    if origin.at_end.x {
        rect.right = f32_min(rect.right, origin.scrollport_size.width);
        rect.left = f32_min(rect.left, rect.right);
    } else {
        rect.left = f32_max(rect.left, 0.0);
        rect.right = f32_max(rect.right, rect.left);
    }
    if origin.at_end.y {
        rect.bottom = f32_min(rect.bottom, origin.scrollport_size.height);
        rect.top = f32_min(rect.top, rect.bottom);
    } else {
        rect.top = f32_max(rect.top, 0.0);
        rect.bottom = f32_max(rect.bottom, rect.top);
    }
}
