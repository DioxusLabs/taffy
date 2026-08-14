//! Shared logic for CSS size containment (`contain: size` and `contain: inline-size`)
//! <https://drafts.csswg.org/css-contain-2/#containment-size>
use crate::geometry::Size;
use crate::style::Contain;
use crate::tree::{LayoutInput, LayoutOutput, NodeId, RunMode};
use crate::RequestedAxis;

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
///      fixed-size box by running the algorithm again with the contained size passed as definite
///      `known_dimensions`.
///
/// The phase 2 pass is skipped when only the box's size was requested (`RunMode::ComputeSize`),
/// and the phase 1 pass is skipped for axes whose size is already known.
pub(crate) fn compute_contained_size_layout<Tree, ComputeInner>(
    tree: &mut Tree,
    node: NodeId,
    inputs: LayoutInput,
    contain: Contain,
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
            LayoutInput {
                known_dimensions: size.map(Some),
                known_dimensions_are_definite: Size { width: true, height: true },
                ..inputs
            },
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
                    width: true,
                    height: inputs.known_dimensions_are_definite.height,
                },
                ..inputs
            },
            false,
        )
    }
}
