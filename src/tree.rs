//! The baseline requirements of any UI Tree so Taffy can efficiently calculate the layout

use crate::{
    layout::{Cache, Layout},
    prelude::*,
};

/// Any item that implements the LayoutTree can be layed out using Taffy's algorithms.
///
/// Generally, Taffy expects your Node tree to be indexable by stable indicies. A "stable" index means that the Node's ID
/// remains the same between re-layouts.
pub trait LayoutTree {
    /// Get the list of children IDs for the given node
    fn children(&self, node: Node) -> &[Node];

    /// Get a specific child of a node, where the index represents the nth child
    fn child(&self, node: Node, index: usize) -> Node;

    /// Get any available parent for this node
    fn parent(&self, node: Node) -> Option<Node>;

    /// Get the [`FlexboxLayout`] for this Node.
    fn style(&self, node: Node) -> &FlexboxLayout;

    /// Get the node's output "Final Layout"
    fn layout(&self, node: Node) -> &Layout;

    /// Modify the node's output layout
    fn layout_mut(&mut self, node: Node) -> &mut Layout;

    /// Mark a node as finished
    fn mark_dirty(&mut self, node: Node);

    /// Measure a node. Taffy uses this to force reflows of things like text and overflowing content.
    fn measure_node(&self, node: Node) -> Option<Size<f32>>;

    /// Node needs to be measured
    fn needs_measure(&self, node: Node) -> bool;

    /// Get the primary cache for this Node.
    ///
    /// Taffy caches results of computations for nodes to not need re-caculating nodes it already knows
    ///
    /// When a node does not have a cache, Taffy will layout that node appropriately.
    fn primary_cache(&mut self, node: Node) -> &mut Option<Cache>;

    /// Get the secondary cache for this Node.
    ///
    /// Taffy caches results of computations for nodes to not need re-caculating nodes it already knows
    ///
    /// When a node does not have a cache, Taffy will layout that node appropriately.
    ///
    /// The secondary cache is for nodes who have a main size already calculated, but need to calculate a secondary size.
    /// This typically happens due to conflicting constraints.
    fn secondary_cache(&mut self, node: Node) -> &mut Option<Cache>;
}

impl LayoutTree for Taffy {
    fn children(&self, node: Node) -> &[Node] {
        &self.children[node]
    }

    fn parent(&self, node: Node) -> Option<Node> {
        self.parents.get(node).copied().flatten()
    }

    fn style(&self, node: Node) -> &FlexboxLayout {
        &self.nodes[node].style
    }

    fn layout(&self, node: Node) -> &Layout {
        &self.nodes[node].layout
    }

    fn layout_mut(&mut self, node: Node) -> &mut Layout {
        &mut self.nodes[node].layout
    }

    fn mark_dirty(&mut self, node: Node) {
        self.nodes[node].is_dirty = true;
    }

    fn measure_node(&self, node: Node) -> Option<Size<f32>> {
        todo!()
    }

    fn needs_measure(&self, node: Node) -> bool {
        todo!()
    }

    fn primary_cache(&mut self, node: Node) -> &mut Option<Cache> {
        &mut self.nodes[node].main_size_layout_cache
    }

    fn secondary_cache(&mut self, node: Node) -> &mut Option<Cache> {
        &mut self.nodes[node].other_layout_cache
    }

    fn child(&self, node: Node, id: usize) -> Node {
        self.children[node][id]
    }
}
