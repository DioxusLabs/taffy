//! Computes size using styles and measure functions

use crate::geometry::Size;
use crate::layout::{AvailableSpace, RunMode, SizingMode};
use crate::math::MaybeMath;
use crate::node::Node;
use crate::resolve::{MaybeResolve, ResolveOrZero};
use crate::tree::LayoutTree;

#[cfg(feature = "debug")]
use crate::debug::NODE_LOGGER;

/// Compute the size of a leaf node (node with no children)
pub(crate) fn compute(
    tree: &mut impl LayoutTree,
    node: Node,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    _run_mode: RunMode,
    sizing_mode: SizingMode,
) -> Size<f32> {
    let style = tree.style(node);

    // Resolve node's preferred/min/max sizes (width/heights) against the available space (percentages resolve to pixel values)
    // For ContentSize mode, we pretend that the node has no size styles as these should be ignored.
    let (node_size, node_min_size, node_max_size) = match sizing_mode {
        SizingMode::ContentSize => {
            let node_size = known_dimensions;
            let node_min_size = Size::NONE;
            let node_max_size = Size::NONE;
            (node_size, node_min_size, node_max_size)
        }
        SizingMode::InherentSize => {
            let style_size = style.size.maybe_resolve(available_space.as_options());
            let node_size = known_dimensions.or(style_size);
            let node_min_size = style.min_size.maybe_resolve(available_space.as_options());
            let node_max_size = style.max_size.maybe_resolve(available_space.as_options());
            (node_size, node_min_size, node_max_size)
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
        return Size { width, height }.maybe_clamp(node_min_size, node_max_size);
    };

    if tree.needs_measure(node) {
        // Compute available space
        let available_space = Size {
            width: available_space.width.maybe_set(node_size.width),
            height: available_space.height.maybe_set(node_size.height),
        };

        // Measure node
        let measured_size = tree.measure_node(node, known_dimensions, available_space);

        return node_size.unwrap_or(measured_size).maybe_clamp(node_min_size, node_max_size);
    }

    let padding = style.padding.resolve_or_zero(available_space.width.into_option());
    let border = style.border.resolve_or_zero(available_space.width.into_option());

    Size {
        width: node_size
            .width
            // .unwrap_or(0.0) + padding.horizontal_axis_sum() + border.horizontal_axis_sum(), // content-box
            .unwrap_or(0.0 + padding.horizontal_axis_sum() + border.horizontal_axis_sum()) // border-box
            .maybe_clamp(node_min_size.width, node_max_size.width),
        height: node_size
            .height
            // .unwrap_or(0.0) + padding.horizontal_axis_sum() + border.horizontal_axis_sum(), // content-box
            .unwrap_or(0.0 + padding.horizontal_axis_sum() + border.horizontal_axis_sum()) // border-box
            .maybe_clamp(node_min_size.height, node_max_size.height),
    }
}
