//! Yoga's cache lookup rules (`calculateLayoutInternal` in `yoga/algorithm/CalculateLayout.cpp` and
//! `yoga/algorithm/Cache.cpp`) applied to the ordinary [`Cache`] entries.

use super::{Cache, CacheKey, CacheMode, INFINITY_BITS, NEG_INFINITY_BITS, SIGN_BIT_2};
use crate::compute::yoga_flexbox::cache::{can_use_cached_measurement, AxisConstraint};
use crate::compute::yoga_flexbox::numeric::inexact_equals;
use crate::compute::yoga_flexbox::YogaSizingMode;
use crate::geometry::Size;
use crate::tree::{LayoutInput, LayoutOutput, RunMode};

/// What [`Cache::get_yoga`] needs besides the [`LayoutInput`] to apply Yoga's cache rules.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct YogaCacheParams {
    /// `YogaConfig::point_scale_factor`; `0.0` compares available sizes unrounded.
    pub point_scale_factor: f32,
    /// Whether the node has a measure function. Such nodes may reuse measurements taken under
    /// different constraints; other nodes only reuse exact matches.
    pub has_measure_func: bool,
    /// The node's resolved margin, summed per axis (`width` = left + right), which Yoga's available
    /// sizes include and Taffy's do not.
    pub margin: Size<f32>,
}

impl Cache {
    /// Looks up a cached layout with Yoga's matching rules instead of Taffy's. Entries are the ones
    /// [`store`](Self::store) writes; `parent_size` and the requested axis are ignored, as Yoga ignores them.
    ///
    /// Yoga sizing modes are decoded from the input: a known dimension is `StretchFit`, a definite available
    /// space `FitContent`, `MaxContent` is `MaxContent`, and `MinContent` is treated as `FitContent 0`.
    pub fn get_yoga(&mut self, input: &LayoutInput, params: &YogaCacheParams) -> Option<LayoutOutput> {
        if input.run_mode == RunMode::PerformHiddenLayout || self.mode != CacheMode::from(input) {
            return None;
        }
        let key = CacheKey::from(input);
        let new = yoga_constraints(key.kd_available_space, params.margin);
        let matches = |entry_key: &CacheKey, computed: Size<f32>| {
            let last = yoga_constraints(entry_key.kd_available_space, params.margin);
            if params.has_measure_func {
                can_use_cached_measurement(
                    new,
                    last,
                    [computed.width, computed.height],
                    [params.margin.width, params.margin.height],
                    params.point_scale_factor,
                )
            } else {
                exact_yoga_match(new, last)
            }
        };

        // A node with a measure function tries the layout entry first, then the measurement entries,
        // whatever the run mode; other nodes only look in the entries for their run mode.
        if params.has_measure_func || input.run_mode == RunMode::PerformLayout {
            if let Some(entry) = &self.final_layout_entry {
                if matches(&entry.key, entry.content.size) {
                    return Some(entry.content.clone());
                }
            }
        }
        if params.has_measure_func || input.run_mode == RunMode::ComputeSize {
            for (index, entry) in self.measure_entries.iter().enumerate() {
                let Some(entry) = entry else { continue };
                if matches(&entry.key, entry.content) {
                    self.recently_used_entries |= 1 << index;
                    return Some(LayoutOutput::from_outer_size(entry.content));
                }
            }
        }
        None
    }
}

/// Decodes one axis of a `kd_available_space` key into Yoga's sizing mode and available size.
fn yoga_axis_constraint(bits: u32, margin: f32) -> AxisConstraint {
    let (mode, size) = match bits {
        INFINITY_BITS => (YogaSizingMode::MaxContent, f32::NAN),
        NEG_INFINITY_BITS => (YogaSizingMode::FitContent, 0.0),
        _ if bits & SIGN_BIT_2 as u32 != 0 => (YogaSizingMode::FitContent, -f32::from_bits(bits)),
        _ => (YogaSizingMode::StretchFit, f32::from_bits(bits)),
    };
    AxisConstraint { mode, available: size + margin }
}

/// Both axes of a `kd_available_space` key, `[width, height]`.
fn yoga_constraints(kd_available_space: u64, margin: Size<f32>) -> [AxisConstraint; 2] {
    [
        yoga_axis_constraint((kd_available_space >> 32) as u32, margin.width),
        yoga_axis_constraint(kd_available_space as u32, margin.height),
    ]
}

/// The lookup for nodes without a measure function: same modes, available sizes equal within `1e-4`.
fn exact_yoga_match(new: [AxisConstraint; 2], last: [AxisConstraint; 2]) -> bool {
    inexact_equals(last[0].available, new[0].available)
        && inexact_equals(last[1].available, new[1].available)
        && last[0].mode == new[0].mode
        && last[1].mode == new[1].mode
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Line;
    use crate::style::AvailableSpace;
    use crate::tree::{RequestedAxis, SizingMode};

    fn input(run_mode: RunMode, known: Size<Option<f32>>, available: Size<AvailableSpace>) -> LayoutInput {
        LayoutInput {
            run_mode,
            sizing_mode: SizingMode::InherentSize,
            axis: RequestedAxis::Both,
            known_dimensions: known,
            known_dimensions_are_definite: Size { width: true, height: true },
            parent_size: Size::NONE,
            available_space: available,
            vertical_margins_are_collapsible: Line::FALSE,
        }
    }

    fn fit(w: f32, h: f32) -> Size<AvailableSpace> {
        Size { width: AvailableSpace::Definite(w), height: AvailableSpace::Definite(h) }
    }

    fn params(has_measure_func: bool) -> YogaCacheParams {
        YogaCacheParams { point_scale_factor: 0.0, has_measure_func, margin: Size::ZERO }
    }

    #[test]
    fn exact_match_ignores_parent_size_and_axis() {
        let mut cache = Cache::new();
        let stored = input(RunMode::ComputeSize, Size::NONE, fit(100.0, 50.0));
        cache.store(&stored, LayoutOutput::from_outer_size(Size { width: 80.0, height: 20.0 }));

        let mut lookup = stored;
        lookup.parent_size = Size { width: Some(300.0), height: None };
        lookup.axis = RequestedAxis::Horizontal;
        assert_eq!(cache.get(&lookup), None);
        assert_eq!(cache.get_yoga(&lookup, &params(false)).map(|o| o.size), Some(Size { width: 80.0, height: 20.0 }));

        // Within 1e-4 of the stored available size still matches; a different mode does not.
        lookup.available_space = fit(100.00005, 50.0);
        assert!(cache.get_yoga(&lookup, &params(false)).is_some());
        lookup.available_space = fit(100.1, 50.0);
        assert!(cache.get_yoga(&lookup, &params(false)).is_none());
        lookup.known_dimensions = Size { width: Some(100.0), height: None };
        assert!(cache.get_yoga(&lookup, &params(false)).is_none());
        // Measurement entries are not used for a layout pass of a node without a measure function.
        let layout = input(RunMode::PerformLayout, Size::NONE, fit(100.0, 50.0));
        assert!(cache.get_yoga(&layout, &params(false)).is_none());
    }

    #[test]
    fn measure_func_nodes_reuse_compatible_measurements_in_either_entry() {
        let mut cache = Cache::new();
        let max_content = Size { width: AvailableSpace::MaxContent, height: AvailableSpace::MaxContent };
        let measured = input(RunMode::ComputeSize, Size::NONE, max_content);
        cache.store(&measured, LayoutOutput::from_outer_size(Size { width: 80.0, height: 20.0 }));

        // A max-content measurement still fits a fit-content lookup, from a layout pass too.
        let layout = input(RunMode::PerformLayout, Size::NONE, fit(100.0, 30.0));
        assert_eq!(cache.get_yoga(&layout, &params(true)).map(|o| o.size), Some(Size { width: 80.0, height: 20.0 }));
        assert!(cache.get_yoga(&layout, &params(false)).is_none());
        // ... but not when the lookup is narrower than the measured size.
        let narrow = input(RunMode::PerformLayout, Size::NONE, fit(60.0, 30.0));
        assert!(cache.get_yoga(&narrow, &params(true)).is_none());
        // Known dimensions equal to the measured size reuse it as well.
        let exact = input(RunMode::ComputeSize, Size { width: Some(80.0), height: Some(20.0) }, fit(100.0, 30.0));
        assert!(cache.get_yoga(&exact, &params(true)).is_some());

        // The layout entry is also consulted for a measurement of a measure-func node.
        let mut cache = Cache::new();
        cache.store(&layout, LayoutOutput::from_outer_size(Size { width: 80.0, height: 20.0 }));
        assert!(cache.get_yoga(&measured, &params(false)).is_none());
        let stricter = input(RunMode::ComputeSize, Size::NONE, fit(90.0, 30.0));
        assert!(cache.get_yoga(&stricter, &params(true)).is_some());
    }

    #[test]
    fn margin_is_added_back_and_sizes_are_compared_on_the_pixel_grid() {
        let mut cache = Cache::new();
        let stored = input(RunMode::ComputeSize, Size::NONE, fit(100.0, 50.0));
        cache.store(&stored, LayoutOutput::from_outer_size(Size { width: 80.0, height: 20.0 }));
        // A known width of 80 with 10 of margin is Yoga's `StretchFit 90`; the fuzzy rule compares the
        // content size (90 - 10) with the measurement (80).
        let lookup = input(RunMode::ComputeSize, Size { width: Some(80.0), height: None }, fit(100.0, 50.0));
        let with_margin = YogaCacheParams { margin: Size { width: 10.0, height: 0.0 }, ..params(true) };
        assert!(cache.get_yoga(&lookup, &with_margin).is_some());

        let rounded = input(RunMode::ComputeSize, Size::NONE, fit(100.3, 50.0));
        assert!(cache.get_yoga(&rounded, &params(true)).is_none());
        assert!(cache.get_yoga(&rounded, &YogaCacheParams { point_scale_factor: 1.0, ..params(true) }).is_some());
        assert!(cache.get_yoga(&rounded, &YogaCacheParams { point_scale_factor: 2.0, ..params(true) }).is_none());
    }

    #[test]
    fn min_content_is_fit_content_zero() {
        let mut cache = Cache::new();
        let min_content = input(
            RunMode::ComputeSize,
            Size::NONE,
            Size { width: AvailableSpace::MinContent, height: AvailableSpace::MaxContent },
        );
        cache.store(&min_content, LayoutOutput::from_outer_size(Size { width: 30.0, height: 60.0 }));
        let zero = input(
            RunMode::ComputeSize,
            Size::NONE,
            Size { width: AvailableSpace::Definite(0.0), height: AvailableSpace::MaxContent },
        );
        assert!(cache.get_yoga(&zero, &params(false)).is_some());
    }
}
