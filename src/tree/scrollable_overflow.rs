//! The [`ScrollableOverflowRect`] type: the scrollable overflow rectangle of a box and the
//! scroll-related quantities that can be derived from it.
use crate::geometry::{Point, Rect, Size};
use crate::util::sys::{f32_max, f32_min};
use core::ops::{Deref, DerefMut};

/// The scrollable overflow rectangle of a box: the axis-aligned rectangle containing the content
/// of the box (the border boxes of its descendants plus their non-clipped overflow), corresponding
/// to the CSS "scrollable overflow rectangle" (<https://www.w3.org/TR/css-overflow-3/#scrollable>),
/// except that transforms are not accounted for.
///
/// Coordinates are *physical*: `left`/`right` are horizontal offsets and `top`/`bottom` vertical
/// offsets from the top-left corner of the box's scrollport (its padding box less any scrollbar
/// gutter, which is placed at the inline-end side; for LTR boxes this is the top-left corner of the
/// padding box). This is the same coordinate system as the `location`s of the box's children,
/// offset by the box's border (and any inline-start scrollbar gutter).
///
/// For scroll containers, the rectangle always contains the box's scroll origin, and overflow that
/// is unreachable by scrolling is already excluded: content can only extend the rectangle in the
/// direction(s) away from the scroll origin (<https://www.w3.org/TR/css-overflow-3/#scroll-origin>).
/// Overflow towards the start side of an axis (negative `left`/`top`) therefore indicates that the
/// scroll origin is at the *end* of that axis (e.g. the right edge for an RTL block container or an
/// LTR `flex-direction: row-reverse` flex container), and represents content that is reached by
/// scrolling towards *negative* scroll offsets. Overflow towards the end side (`right`/`bottom`
/// beyond the scrollport size) is reached by scrolling towards positive offsets. See
/// [`scroll_range`](Self::scroll_range).
///
/// For non-scroll-containers the rectangle always contains the scrollport's top-left corner and
/// all of the box's content, including content before that corner (negative `left`/`top`).
///
/// The type dereferences to the underlying [`Rect`] for direct access to its edges.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ScrollableOverflowRect {
    /// The rectangle (see the type-level documentation for the coordinate conventions)
    pub rect: Rect<f32>,
}

impl ScrollableOverflowRect {
    /// An empty scrollable overflow rectangle located at the scrollport origin
    pub const ZERO: Self = Self { rect: Rect::ZERO };

    /// Create a `ScrollableOverflowRect` from a `Rect` (see the type-level documentation for the
    /// coordinate conventions)
    #[inline]
    pub const fn new(rect: Rect<f32>) -> Self {
        Self { rect }
    }

    /// Whether the scroll origin of the box is at the end (right/bottom) of each axis rather than
    /// the start (left/top). This is only meaningful for a scroll container that overflows in the
    /// axis: it is `false` in an axis without overflow.
    #[inline]
    pub fn scroll_origin_at_end(self) -> Point<bool> {
        Point { x: self.rect.left < 0.0, y: self.rect.top < 0.0 }
    }

    /// The smallest rectangle containing both `self` and `other`
    #[inline]
    #[must_use]
    pub fn union(self, other: Self) -> Self {
        Self { rect: self.rect.union(other.rect) }
    }

    /// The scrollable overflow *area* of the box (the union of the rectangle and the scrollport),
    /// as a rectangle in the same coordinate system as `self`
    ///
    /// `scrollport_size` is the size of the box's scrollport (padding box less scrollbars)
    #[inline]
    pub fn scrollable_overflow_area(self, scrollport_size: Size<f32>) -> Rect<f32> {
        Rect {
            left: f32_min(self.rect.left, 0.0),
            right: f32_max(self.rect.right, scrollport_size.width),
            top: f32_min(self.rect.top, 0.0),
            bottom: f32_max(self.rect.bottom, scrollport_size.height),
        }
    }

    /// The size of the scrollable overflow area of the box. For a scroll container, this
    /// corresponds to the CSSOM `scrollWidth`/`scrollHeight`.
    ///
    /// `scrollport_size` is the size of the box's scrollport (padding box less scrollbars)
    #[inline]
    pub fn scroll_size(self, scrollport_size: Size<f32>) -> Size<f32> {
        let area = self.scrollable_overflow_area(scrollport_size);
        Size { width: area.right - area.left, height: area.bottom - area.top }
    }

    /// The range of valid scroll offsets of the box in each axis, as a rectangle whose
    /// `left`/`right` are the minimum/maximum horizontal scroll offsets and whose `top`/`bottom`
    /// are the minimum/maximum vertical scroll offsets.
    ///
    /// A scroll offset of zero in an axis corresponds to the box's initial scroll position (scrolled
    /// to its scroll origin), and is always within the range. Offsets are *negative* when the
    /// scroll origin is at the end of the axis (e.g. horizontally for RTL scroll containers), and
    /// positive otherwise, matching the CSSOM `scrollLeft`/`scrollTop` conventions. Content is
    /// painted translated by the negation of the scroll offset.
    ///
    /// `scrollport_size` is the size of the box's scrollport (padding box less scrollbars)
    #[inline]
    pub fn scroll_range(self, scrollport_size: Size<f32>) -> Rect<f32> {
        Rect {
            left: f32_min(self.rect.left, 0.0),
            right: f32_max(self.rect.right - scrollport_size.width, 0.0),
            top: f32_min(self.rect.top, 0.0),
            bottom: f32_max(self.rect.bottom - scrollport_size.height, 0.0),
        }
    }

    /// The total distance that the box can be scrolled in each axis (the extent of
    /// [`scroll_range`](Self::scroll_range))
    ///
    /// `scrollport_size` is the size of the box's scrollport (padding box less scrollbars)
    #[inline]
    pub fn scroll_distance(self, scrollport_size: Size<f32>) -> Size<f32> {
        let range = self.scroll_range(scrollport_size);
        Size { width: range.right - range.left, height: range.bottom - range.top }
    }

    /// Clamp a scroll offset to the box's [`scroll_range`](Self::scroll_range)
    ///
    /// `scrollport_size` is the size of the box's scrollport (padding box less scrollbars)
    #[inline]
    pub fn clamp_scroll_offset(self, offset: Point<f32>, scrollport_size: Size<f32>) -> Point<f32> {
        let range = self.scroll_range(scrollport_size);
        Point { x: offset.x.clamp(range.left, range.right), y: offset.y.clamp(range.top, range.bottom) }
    }
}

impl From<Rect<f32>> for ScrollableOverflowRect {
    #[inline]
    fn from(rect: Rect<f32>) -> Self {
        Self { rect }
    }
}

impl Deref for ScrollableOverflowRect {
    type Target = Rect<f32>;

    #[inline]
    fn deref(&self) -> &Rect<f32> {
        &self.rect
    }
}

impl DerefMut for ScrollableOverflowRect {
    #[inline]
    fn deref_mut(&mut self) -> &mut Rect<f32> {
        &mut self.rect
    }
}

#[cfg(test)]
mod tests {
    use super::ScrollableOverflowRect;
    use crate::geometry::{Point, Rect, Size};

    const SCROLLPORT: Size<f32> = Size { width: 100.0, height: 50.0 };

    #[test]
    fn no_overflow() {
        let rect = ScrollableOverflowRect::new(Rect { left: 0.0, right: 60.0, top: 0.0, bottom: 20.0 });
        assert_eq!(rect.scroll_size(SCROLLPORT), SCROLLPORT);
        assert_eq!(rect.scroll_range(SCROLLPORT), Rect::ZERO);
        assert_eq!(rect.scroll_distance(SCROLLPORT), Size::ZERO);
        assert_eq!(rect.scroll_origin_at_end(), Point { x: false, y: false });
        assert_eq!(rect.clamp_scroll_offset(Point { x: 10.0, y: -10.0 }, SCROLLPORT), Point::ZERO);
    }

    #[test]
    fn end_side_overflow() {
        let rect = ScrollableOverflowRect::new(Rect { left: 0.0, right: 250.0, top: 0.0, bottom: 80.0 });
        assert_eq!(rect.scroll_size(SCROLLPORT), Size { width: 250.0, height: 80.0 });
        assert_eq!(rect.scroll_range(SCROLLPORT), Rect { left: 0.0, right: 150.0, top: 0.0, bottom: 30.0 });
        assert_eq!(rect.scroll_distance(SCROLLPORT), Size { width: 150.0, height: 30.0 });
        assert_eq!(rect.scroll_origin_at_end(), Point { x: false, y: false });
        assert_eq!(rect.clamp_scroll_offset(Point { x: 1000.0, y: -1000.0 }, SCROLLPORT), Point { x: 150.0, y: 0.0 });
    }

    #[test]
    fn start_side_overflow() {
        let rect = ScrollableOverflowRect::new(Rect { left: -150.0, right: 100.0, top: -30.0, bottom: 50.0 });
        assert_eq!(rect.scroll_size(SCROLLPORT), Size { width: 250.0, height: 80.0 });
        assert_eq!(rect.scroll_range(SCROLLPORT), Rect { left: -150.0, right: 0.0, top: -30.0, bottom: 0.0 });
        assert_eq!(rect.scroll_distance(SCROLLPORT), Size { width: 150.0, height: 30.0 });
        assert_eq!(rect.scroll_origin_at_end(), Point { x: true, y: true });
        assert_eq!(rect.clamp_scroll_offset(Point { x: -1000.0, y: 1000.0 }, SCROLLPORT), Point { x: -150.0, y: 0.0 });
    }

    #[test]
    fn overflow_on_both_sides() {
        // Non-scroll-containers can overflow on both sides of an axis
        let rect = ScrollableOverflowRect::new(Rect { left: -20.0, right: 130.0, top: 0.0, bottom: 10.0 });
        assert_eq!(
            rect.scrollable_overflow_area(SCROLLPORT),
            Rect { left: -20.0, right: 130.0, top: 0.0, bottom: 50.0 }
        );
        assert_eq!(rect.scroll_size(SCROLLPORT), Size { width: 150.0, height: 50.0 });
        assert_eq!(rect.scroll_range(SCROLLPORT), Rect { left: -20.0, right: 30.0, top: 0.0, bottom: 0.0 });
    }

    #[test]
    fn union() {
        let a = ScrollableOverflowRect::new(Rect { left: -10.0, right: 20.0, top: 0.0, bottom: 5.0 });
        let b = ScrollableOverflowRect::new(Rect { left: 0.0, right: 40.0, top: -5.0, bottom: 1.0 });
        assert_eq!(a.union(b).rect, Rect { left: -10.0, right: 40.0, top: -5.0, bottom: 5.0 });
    }
}
