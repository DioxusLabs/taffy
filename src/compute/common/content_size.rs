//! Generic CSS content size code that is shared between all CSS algorithms.
use crate::geometry::{Point, Size};
use crate::style::Overflow;
use crate::util::sys::f32_max;

#[inline(always)]
/// Determine how much width/height a given node contributes to it's parent's content size
pub(crate) fn compute_content_size_contribution(
    location: Point<f32>,
    size: Size<f32>,
    content_size: Size<f32>,
    overflow: Point<Overflow>,
) -> Size<f32> {
    let size_content_size_contribution = Size {
        width: match overflow.x {
            Overflow::Visible => f32_max(size.width, content_size.width),
            _ => size.width,
        },
        height: match overflow.y {
            Overflow::Visible => f32_max(size.height, content_size.height),
            _ => size.height,
        },
    };
    if size_content_size_contribution.width > 0.0 && size_content_size_contribution.height > 0.0 {
        // `location` is measured from the scroll origin (logical start edge): callers pass
        // coordinates that are mirrored for RTL. Overflow past the scroll origin (negative
        // coordinates) is unreachable and does not contribute to the scrollable overflow region.
        Size {
            width: f32_max(location.x + size_content_size_contribution.width, 0.0),
            height: f32_max(location.y + size_content_size_contribution.height, 0.0),
        }
    } else {
        Size::ZERO
    }
}
