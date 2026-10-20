//! Conversions between CSS `f32` lengths and `Au` layout geometry.
//!
//! These deliberately avoid libm calls (`round`, `ceil`, `floor`) and `Au`'s float helpers, so that rounding
//! is a plain add-and-truncate on the sign-corrected value.

use app_units::{Au, AU_PER_PX};

/// Rounds half away from zero. `as` saturates on overflow and maps NaN to zero.
#[inline(always)]
fn round_to_i32(value: f32) -> i32 {
    (value + 0.5f32.copysign(value)) as i32
}

/// Converts a CSS pixel value to `Au`, saturating at `MIN_AU`/`MAX_AU` (so infinities become the limits)
#[inline(always)]
pub(crate) fn px_to_au(px: f32) -> Au {
    Au::new(round_to_i32(px * AU_PER_PX as f32))
}

/// Rounds an `Au` value to the nearest whole pixel, with halves rounding up
#[inline(always)]
pub(crate) fn round_au(value: Au) -> Au {
    Au((value.0 + AU_PER_PX / 2).div_euclid(AU_PER_PX) * AU_PER_PX)
}

/// Rounds a raw `Au` count computed in `f32` to the nearest `Au`
#[inline(always)]
pub(crate) fn raw_to_au(raw: f32) -> Au {
    Au::new(round_to_i32(raw))
}

/// Scales an `Au` value by a unitless factor (e.g. a percentage), rounding to the nearest `Au`
#[inline(always)]
pub(crate) fn scale_au(basis: Au, factor: f32) -> Au {
    raw_to_au(basis.0 as f32 * factor)
}
