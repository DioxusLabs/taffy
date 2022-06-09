//! UI [`Node`] types and related data structures.
//!
//! Layouts are composed of multiple nodes, which live in a forest-like data structure.
use crate::forest::Forest;
use crate::geometry::Size;
use crate::layout::Layout;
use crate::number::Number;
use crate::style::Style;
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::sys::Box;
use crate::sys::{new_map_with_capacity, ChildrenVec, Map, Vec};
use crate::Error;
use core::sync::atomic::{AtomicUsize, Ordering};

/// A function that can be applied to a `Size<Number>` to obetain a `<Size<f32>`
pub enum MeasureFunc {
    /// Stores a unboxed function
    Raw(fn(Size<Number>) -> Size<f32>),
    /// Stores a boxed function
    #[cfg(any(feature = "std", feature = "alloc"))]
    Boxed(Box<dyn Fn(Size<Number>) -> Size<f32>>),
}

/// Global sprawl instance id allocator.
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
pub struct Sprawl {
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

impl Default for Sprawl {
    fn default() -> Self {
        Self::with_capacity(16)
    }
}

impl Sprawl {
    /// Creates a new [`Sprawl`]
    ///
    /// The default capacity of a [`Sprawl`] is 16 nodes.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new [`Sprawl`] that can store `capacity` nodes before reallocation
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

    /// Stores a new
    fn add_node(&mut self, node: Node, id: NodeId) {
        let _ = self.nodes_to_ids.insert(node, id);
        let _ = self.ids_to_nodes.insert(id, node);
    }

    // Returns the `NodeId` of the provided node within the forest
    fn find_node(&self, node: Node) -> Result<NodeId, Error> {
        match self.nodes_to_ids.get(&node) {
            Some(id) => Ok(*id),
            None => Err(Error::InvalidNode(node)),
        }
    }

    /// Adds a new leaf node, which does not have any children
    pub fn new_leaf(&mut self, style: Style, measure: MeasureFunc) -> Result<Node, Error> {
        let node = self.allocate_node();
        let id = self.forest.new_leaf(style, measure);
        self.add_node(node, id);
        Ok(node)
    }

    /// Adds a new node, which may have any number of `children`
    pub fn new_node(&mut self, style: Style, children: &[Node]) -> Result<Node, Error> {
        let node = self.allocate_node();
        let children =
            children.iter().map(|child| self.find_node(*child)).collect::<Result<ChildrenVec<_>, Error>>()?;
        let id = self.forest.new_node(style, children);
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
    /// Its [`Id`] is rendered invalid
    pub fn remove(&mut self, node: Node) {
        let id = if let Ok(id) = self.find_node(node) { id } else { return };

        self.nodes_to_ids.remove(&node);
        self.ids_to_nodes.remove(&id);

        if let Some(new_id) = self.forest.swap_remove(id) {
            let new = self.ids_to_nodes.remove(&new_id).unwrap();
            let _ = self.nodes_to_ids.insert(new, id);
            let _ = self.ids_to_nodes.insert(id, new);
        }
    }

    /// Sets the [`MeasureFunc`] of the associated node
    pub fn set_measure(&mut self, node: Node, measure: Option<MeasureFunc>) -> Result<(), Error> {
        let id = self.find_node(node)?;
        self.forest.nodes[id].measure = measure;
        self.forest.mark_dirty(id);
        Ok(())
    }

    /// Adds a `child` [`Node`] under the supplied `node`
    pub fn add_child(&mut self, node: Node, child: Node) -> Result<(), Error> {
        let node_id = self.find_node(node)?;
        let child_id = self.find_node(child)?;

        self.forest.add_child(node_id, child_id);
        Ok(())
    }

    /// Directly sets the `children` of the supplied `Node`
    pub fn set_children(&mut self, node: Node, children: &[Node]) -> Result<(), Error> {
        let node_id = self.find_node(node)?;
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
    pub fn remove_child(&mut self, node: Node, child: Node) -> Result<Node, Error> {
        let node_id = self.find_node(node)?;
        let child_id = self.find_node(child)?;

        let prev_id = self.forest.remove_child(node_id, child_id);
        Ok(self.ids_to_nodes[&prev_id])
    }

    /// Removes the "n-th" child from the parent `node`
    ///
    /// The child is not removed from the forest entirely, it is simply no longer attached to its previous parent.
    pub fn remove_child_at_index(&mut self, node: Node, index: usize) -> Result<Node, Error> {
        let node_id = self.find_node(node)?;
        // TODO: index check

        let prev_id = self.forest.remove_child_at_index(node_id, index);
        Ok(self.ids_to_nodes[&prev_id])
    }

    /// Replaces the "n-th" child from the parent `node` with the new `child` node
    ///
    /// The child is not removed from the forest entirely, it is simply no longer attached to its previous parent.
    pub fn replace_child_at_index(&mut self, node: Node, index: usize, child: Node) -> Result<Node, Error> {
        let node_id = self.find_node(node)?;
        let child_id = self.find_node(child)?;
        // TODO: index check

        self.forest.parents[child_id].push(node_id);
        let old_child = core::mem::replace(&mut self.forest.children[node_id][index], child_id);
        self.forest.parents[old_child].retain(|p| *p != node_id);

        self.forest.mark_dirty(node_id);

        Ok(self.ids_to_nodes[&old_child])
    }

    /// Returns a list of children of the given [`Node`]
    pub fn children(&self, node: Node) -> Result<Vec<Node>, Error> {
        let id = self.find_node(node)?;
        Ok(self.forest.children[id].iter().map(|child| self.ids_to_nodes[child]).collect())
    }

    /// Returns the child [`Node`] of the parent `node` at the provided `index`
    pub fn child_at_index(&self, node: Node, index: usize) -> Result<Node, Error> {
        let id = self.find_node(node)?;
        Ok(self.ids_to_nodes[&self.forest.children[id][index]])
    }

    /// Returns the number of children of the parent `node`
    pub fn child_count(&self, node: Node) -> Result<usize, Error> {
        let id = self.find_node(node)?;
        Ok(self.forest.children[id].len())
    }

    /// Sets the [`Style`] of the provided `node`
    pub fn set_style(&mut self, node: Node, style: Style) -> Result<(), Error> {
        let id = self.find_node(node)?;
        self.forest.nodes[id].style = style;
        self.forest.mark_dirty(id);
        Ok(())
    }

    /// Gets the [`Style`] of the provided `node`
    pub fn style(&self, node: Node) -> Result<&Style, Error> {
        let id = self.find_node(node)?;
        Ok(&self.forest.nodes[id].style)
    }

    /// Return this node layout relative to its parent
    pub fn layout(&self, node: Node) -> Result<&Layout, Error> {
        let id = self.find_node(node)?;
        Ok(&self.forest.nodes[id].layout)
    }

    /// Indicates that the layout of this node and its children must be recomputed
    pub fn mark_dirty(&mut self, node: Node) -> Result<(), Error> {
        let id = self.find_node(node)?;
        self.forest.mark_dirty(id);
        Ok(())
    }

    /// Does the layout of this node (and its children) need to be recomputed
    pub fn dirty(&self, node: Node) -> Result<bool, Error> {
        let id = self.find_node(node)?;
        Ok(self.forest.nodes[id].is_dirty)
    }

    /// Updates the stored layout of the provided `node` and its children
    pub fn compute_layout(&mut self, node: Node, size: Size<Number>) -> Result<(), Error> {
        let id = self.find_node(node)?;
        self.forest.compute_layout(id, size);
        Ok(())
    }
}

/// Internal node id.
pub(crate) type NodeId = usize;

/// The identifier of a [`Node`]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(not(any(feature = "std", feature = "alloc")), derive(hash32_derive::Hash32))]
pub(crate) struct Id(usize);

/// An bump-allocator index that tracks how many [`Nodes`](Node) have been allocated in a [`Sprawl`].
pub(crate) struct Allocator {
    new_id: AtomicUsize,
}

impl Allocator {
    /// Creates a fresh [`Allocator`]
    pub const fn new() -> Self {
        Self { new_id: AtomicUsize::new(0) }
    }

    /// Allocates space for one more [`Node`]
    pub fn allocate(&self) -> Id {
        Id(self.new_id.fetch_add(1, Ordering::Relaxed))
    }
}
