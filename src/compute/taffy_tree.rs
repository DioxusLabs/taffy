//! Computation specific for the default `Taffy` tree implementation

use crate::compute::common::StackFrameCache;
use crate::compute::{leaf, LayoutAlgorithm};
use crate::geometry::{Line, Point, Size};
use crate::style::{AvailableSpace, Display};
use crate::tree::{Layout, LayoutTree, NodeId, RunMode, SizeBaselinesAndMargins, SizingMode, Taffy, TaffyError};
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
pub(crate) fn compute_layout(
    taffy: &mut Taffy,
    root: NodeId,
    available_space: Size<AvailableSpace>,
) -> Result<(), TaffyError> {
    // Recursively compute node layout
    let size_and_baselines = perform_node_layout(
        taffy,
        root,
        Size::NONE,
        available_space.into_options(),
        available_space,
        SizingMode::InherentSize,
        Line::FALSE,
    );

    let layout = Layout { order: 0, size: size_and_baselines.size, location: Point::ZERO };
    *taffy.layout_mut(root) = layout;

    // If rounding is enabled, recursively round the layout's of this node and all children
    if taffy.config.use_rounding {
        round_layout(taffy, root, 0.0, 0.0);
    }

    Ok(())
}

/// Perform full layout on a node. Chooses which algorithm to use based on the `display` property.
pub(crate) fn perform_node_layout(
    tree: &mut Taffy,
    node: NodeId,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    sizing_mode: SizingMode,
    vertical_margins_are_collapsible: Line<bool>,
) -> SizeBaselinesAndMargins {
    compute_node_layout(
        tree,
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
pub(crate) fn measure_node_size(
    tree: &mut Taffy,
    node: NodeId,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    sizing_mode: SizingMode,
    vertical_margins_are_collapsible: Line<bool>,
) -> Size<f32> {
    compute_node_layout(
        tree,
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
fn compute_node_layout(
    tree: &mut Taffy,
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
    let has_children = !tree.children[node_key].is_empty();

    // First we check if we have a cached result for the given input
    let cache_run_mode = if !has_children { RunMode::PerformLayout } else { run_mode };
    if let Some(cached_size_and_baselines) =
        tree.nodes[node_key].cache.get(known_dimensions, available_space, cache_run_mode)
    {
        #[cfg(feature = "debug")]
        NODE_LOGGER.labelled_debug_log("CACHE", cached_size_and_baselines.size);
        #[cfg(feature = "debug")]
        debug_log_node(known_dimensions, parent_size, available_space, run_mode, sizing_mode);
        #[cfg(any(feature = "debug", feature = "profile"))]
        NODE_LOGGER.pop_node();
        return cached_size_and_baselines;
    }
    if let Some(cached_size_and_baselines) =
        StackFrameCache::get(node, &parent_size, &known_dimensions, &available_space, run_mode, sizing_mode)
    {
        return cached_size_and_baselines;
    }
    StackFrameCache::increment_frame_count();

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

    let display_mode = tree.nodes[node_key].style.display;
    let computed_size_and_baselines = match (display_mode, has_children) {
        (Display::None, _) => {
            perform_taffy_tree_hidden_layout(tree, node);
            SizeBaselinesAndMargins::HIDDEN
        }
        #[cfg(feature = "block_layout")]
        (Display::Block, true) => perform_computations::<BlockAlgorithm>(
            tree,
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
            tree,
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
            tree,
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
                &tree.nodes[node_key].style,
                tree.nodes[node_key].needs_measure.then(|| &tree.measure_funcs[node_key]),
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                vertical_margins_are_collapsible,
            ),
            RunMode::ComputeSize => leaf::measure_size(
                &tree.nodes[node_key].style,
                tree.nodes[node_key].needs_measure.then(|| &tree.measure_funcs[node_key]),
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                vertical_margins_are_collapsible,
            )
            .into(),
        },
    };

    // Cache result in stack frame cache
    StackFrameCache::insert(
        node,
        parent_size,
        known_dimensions,
        available_space,
        // TODO: should rearrange these params so they make sense (keys first, then payload)
        computed_size_and_baselines,
        run_mode,
        sizing_mode,
    );

    // Cache result
    tree.nodes[node_key].cache.store(known_dimensions, available_space, cache_run_mode, computed_size_and_baselines);

    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("RESULT", computed_size_and_baselines.size);
    #[cfg(any(feature = "debug", feature = "profile"))]
    NODE_LOGGER.pop_node();

    StackFrameCache::decrement_frame_count();
    computed_size_and_baselines
}

/// Creates a layout for this node and its children, recursively.
/// Each hidden node has zero size and is placed at the origin
fn perform_taffy_tree_hidden_layout(tree: &mut Taffy, node: NodeId) {
    /// Recursive function to apply hidden layout to all descendents
    fn perform_hidden_layout_inner(tree: &mut Taffy, node: NodeId, order: u32) {
        let node_key = node.into();
        *tree.layout_mut(node) = Layout::with_order(order);
        tree.nodes[node_key].cache.clear();
        for order in 0..tree.children[node_key].len() {
            perform_hidden_layout_inner(tree, tree.child(node, order), order as _);
        }
    }

    for order in 0..tree.children[node.into()].len() {
        perform_hidden_layout_inner(tree, tree.child(node, order), order as _);
    }
}

/// Rounds the calculated [`Layout`] to exact pixel values
/// In order to ensure that no gaps in the layout are introduced we:
///   - Always round based on the absolute coordinates rather than parent-relative coordinates
///   - Compute width/height by first rounding the top/bottom/left/right and then computing the difference
///     rather than rounding the width/height directly
///
/// See <https://github.com/facebook/yoga/commit/aa5b296ac78f7a22e1aeaf4891243c6bb76488e2> for more context
fn round_layout(tree: &mut impl LayoutTree, node: NodeId, abs_x: f32, abs_y: f32) {
    let layout = tree.layout_mut(node);
    let abs_x = abs_x + layout.location.x;
    let abs_y = abs_y + layout.location.y;

    layout.location.x = round(layout.location.x);
    layout.location.y = round(layout.location.y);
    layout.size.width = round(abs_x + layout.size.width) - round(abs_x);
    layout.size.height = round(abs_y + layout.size.height) - round(abs_y);

    let child_count = tree.child_count(node);
    for index in 0..child_count {
        let child = tree.child(node, index);
        round_layout(tree, child, abs_x, abs_y);
    }
}
