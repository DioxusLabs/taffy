//! Computes size using styles and measure functions

use crate::geometry::{Point, Size};
use crate::layout::{SizeAndBaselines, SizingMode};
use crate::math::MaybeMath;
use crate::node::{Node, Taffy};
use crate::resolve::{MaybeResolve, ResolveOrZero};
use crate::style::AvailableSpace;
use crate::sys::f32_max;

#[cfg(feature = "debug")]
use crate::debug::NODE_LOGGER;

/// Compute the size of the node given the specified constraints
#[inline(always)]
pub(crate) fn perform_layout(
    tree: &mut Taffy,
    node: Node,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    sizing_mode: SizingMode,
) -> SizeAndBaselines {
    compute(tree, node, known_dimensions, parent_size, available_space, sizing_mode)
}

/// Perform a full layout on the node given the specified constraints
#[inline(always)]
pub(crate) fn measure_size(
    tree: &mut Taffy,
    node: Node,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    sizing_mode: SizingMode,
) -> Size<f32> {
    compute(tree, node, known_dimensions, parent_size, available_space, sizing_mode).size
}

/// Compute the size of a leaf node (node with no children)
pub(crate) fn compute(
    tree: &mut Taffy,
    node: Node,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    sizing_mode: SizingMode,
) -> SizeAndBaselines {
    let style = &tree.nodes[node].style;

    // Resolve node's preferred/min/max sizes (width/heights) against the available space (percentages resolve to pixel values)
    // For ContentSize mode, we pretend that the node has no size styles as these should be ignored.
    let (node_size, node_min_size, node_max_size, aspect_ratio) = match sizing_mode {
        SizingMode::ContentSize => {
            let node_size = known_dimensions;
            let node_min_size = Size::NONE;
            let node_max_size = Size::NONE;
            (node_size, node_min_size, node_max_size, None)
        }
        SizingMode::InherentSize => {
            let aspect_ratio = style.aspect_ratio;
            let style_size = style.size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);
            let style_min_size = style.min_size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);
            let style_max_size = style.max_size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);

            let node_size = known_dimensions.or(style_size);
            (node_size, style_min_size, style_max_size, aspect_ratio)
        }
    };

    #[cfg(feature = "debug")]
    NODE_LOGGER.log("LEAF");
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("node_size", node_size);
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("min_size ", node_min_size);
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("max_size ", node_max_size);

    // Return early if both width and height are known
    if let Size { width: Some(width), height: Some(height) } = node_size {
        let size = Size { width, height }.maybe_clamp(node_min_size, node_max_size);
        return SizeAndBaselines { size, first_baselines: Point::NONE };
    };

    if let Some(measure_func) = tree.measure_funcs.get_mut(node) {
        // Compute available space
        let available_space = Size {
            width: available_space
                .width
                .maybe_set(node_size.width)
                .maybe_set(node_max_size.width)
                .map_definite_value(|size| size.maybe_clamp(node_min_size.width, node_max_size.width)),
            height: available_space
                .height
                .maybe_set(node_size.height)
                .maybe_set(node_max_size.height)
                .map_definite_value(|size| size.maybe_clamp(node_min_size.height, node_max_size.height)),
        };

        // Measure node
        let measured_size = measure_func.measure(known_dimensions, available_space);

        let measured_size = Size {
            width: measured_size.width,
            height: f32_max(measured_size.height, aspect_ratio.map(|ratio| measured_size.width / ratio).unwrap_or(0.0)),
        };

        let size = node_size.unwrap_or(measured_size).maybe_clamp(node_min_size, node_max_size);
        return SizeAndBaselines { size, first_baselines: Point::NONE };
    }

    // Note: both horizontal and vertical percentage padding/borders are resolved against the container's inline size (i.e. width).
    // This is not a bug, but is how CSS is specified (see: https://developer.mozilla.org/en-US/docs/Web/CSS/padding#values)
    let padding = style.padding.resolve_or_zero(parent_size.width);
    let border = style.border.resolve_or_zero(parent_size.width);

    let size = Size {
        width: node_size
            .width
            // .unwrap_or(0.0) + padding.horizontal_axis_sum() + border.horizontal_axis_sum(), // content-box
            .unwrap_or(0.0 + padding.horizontal_axis_sum() + border.horizontal_axis_sum()) // border-box
            .maybe_clamp(node_min_size.width, node_max_size.width),
        height: node_size
            .height
            // .unwrap_or(0.0) + padding.vertical_axis_sum() + border.vertical_axis_sum(), // content-box
            .unwrap_or(0.0 + padding.vertical_axis_sum() + border.vertical_axis_sum()) // border-box
            .maybe_clamp(node_min_size.height, node_max_size.height),
    };
    SizeAndBaselines { size, first_baselines: Point::NONE }
}
