//! Computation specific for the default `Taffy` tree implementation

use crate::compute::{leaf, LayoutAlgorithm};
use crate::geometry::{Line, Point, Size};
use crate::style::{AvailableSpace, Display};
use crate::tree::{
    Layout, LayoutTree, Measurable, NodeId, RunMode, SizeBaselinesAndMargins, SizingMode, Taffy, TaffyError, TaffyView,
};
use crate::util::sys::round;

#[cfg(feature = "block_layout")]
use crate::compute::BlockAlgorithm;

#[cfg(feature = "flexbox")]
use crate::compute::FlexboxAlgorithm;

#[cfg(feature = "grid")]
use crate::compute::CssGridAlgorithm;

#[cfg(any(feature = "debug", feature = "profile"))]
use crate::util::debug::NODE_LOGGER;

#[cfg(feature = "debug")]
fn debug_log_node(
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
    sizing_mode: SizingMode,
) {
    NODE_LOGGER.debug_log(run_mode);
    NODE_LOGGER.labelled_debug_log("sizing_mode", sizing_mode);
    NODE_LOGGER.labelled_debug_log("known_dimensions", known_dimensions);
    NODE_LOGGER.labelled_debug_log("parent_size", parent_size);
    NODE_LOGGER.labelled_debug_log("available_space", available_space);
}

/// Updates the stored layout of the provided `node` and its children
pub(crate) fn compute_layout<Measure: Measurable>(
    taffy_view: &mut TaffyView<Measure>,
    root: NodeId,
    available_space: Size<AvailableSpace>,
) -> Result<(), TaffyError> {
    // Recursively compute node layout
    let size_and_baselines = perform_node_layout(
        taffy_view,
        root,
        Size::NONE,
        available_space.into_options(),
        available_space,
        SizingMode::InherentSize,
        Line::FALSE,
    );

    let layout = Layout { order: 0, size: size_and_baselines.size, location: Point::ZERO };
    *taffy_view.layout_mut(root) = layout;

    // If rounding is enabled, recursively round the layout's of this node and all children
    if taffy_view.taffy.config.use_rounding {
        round_layout(taffy_view.taffy, root, 0.0, 0.0);
    }

    Ok(())
}

/// Perform full layout on a node. Chooses which algorithm to use based on the `display` property.
pub(crate) fn perform_node_layout<Measure: Measurable>(
    taffy_view: &mut TaffyView<Measure>,
    node: NodeId,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    sizing_mode: SizingMode,
    vertical_margins_are_collapsible: Line<bool>,
) -> SizeBaselinesAndMargins {
    compute_node_layout(
        taffy_view,
        node,
        known_dimensions,
        parent_size,
        available_space,
        RunMode::PerformLayout,
        sizing_mode,
        vertical_margins_are_collapsible,
    )
}

/// Measure a node's size. Chooses which algorithm to use based on the `display` property.
pub(crate) fn measure_node_size<Measure: Measurable>(
    taffy_view: &mut TaffyView<Measure>,
    node: NodeId,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    sizing_mode: SizingMode,
    vertical_margins_are_collapsible: Line<bool>,
) -> Size<f32> {
    compute_node_layout(
        taffy_view,
        node,
        known_dimensions,
        parent_size,
        available_space,
        RunMode::ComputeSize,
        sizing_mode,
        vertical_margins_are_collapsible,
    )
    .size
}

/// Updates the stored layout of the provided `node` and its children
#[allow(clippy::too_many_arguments)]
fn compute_node_layout<Measure: Measurable>(
    taffy_view: &mut TaffyView<Measure>,
    node: NodeId,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
    sizing_mode: SizingMode,
    vertical_margins_are_collapsible: Line<bool>,
) -> SizeBaselinesAndMargins {
    #[cfg(any(feature = "debug", feature = "profile"))]
    NODE_LOGGER.push_node(node);
    #[cfg(feature = "debug")]
    println!();

    let node_key = node.into();
    let has_children = !taffy_view.taffy.children[node_key].is_empty();

    // First we check if we have a cached result for the given input
    let cache_run_mode = if !has_children { RunMode::PerformLayout } else { run_mode };
    if let Some(cached_size_and_baselines) =
        taffy_view.taffy.nodes[node_key].cache.get(known_dimensions, available_space, cache_run_mode)
    {
        #[cfg(feature = "debug")]
        NODE_LOGGER.labelled_debug_log("CACHE", cached_size_and_baselines.size);
        #[cfg(feature = "debug")]
        debug_log_node(known_dimensions, parent_size, available_space, run_mode, sizing_mode);
        #[cfg(any(feature = "debug", feature = "profile"))]
        NODE_LOGGER.pop_node();

        return cached_size_and_baselines;
    }

    #[cfg(feature = "debug")]
    debug_log_node(known_dimensions, parent_size, available_space, run_mode, sizing_mode);

    /// Inlined function generic over the LayoutAlgorithm to reduce code duplication
    #[inline(always)]
    fn perform_computations<Algorithm: LayoutAlgorithm>(
        tree: &mut impl LayoutTree,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: RunMode,
        sizing_mode: SizingMode,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> SizeBaselinesAndMargins {
        #[cfg(feature = "debug")]
        NODE_LOGGER.log(Algorithm::NAME);

        match run_mode {
            RunMode::PerformLayout => Algorithm::perform_layout(
                tree,
                node,
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                vertical_margins_are_collapsible,
            ),
            RunMode::ComputeSize => Algorithm::measure_size(
                tree,
                node,
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                vertical_margins_are_collapsible,
            )
            .into(),
        }
    }

    let display_mode = taffy_view.taffy.nodes[node_key].style.display;
    let computed_size_and_baselines = match (display_mode, has_children) {
        (Display::None, _) => {
            perform_taffy_tree_hidden_layout(taffy_view.taffy, node);
            SizeBaselinesAndMargins::HIDDEN
        }
        #[cfg(feature = "block_layout")]
        (Display::Block, true) => perform_computations::<BlockAlgorithm>(
            taffy_view,
            node,
            known_dimensions,
            parent_size,
            available_space,
            run_mode,
            sizing_mode,
            vertical_margins_are_collapsible,
        ),
        #[cfg(feature = "flexbox")]
        (Display::Flex, true) => perform_computations::<FlexboxAlgorithm>(
            taffy_view,
            node,
            known_dimensions,
            parent_size,
            available_space,
            run_mode,
            sizing_mode,
            vertical_margins_are_collapsible,
        ),
        #[cfg(feature = "grid")]
        (Display::Grid, true) => perform_computations::<CssGridAlgorithm>(
            taffy_view,
            node,
            known_dimensions,
            parent_size,
            available_space,
            run_mode,
            sizing_mode,
            vertical_margins_are_collapsible,
        ),
        (_, false) => match run_mode {
            RunMode::PerformLayout => leaf::perform_layout(
                &taffy_view.taffy.nodes[node_key].style,
                taffy_view.taffy.nodes[node_key].needs_measure.then(|| &mut taffy_view.taffy.measure_funcs[node_key]),
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                &mut taffy_view.context,
            ),
            RunMode::ComputeSize => leaf::measure_size(
                &taffy_view.taffy.nodes[node_key].style,
                taffy_view.taffy.nodes[node_key].needs_measure.then(|| &mut taffy_view.taffy.measure_funcs[node_key]),
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                &mut taffy_view.context,
            )
            .into(),
        },
    };

    // Cache result
    taffy_view.taffy.nodes[node_key].cache.store(
        known_dimensions,
        available_space,
        cache_run_mode,
        computed_size_and_baselines,
    );

    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("RESULT", computed_size_and_baselines.size);
    #[cfg(any(feature = "debug", feature = "profile"))]
    NODE_LOGGER.pop_node();

    computed_size_and_baselines
}

/// Creates a layout for this node and its children, recursively.
/// Each hidden node has zero size and is placed at the origin
fn perform_taffy_tree_hidden_layout<Measure: Measurable>(tree: &mut Taffy<Measure>, node: NodeId) {
    /// Recursive function to apply hidden layout to all descendents
    fn perform_hidden_layout_inner<Measure: Measurable>(tree: &mut Taffy<Measure>, node: NodeId, order: u32) {
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
fn round_layout<Measure: Measurable>(tree: &mut Taffy<Measure>, node_id: NodeId, cumulative_x: f32, cumulative_y: f32) {
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
