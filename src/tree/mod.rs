//! Contains both [a high-level interface to Taffy](crate::Taffy) using a ready-made node tree, and [a trait for defining a custom node trees](crate::tree::LayoutTree) / utility types to help with that.

use crate::geometry::{Line, Size};
use crate::style::{AvailableSpace, Style};

// Submodules
mod cache;
pub use cache::{Cache, CacheEntry};
mod node;
#[cfg(feature = "taffy_tree")]
use node::NodeData;
pub use node::NodeId;
#[cfg(feature = "taffy_tree")]
mod taffy_tree;
#[cfg(feature = "taffy_tree")]
pub use taffy_tree::{Taffy, TaffyChildIter, TaffyError, TaffyResult};
mod layout;
use crate::compute::compute_cached_layout;
pub use layout::{CollapsibleMarginSet, Layout, LayoutInput, LayoutOutput, RunMode, SizingMode};

/// Any item that implements the LayoutTree can be layed out using Taffy's algorithms.
///
/// Generally, Taffy expects your Node tree to be indexable by stable indices. A "stable" index means that the Node's ID
/// remains the same between re-layouts.
pub trait LayoutTree {
    /// Type representing an iterator of the children of a node
    type ChildIter<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    /// Get the list of children IDs for the given node
    fn children(&self, node: NodeId) -> Self::ChildIter<'_>;

    /// Get the number of children for the given node
    fn child_count(&self, node: NodeId) -> usize;

    /// Get a specific child of a node, where the index represents the nth child
    fn child(&self, node: NodeId, index: usize) -> NodeId;

    /// Get the [`Style`] for this node.
    fn style(&self, node: NodeId) -> &Style;

    /// Modify the node's output layout
    fn unrounded_layout_mut(&mut self, node: NodeId) -> &mut Layout;

    /// Get a reference to the node's output final layout
    fn final_layout(&self, node: NodeId) -> &Layout;

    /// Get a mutable reference to the node's output final layout
    fn final_layout_mut(&mut self, node: NodeId) -> &mut Layout;

    /// Get a mutable reference to the [`Cache`] for this node.
    fn cache_mut(&mut self, node: NodeId) -> &mut Cache;

    /// Compute the specified node's size or full layout given the specified constraints
    fn compute_child_layout(&mut self, node: NodeId, inputs: LayoutInput) -> LayoutOutput;
}

pub(crate) trait LayoutTreeExt: LayoutTree {
    /// Compute the size of the node given the specified constraints
    fn measure_child_size(
        &mut self,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> Size<f32> {
        compute_cached_layout(
            self,
            node,
            LayoutInput {
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                run_mode: RunMode::ComputeSize,
                vertical_margins_are_collapsible,
            },
        )
        .size
    }

    /// Perform a full layout on the node given the specified constraints
    fn perform_child_layout(
        &mut self,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> LayoutOutput {
        compute_cached_layout(
            self,
            node,
            LayoutInput {
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                run_mode: RunMode::PerformLayout,
                vertical_margins_are_collapsible,
            },
        )
    }
}

impl<T: LayoutTree> LayoutTreeExt for T {}
