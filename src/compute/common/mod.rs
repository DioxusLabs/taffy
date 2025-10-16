//! Generic code that is shared between multiple layout algorithms

use crate::{
    geometry::{Point, Rect, Size},
    style::Overflow,
};

pub(crate) mod alignment;

#[cfg(feature = "content_size")]
pub(crate) mod content_size;

/// Return the widths of the scrollbars present for the particular
/// `Overflow` factoring in that the scrollbars can't cause the
/// dimensions of the content rectange to go negative.
pub(crate) fn compute_scrollbar_size(
    overflow: Point<Overflow>,
    outer_size: Size<f32>,
    padding: Rect<f32>,
    border: Rect<f32>,
    scrollbar_width: f32,
) -> Size<f32> {
    // Earlier stuff is supposed to inflate outer_size enough to hold
    // padding and border, but clamp the value at zero just in case.
    let avail = outer_size - (padding + border).sum_axes().map(|val| val.max(0.0));
    // Scrollbars consume space on the perpendicular axis, hence the
    // `.transpose()`.
    Size::from(overflow.transpose()).zip_map(avail, |overflow, avail| {
        if overflow == Overflow::Scroll {
            avail.min(scrollbar_width)
        } else {
            0.0
        }
    })
}
