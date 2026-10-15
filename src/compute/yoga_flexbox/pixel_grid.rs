//! Port of `yoga/algorithm/PixelGrid.cpp`. Only `roundValueToPixelGrid` so far; the layout rounding pass
//! (`roundLayoutResultsToPixelGrid`) comes with `round_yoga_layout`.

use super::numeric::inexact_equals_f64;

/// Rounds `value` to the pixel grid of `point_scale_factor` pixels per point, in `f64` as Yoga does.
///
/// `force_ceil`/`force_floor` are used for text nodes. Returns NaN for an undefined value or scale.
pub(crate) fn round_value_to_pixel_grid(
    value: f64,
    point_scale_factor: f64,
    force_ceil: bool,
    force_floor: bool,
) -> f32 {
    let mut scaled_value = value * point_scale_factor;
    // `fractional` is such that `floor(scaled_value) == scaled_value - fractional`, also for negatives.
    let mut fractional = scaled_value % 1.0;
    if fractional < 0.0 {
        fractional += 1.0;
    }
    if inexact_equals_f64(fractional, 0.0) {
        // Already rounded.
        scaled_value -= fractional;
    } else if inexact_equals_f64(fractional, 1.0) {
        scaled_value = scaled_value - fractional + 1.0;
    } else if force_ceil {
        scaled_value = scaled_value - fractional + 1.0;
    } else if force_floor {
        scaled_value -= fractional;
    } else {
        let round_up = !fractional.is_nan() && (fractional > 0.5 || inexact_equals_f64(fractional, 0.5));
        scaled_value = scaled_value - fractional + if round_up { 1.0 } else { 0.0 };
    }
    if scaled_value.is_nan() || point_scale_factor.is_nan() {
        f32::NAN
    } else {
        (scaled_value / point_scale_factor) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_to_scale() {
        assert_eq!(round_value_to_pixel_grid(10.4, 1.0, false, false), 10.0);
        assert_eq!(round_value_to_pixel_grid(10.5, 1.0, false, false), 11.0);
        assert_eq!(round_value_to_pixel_grid(10.49996, 1.0, false, false), 11.0);
        assert_eq!(round_value_to_pixel_grid(-2.2, 1.0, false, false), -2.0);
        assert_eq!(round_value_to_pixel_grid(-2.7, 1.0, false, false), -3.0);
        assert_eq!(round_value_to_pixel_grid(10.3, 2.0, false, false), 10.5);
        assert_eq!(round_value_to_pixel_grid(10.2, 2.0, false, false), 10.0);
        assert_eq!(round_value_to_pixel_grid(10.2, 1.0, true, false), 11.0);
        assert_eq!(round_value_to_pixel_grid(10.8, 1.0, false, true), 10.0);
        assert_eq!(round_value_to_pixel_grid(10.00005, 1.0, true, false), 10.0);
        assert!(round_value_to_pixel_grid(f64::NAN, 1.0, false, false).is_nan());
    }
}
