//! The abstractions that make up the core of Taffy's low-level API
use super::{Cache, Layout, LayoutInput, LayoutOutput, NodeId, RequestedAxis, RunMode, SizingMode};
use crate::geometry::{AbsoluteAxis, Line, Size};
use crate::style::{AvailableSpace, Style};

/// This trait is Taffy's abstraction for downward tree traversal.
/// However, this trait does *not* require access to any node's other than a single container node's immediate children unless you also intend to implement `TraverseTree`.
pub trait TraversePartialTree {
    /// Type representing an iterator of the children of a node
    type ChildIter<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    /// Get the list of children IDs for the given node
    fn child_ids(&self, parent_node_id: NodeId) -> Self::ChildIter<'_>;

    /// Get the number of children for the given node
    fn child_count(&self, parent_node_id: NodeId) -> usize;

    /// Get a specific child of a node, where the index represents the nth child
    fn get_child_id(&self, parent_node_id: NodeId, child_index: usize) -> NodeId;
}

/// A marker trait which extends `TraversePartialTree` with the additional guarantee that the child/children methods can be used to recurse
/// infinitely down the tree. Is required by the `RoundTree` and the `PrintTree` traits.
pub trait TraverseTree: TraversePartialTree {}

/// Any type that implements [`LayoutPartialTree`] can be laid out using [Taffy's algorithms](crate::compute)
///
/// Note that this trait extends [`TraversePartialTree`] (not [`TraverseTree`]). Taffy's algorithm implementations have been designed such that they can be used for a laying out a single
/// node that only has access to it's immediate children.
pub trait LayoutPartialTree: TraversePartialTree {
    /// Get a reference to the [`Style`] for this node.
    fn get_style(&self, node_id: NodeId) -> &Style;

    /// Set the node's unrounded layout
    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout);

    /// Get a mutable reference to the [`Cache`] for this node.
    fn get_cache_mut(&mut self, node_id: NodeId) -> &mut Cache;

    /// Compute the specified node's size or full layout given the specified constraints
    fn compute_child_layout(&mut self, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput;
}

/// Trait used by the `round_layout` method which takes a tree of unrounded float-valued layouts and performs
/// rounding to snap the values to the pixel grid.
///
/// As indicated by it's dependence on `TraverseTree`, it required full recursive access to the tree.
pub trait RoundTree: TraverseTree {
    /// Get the node's unrounded layout
    fn get_unrounded_layout(&self, node_id: NodeId) -> &Layout;
    /// Get a reference to the node's final layout
    fn set_final_layout(&mut self, node_id: NodeId, layout: &Layout);
}

/// Trait used by the `print_tree` method which prints a debug representation
///
/// As indicated by it's dependence on `TraverseTree`, it required full recursive access to the tree.
pub trait PrintTree: TraverseTree {
    /// Get a debug label for the node (typically the type of node: flexbox, grid, text, image, etc)
    fn get_debug_label(&self, node_id: NodeId) -> &'static str;
    /// Get a reference to the node's final layout
    fn get_final_layout(&self, node_id: NodeId) -> &Layout;
}

// --- PRIVATE TRAITS

/// A private trait which allows us to add extra convenience methods to types which implement
/// LayoutTree without making those methods public.
pub(crate) trait LayoutPartialTreeExt: LayoutPartialTree {
    /// Compute the size of the node given the specified constraints
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn measure_child_size(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
        axis: AbsoluteAxis,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> f32 {
        self.compute_child_layout(
            node_id,
            LayoutInput {
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                axis: axis.into(),
                run_mode: RunMode::ComputeSize,
                vertical_margins_are_collapsible,
            },
        )
        .size
        .get_abs(axis)
    }

    /// Perform a full layout on the node given the specified constraints
    #[inline(always)]
    fn perform_child_layout(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> LayoutOutput {
        self.compute_child_layout(
            node_id,
            LayoutInput {
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                axis: RequestedAxis::Both,
                run_mode: RunMode::PerformLayout,
                vertical_margins_are_collapsible,
            },
        )
    }
}

impl<T: LayoutPartialTree> LayoutPartialTreeExt for T {}
