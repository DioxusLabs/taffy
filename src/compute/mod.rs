//! The layout algorithms themselves

pub(crate) mod common;
pub(crate) mod leaf;

#[cfg(feature = "flexbox")]
pub(crate) mod flexbox;

#[cfg(feature = "grid")]
pub(crate) mod grid;

use crate::data::CACHE_SIZE;
use crate::error::TaffyError;
use crate::geometry::{Point, Size};
use crate::layout::{Cache, Layout, RunMode, SizeAndBaselines, SizingMode};
use crate::node::{Node, Taffy};
use crate::style::{AvailableSpace, Display};
use crate::sys::round;
use crate::tree::LayoutTree;

#[cfg(feature = "flexbox")]
use self::flexbox::FlexboxAlgorithm;

#[cfg(feature = "grid")]
use self::grid::CssGridAlgorithm;
use self::leaf::LeafAlgorithm;

#[cfg(any(feature = "debug", feature = "profile"))]
use crate::debug::NODE_LOGGER;

/// Updates the stored layout of the provided `node` and its children
pub fn compute_layout(taffy: &mut Taffy, root: Node, available_space: Size<AvailableSpace>) -> Result<(), TaffyError> {
    // Recursively compute node layout
    let size_and_baselines = GenericAlgorithm::perform_layout(
        taffy,
        root,
        Size::NONE,
        available_space.into_options(),
        available_space,
        SizingMode::InherentSize,
    );

    let layout = Layout { order: 0, size: size_and_baselines.size, location: Point::ZERO };
    *taffy.layout_mut(root) = layout;

    // If rounding is enabled, recursively round the layout's of this node and all children
    if taffy.config.use_rounding {
        round_layout(taffy, root, 0.0, 0.0);
    }

    Ok(())
}

/// A common interface that all Taffy layout algorithms conform to
pub(crate) trait LayoutAlgorithm {
    /// The name of the algorithm (mainly used for debug purposes)
    const NAME: &'static str;

    /// Compute the size of the node given the specified constraints
    fn measure_size(
        tree: &mut impl LayoutTree,
        node: Node,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// Perform a full layout on the node given the specified constraints
    fn perform_layout(
        tree: &mut impl LayoutTree,
        node: Node,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> SizeAndBaselines;
}

/// The public interface to a generic algorithm that abstracts over all of Taffy's algorithms
/// and applies the correct one based on the `Display` style
pub struct GenericAlgorithm;
impl LayoutAlgorithm for GenericAlgorithm {
    const NAME: &'static str = "GENERIC";

    fn perform_layout(
        tree: &mut impl LayoutTree,
        node: Node,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> SizeAndBaselines {
        compute_node_layout(
            tree,
            node,
            known_dimensions,
            parent_size,
            available_space,
            RunMode::PeformLayout,
            sizing_mode,
        )
    }

    fn measure_size(
        tree: &mut impl LayoutTree,
        node: Node,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32> {
        compute_node_layout(
            tree,
            node,
            known_dimensions,
            parent_size,
            available_space,
            RunMode::ComputeSize,
            sizing_mode,
        )
        .size
    }
}

/// Updates the stored layout of the provided `node` and its children
fn compute_node_layout(
    tree: &mut impl LayoutTree,
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

    // First we check if we have a cached result for the given input
    let cache_run_mode = if tree.is_childless(node) { RunMode::PeformLayout } else { run_mode };
    if let Some(cached_size_and_baselines) =
        compute_from_cache(tree, node, known_dimensions, available_space, cache_run_mode)
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
        node: Node,
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
                Algorithm::perform_layout(tree, node, known_dimensions, parent_size, available_space, sizing_mode)
            }
            RunMode::ComputeSize => {
                let size =
                    Algorithm::measure_size(tree, node, known_dimensions, parent_size, available_space, sizing_mode);
                SizeAndBaselines { size, first_baselines: Point::NONE }
            }
        }
    }

    let display_mode = tree.style(node).display;
    let has_children = !tree.is_childless(node);
    let computed_size_and_baselines = match (display_mode, has_children) {
        (Display::None, _) => perform_computations::<HiddenAlgorithm>(
            tree,
            node,
            known_dimensions,
            parent_size,
            available_space,
            run_mode,
            sizing_mode,
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
        ),
        (_, false) => perform_computations::<LeafAlgorithm>(
            tree,
            node,
            known_dimensions,
            parent_size,
            available_space,
            run_mode,
            sizing_mode,
        ),
    };

    // Cache result
    let cache_slot = compute_cache_slot(known_dimensions, available_space);
    *tree.cache_mut(node, cache_slot) = Some(Cache {
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
    tree: &mut impl LayoutTree,
    node: Node,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
) -> Option<SizeAndBaselines> {
    for idx in 0..CACHE_SIZE {
        let entry = tree.cache_mut(node, idx);
        if let Some(entry) = entry {
            // Cached ComputeSize results are not valid if we are running in PerformLayout mode
            if entry.run_mode == RunMode::ComputeSize && run_mode == RunMode::PeformLayout {
                return None;
            }

            let cached_size = entry.cached_size_and_baselines.size;

            if (known_dimensions.width == entry.known_dimensions.width
                || known_dimensions.width == Some(cached_size.width))
                && (known_dimensions.height == entry.known_dimensions.height
                    || known_dimensions.height == Some(cached_size.height))
                && (known_dimensions.width.is_some()
                    || entry.available_space.width.is_roughly_equal(available_space.width))
                && (known_dimensions.height.is_some()
                    || entry.available_space.height.is_roughly_equal(available_space.height))
            {
                return Some(entry.cached_size_and_baselines);
            }
        }
    }

    None
}

/// The public interface to Taffy's hidden node algorithm implementation
struct HiddenAlgorithm;
impl LayoutAlgorithm for HiddenAlgorithm {
    const NAME: &'static str = "NONE";

    fn perform_layout(
        tree: &mut impl LayoutTree,
        node: Node,
        _known_dimensions: Size<Option<f32>>,
        _parent_size: Size<Option<f32>>,
        _available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
    ) -> SizeAndBaselines {
        perform_hidden_layout(tree, node);
        SizeAndBaselines { size: Size::ZERO, first_baselines: Point::NONE }
    }

    fn measure_size(
        _tree: &mut impl LayoutTree,
        _node: Node,
        _known_dimensions: Size<Option<f32>>,
        _parent_size: Size<Option<f32>>,
        _available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
    ) -> Size<f32> {
        Size::ZERO
    }
}

/// Creates a layout for this node and its children, recursively.
/// Each hidden node has zero size and is placed at the origin
fn perform_hidden_layout(tree: &mut impl LayoutTree, node: Node) {
    /// Recursive function to apply hidden layout to all descendents
    fn perform_hidden_layout_inner(tree: &mut impl LayoutTree, node: Node, order: u32) {
        *tree.layout_mut(node) = Layout::with_order(order);
        for order in 0..tree.child_count(node) {
            perform_hidden_layout_inner(tree, tree.child(node, order), order as _);
        }
    }

    for order in 0..tree.child_count(node) {
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
fn round_layout(tree: &mut impl LayoutTree, node: Node, abs_x: f32, abs_y: f32) {
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

#[cfg(test)]
mod tests {
    use super::perform_hidden_layout;
    use crate::geometry::{Point, Size};
    use crate::style::{Display, Style};
    use crate::Taffy;

    #[test]
    fn hidden_layout_should_hide_recursively() {
        let mut taffy = Taffy::new();

        let style: Style = Style { display: Display::Flex, size: Size::from_points(50.0, 50.0), ..Default::default() };

        let grandchild_00 = taffy.new_leaf(style.clone()).unwrap();
        let grandchild_01 = taffy.new_leaf(style.clone()).unwrap();
        let child_00 = taffy.new_with_children(style.clone(), &[grandchild_00, grandchild_01]).unwrap();

        let grandchild_02 = taffy.new_leaf(style.clone()).unwrap();
        let child_01 = taffy.new_with_children(style.clone(), &[grandchild_02]).unwrap();

        let root = taffy
            .new_with_children(
                Style { display: Display::None, size: Size::from_points(50.0, 50.0), ..Default::default() },
                &[child_00, child_01],
            )
            .unwrap();

        perform_hidden_layout(&mut taffy, root);

        // Whatever size and display-mode the nodes had previously,
        // all layouts should resolve to ZERO due to the root's DISPLAY::NONE
        for (node, _) in taffy.nodes.iter().filter(|(node, _)| *node != root) {
            if let Ok(layout) = taffy.layout(node) {
                assert_eq!(layout.size, Size::zero());
                assert_eq!(layout.location, Point::zero());
            }
        }
    }
}
