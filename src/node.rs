//! UI [`Node`] types and related data structures.
//!
//! Layouts are composed of multiple nodes, which live in a forest-like data structure.
use slotmap::SlotMap;

/// A node in a layout.
pub type Node = slotmap::DefaultKey;

use crate::error::InvalidNode;
use crate::geometry::Size;
use crate::layout::Layout;
use crate::style::FlexboxLayout;
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::sys::Box;
use crate::sys::{new_vec_with_capacity, ChildrenVec, Vec};
use crate::{error, forest::NodeData};

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

/// A forest of UI [`Nodes`](`Node`), suitable for UI layout
pub struct Taffy {
    /// The [`NodeData`] for each node stored in this forest
    pub(crate) nodes: SlotMap<Node, NodeData>,

    /// The children of each node
    ///
    /// The indexes in the outer vector correspond to the position of the parent [`NodeData`]
    pub(crate) children: SlotMap<Node, ChildrenVec<Node>>,

    /// The parents of each node
    ///
    /// The indexes in the outer vector correspond to the position of the child [`NodeData`]
    pub(crate) parents: SlotMap<Node, Option<Node>>,
}

impl Default for Taffy {
    fn default() -> Self {
        Self::with_capacity(16)
    }
}

#[allow(clippy::iter_cloned_collect)] // due to no-std support, we need to use `iter_cloned` instead of `collect`
impl Taffy {
    /// Creates a new [`Taffy`]
    ///
    /// The default capacity of a [`Taffy`] is 16 nodes.
    #[must_use]
    pub fn new() -> Self {
        Taffy::default()
    }

    /// Creates a new [`Taffy`] that can store `capacity` nodes before reallocation
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: SlotMap::with_capacity(capacity),
            children: SlotMap::with_capacity(capacity),
            parents: SlotMap::with_capacity(capacity),
        }
    }

    /// Creates and adds a new unattached leaf node to the forest, and returns the [`NodeId`] of the new node
    pub fn new_leaf(&mut self, layout: FlexboxLayout) -> Node {
        let id = self.nodes.insert(NodeData::new(layout));
        let _ = self.children.insert(new_vec_with_capacity(0));
        let _ = self.parents.insert(None);

        id
    }

    /// Creates and adds a new unattached leaf node to the forest, and returns the [`NodeId`] of the new node
    ///
    /// Creates and adds a new leaf node with a supplied [`MeasureFunc`]
    pub fn new_leaf_with_measure(&mut self, layout: FlexboxLayout, measure: MeasureFunc) -> Node {
        let id = self.nodes.insert(NodeData::new_with_measure(layout, measure));
        let _ = self.children.insert(new_vec_with_capacity(0));
        let _ = self.parents.insert(None);

        id
    }

    /// Creates and adds a new node, which may have any number of `children`
    pub fn new_with_children(&mut self, layout: FlexboxLayout, children: &[Node]) -> Node {
        let id = self.nodes.insert(NodeData::new(layout));

        for child in children {
            self.parents[*child] = Some(id);
        }

        let _ = self.children.insert(children.iter().copied().collect::<_>());
        let _ = self.parents.insert(None);

        id
    }

    /// Removes all nodes
    ///
    /// All associated [`Id`] will be rendered invalid.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.children.clear();
        self.parents.clear();
    }

    /// Remove a specific [`Node`] from the tree
    ///
    /// Its [`Id`] is marked as invalid. Returns the id of the node removed.
    pub fn remove(&mut self, node: Node) -> Node {
        if let Some(children) = self.children.get_mut(self.parents[node].unwrap()) {
            children.retain(|f| *f != node);
        }

        let _ = self.children.remove(node);
        let _ = self.parents.remove(node);
        let _ = self.nodes.remove(node);

        node
    }

    /// Sets the [`MeasureFunc`] of the associated node
    pub fn set_measure(&mut self, node: Node, measure: Option<MeasureFunc>) {
        self.nodes[node].measure = measure;
        self.mark_dirty(node);
    }

    /// Adds a `child` [`Node`] under the supplied `parent`
    pub fn add_child(&mut self, parent: Node, child: Node) {
        self.parents[child] = Some(parent);
        self.children[parent].push(child);
        self.mark_dirty(parent);
    }

    /// Directly sets the `children` of the supplied `parent`
    pub fn set_children(&mut self, parent: Node, children: &[Node]) {
        // Remove node as parent from all its current children.
        for child in &self.children[parent] {
            self.parents[*child] = None;
        }

        // Build up relation node <-> child
        for child in children {
            self.parents[*child] = Some(parent);
        }

        self.children[parent] = children.iter().copied().collect::<_>();

        self.mark_dirty(parent);
    }

    /// Removes the `child` of the parent `node`
    ///
    /// The child is not removed from the forest entirely, it is simply no longer attached to its previous parent.
    pub fn remove_child(&mut self, parent: Node, child: Node) -> Node {
        let index = self.children[parent].iter().position(|n| *n == child).unwrap();
        self.remove_child_at_index(parent, index).unwrap()
    }

    /// Removes the child at the given `index` from the `parent`
    ///
    /// The child is not removed from the forest entirely, it is simply no longer attached to its previous parent.
    pub fn remove_child_at_index(&mut self, parent: Node, child_index: usize) -> Result<Node, error::InvalidChild> {
        let child_count = self.children[parent].len();
        if child_index >= child_count {
            return Err(error::InvalidChild::ChildIndexOutOfBounds { parent, child_index, child_count });
        }

        let child = self.children[parent].remove(child_index);
        self.parents[child] = None;

        self.mark_dirty(parent);

        Ok(child)
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
        let child_count = self.children[parent].len();
        if child_index >= child_count {
            return Err(error::InvalidChild::ChildIndexOutOfBounds { parent, child_index, child_count });
        }

        self.parents[new_child] = Some(parent);
        let old_child = core::mem::replace(&mut self.children[parent][child_index], new_child);
        self.parents[old_child] = None;

        self.mark_dirty(parent);

        Ok(old_child)
    }

    /// Returns the child [`Node`] of the parent `node` at the provided `child_index`
    pub fn child_at_index(&self, parent: Node, child_index: usize) -> Result<Node, error::InvalidChild> {
        let child_count = self.children[parent].len();
        if child_index >= child_count {
            return Err(error::InvalidChild::ChildIndexOutOfBounds { parent, child_index, child_count });
        }

        Ok(self.children[parent][child_index])
    }

    /// Returns the number of children of the `parent` [`Node`]
    pub fn child_count(&self, parent: Node) -> usize {
        self.children[parent].len()
    }

    /// Returns a list of children that belong to the [`Parent`]
    pub fn children(&self, parent: Node) -> Vec<Node> {
        self.children[parent].iter().copied().collect::<_>()
    }

    /// Sets the [`Style`] of the provided `node`
    pub fn set_style(&mut self, node: Node, style: FlexboxLayout) {
        self.nodes[node].style = style;
        self.mark_dirty(node);
    }

    /// Gets the [`Style`] of the provided `node`
    pub fn style(&self, node: Node) -> &FlexboxLayout {
        &self.nodes[node].style
    }

    /// Return this node layout relative to its parent
    pub fn layout(&self, node: Node) -> &Layout {
        &self.nodes[node].layout
    }

    /// Marks the layout computation of this node and its children as outdated
    pub fn mark_dirty(&mut self, node: Node) {
        /// Performs a recursive depth-first search up the tree until the root node is reached
        ///
        ///  WARNING: this will stack-overflow if the tree contains a cycle
        fn mark_dirty_recursive(
            nodes: &mut SlotMap<Node, NodeData>,
            parents: &SlotMap<Node, Option<Node>>,
            node_id: Node,
        ) {
            nodes[node_id].mark_dirty();

            if let Some(Some(node)) = parents.get(node_id) {
                mark_dirty_recursive(nodes, parents, *node);
            }
        }

        mark_dirty_recursive(&mut self.nodes, &self.parents, node);
    }

    /// Indicates whether the layout of this node (and its children) need to be recomputed
    pub fn dirty(&self, node: Node) -> bool {
        self.nodes[node].is_dirty
    }

    /// Updates the stored layout of the provided `node` and its children
    pub fn compute_layout(&mut self, node: Node, size: Size<Option<f32>>) -> Result<(), InvalidNode> {
        self.compute(node, size);
        Ok(())
    }
}

#[allow(clippy::bool_assert_comparison)]
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

        assert!(taffy.children.capacity() >= DEFAULT_CAPACITY);
        assert!(taffy.parents.capacity() >= DEFAULT_CAPACITY);
        assert!(taffy.nodes.capacity() >= DEFAULT_CAPACITY);
    }

    #[test]
    fn test_with_capacity() {
        const CAPACITY: usize = 8;
        let taffy = Taffy::with_capacity(CAPACITY);

        assert!(taffy.children.capacity() >= CAPACITY);
        assert!(taffy.parents.capacity() >= CAPACITY);
        assert!(taffy.nodes.capacity() >= CAPACITY);
    }

    // #[test]
    // fn find_node() {
    //     let mut taffy = Taffy::new();

    //     let child0 = taffy.new_leaf(FlexboxLayout::default());
    //     let child1 = taffy.new_leaf(FlexboxLayout::default());
    //     let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]).unwrap();

    //     // Should find the nodes
    //     assert!(if let Ok(node_id) = taffy.find_node(node) { node_id == node.local.0 } else { false });
    //     assert!(if let Ok(node_id) = taffy.find_node(child0) { node_id == child0.local.0 } else { false });
    //     assert!(if let Ok(node_id) = taffy.find_node(child1) { node_id == child1.local.0 } else { false });

    //     let _ = taffy.remove(node);
    //     let _ = taffy.remove(child0);
    //     let _ = taffy.remove(child1);

    //     // Should not find the nodes
    //     assert!(taffy.find_node(node).is_err());
    //     assert!(taffy.find_node(child0).is_err());
    //     assert!(taffy.find_node(child1).is_err());
    // }

    // #[test]
    // fn test_new_leaf() {
    //     let mut taffy = Taffy::new();

    //     let res = taffy.new_leaf(FlexboxLayout::default());
    //     assert!(res.is_ok());
    //     let node = res.unwrap();

    //     // node should be in the taffy tree and have no children
    //     assert!(taffy.find_node(node).is_ok());
    //     assert!(taffy.child_count(node).unwrap() == 0);
    // }

    // #[test]
    // fn new_leaf_with_measure() {
    //     let mut taffy = Taffy::new();

    //     let res = taffy.new_leaf_with_measure(FlexboxLayout::default(), MeasureFunc::Raw(|_| Size::ZERO));
    //     assert!(res.is_ok());
    //     let node = res.unwrap();

    //     // node should be in the taffy tree and have no children
    //     assert!(taffy.find_node(node).is_ok());
    //     assert!(taffy.child_count(node).unwrap() == 0);
    // }

    // /// Test that new_with_children works as expected
    // #[test]
    // fn test_new_with_children() {
    //     let mut taffy = Taffy::new();
    //     let child0 = taffy.new_leaf(FlexboxLayout::default());
    //     let child1 = taffy.new_leaf(FlexboxLayout::default());
    //     let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]).unwrap();

    //     // node should have two children
    //     assert_eq!(taffy.child_count(node).unwrap(), 2);
    //     assert_eq!(taffy.children(node).unwrap()[0], child0);
    //     assert_eq!(taffy.children(node).unwrap()[1], child1);
    // }

    // #[test]
    // fn clear_should_clear_nodes() {
    //     let mut taffy = Taffy::new();
    //     let child0 = taffy.new_leaf(FlexboxLayout::default());
    //     let child1 = taffy.new_leaf(FlexboxLayout::default());
    //     let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]).unwrap();

    //     // Nodes should be present
    //     assert!(taffy.find_node(node).is_ok());
    //     assert!(taffy.find_node(child0).is_ok());
    //     assert!(taffy.find_node(child1).is_ok());

    //     taffy.clear();

    //     // nodes should not be present
    //     assert!(taffy.find_node(node).is_err());
    //     assert!(taffy.find_node(child0).is_err());
    //     assert!(taffy.find_node(child1).is_err());
    // }

    // #[test]
    // fn remove_node_should_remove() {
    //     let mut taffy = Taffy::new();

    //     let node = taffy.new_leaf(FlexboxLayout::default());

    //     // node should exist
    //     assert!(taffy.find_node(node).is_ok());

    //     let _ = taffy.remove(node).unwrap();

    //     // node should no longer exist
    //     assert!(taffy.find_node(node).is_err());
    // }

    #[test]
    fn remove_node_should_detach_herarchy() {
        let mut taffy = Taffy::new();

        // Build a linear tree layout: <0> <- <1> <- <2>
        let node2 = taffy.new_leaf(FlexboxLayout::default());
        let node1 = taffy.new_with_children(FlexboxLayout::default(), &[node2]);
        let node0 = taffy.new_with_children(FlexboxLayout::default(), &[node1]);

        // Node1 should exist
        assert!(taffy.nodes.get(node1).is_some());

        // Both node0 and node1 should have 1 child nodes
        assert_eq!(taffy.children(node0).as_slice(), &[node1]);
        assert_eq!(taffy.children(node1).as_slice(), &[node2]);

        // Disconnect the tree: <0> <2>
        let _ = taffy.remove(node1);

        // Node1 should no longer exist
        assert!(taffy.nodes.get(node1).is_none());

        // Both remaining nodes should have no child nodes
        assert!(taffy.children(node0).is_empty());
        assert!(taffy.children(node2).is_empty());
    }

    #[test]
    fn remove_last_node() {
        let mut taffy = Taffy::new();

        let parent = taffy.new_leaf(FlexboxLayout::default());
        let child = taffy.new_leaf(FlexboxLayout::default());
        taffy.add_child(parent, child);

        taffy.remove(child);
        taffy.remove(parent);
    }

    #[test]
    fn set_measure() {
        let mut taffy = Taffy::new();
        let node = taffy.new_leaf_with_measure(
            FlexboxLayout::default(),
            MeasureFunc::Raw(|_| Size { width: 200.0, height: 200.0 }),
        );

        taffy.compute_layout(node, Size::undefined());
        assert_eq!(taffy.layout(node).size.width, 200.0);

        taffy.set_measure(node, Some(MeasureFunc::Raw(|_| Size { width: 100.0, height: 100.0 })));
        taffy.compute_layout(node, Size::undefined());
        assert_eq!(taffy.layout(node).size.width, 100.0);
    }

    /// Test that adding `add_child()` works
    #[test]
    fn add_child() {
        let mut taffy = Taffy::new();
        let node = taffy.new_leaf(FlexboxLayout::default());
        assert_eq!(taffy.child_count(node), 0);

        let child0 = taffy.new_leaf(FlexboxLayout::default());
        taffy.add_child(node, child0);
        assert_eq!(taffy.child_count(node), 1);

        let child1 = taffy.new_leaf(FlexboxLayout::default());
        taffy.add_child(node, child1);
        assert_eq!(taffy.child_count(node), 2);
    }

    #[test]
    fn set_children() {
        let mut taffy = Taffy::new();

        let child0 = taffy.new_leaf(FlexboxLayout::default());
        let child1 = taffy.new_leaf(FlexboxLayout::default());
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]);

        assert_eq!(taffy.child_count(node), 2);
        assert_eq!(taffy.children(node)[0], child0);
        assert_eq!(taffy.children(node)[1], child1);

        let child2 = taffy.new_leaf(FlexboxLayout::default());
        let child3 = taffy.new_leaf(FlexboxLayout::default());
        taffy.set_children(node, &[child2, child3]);

        assert_eq!(taffy.child_count(node), 2);
        assert_eq!(taffy.children(node)[0], child2);
        assert_eq!(taffy.children(node)[1], child3);
    }

    /// Test that removing a child works
    #[test]
    fn remove_child() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default());
        let child1 = taffy.new_leaf(FlexboxLayout::default());
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]);

        assert_eq!(taffy.child_count(node), 2);

        taffy.remove_child(node, child0);
        assert_eq!(taffy.child_count(node), 1);
        assert_eq!(taffy.children(node)[0], child1);

        taffy.remove_child(node, child1);
        assert_eq!(taffy.child_count(node), 0);
    }

    #[test]
    fn remove_child_at_index() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default());
        let child1 = taffy.new_leaf(FlexboxLayout::default());
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]);

        assert_eq!(taffy.child_count(node), 2);

        taffy.remove_child_at_index(node, 0).unwrap();
        assert_eq!(taffy.child_count(node), 1);
        assert_eq!(taffy.children(node)[0], child1);

        taffy.remove_child_at_index(node, 0).unwrap();
        assert_eq!(taffy.child_count(node), 0);
    }

    #[test]
    fn replace_child_at_index() {
        let mut taffy = Taffy::new();

        let child0 = taffy.new_leaf(FlexboxLayout::default());
        let child1 = taffy.new_leaf(FlexboxLayout::default());

        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0]);
        assert_eq!(taffy.child_count(node), 1);
        assert_eq!(taffy.children(node)[0], child0);

        taffy.replace_child_at_index(node, 0, child1).unwrap();
        assert_eq!(taffy.child_count(node), 1);
        assert_eq!(taffy.children(node)[0], child1);
    }
    #[test]
    fn test_child_at_index() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default());
        let child1 = taffy.new_leaf(FlexboxLayout::default());
        let child2 = taffy.new_leaf(FlexboxLayout::default());
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1, child2]);

        assert!(if let Ok(result) = taffy.child_at_index(node, 0) { result == child0 } else { false });
        assert!(if let Ok(result) = taffy.child_at_index(node, 1) { result == child1 } else { false });
        assert!(if let Ok(result) = taffy.child_at_index(node, 2) { result == child2 } else { false });
    }
    #[test]
    fn test_child_count() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default());
        let child1 = taffy.new_leaf(FlexboxLayout::default());
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]);

        assert_eq!(taffy.child_count(node), 2);
        assert_eq!(taffy.child_count(child0), 0);
        assert_eq!(taffy.child_count(child1), 0);
    }

    #[allow(clippy::all)]
    #[test]
    fn test_children() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default());
        let child1 = taffy.new_leaf(FlexboxLayout::default());
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]);

        let mut children: sys::Vec<Node> = sys::Vec::new();
        children.push(child0);
        children.push(child1);

        let children_result = taffy.children(node);
        assert_eq!(children_result, children);

        assert!(taffy.children(child0).is_empty());
    }
    #[test]
    fn test_set_style() {
        let mut taffy = Taffy::new();

        let node = taffy.new_leaf(FlexboxLayout::default());
        assert_eq!(taffy.style(node).display, Display::Flex);

        taffy.set_style(node, FlexboxLayout { display: Display::None, ..FlexboxLayout::default() });
        assert_eq!(taffy.style(node).display, Display::None);
    }
    #[test]
    fn test_style() {
        let mut taffy = Taffy::new();

        let style =
            FlexboxLayout { display: Display::None, flex_direction: FlexDirection::RowReverse, ..Default::default() };

        let node = taffy.new_leaf(style);

        let res = taffy.style(node);
        assert!(res == &style);
    }
    #[test]
    fn test_layout() {
        let mut taffy = Taffy::new();
        let node = taffy.new_leaf(FlexboxLayout::default());

        // TODO: Improve this test?
        taffy.layout(node);
    }

    #[test]
    fn test_mark_dirty() {
        let mut taffy = Taffy::new();
        let child0 = taffy.new_leaf(FlexboxLayout::default());
        let child1 = taffy.new_leaf(FlexboxLayout::default());
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child0, child1]);

        taffy.compute_layout(node, Size::undefined());

        assert_eq!(taffy.dirty(child0), false);
        assert_eq!(taffy.dirty(child1), false);
        assert_eq!(taffy.dirty(node), false);

        taffy.mark_dirty(node);
        assert_eq!(taffy.dirty(child0), false);
        assert_eq!(taffy.dirty(child1), false);
        assert_eq!(taffy.dirty(node), true);

        taffy.compute_layout(node, Size::undefined());
        taffy.mark_dirty(child0);
        assert_eq!(taffy.dirty(child0), true);
        assert_eq!(taffy.dirty(child1), false);
        assert_eq!(taffy.dirty(node), true);
    }

    #[test]
    fn compute_layout_should_produce_valid_result() {
        let mut taffy = Taffy::new();
        let node = taffy.new_leaf(FlexboxLayout {
            size: Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
            ..Default::default()
        });
        taffy.compute_layout(node, Size { width: Some(100.), height: Some(100.) });
    }

    #[test]
    fn measure_func_is_send_and_sync() {
        fn is_send_and_sync<T: Send + Sync>() {}
        is_send_and_sync::<MeasureFunc>();
    }
}
