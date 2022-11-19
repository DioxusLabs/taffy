use crate::debug::NODE_LOGGER;
use crate::geometry::Size;
use crate::layout::{AvailableSpace, Cache, RunMode, SizingMode};
use crate::math::MaybeMath;
use crate::node::Node;
use crate::resolve::{MaybeResolve, ResolveOrDefault};
use crate::style::Dimension;
use crate::tree::LayoutTree;

// Define some general constants we will need for the remainder of the algorithm.
// let mut constants = compute_constants(tree.style(node), node_size, available_space);

pub(crate) fn compute(
    tree: &mut impl LayoutTree,
    node: Node,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
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
            let node_size =
                style_size.zip_map(known_dimensions, |style_size, known_dimensions| known_dimensions.or(style_size));
            let node_min_size = style.min_size.maybe_resolve(available_space.as_options());
            let node_max_size = style.max_size.maybe_resolve(available_space.as_options());
            (node_size, node_min_size, node_max_size)
        }
    };

    // NODE_LOGGER.log("LEAF");
    // NODE_LOGGER.debug_llog("node_size", node_size);
    // NODE_LOGGER.debug_llog("min_size ", node_min_size);
    // NODE_LOGGER.debug_llog("max_size ", node_max_size);

    if node_size.width.is_some() && node_size.height.is_some() {
        return Size {
            width: node_size.width.maybe_max(node_min_size.width).maybe_min(node_max_size.width).unwrap_or(0.0),
            height: node_size.height.maybe_max(node_min_size.height).maybe_min(node_max_size.height).unwrap_or(0.0),
        };
    };

    if tree.needs_measure(node) {
        // Compute available space
        let available_space = Size {
            width: available_space.width.maybe_set(node_size.width),
            height: available_space.height.maybe_set(node_size.height),
        };

        // Measure node
        let measured_size = tree.measure_node(node, known_dimensions, available_space);

        let clamped_measured_size = Size {
            width: node_size
                .width
                .unwrap_or(measured_size.width)
                .maybe_max(node_min_size.width)
                .maybe_min(node_max_size.width),
            height: node_size
                .height
                .unwrap_or(measured_size.height)
                .maybe_max(node_min_size.height)
                .maybe_min(node_max_size.height),
        };

        return clamped_measured_size;
    }

    let padding = style.padding.resolve_or_default(available_space.width.as_option());
    let border = style.border.resolve_or_default(available_space.width.as_option());
    return Size {
        width: node_size
            .width
            // .unwrap_or(0.0) + padding.horizontal_axis_sum() + border.horizontal_axis_sum(), // content-box
            .unwrap_or(0.0 + padding.horizontal_axis_sum() + border.horizontal_axis_sum()) // border-box
            .maybe_max(node_min_size.width)
            .maybe_min(node_max_size.width),
        height: node_size
            .height
            // .unwrap_or(0.0) + padding.horizontal_axis_sum() + border.horizontal_axis_sum(), // content-box
            .unwrap_or(0.0 + padding.horizontal_axis_sum() + border.horizontal_axis_sum()) // border-box
            .maybe_max(node_min_size.height)
            .maybe_min(node_max_size.height),
    };
}

fn maybe_clamp(
    size: Size<Option<f32>>,
    min_size: Size<Option<f32>>,
    max_size: Size<Option<f32>>,
    sizing_mode: SizingMode,
) -> Size<Option<f32>> {
    match sizing_mode {
        SizingMode::ContentSize => size,
        SizingMode::InherentSize => Size {
            width: size.width.maybe_max(min_size.width).maybe_min(max_size.width),
            height: size.height.maybe_max(min_size.height).maybe_min(max_size.height),
        },
    }
}
