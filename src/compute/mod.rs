//! The layout algorithms themselves

pub(crate) mod common;
pub(crate) mod flexbox;
pub(crate) mod leaf;

#[cfg(feature = "grid")]
pub(crate) mod grid;

use crate::data::CACHE_SIZE;
use crate::error::TaffyError;
use crate::geometry::{Point, Size};
use crate::layout::{Cache, Layout, RunMode, SizeAndBaselines, SizingMode};
use crate::node::{Node, Taffy};
use crate::style::{AvailableSpace, Display};
use crate::tree::LayoutTree;

use self::flexbox::FlexboxAlgorithm;
use self::grid::CssGridAlgorithm;

#[cfg(any(feature = "debug", feature = "profile"))]
use crate::debug::NODE_LOGGER;

/// Updates the stored layout of the provided `node` and its children
pub fn compute_layout(tree: &mut Taffy, root: Node, available_space: Size<AvailableSpace>) -> Result<(), TaffyError> {
    // Recursively compute node layout
    let size_and_baselines = perform_layout(
        tree,
        root,
        Size::NONE,
        available_space.into_options(),
        available_space,
        SizingMode::InherentSize,
    );

    let layout = Layout { order: 0, size: size_and_baselines.size, location: Point::ZERO };
    tree.nodes[root].layout = layout;

    // Recursively round the layout's of this node and all children
    recursively_round_layout(tree, root);

    Ok(())
}

/// A common interface that all Taffy layout algorithms conform to
pub trait LayoutAlgorithm {
    /// The name of the algorithm (mainly used for debug purposes)
    const NAME: &'static str;

    /// Compute the size of the node given the specified constraints
    fn measure_size(
        tree: &mut impl LayoutTree,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// Perform a full layout on the node given the specified constraints
    fn perform_layout(
        tree: &mut impl LayoutTree,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> SizeAndBaselines;
}

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
    compute_node_layout(tree, node, known_dimensions, parent_size, available_space, RunMode::PeformLayout, sizing_mode)
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
    compute_node_layout(tree, node, known_dimensions, parent_size, available_space, RunMode::ComputeSize, sizing_mode)
        .size
}

/// Updates the stored layout of the provided `node` and its children
fn compute_node_layout(
    tree: &mut Taffy,
    node: Node,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
    sizing_mode: SizingMode,
) -> SizeAndBaselines {
    #[cfg(any(feature = "debug", feature = "profile"))]
    NODE_LOGGER.push_node(node);
    #[cfg(feature = "debug")]
    println!();

    let child_count = tree.children[node].len();

    // First we check if we have a cached result for the given input
    let cache_run_mode = if child_count == 0 { RunMode::PeformLayout } else { run_mode };
    if let Some(cached_size_and_baselines) =
        compute_from_cache(&tree.nodes[node].size_cache, known_dimensions, available_space, cache_run_mode, sizing_mode)
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
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: RunMode,
        sizing_mode: SizingMode,
    ) -> SizeAndBaselines {
        #[cfg(feature = "debug")]
        NODE_LOGGER.log(Algorithm::NAME);

        match run_mode {
            RunMode::PeformLayout => {
                Algorithm::perform_layout(tree, known_dimensions, parent_size, available_space, sizing_mode)
            }
            RunMode::ComputeSize => {
                let size = Algorithm::measure_size(tree, known_dimensions, parent_size, available_space, sizing_mode);
                SizeAndBaselines { size, first_baselines: Point::NONE }
            }
        }
    }

    let computed_size_and_baselines = if child_count == 0 {
        #[cfg(feature = "debug")]
        NODE_LOGGER.log("LEAF");

        match run_mode {
            RunMode::PeformLayout => {
                leaf::perform_layout(tree, node, known_dimensions, parent_size, available_space, sizing_mode)
            }
            RunMode::ComputeSize => {
                let size = leaf::measure_size(tree, node, known_dimensions, parent_size, available_space, sizing_mode);
                SizeAndBaselines { size, first_baselines: Point::NONE }
            }
        }
    } else {
        match tree.nodes[node].style.display {
            Display::Flex => perform_computations::<FlexboxAlgorithm>(
                &mut tree.node_ref_mut(node).unwrap(),
                known_dimensions,
                parent_size,
                available_space,
                run_mode,
                sizing_mode,
            ),
            #[cfg(feature = "grid")]
            Display::Grid => perform_computations::<CssGridAlgorithm>(
                &mut tree.node_ref_mut(node).unwrap(),
                known_dimensions,
                parent_size,
                available_space,
                run_mode,
                sizing_mode,
            ),
            Display::None => {
                tree.nodes[node].layout = Layout::with_order(0);
                for index in 0..child_count {
                    let child_id = tree.children[node][index];
                    let node_ref = &mut tree.node_ref_mut(node).unwrap();
                    node_ref.perform_child_hidden_layout(child_id, index as _)
                }
                SizeAndBaselines { size: Size::ZERO, first_baselines: Point::NONE }
            }
        }
    };

    // Cache result
    let cache_slot = compute_cache_slot(known_dimensions, available_space);
    tree.nodes[node].size_cache[cache_slot] = Some(Cache {
        known_dimensions,
        available_space,
        run_mode: cache_run_mode,
        cached_size_and_baselines: computed_size_and_baselines,
    });

    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("RESULT", computed_size_and_baselines.size);
    #[cfg(any(feature = "debug", feature = "profile"))]
    NODE_LOGGER.pop_node();

    computed_size_and_baselines
}

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

/// Return the cache slot to cache the current computed result in
///
/// ## Caching Strategy
///
/// We need multiple cache slots, because a node's size is often queried by it's parent multiple times in the course of the layout
/// process, and we don't want later results to clobber earlier ones.
///
/// The two variables that we care about when determining cache slot are:
///
///   - How many "known_dimensions" are set. In the worst case, a node may be called first with neither dimensions known known_dimensions,
///     then with one dimension known (either width of height - which doesn't matter for our purposes here), and then with both dimensions
///     known.
///   - Whether unknown dimensions are being sized under a min-content or a max-content available space constraint (definite available space
///     shares a cache slot with max-content because a node will generally be sized under one or the other but not both).
///
/// ## Cache slots:
///
/// - Slot 0: Both known_dimensions were set
/// - Slot 1: 1 of 2 known_dimensions were set and the other dimension was either a MaxContent or Definite available space constraint
/// - Slot 2: 1 of 2 known_dimensions were set and the other dimension was a MinContent constraint
/// - Slot 3: Neither known_dimensions were set and we are sizing under a MaxContent or Definite available space constraint
/// - Slot 4: Neither known_dimensions were set and we are sizing under a MinContent constraint
#[inline]
fn compute_cache_slot(known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> usize {
    let has_known_width = known_dimensions.width.is_some();
    let has_known_height = known_dimensions.height.is_some();

    // Slot 0: Both known_dimensions were set
    if has_known_width && has_known_height {
        return 0;
    }

    // Slot 1: width but not height known_dimension was set and the other dimension was either a MaxContent or Definite available space constraint
    // Slot 2: width but not height known_dimension was set and the other dimension was a MinContent constraint
    if has_known_width && !has_known_height {
        return 1 + (available_space.height == AvailableSpace::MinContent) as usize;
    }

    // Slot 3: height but not width known_dimension was set and the other dimension was either a MaxContent or Definite available space constraint
    // Slot 4: height but not width known_dimension was set and the other dimension was a MinContent constraint
    if !has_known_width && has_known_height {
        return 3 + (available_space.width == AvailableSpace::MinContent) as usize;
    }

    // Slot 5: Neither known_dimensions were set and we are sizing under a MaxContent or Definite available space constraint
    // Slot 6: Neither known_dimensions were set and we are sizing under a MinContent constraint
    5 + (available_space.width == AvailableSpace::MinContent) as usize
}

/// Try to get the computation result from the cache.
#[inline]
fn compute_from_cache(
    cache: &[Option<Cache>; CACHE_SIZE],
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
    sizing_mode: SizingMode,
) -> Option<SizeAndBaselines> {
    for entry in cache.iter().flatten() {
        // Cached ComputeSize results are not valid if we are running in PerformLayout mode
        if entry.run_mode == RunMode::ComputeSize && run_mode == RunMode::PeformLayout {
            return None;
        }

        let cached_size = entry.cached_size_and_baselines.size;

        if (known_dimensions.width == entry.known_dimensions.width || known_dimensions.width == Some(cached_size.width))
            && (known_dimensions.height == entry.known_dimensions.height
                || known_dimensions.height == Some(cached_size.height))
            && (known_dimensions.width.is_some()
                || entry.available_space.width.is_roughly_equal(available_space.width)
                || (sizing_mode == SizingMode::ContentSize
                    && available_space.width.is_definite()
                    && available_space.width.unwrap() >= cached_size.width))
            && (known_dimensions.height.is_some()
                || entry.available_space.height.is_roughly_equal(available_space.height)
                || (sizing_mode == SizingMode::ContentSize
                    && available_space.height.is_definite()
                    && available_space.height.unwrap() >= cached_size.height))
        {
            return Some(entry.cached_size_and_baselines);
        }
    }

    None
}

/// Creates a layout for this node and its children, recursively.
/// Each hidden node has zero size and is placed at the origin
pub(crate) fn recursively_perform_hidden_layout(tree: &mut Taffy, node: Node, order: u32) {
    tree.nodes[node].layout = Layout::with_order(order);
    for index in 0..tree.children[node].len() {
        recursively_perform_hidden_layout(tree, tree.children[node][index], index as _);
    }
}

/// Rounds the calculated [`NodeData`] according to the spec
fn recursively_round_layout(tree: &mut Taffy, node: Node) {
    tree.nodes[node].layout.round();

    // Satisfy the borrow checker here by re-indexing to shorten the lifetime to the loop scope
    for index in 0..tree.children[node].len() {
        recursively_round_layout(tree, tree.children[node][index]);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::perform_hidden_layout;
//     use crate::geometry::{Point, Size};
//     use crate::style::{Display, Style};
//     use crate::Taffy;

//     #[test]
//     fn hidden_layout_should_hide_recursively() {
//         let mut taffy = Taffy::new();

//         let style: Style = Style { display: Display::Flex, size: Size::from_points(50.0, 50.0), ..Default::default() };

//         let grandchild_00 = taffy.new_leaf(style.clone()).unwrap();
//         let grandchild_01 = taffy.new_leaf(style.clone()).unwrap();
//         let child_00 = taffy.new_with_children(style.clone(), &[grandchild_00, grandchild_01]).unwrap();

//         let grandchild_02 = taffy.new_leaf(style.clone()).unwrap();
//         let child_01 = taffy.new_with_children(style.clone(), &[grandchild_02]).unwrap();

//         let root = taffy
//             .new_with_children(
//                 Style { display: Display::None, size: Size::from_points(50.0, 50.0), ..Default::default() },
//                 &[child_00, child_01],
//             )
//             .unwrap();

//         perform_hidden_layout(&mut taffy, root);

//         // Whatever size and display-mode the nodes had previously,
//         // all layouts should resolve to ZERO due to the root's DISPLAY::NONE
//         for (node, _) in taffy.nodes.iter().filter(|(node, _)| *node != root) {
//             if let Ok(layout) = taffy.layout(node) {
//                 assert_eq!(layout.size, Size::zero());
//                 assert_eq!(layout.location, Point::zero());
//             }
//         }
//     }
// }
