//! Shared logic for CSS size containment (`contain: size` and `contain: inline-size`)
//! <https://drafts.csswg.org/css-contain-2/#containment-size>
use crate::geometry::Size;
use crate::style::{Contain, CoreStyle};
use crate::tree::{LayoutInput, LayoutOutput, NodeId, RunMode, SizingMode};
use crate::util::MaybeResolve;
use crate::RequestedAxis;

/// Determine, from a node's own style and sizing constraints, whether the node's size is definite
/// in each axis for the purposes of resolving its children's percentage sizes.
///
/// Size containment allows a used size to be computed for an `auto` sized axis (by sizing the box
/// as if it were empty), but that does not make the axis definite: percentages resolved against it
/// still behave as they would against an indefinite size.
/// <https://github.com/w3c/csswg-drafts/issues/7206>
pub(crate) fn contained_size_is_definite(
    style: &impl CoreStyle,
    inputs: &LayoutInput,
    calc: impl Fn(*const (), f32) -> f32 + Copy,
) -> Size<bool> {
    let parent_size = inputs.parent_size;
    let aspect_ratio = style.aspect_ratio();

    let style_size = if inputs.sizing_mode == SizingMode::InherentSize {
        style.size().maybe_resolve(parent_size, calc).maybe_apply_aspect_ratio(aspect_ratio)
    } else {
        Size::NONE
    };
    let min_size = style.min_size().maybe_resolve(parent_size, calc).maybe_apply_aspect_ratio(aspect_ratio);
    let max_size = style.max_size().maybe_resolve(parent_size, calc).maybe_apply_aspect_ratio(aspect_ratio);

    // If both min and max in a given axis are set and max <= min then this determines the size in that axis
    let min_max_definite_size = min_size.zip_map(max_size, |min, max| match (min, max) {
        (Some(min), Some(max)) if max <= min => Some(min),
        _ => None,
    });
    let styled_size_is_definite = style_size.or(min_max_definite_size).map(|size| size.is_some());

    // Sizes imposed by the parent keep the parent's definiteness; otherwise definiteness is
    // determined by whether the node's own style resolves to a definite size.
    Size {
        width: match inputs.known_dimensions.width {
            Some(_) => inputs.known_dimensions_are_definite.width,
            None => styled_size_is_definite.width,
        },
        height: match inputs.known_dimensions.height {
            Some(_) => inputs.known_dimensions_are_definite.height,
            None => styled_size_is_definite.height,
        },
    }
}

/// Compute the layout of a node with size containment (`contain: size` or `contain: inline-size`)
/// in one or both axes.
///
/// Size containment is implemented in two phases:
///
///   1. **Sizing as if empty**: the size of the box in the contained axes is determined by running
///      the node's normal layout algorithm with child enumeration suppressed (the `bool` parameter
///      of `compute_inner`). Running the real algorithm (rather than treating the node as a leaf)
///      means that padding/border, explicit grid tracks, aspect-ratio, min/max clamping etc. all
///      still apply.
///   2. **Laying out in place**: the node's contents are then laid out normally into the resulting
///      fixed-size box by running the algorithm again with the contained size passed as
///      `known_dimensions`. An axis is only marked *definite* if it was already definite from the
///      node's sizing constraints (`size_is_definite`): a used size computed for an `auto` axis by
///      the as-if-empty pass is not definite for percentage resolution purposes.
///
/// The phase 2 pass is skipped when only the box's size was requested (`RunMode::ComputeSize`),
/// and the phase 1 pass is skipped for axes whose size is already known.
pub(crate) fn compute_contained_size_layout<Tree, ComputeInner>(
    tree: &mut Tree,
    node: NodeId,
    inputs: LayoutInput,
    contain: Contain,
    size_is_definite: Size<bool>,
    mut compute_inner: ComputeInner,
) -> LayoutOutput
where
    ComputeInner: FnMut(&mut Tree, NodeId, LayoutInput, bool) -> LayoutOutput,
{
    if contain.contains(Contain::SIZE) {
        // Phase 1: determine the box's size as if it were empty
        let known_dimensions = inputs.known_dimensions;
        let size = if let Size { width: Some(width), height: Some(height) } = known_dimensions {
            Size { width, height }
        } else {
            let empty_size = compute_inner(
                tree,
                node,
                LayoutInput { run_mode: RunMode::ComputeSize, axis: RequestedAxis::Both, ..inputs },
                true,
            )
            .size;
            Size {
                width: known_dimensions.width.unwrap_or(empty_size.width),
                height: known_dimensions.height.unwrap_or(empty_size.height),
            }
        };

        if inputs.run_mode == RunMode::ComputeSize {
            return LayoutOutput::from_outer_size(size);
        }

        // Phase 2: lay the box's contents out into the fixed-size box
        let mut output = compute_inner(
            tree,
            node,
            LayoutInput { known_dimensions: size.map(Some), known_dimensions_are_definite: size_is_definite, ..inputs },
            false,
        );
        output.size = size;
        output
    } else {
        // Inline-size containment. Taffy only supports the `horizontal-tb` writing mode, so the
        // inline axis is always the horizontal axis.

        // Phase 1: determine the box's width as if it were empty
        let width = match inputs.known_dimensions.width {
            Some(width) => width,
            None => {
                compute_inner(
                    tree,
                    node,
                    LayoutInput { run_mode: RunMode::ComputeSize, axis: RequestedAxis::Horizontal, ..inputs },
                    true,
                )
                .size
                .width
            }
        };

        if inputs.run_mode == RunMode::ComputeSize && inputs.axis == RequestedAxis::Horizontal {
            return LayoutOutput::from_outer_size(Size { width, height: 0.0 });
        }

        // Phase 2: lay the box's contents out normally with the width fixed. The height
        // continues to depend on the box's contents.
        compute_inner(
            tree,
            node,
            LayoutInput {
                known_dimensions: Size { width: Some(width), height: inputs.known_dimensions.height },
                known_dimensions_are_definite: Size {
                    width: size_is_definite.width,
                    height: inputs.known_dimensions_are_definite.height,
                },
                ..inputs
            },
            false,
        )
    }
}
