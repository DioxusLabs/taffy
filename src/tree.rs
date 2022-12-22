//! The baseline requirements of any UI Tree so Taffy can efficiently calculate the layout

use slotmap::DefaultKey;

use crate::{
    error::TaffyResult,
    layout::{Cache, Layout},
    prelude::*,
};

/// Any item that implements the LayoutTree can be layed out using Taffy's algorithms.
///
/// Generally, Taffy expects your Node tree to be indexable by stable indices. A "stable" index means that the Node's ID
/// remains the same between re-layouts.
pub trait LayoutTree {
    /// Type representing an iterator of the children of a node
    type ChildIter<'a>: Iterator<Item = &'a DefaultKey>
    where
        Self: 'a;

    /// Get the list of children IDs for the given node
    fn children(&self, node: Node) -> Self::ChildIter<'_>;

    /// Get the number of children for the given node
    fn child_count(&self, node: Node) -> usize;

    /// Returns true if the node has no children
    fn is_childless(&self, node: Node) -> bool;

    /// Get a specific child of a node, where the index represents the nth child
    fn child(&self, node: Node, index: usize) -> Node;

    /// Get any available parent for this node
    fn parent(&self, node: Node) -> Option<Node>;

    // todo: allow abstractions over this so we don't prescribe how layout works
    // for reference, CSS cascades require context, and storing a full flexbox layout for each node could be inefficient
    //
    /// Get the [`Style`] for this Node.
    fn style(&self, node: Node) -> &Style;

    /// Get the node's output "Final Layout"
    fn layout(&self, node: Node) -> &Layout;

    /// Modify the node's output layout
    fn layout_mut(&mut self, node: Node) -> &mut Layout;

    /// Mark a node as dirty to tell Taffy that something has changed and it needs to be recomputed.
    ///
    /// Commonly done if the style of the node has changed.
    fn mark_dirty(&mut self, node: Node) -> TaffyResult<()>;

    /// Measure a node. Taffy uses this to force reflows of things like text and overflowing content.
    fn measure_node(
        &self,
        node: Node,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
    ) -> Size<f32>;

    /// Node needs to be measured
    fn needs_measure(&self, node: Node) -> bool;

    /// Get a cache entry for this Node by index
    fn cache_mut(&mut self, node: Node, index: usize) -> &mut Option<Cache>;
}
