//! Computation specific for the default `Taffy` tree implementation
use crate::compute::leaf::compute_leaf_layout;
use crate::geometry::{Line, Point, Size};
use crate::style::{AvailableSpace, Display};
use crate::tree::{
    Layout, LayoutInput, LayoutOutput, LayoutTree, NodeId, RunMode, SizingMode, Taffy, TaffyError, TaffyView,
};
use crate::util::debug::{debug_log, debug_log_node, debug_pop_node, debug_push_node};
use crate::util::sys::round;

#[cfg(feature = "block_layout")]
use crate::compute::compute_block_layout;
#[cfg(feature = "flexbox")]
use crate::compute::compute_flexbox_layout;
#[cfg(feature = "grid")]
use crate::compute::compute_grid_layout;

/// Updates the stored layout of the provided `node` and its children
pub(crate) fn compute_layout<NodeContext, MeasureFunction>(
    taffy_view: &mut TaffyView<NodeContext, MeasureFunction>,
    root: NodeId,
    available_space: Size<AvailableSpace>,
) -> Result<(), TaffyError>
where
    MeasureFunction: FnMut(Size<Option<f32>>, Size<AvailableSpace>, NodeId, Option<&mut NodeContext>) -> Size<f32>,
{
    // Recursively compute node layout
    let size_and_baselines = compute_node_layout(
        taffy_view,
        root,
        LayoutInput {
            known_dimensions: Size::NONE,
            parent_size: available_space.into_options(),
            available_space,
            sizing_mode: SizingMode::InherentSize,
            run_mode: RunMode::PerformLayout,
            vertical_margins_are_collapsible: Line::FALSE,
        },
    );

    let layout = Layout { order: 0, size: size_and_baselines.size, location: Point::ZERO };
    *taffy_view.layout_mut(root) = layout;

    // If rounding is enabled, recursively round the layout's of this node and all children
    if taffy_view.taffy.config.use_rounding {
        round_layout(taffy_view.taffy, root, 0.0, 0.0);
    }

    Ok(())
}

/// Updates the stored layout of the provided `node` and its children
pub(crate) fn compute_node_layout<NodeContext, MeasureFunction>(
    taffy_view: &mut TaffyView<NodeContext, MeasureFunction>,
    node: NodeId,
    inputs: LayoutInput,
) -> LayoutOutput
where
    MeasureFunction: FnMut(Size<Option<f32>>, Size<AvailableSpace>, NodeId, Option<&mut NodeContext>) -> Size<f32>,
{
    debug_push_node!(node);

    let LayoutInput { known_dimensions, available_space, run_mode, .. } = inputs;
    let node_key = node.into();
    let has_children = !taffy_view.taffy.children[node_key].is_empty();

    // First we check if we have a cached result for the given input
    let cache_run_mode = if !has_children { RunMode::PerformLayout } else { run_mode };
    let cache = &taffy_view.taffy.nodes[node_key].cache;
    let cache_entry = cache.get(known_dimensions, available_space, cache_run_mode);
    if let Some(cached_size_and_baselines) = cache_entry {
        debug_log!("CACHE", dbg:cached_size_and_baselines.size);
        debug_log_node!(known_dimensions, parent_size, available_space, run_mode, sizing_mode);
        debug_pop_node!();
        return cached_size_and_baselines;
    }

    // Fetch `Display` style (+ debug log)
    let display_mode = taffy_view.taffy.nodes[node_key].style.display;
    debug_log!(display_mode);
    debug_log_node!(known_dimensions, parent_size, available_space, run_mode, sizing_mode);

    let computed_size_and_baselines = match (display_mode, has_children) {
        (Display::None, _) => perform_taffy_tree_hidden_layout(taffy_view.taffy, node),
        #[cfg(feature = "block_layout")]
        (Display::Block, true) => compute_block_layout(taffy_view, node, inputs),
        #[cfg(feature = "flexbox")]
        (Display::Flex, true) => compute_flexbox_layout(taffy_view, node, inputs),
        #[cfg(feature = "grid")]
        (Display::Grid, true) => compute_grid_layout(taffy_view, node, inputs),
        (_, false) => compute_leaf_layout(
            &mut taffy_view.measure_function,
            node,
            inputs,
            &taffy_view.taffy.nodes[node_key].style,
            taffy_view.taffy.nodes[node_key].needs_measure.then(|| &mut taffy_view.taffy.node_context_data[node_key]),
        ),
    };

    // Cache result
    taffy_view.taffy.nodes[node_key].cache.store(
        known_dimensions,
        available_space,
        cache_run_mode,
        computed_size_and_baselines,
    );

    debug_log!("RESULT", dbg:computed_size_and_baselines.size);
    debug_pop_node!();

    computed_size_and_baselines
}

/// Creates a layout for this node and its children, recursively.
/// Each hidden node has zero size and is placed at the origin
fn perform_taffy_tree_hidden_layout<NodeContext>(tree: &mut Taffy<NodeContext>, node: NodeId) -> LayoutOutput {
    /// Recursive function to apply hidden layout to all descendents
    fn perform_hidden_layout_inner<NodeContext>(tree: &mut Taffy<NodeContext>, node: NodeId, order: u32) {
        let node_key = node.into();
        tree.nodes[node_key].unrounded_layout = Layout::with_order(order);
        tree.nodes[node_key].final_layout = Layout::with_order(order);
        tree.nodes[node_key].cache.clear();
        for order in 0..tree.children[node_key].len() {
            perform_hidden_layout_inner(tree, tree.children[node_key][order], order as _);
        }
    }

    let node_key = node.into();
    for order in 0..tree.children[node.into()].len() {
        perform_hidden_layout_inner(tree, tree.children[node_key][order], order as _);
    }

    LayoutOutput::HIDDEN
}

/// Rounds the calculated [`Layout`] to exact pixel values
///
/// In order to ensure that no gaps in the layout are introduced we:
///   - Always round based on the cumulative x/y coordinates (relative to the viewport) rather than
///     parent-relative coordinates
///   - Compute width/height by first rounding the top/bottom/left/right and then computing the difference
///     rather than rounding the width/height directly
/// See <https://github.com/facebook/yoga/commit/aa5b296ac78f7a22e1aeaf4891243c6bb76488e2> for more context
///
/// In order to prevent innacuracies caused by rounding already-rounded values, we read from `unrounded_layout`
/// and write to `final_layout`.
fn round_layout<NodeContext>(tree: &mut Taffy<NodeContext>, node_id: NodeId, cumulative_x: f32, cumulative_y: f32) {
    let node = &mut tree.nodes[node_id.into()];
    let unrounded_layout = node.unrounded_layout;
    let layout = &mut node.final_layout;

    let cumulative_x = cumulative_x + unrounded_layout.location.x;
    let cumulative_y = cumulative_y + unrounded_layout.location.y;

    layout.location.x = round(unrounded_layout.location.x);
    layout.location.y = round(unrounded_layout.location.y);
    layout.size.width = round(cumulative_x + unrounded_layout.size.width) - round(cumulative_x);
    layout.size.height = round(cumulative_y + unrounded_layout.size.height) - round(cumulative_y);

    let child_count = tree.child_count(node_id).unwrap();
    for index in 0..child_count {
        let child = tree.children[node_id.into()][index];
        round_layout(tree, child, cumulative_x, cumulative_y);
    }
}
