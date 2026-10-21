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

/// Multiple of `AU_PER_PX` that puts `value + AU_PER_PX / 2` in `u32` range for every `value` up to `ROUND_MAX`
const ROUND_OFFSET: u32 = AU_PER_PX as u32 * 35_791_394;

/// Inputs above this round to the same value as this one, so clamping them keeps `round_au` exact
const ROUND_MAX: i32 = 2_147_483_625;

/// Rounds an `Au` value to the nearest whole pixel, with halves rounding up
#[inline(always)]
pub(crate) fn round_au(value: Au) -> Au {
    let shifted = (value.0.min(ROUND_MAX) as u32).wrapping_add(AU_PER_PX as u32 / 2 + ROUND_OFFSET);
    let rounded = (shifted / AU_PER_PX as u32) * AU_PER_PX as u32;
    Au(rounded.wrapping_sub(ROUND_OFFSET) as i32)
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
