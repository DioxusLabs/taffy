//! Contains both [a high-level interface to Taffy](crate::Taffy) using a ready-made node tree, and [a trait for defining a custom node trees](crate::tree::LayoutTree) / utility types to help with that.

use crate::geometry::{Line, Size};
use crate::style::{AvailableSpace, Style};

// Submodules
mod cache;
pub use cache::{Cache, CacheEntry};
mod measure_func;
pub use measure_func::{Measurable, MeasureFunc};
mod node;
#[cfg(feature = "taffy_tree")]
use node::NodeData;
pub use node::NodeId;
#[cfg(feature = "taffy_tree")]
mod taffy_tree;
#[cfg(feature = "taffy_tree")]
pub use taffy_tree::{Taffy, TaffyChildIter, TaffyError, TaffyResult};
mod layout;
pub use layout::{CollapsibleMarginSet, Layout, RunMode, SizeBaselinesAndMargins, SizingMode};

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

    /// Get a reference to the node's output layout
    fn layout(&self, node: NodeId) -> &Layout;

    /// Modify the node's output layout
    fn layout_mut(&mut self, node: NodeId) -> &mut Layout;

    /// Compute the size of the node given the specified constraints
    fn measure_child_size(
        &mut self,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> Size<f32>;

    /// Perform a full layout on the node given the specified constraints
    fn perform_child_layout(
        &mut self,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> SizeBaselinesAndMargins;
}
