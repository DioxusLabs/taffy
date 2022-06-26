//! UI [`Node`] types and related data structures.
//!
//! Layouts are composed of multiple nodes, which live in a forest-like data structure.
use crate::error;
use crate::forest::Forest;
use crate::geometry::Size;
use crate::layout::Layout;
use crate::style::FlexboxLayout;
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::sys::Box;
use crate::sys::{new_map_with_capacity, ChildrenVec, Map, Vec};
use core::sync::atomic::{AtomicUsize, Ordering};

/// Internal node id.
pub(crate) type NodeId = usize;

/// The identifier of a [`Node`]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(not(any(feature = "std", feature = "alloc")), derive(hash32_derive::Hash32))]
pub(crate) struct Id(usize);

/// An bump-allocator index that tracks how many [`Nodes`](Node) have been allocated in a [`Taffy`].
pub(crate) struct Allocator {
    /// The last reserved [`NodeId`]
    last_id: AtomicUsize,
}

impl Allocator {
    /// Creates a fresh [`Allocator`]
    #[must_use]
    pub const fn new() -> Self {
        Self { last_id: AtomicUsize::new(0) }
    }

    /// Allocates space for one more [`Node`]
    pub fn allocate(&self) -> Id {
        Id(self.last_id.fetch_add(1, Ordering::Relaxed))
    }
}

/// A function type that can be used in a [`MeasureFunc`]
///
/// This trait is automatically implemented for all types (including closures) that define a function with the appropriate type signature.
pub trait Measurable: Send + Sync + Fn(Size<Option<f32>>) -> Size<f32> {}

impl<F: Send + Sync + Fn(Size<Option<f32>>) -> Size<f32>> Measurable for F {}

/// A function that can be used to compute the intrinsic size of a node
pub enum MeasureFunc {
    /// Stores an unboxed function
    Raw(fn(Size<Option<f32>>) -> Size<f32>),
    /// Stores a boxed function
    #[cfg(any(feature = "std", feature = "alloc"))]
    Boxed(Box<dyn Measurable>),
}

/// Global taffy instance id allocator.
static INSTANCE_ALLOCATOR: Allocator = Allocator::new();

/// An [`Id`]-containing identifier
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(not(any(feature = "std", feature = "alloc")), derive(hash32_derive::Hash32))]
pub struct Node {
    /// The id of the forest that this node lives with
    instance: Id,
    /// The identifier of this particular node within the tree
    local: Id,
}

/// A forest of UI [`Nodes`](`Node`), suitable for UI layout
pub struct Taffy {
    /// The ID of the root node
    id: Id,
    /// A monotonically-increasing index that tracks the [`Id`] of the next node
    allocator: Allocator,
    /// A map from Node -> NodeId
    nodes_to_ids: Map<Node, NodeId>,
    /// A map from NodeId -> Node
    ids_to_nodes: Map<NodeId, Node>,
    /// An efficient data structure that stores the node trees
    forest: Forest,
}

impl Default for Taffy {
    fn default() -> Self {
        Self::with_capacity(16)
    }
}

impl Taffy {
    /// Creates a new [`Taffy`]
    ///
    /// The default capacity of a [`Taffy`] is 16 nodes.
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new [`Taffy`] that can store `capacity` nodes before reallocation
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            id: INSTANCE_ALLOCATOR.allocate(),
            allocator: Allocator::new(),
            nodes_to_ids: new_map_with_capacity(capacity),
            ids_to_nodes: new_map_with_capacity(capacity),
            forest: Forest::with_capacity(capacity),
        }
    }

    /// Allocates memory for a new node, and returns a matching generated [`Node`]
    fn allocate_node(&mut self) -> Node {
        let local = self.allocator.allocate();
        Node { instance: self.id, local }
    }

    /// Stores a new node in the tree
    fn add_node(&mut self, node: Node, id: NodeId) {
        let _ = self.nodes_to_ids.insert(node, id);
        let _ = self.ids_to_nodes.insert(id, node);
    }

    /// Returns the `NodeId` of the provided node within the forest
    fn find_node(&self, node: Node) -> Result<NodeId, error::InvalidNode> {
        match self.nodes_to_ids.get(&node) {
            Some(id) => Ok(*id),
            None => Err(error::InvalidNode(node)),
        }
    }

    /// Creates and adds a new leaf node
    pub fn new_leaf(&mut self, layout: FlexboxLayout) -> Result<Node, error::InvalidNode> {
        let node = self.allocate_node();
        let id = self.forest.new_leaf(layout);
        self.add_node(node, id);
        Ok(node)
    }

    /// Creates and adds a new leaf node with a supplied [`MeasureFunc`]
    pub fn new_leaf_with_measure(
        &mut self,
        layout: FlexboxLayout,
        measure: MeasureFunc,
    ) -> Result<Node, error::InvalidNode> {
        let node = self.allocate_node();
        let id = self.forest.new_leaf_with_measure(layout, measure);
        self.add_node(node, id);
        Ok(node)
    }

    /// Creates and adds a new node, which may have any number of `children`
    pub fn new_with_children(&mut self, layout: FlexboxLayout, children: &[Node]) -> Result<Node, error::InvalidNode> {
        let node = self.allocate_node();
        let children = children
            .iter()
            .map(|child| self.find_node(*child))
            .collect::<Result<ChildrenVec<_>, error::InvalidNode>>()?;
        let id = self.forest.new_with_children(layout, children);
        self.add_node(node, id);
        Ok(node)
    }

    /// Removes all nodes
    ///
    /// All associated [`Id`] will be rendered invalid.
    pub fn clear(&mut self) {
        self.nodes_to_ids.clear();
        self.ids_to_nodes.clear();
        self.forest.clear();
    }

    /// Remove a specific [`Node`] from the tree
    ///
    /// Its [`Id`] is marked as invalid. Returns the id of the node removed.
    pub fn remove(&mut self, node: Node) -> Result<usize, error::InvalidNode> {
        let id = self.find_node(node)?;

        self.nodes_to_ids.remove(&node);
        self.ids_to_nodes.remove(&id);

        if let Some(new_id) = self.forest.swap_remove(id) {
            let new = self.ids_to_nodes.remove(&new_id).unwrap();
            let _ = self.nodes_to_ids.insert(new, id);
            let _ = self.ids_to_nodes.insert(id, new);
        }

        Ok(id)
    }

    /// Sets the [`MeasureFunc`] of the associated node
    pub fn set_measure(&mut self, node: Node, measure: Option<MeasureFunc>) -> Result<(), error::InvalidNode> {
        let id = self.find_node(node)?;
        self.forest.nodes[id].measure = measure;
        self.forest.mark_dirty(id);
        Ok(())
    }

    /// Adds a `child` [`Node`] under the supplied `parent`
    pub fn add_child(&mut self, parent: Node, child: Node) -> Result<(), error::InvalidNode> {
        let node_id = self.find_node(parent)?;
        let child_id = self.find_node(child)?;

        self.forest.add_child(node_id, child_id);
        Ok(())
    }

    /// Directly sets the `children` of the supplied `parent`
    pub fn set_children(&mut self, parent: Node, children: &[Node]) -> Result<(), error::InvalidNode> {
        let node_id = self.find_node(parent)?;
        let children_id = children.iter().map(|child| self.find_node(*child)).collect::<Result<ChildrenVec<_>, _>>()?;

        // Remove node as parent from all its current children.
        for child in &self.forest.children[node_id] {
            self.forest.parents[*child].retain(|p| *p != node_id);
        }

        // Build up relation node <-> child
        for child in &children_id {
            self.forest.parents[*child].push(node_id);
        }
        self.forest.children[node_id] = children_id;

        self.forest.mark_dirty(node_id);
        Ok(())
    }

    /// Removes the `child` of the parent `node`
    ///
    /// The child is not removed from the forest entirely, it is simply no longer attached to its previous parent.
    pub fn remove_child(&mut self, parent: Node, child: Node) -> Result<Node, error::InvalidNode> {
        let node_id = self.find_node(parent)?;
        let child_id = self.find_node(child)?;

        let prev_id = self.forest.remove_child(node_id, child_id);
        Ok(self.ids_to_nodes[&prev_id])
    }

    /// Removes the child at the given `index` from the `parent`
    ///
    /// The child is not removed from the forest entirely, it is simply no longer attached to its previous parent.
    pub fn remove_child_at_index(&mut self, parent: Node, child_index: usize) -> Result<Node, error::InvalidChild> {
        let node_id = self.find_node(parent).map_err(|e| error::InvalidChild::InvalidParentNode(e.0))?;

        let child_count = self.forest.children[node_id].len();
        if child_index >= child_count {
            return Err(error::InvalidChild::ChildIndexOutOfBounds { parent, child_index, child_count });
        }

        let prev_id = self.forest.remove_child_at_index(node_id, child_index);
        Ok(self.ids_to_nodes[&prev_id])
    }

    /// Replaces the child at the given `child_index` from the `parent` node with the new `child` node
    ///
    /// The child is not removed from the forest entirely, it is simply no longer attached to its previous parent.
    pub fn replace_child_at_index(
        &mut self,
        parent: Node,
        child_index: usize,
        new_child: Node,
    ) -> Result<Node, error::InvalidChild> {
        let node_id = self.find_node(parent).map_err(|e| error::InvalidChild::InvalidParentNode(e.0))?;
        let child_id = self.find_node(new_child).map_err(|e| error::InvalidChild::InvalidChildNode(e.0))?;

        let child_count = self.forest.children[node_id].len();
        if child_index >= child_count {
            return Err(error::InvalidChild::ChildIndexOutOfBounds { parent, child_index, child_count });
        }

        self.forest.parents[child_id].push(node_id);
        let old_child = core::mem::replace(&mut self.forest.children[node_id][child_index], child_id);
        self.forest.parents[old_child].retain(|p| *p != node_id);

        self.forest.mark_dirty(node_id);

        Ok(self.ids_to_nodes[&old_child])
    }

    /// Returns the child [`Node`] of the parent `node` at the provided `child_index`
    pub fn child_at_index(&self, parent: Node, child_index: usize) -> Result<Node, error::InvalidChild> {
        let id = self.find_node(parent).map_err(|e| error::InvalidChild::InvalidParentNode(e.0))?;

        let child_count = self.forest.children[id].len();
        if child_index >= child_count {
            return Err(error::InvalidChild::ChildIndexOutOfBounds { parent, child_index, child_count });
        }

        Ok(self.ids_to_nodes[&self.forest.children[id][child_index]])
    }

    /// Returns the number of children of the `parent` [`Node`]
    pub fn child_count(&self, parent: Node) -> Result<usize, error::InvalidNode> {
        let id = self.find_node(parent)?;
        Ok(self.forest.children[id].len())
    }

    /// Returns a list of children that belong to the [`Parent`]
    pub fn children(&self, parent: Node) -> Result<Vec<Node>, error::InvalidNode> {
        let id = self.find_node(parent)?;
        Ok(self.forest.children[id].iter().map(|child| self.ids_to_nodes[child]).collect())
    }

    /// Sets the [`Style`] of the provided `node`
    pub fn set_style(&mut self, node: Node, style: FlexboxLayout) -> Result<(), error::InvalidNode> {
        let id = self.find_node(node)?;
        self.forest.nodes[id].style = style;
        self.forest.mark_dirty(id);
        Ok(())
    }

    /// Gets the [`Style`] of the provided `node`
    pub fn style(&self, node: Node) -> Result<&FlexboxLayout, error::InvalidNode> {
        let id = self.find_node(node)?;
        Ok(&self.forest.nodes[id].style)
    }

    /// Return this node layout relative to its parent
    pub fn layout(&self, node: Node) -> Result<&Layout, error::InvalidNode> {
        let id = self.find_node(node)?;
        Ok(&self.forest.nodes[id].layout)
    }

    /// Marks the layout computation of this node and its children as outdated
    pub fn mark_dirty(&mut self, node: Node) -> Result<(), error::InvalidNode> {
        let id = self.find_node(node)?;
        self.forest.mark_dirty(id);
        Ok(())
    }

    /// Indicates whether the layout of this node (and its children) need to be recomputed
    pub fn dirty(&self, node: Node) -> Result<bool, error::InvalidNode> {
        let id = self.find_node(node)?;
        Ok(self.forest.nodes[id].is_dirty)
    }

    /// Updates the stored layout of the provided `node` and its children
    pub fn compute_layout(&mut self, node: Node, size: Size<Option<f32>>) -> Result<(), error::InvalidNode> {
        let id = self.find_node(node)?;
        self.forest.compute(id, size);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        style::{Dimension, Display, FlexDirection},
        sys,
    };

    #[test]
    fn new_should_allocate_default_capacity() {
        const DEFAULT_CAPACITY: usize = 16; // This is the capacity defined in the `impl Default`
        let taffy = Taffy::new();

        assert!(taffy.nodes_to_ids.capacity() >= DEFAULT_CAPACITY);
        assert!(taffy.ids_to_nodes.capacity() >= DEFAULT_CAPACITY);
        assert!(taffy.forest.children.capacity() >= DEFAULT_CAPACITY);
        assert!(taffy.forest.parents.capacity() >= DEFAULT_CAPACITY);
        assert!(taffy.forest.nodes.capacity() >= DEFAULT_CAPACITY);
    }

    #[test]
    fn test_with_capacity() {
        const CAPACITY: usize = 8;
        let taffy = Taffy::with_capacity(CAPACITY);

        assert!(taffy.nodes_to_ids.capacity() >= CAPACITY);
        assert!(taffy.ids_to_nodes.capacity() >= CAPACITY);
        assert!(taffy.forest.children.capacity() >= CAPACITY);
        assert!(taffy.forest.parents.capacity() >= CAPACITY);
        assert!(taffy.forest.nodes.capacity() >= CAPACITY);
    }

    #[test]
    fn add_node() {
        let mut taffy = Taffy::new();
        let node = taffy.allocate_node();
        let id = taffy.forest.new_leaf(FlexboxLayout::default());

        // Should not contain nodes
        assert!(!taffy.nodes_to_ids.contains_key(&node));
        assert!(!taffy.ids_to_nodes.contains_key(&id));

        taffy.add_node(node, id);

        // Should contain nodes
        assert!(taffy.nodes_to_ids.contains_key(&node));
        assert!(taffy.ids_to_nodes.contains_key(&id));
    }

    #[test]
    fn find_node() {
        let mut taffy = Taffy::new();

        let child0 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child1 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]).unwrap();

        // Should find the nodes
        assert!(if let Ok(node_id) = taffy.find_node(node) { node_id == node.local.0 } else { false });
        assert!(if let Ok(node_id) = taffy.find_node(child0) { node_id == child0.local.0 } else { false });
        assert!(if let Ok(node_id) = taffy.find_node(child1) { node_id == child1.local.0 } else { false });

        let _ = taffy.remove(node);
        let _ = taffy.remove(child0);
        let _ = taffy.remove(child1);

        // Should not find the nodes
        assert!(taffy.find_node(node).is_err());
        assert!(taffy.find_node(child0).is_err());
        assert!(taffy.find_node(child1).is_err());
    }

    #[test]
    fn test_new_leaf() {
        let mut taffy = Taffy::new();

        let res = taffy.new_leaf(FlexboxLayout::default());
        assert!(res.is_ok());
        let node = res.unwrap();

        // node should be in the taffy tree and have no children
        assert!(taffy.find_node(node).is_ok());
        assert!(taffy.child_count(node).unwrap() == 0);
    }

    #[test]
    fn new_leaf_with_measure() {
        let mut taffy = Taffy::new();

        let res = taffy.new_leaf_with_measure(FlexboxLayout::default(), MeasureFunc::Raw(|_| Size::ZERO));
        assert!(res.is_ok());
        let node = res.unwrap();

        // node should be in the taffy tree and have no children
        assert!(taffy.find_node(node).is_ok());
        assert!(taffy.child_count(node).unwrap() == 0);
    }

    /// Test that new_with_children works as expected
    #[test]
    fn test_new_with_children() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child1 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]).unwrap();

        // node should have two children
        assert_eq!(taffy.child_count(node).unwrap(), 2);
        assert_eq!(taffy.children(node).unwrap()[0], child0);
        assert_eq!(taffy.children(node).unwrap()[1], child1);
    }

    #[test]
    fn clear_should_clear_nodes() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child1 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]).unwrap();

        // Nodes should be present
        assert!(taffy.find_node(node).is_ok());
        assert!(taffy.find_node(child0).is_ok());
        assert!(taffy.find_node(child1).is_ok());

        taffy.clear();

        // nodes should not be present
        assert!(taffy.find_node(node).is_err());
        assert!(taffy.find_node(child0).is_err());
        assert!(taffy.find_node(child1).is_err());
    }

    #[test]
    fn remove_node_should_remove() {
        let mut taffy = Taffy::new();

        let node = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();

        // node should exist
        assert!(taffy.find_node(node).is_ok());

        let _ = taffy.remove(node).unwrap();

        // node should no longer exist
        assert!(taffy.find_node(node).is_err());
    }

    #[test]
    fn remove_node_should_detach_herarchy() {
        let mut taffy = Taffy::new();

        // Build a linear tree layout: <0> <- <1> <- <2>
        let node2 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let node1 = taffy.new_with_children(FlexboxLayout::default(), &[node2]).unwrap();
        let node0 = taffy.new_with_children(FlexboxLayout::default(), &[node1]).unwrap();

        // Node1 should exist
        assert!(taffy.find_node(node1).is_ok());

        // Both node0 and node1 should have 1 child nodes
        assert_eq!(taffy.children(node0).unwrap().as_slice(), &[node1]);
        assert_eq!(taffy.children(node1).unwrap().as_slice(), &[node2]);

        // Disconnect the tree: <0> <2>
        let _ = taffy.remove(node1).unwrap();

        // Node1 should no longer exist
        assert!(taffy.find_node(node1).is_err());

        // Both remaining nodes should have no child nodes
        assert!(taffy.children(node0).unwrap().is_empty());
        assert!(taffy.children(node2).unwrap().is_empty());
    }

    #[test]
    fn remove_last_node() {
        let mut taffy = Taffy::new();

        let parent = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        taffy.add_child(parent, child).unwrap();

        taffy.remove(child).unwrap();
        taffy.remove(parent).unwrap();
    }

    #[test]
    fn set_measure() {
        let mut taffy = Taffy::new();
        let node = taffy
            .new_leaf_with_measure(FlexboxLayout::default(), MeasureFunc::Raw(|_| Size { width: 200.0, height: 200.0 }))
            .unwrap();
        taffy.compute_layout(node, Size::undefined()).unwrap();
        assert_eq!(taffy.layout(node).unwrap().size.width, 200.0);

        taffy.set_measure(node, Some(MeasureFunc::Raw(|_| Size { width: 100.0, height: 100.0 }))).unwrap();
        taffy.compute_layout(node, Size::undefined()).unwrap();
        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
    }

    /// Test that adding `add_child()` works
    #[test]
    fn add_child() {
        let mut taffy = Taffy::new();
        let node = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 0);

        let child0 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        taffy.add_child(node, child0).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 1);

        let child1 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        taffy.add_child(node, child1).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 2);
    }

    #[test]
    fn set_children() {
        let mut taffy = Taffy::new();

        let child0 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child1 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]).unwrap();

        assert_eq!(taffy.child_count(node).unwrap(), 2);
        assert_eq!(taffy.children(node).unwrap()[0], child0);
        assert_eq!(taffy.children(node).unwrap()[1], child1);

        let child2 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child3 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        taffy.set_children(node, &[child2, child3]).unwrap();

        assert_eq!(taffy.child_count(node).unwrap(), 2);
        assert_eq!(taffy.children(node).unwrap()[0], child2);
        assert_eq!(taffy.children(node).unwrap()[1], child3);
    }

    /// Test that removing a child works
    #[test]
    fn remove_child() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child1 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]).unwrap();

        assert_eq!(taffy.child_count(node).unwrap(), 2);

        taffy.remove_child(node, child0).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 1);
        assert_eq!(taffy.children(node).unwrap()[0], child1);

        taffy.remove_child(node, child1).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 0);
    }

    #[test]
    fn remove_child_at_index() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child1 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]).unwrap();

        assert_eq!(taffy.child_count(node).unwrap(), 2);

        taffy.remove_child_at_index(node, 0).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 1);
        assert_eq!(taffy.children(node).unwrap()[0], child1);

        taffy.remove_child_at_index(node, 0).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 0);
    }

    #[test]
    fn replace_child_at_index() {
        let mut taffy = Taffy::new();

        let child0 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child1 = taffy.new_leaf(FlexboxLayout::default()).unwrap();

        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0]).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 1);
        assert_eq!(taffy.children(node).unwrap()[0], child0);

        taffy.replace_child_at_index(node, 0, child1).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 1);
        assert_eq!(taffy.children(node).unwrap()[0], child1);
    }
    #[test]
    fn test_child_at_index() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child1 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child2 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1, child2]).unwrap();

        assert!(if let Ok(result) = taffy.child_at_index(node, 0) { result == child0 } else { false });
        assert!(if let Ok(result) = taffy.child_at_index(node, 1) { result == child1 } else { false });
        assert!(if let Ok(result) = taffy.child_at_index(node, 2) { result == child2 } else { false });
    }
    #[test]
    fn test_child_count() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child1 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]).unwrap();

        assert!(if let Ok(count) = taffy.child_count(node) { count == 2 } else { false });
        assert!(if let Ok(count) = taffy.child_count(child0) { count == 0 } else { false });
        assert!(if let Ok(count) = taffy.child_count(child1) { count == 0 } else { false });
    }
    #[test]
    fn test_children() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child1 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]).unwrap();

        let mut children: sys::Vec<Node> = sys::Vec::new();
        children.push(child0);
        children.push(child1);

        let children_result = taffy.children(node).unwrap();
        assert_eq!(children_result, children);

        assert!(taffy.children(child0).unwrap().len() == 0);
    }
    #[test]
    fn test_set_style() {
        let mut taffy = Taffy::new();

        let node = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        assert_eq!(taffy.style(node).unwrap().display, Display::Flex);

        taffy.set_style(node, FlexboxLayout { display: Display::None, ..FlexboxLayout::default() }).unwrap();
        assert_eq!(taffy.style(node).unwrap().display, Display::None);
    }
    #[test]
    fn test_style() {
        let mut taffy = Taffy::new();

        let style =
            FlexboxLayout { display: Display::None, flex_direction: FlexDirection::RowReverse, ..Default::default() };

        let node = taffy.new_leaf(style).unwrap();

        let res = taffy.style(node);
        assert!(res.is_ok());
        assert!(res.unwrap() == &style);
    }
    #[test]
    fn test_layout() {
        let mut taffy = Taffy::new();
        let node = taffy.new_leaf(FlexboxLayout::default()).unwrap();

        // TODO: Improve this test?
        let res = taffy.layout(node);
        assert!(res.is_ok());
    }

    #[test]
    fn test_mark_dirty() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let child1 = taffy.new_leaf(FlexboxLayout::default()).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]).unwrap();

        taffy.compute_layout(node, Size::undefined()).unwrap();

        assert_eq!(taffy.dirty(child0).unwrap(), false);
        assert_eq!(taffy.dirty(child1).unwrap(), false);
        assert_eq!(taffy.dirty(node).unwrap(), false);

        taffy.mark_dirty(node).unwrap();
        assert_eq!(taffy.dirty(child0).unwrap(), false);
        assert_eq!(taffy.dirty(child1).unwrap(), false);
        assert_eq!(taffy.dirty(node).unwrap(), true);

        taffy.compute_layout(node, Size::undefined()).unwrap();
        taffy.mark_dirty(child0).unwrap();
        assert_eq!(taffy.dirty(child0).unwrap(), true);
        assert_eq!(taffy.dirty(child1).unwrap(), false);
        assert_eq!(taffy.dirty(node).unwrap(), true);
    }

    #[test]
    fn compute_layout_should_produce_valid_result() {
        let mut taffy = Taffy::new();
        let node_result = taffy.new_leaf(FlexboxLayout {
            size: Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
            ..Default::default()
        });
        assert!(node_result.is_ok());
        let node = node_result.unwrap();
        let layout_result = taffy.compute_layout(node, Size { width: Some(100.), height: Some(100.) });
        assert!(layout_result.is_ok());
    }

    #[test]
    fn measure_func_is_send_and_sync() {
        fn is_send_and_sync<T: Send + Sync>() {}
        is_send_and_sync::<MeasureFunc>();
    }
}
