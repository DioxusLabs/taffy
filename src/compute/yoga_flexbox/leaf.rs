//! Leaf measurement with Yoga's semantics.
//!
//! Ported from `measureNodeWithMeasureFunc`, `measureNodeWithoutChildren` and `boundAxis` in
//! `yoga/algorithm/CalculateLayout.cpp` / `BoundAxis.h` at Yoga commit
//! `08250431e09bc4b864917e51f9e9d52998ee2dce`.

use crate::geometry::Size;
use crate::style::AvailableSpace;
use crate::tree::{Baselines, CollapsibleMarginSet, LayoutInput, LayoutOutput};
use crate::util::sys::f32_max;
use crate::util::{MaybeMath, MaybeResolve, ResolveOrZero};
use crate::{BoxSizing, CoreStyle};

use super::config::YogaConfig;
use super::enums::YogaSizingMode;

/// One axis of a Yoga leaf's constraints: the sizing mode and the available size it applies to.
#[derive(Copy, Clone)]
struct AxisInput {
    /// Yoga's sizing mode for the axis
    mode: YogaSizingMode,
    /// The known dimension (`StretchFit`) or available space (`FitContent`); unused for `MaxContent`
    available: f32,
}

impl AxisInput {
    /// Translate Taffy's constraints for one axis into Yoga's sizing modes.
    ///
    /// A known dimension is `StretchFit`, definite available space is `FitContent`, `MaxContent` is
    /// `MaxContent`, and `MinContent` (which Yoga has no mode for) becomes `FitContent` with zero
    /// available space, the probe Yoga itself uses for min-content measurement.
    fn new(known: Option<f32>, available: AvailableSpace) -> Self {
        match (known, available) {
            (Some(size), _) => Self { mode: YogaSizingMode::StretchFit, available: size },
            (None, AvailableSpace::Definite(size)) => Self { mode: YogaSizingMode::FitContent, available: size },
            (None, AvailableSpace::MaxContent) => Self { mode: YogaSizingMode::MaxContent, available: f32::NAN },
            (None, AvailableSpace::MinContent) => Self { mode: YogaSizingMode::FitContent, available: 0.0 },
        }
    }

    /// The constraints to hand to the measure function (Yoga clamps negative available space to zero
    /// before calling the measure callback).
    fn to_taffy(self, padding_border: f32) -> (Option<f32>, AvailableSpace) {
        match self.mode {
            // Yoga measures with `max(0, available - padding_border)` inside; the host's leaf
            // measurement subtracts padding and border itself, so clamp the outer size to them.
            YogaSizingMode::StretchFit => {
                let outer = f32_max(self.available, padding_border);
                (Some(outer), AvailableSpace::Definite(outer))
            }
            YogaSizingMode::FitContent => (None, AvailableSpace::Definite(f32_max(self.available, 0.0))),
            YogaSizingMode::MaxContent => (None, AvailableSpace::MaxContent),
        }
    }
}

/// Yoga's `boundAxis`: clamp to the min/max constraints, then to at least padding + border.
///
/// Like Yoga, a constraint only applies when it is defined and non-negative.
fn bound_axis(value: f32, min: Option<f32>, max: Option<f32>, padding_border: f32) -> f32 {
    let mut bounded = value;
    if let Some(max) = max {
        if max >= 0.0 && bounded > max {
            bounded = max;
        }
    }
    if let Some(min) = min {
        if min >= 0.0 && bounded < min {
            bounded = min;
        }
    }
    f32_max(bounded, padding_border)
}

/// Compute the layout of a `Display::YogaFlexbox` node with no children, using Yoga's leaf rules.
///
/// `measure` is the host's ordinary leaf measurement (a measure function wrapped by
/// [`compute_leaf_layout`](crate::compute::compute_leaf_layout) in `TaffyTree`). It is called with
/// Yoga's translated constraints and is skipped entirely when both axes are `StretchFit`, exactly
/// as Yoga skips its measure callback; the result is then bounded with Yoga's `boundAxis`.
pub fn compute_yoga_leaf_layout(
    inputs: LayoutInput,
    style: &impl CoreStyle,
    _config: &YogaConfig,
    resolve_calc_value: impl Fn(*const (), f32) -> f32,
    measure: impl FnOnce(LayoutInput) -> LayoutOutput,
) -> LayoutOutput {
    let parent_size = inputs.parent_size;
    let padding = style.padding().resolve_or_zero(parent_size.width, &resolve_calc_value);
    let border = style.border().resolve_or_zero(parent_size.width, &resolve_calc_value);
    let padding_border = (padding + border).sum_axes();

    let box_sizing_adjustment = if style.box_sizing() == BoxSizing::ContentBox { padding_border } else { Size::ZERO };
    let min_size = style.min_size().maybe_resolve(parent_size, &resolve_calc_value).maybe_add(box_sizing_adjustment);
    let max_size = style.max_size().maybe_resolve(parent_size, &resolve_calc_value).maybe_add(box_sizing_adjustment);

    let width = AxisInput::new(inputs.known_dimensions.width, inputs.available_space.width);
    let height = AxisInput::new(inputs.known_dimensions.height, inputs.available_space.height);

    let bound = |axis: AxisInput, measured: f32, min: Option<f32>, max: Option<f32>, pb: f32| {
        let value = match axis.mode {
            YogaSizingMode::StretchFit => axis.available,
            YogaSizingMode::FitContent | YogaSizingMode::MaxContent => measured,
        };
        bound_axis(value, min, max, pb)
    };

    let both_exact = width.mode == YogaSizingMode::StretchFit && height.mode == YogaSizingMode::StretchFit;
    let (measured, mut output) = if both_exact {
        (Size::ZERO, LayoutOutput::HIDDEN)
    } else {
        let (known_width, available_width) = width.to_taffy(padding_border.width);
        let (known_height, available_height) = height.to_taffy(padding_border.height);
        let output = measure(LayoutInput {
            known_dimensions: Size { width: known_width, height: known_height },
            available_space: Size { width: available_width, height: available_height },
            ..inputs
        });
        (output.size, output)
    };

    output.size = Size {
        width: bound(width, measured.width, min_size.width, max_size.width, padding_border.width),
        height: bound(height, measured.height, min_size.height, max_size.height, padding_border.height),
    };
    if both_exact {
        output.baselines = Baselines::NONE;
    }
    output.top_margin = CollapsibleMarginSet::ZERO;
    output.bottom_margin = CollapsibleMarginSet::ZERO;
    output.margins_can_collapse_through = false;
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis_input_translates_taffy_constraints() {
        let exact = AxisInput::new(Some(40.0), AvailableSpace::Definite(100.0));
        assert_eq!(exact.mode, YogaSizingMode::StretchFit);
        assert_eq!(exact.to_taffy(0.0), (Some(40.0), AvailableSpace::Definite(40.0)));
        assert_eq!(exact.to_taffy(50.0), (Some(50.0), AvailableSpace::Definite(50.0)));

        let fit = AxisInput::new(None, AvailableSpace::Definite(-5.0));
        assert_eq!(fit.mode, YogaSizingMode::FitContent);
        assert_eq!(fit.to_taffy(0.0), (None, AvailableSpace::Definite(0.0)));

        let max = AxisInput::new(None, AvailableSpace::MaxContent);
        assert_eq!(max.mode, YogaSizingMode::MaxContent);
        assert_eq!(max.to_taffy(0.0), (None, AvailableSpace::MaxContent));

        let min = AxisInput::new(None, AvailableSpace::MinContent);
        assert_eq!(min.mode, YogaSizingMode::FitContent);
        assert_eq!(min.to_taffy(0.0), (None, AvailableSpace::Definite(0.0)));
    }

    #[test]
    fn bound_axis_matches_yoga() {
        assert_eq!(bound_axis(50.0, None, None, 10.0), 50.0);
        assert_eq!(bound_axis(5.0, None, None, 10.0), 10.0);
        assert_eq!(bound_axis(50.0, Some(60.0), None, 0.0), 60.0);
        assert_eq!(bound_axis(50.0, None, Some(40.0), 0.0), 40.0);
        assert_eq!(bound_axis(50.0, Some(60.0), Some(40.0), 0.0), 60.0);
        assert_eq!(bound_axis(50.0, Some(-1.0), Some(-1.0), 0.0), 50.0);
        assert_eq!(bound_axis(2.0, None, Some(1.0), 8.0), 8.0);
    }
}
