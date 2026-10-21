//! Conversions between CSS `f32` lengths and `Au` layout geometry.
//!
//! These deliberately avoid libm calls (`round`, `ceil`, `floor`) and `Au`'s float helpers, so that rounding
//! is a plain add-and-truncate on the sign-corrected value.

use crate::app_unit::{Au, AU_PER_PX};

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

/// Multiple of `AU_PER_PX` that keeps `value + AU_PER_PX / 2 + ROUND_OFFSET` non-negative for every `i32`, so
/// the division in `round_au` can be unsigned
const ROUND_OFFSET: i64 = AU_PER_PX as i64 * 35_791_394;

/// Rounds an `Au` value to the nearest whole pixel, with halves rounding up
#[inline(always)]
pub(crate) fn round_au(value: Au) -> Au {
    let shifted = (value.0 as i64 + (AU_PER_PX / 2) as i64 + ROUND_OFFSET) as u64;
    let rounded = (shifted / AU_PER_PX as u64) * AU_PER_PX as u64;
    Au((rounded as i64 - ROUND_OFFSET) as i32)
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

#[cfg(test)]
mod tests {
    use super::{round_au, Au};

    #[test]
    fn round_au_rounds_halves_up_to_whole_px() {
        assert_eq!(round_au(Au(0)), Au(0));
        assert_eq!(round_au(Au(29)), Au(0));
        assert_eq!(round_au(Au(30)), Au(60));
        assert_eq!(round_au(Au(-30)), Au(0));
        assert_eq!(round_au(Au(-31)), Au(-60));
    }

    #[test]
    fn round_au_covers_full_i32_range() {
        assert_eq!(round_au(Au(i32::MAX)), Au(2_147_483_640));
        assert_eq!(round_au(Au(i32::MIN)), Au(-2_147_483_640));
    }
}
