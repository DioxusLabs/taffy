//! Port of `yoga/algorithm/Cache.cpp`: when a cached measurement of a node with a measure function can be
//! reused for new constraints. [`Cache::get_yoga`](crate::tree::Cache::get_yoga) applies it to Taffy's
//! cache entries.

use super::enums::YogaSizingMode;
use super::numeric::{inexact_equals, is_defined};
use super::pixel_grid::round_value_to_pixel_grid;

/// One axis of a cache lookup or entry: Yoga's sizing mode and the available size (including margin).
#[derive(Copy, Clone, Debug)]
pub(crate) struct AxisConstraint {
    /// How the axis is sized.
    pub(crate) mode: YogaSizingMode,
    /// The available size including the node's margin; NaN for `MaxContent`.
    pub(crate) available: f32,
}

/// A stretch-fit request for exactly the size that was last measured.
fn size_is_exact_and_matches_old_measured_size(mode: YogaSizingMode, size: f32, last_computed_size: f32) -> bool {
    mode == YogaSizingMode::StretchFit && inexact_equals(size, last_computed_size)
}

/// A fit-content request with room for the last max-content measurement.
fn old_size_is_max_content_and_still_fits(
    mode: YogaSizingMode,
    size: f32,
    last_mode: YogaSizingMode,
    last_computed_size: f32,
) -> bool {
    mode == YogaSizingMode::FitContent
        && last_mode == YogaSizingMode::MaxContent
        && (size >= last_computed_size || inexact_equals(size, last_computed_size))
}

/// A tighter fit-content request that the last fit-content measurement still fits in.
fn new_size_is_stricter_and_still_valid(
    mode: YogaSizingMode,
    size: f32,
    last_mode: YogaSizingMode,
    last_size: f32,
    last_computed_size: f32,
) -> bool {
    last_mode == YogaSizingMode::FitContent
        && mode == YogaSizingMode::FitContent
        && is_defined(last_size)
        && is_defined(size)
        && is_defined(last_computed_size)
        && last_size > size
        && (last_computed_size <= size || inexact_equals(size, last_computed_size))
}

/// Whether one axis of a cached measurement is compatible with the new constraint.
fn axis_is_compatible(
    new: AxisConstraint,
    last: AxisConstraint,
    last_computed: f32,
    margin: f32,
    point_scale_factor: f32,
) -> bool {
    let use_rounded_comparison = point_scale_factor != 0.0;
    let effective = |value: f32| {
        if use_rounded_comparison {
            round_value_to_pixel_grid(value as f64, point_scale_factor as f64, false, false)
        } else {
            value
        }
    };
    let has_same_spec = last.mode == new.mode && inexact_equals(effective(last.available), effective(new.available));
    has_same_spec
        || size_is_exact_and_matches_old_measured_size(new.mode, new.available - margin, last_computed)
        || old_size_is_max_content_and_still_fits(new.mode, new.available - margin, last.mode, last_computed)
        || new_size_is_stricter_and_still_valid(
            new.mode,
            new.available - margin,
            last.mode,
            last.available,
            last_computed,
        )
}

/// Yoga's `canUseCachedMeasurement`: whether the measurement `last_computed`, taken under `last`, can be
/// reused for `new`. `margin` is the node's margin per axis (`[row, column]`), which Yoga's available
/// sizes include; `point_scale_factor == 0` disables the rounded comparison.
pub(crate) fn can_use_cached_measurement(
    new: [AxisConstraint; 2],
    last: [AxisConstraint; 2],
    last_computed: [f32; 2],
    margin: [f32; 2],
    point_scale_factor: f32,
) -> bool {
    if (is_defined(last_computed[1]) && last_computed[1] < 0.0)
        || (is_defined(last_computed[0]) && last_computed[0] < 0.0)
    {
        return false;
    }
    axis_is_compatible(new[0], last[0], last_computed[0], margin[0], point_scale_factor)
        && axis_is_compatible(new[1], last[1], last_computed[1], margin[1], point_scale_factor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use YogaSizingMode::{FitContent, MaxContent, StretchFit};

    fn c(mode: YogaSizingMode, available: f32) -> AxisConstraint {
        AxisConstraint { mode, available }
    }

    fn fits(new: [AxisConstraint; 2], last: [AxisConstraint; 2], computed: [f32; 2]) -> bool {
        can_use_cached_measurement(new, last, computed, [0.0, 0.0], 0.0)
    }

    #[test]
    fn same_spec() {
        let last = [c(FitContent, 100.0), c(MaxContent, f32::NAN)];
        assert!(fits(last, last, [80.0, 20.0]));
        assert!(fits([c(FitContent, 100.00005), c(MaxContent, f32::NAN)], last, [80.0, 20.0]));
        assert!(!fits([c(FitContent, 100.1), c(MaxContent, f32::NAN)], last, [80.0, 20.0]));
        assert!(!fits([c(StretchFit, 100.0), c(MaxContent, f32::NAN)], last, [80.0, 20.0]));
    }

    #[test]
    fn exact_size_matches_old_measurement() {
        let last = [c(FitContent, 100.0), c(FitContent, 100.0)];
        assert!(fits([c(StretchFit, 80.0), c(StretchFit, 20.0)], last, [80.0, 20.0]));
        assert!(!fits([c(StretchFit, 81.0), c(StretchFit, 20.0)], last, [80.0, 20.0]));
    }

    #[test]
    fn max_content_still_fits_and_stricter_still_valid() {
        let last = [c(MaxContent, f32::NAN), c(MaxContent, f32::NAN)];
        assert!(fits([c(FitContent, 80.0), c(FitContent, 500.0)], last, [80.0, 20.0]));
        assert!(!fits([c(FitContent, 79.0), c(FitContent, 500.0)], last, [80.0, 20.0]));

        let last = [c(FitContent, 100.0), c(FitContent, 100.0)];
        assert!(fits([c(FitContent, 90.0), c(FitContent, 100.0)], last, [80.0, 20.0]));
        assert!(!fits([c(FitContent, 70.0), c(FitContent, 100.0)], last, [80.0, 20.0]));
        assert!(!fits([c(FitContent, 110.0), c(FitContent, 100.0)], last, [80.0, 20.0]));
    }

    #[test]
    fn margin_and_rounding() {
        // Yoga's available sizes include the margin; the fuzzy rules compare against the content size.
        let last = [c(FitContent, 100.0), c(FitContent, 100.0)];
        assert!(can_use_cached_measurement(
            [c(StretchFit, 90.0), c(FitContent, 100.0)],
            last,
            [80.0, 20.0],
            [10.0, 0.0],
            0.0
        ));
        assert!(!can_use_cached_measurement(
            [c(StretchFit, 90.0), c(FitContent, 100.0)],
            last,
            [80.0, 20.0],
            [0.0, 0.0],
            0.0
        ));
        // With a scale factor the available sizes are compared on the pixel grid.
        assert!(!can_use_cached_measurement(
            [c(FitContent, 100.3), c(FitContent, 100.0)],
            last,
            [80.0, 20.0],
            [0.0, 0.0],
            0.0
        ));
        assert!(can_use_cached_measurement(
            [c(FitContent, 100.3), c(FitContent, 100.0)],
            last,
            [80.0, 20.0],
            [0.0, 0.0],
            1.0
        ));
        assert!(!can_use_cached_measurement(
            [c(FitContent, 100.3), c(FitContent, 100.0)],
            last,
            [80.0, 20.0],
            [0.0, 0.0],
            2.0
        ));
    }

    #[test]
    fn negative_measurements_are_never_reused() {
        let last = [c(FitContent, 100.0), c(FitContent, 100.0)];
        assert!(!fits(last, last, [-1.0, 20.0]));
        assert!(!fits(last, last, [80.0, -1.0]));
    }
}
