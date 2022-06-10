//! Forest - a struct-of-arrays data structure for storing node trees.
//!
//! Backing datastructure for `Taffy` structs.
use crate::geometry::Size;
use crate::layout::{Cache, Layout};
use crate::node::{MeasureFunc, NodeId};
use crate::number::Number;
use crate::style::Style;
use crate::sys::{new_vec_with_capacity, ChildrenVec, ParentsVec, Vec};

/// Layout information for a given [`Node`](crate::node::Node)
///
/// Stored in a [`Forest`].
pub(crate) struct NodeData {
    /// The layout strategy used by this node
    pub(crate) style: Style,
    /// The mapping from the Size<Number> (in units) to Size<f32> (in points) for this node
    pub(crate) measure: Option<MeasureFunc>,
    /// The results of the layout computation
    pub(crate) layout: Layout,
    /// The primary cached results of the layout computation
    pub(crate) main_size_layout_cache: Option<Cache>,
    /// Secondary cached results of the layout computation
    pub(crate) other_layout_cache: Option<Cache>,
    /// Does this node's layout need to be recomputed?
    pub(crate) is_dirty: bool,
}

impl NodeData {
    /// Create the data for a new leaf node
    #[must_use]
    fn new_leaf(style: Style, measure: MeasureFunc) -> Self {
        Self {
            style,
            measure: Some(measure),
            main_size_layout_cache: None,
            other_layout_cache: None,
            layout: Layout::new(),
            is_dirty: true,
        }
    }

    /// Create the data for a new node
    // TODO: why is this different from new_leaf?
    #[must_use]
    fn new(style: Style) -> Self {
        Self {
            style,
            measure: None,
            main_size_layout_cache: None,
            other_layout_cache: None,
            layout: Layout::new(),
            is_dirty: true,
        }
    }

    /// Marks a node and all of its parents (recursively) as dirty
    ///
    /// This clears any cached data and signals that the data must be recomputed.
    #[inline]
    fn mark_dirty(&mut self) {
        self.main_size_layout_cache = None;
        self.other_layout_cache = None;
        self.is_dirty = true;
    }
}

/// A collection of UI layout trees used to store [`NodeData`] associated with specific [`Nodes`](crate::node::Node)
pub(crate) struct Forest {
    /// The [`NodeData`] for each node stored in this forest
    pub(crate) nodes: Vec<NodeData>,
    /// The children of each node
    ///
    /// The indexes in the outer vector correspond to the position of the parent [`NodeData`]
    pub(crate) children: Vec<ChildrenVec<NodeId>>,
    /// The parents of each node
    ///
    /// The indexes in the outer vector correspond to the position of the child [`NodeData`]
    pub(crate) parents: Vec<ParentsVec<NodeId>>,
}

impl Forest {
    /// Creates a new [`Forest`] that can store up to `capacity` [`Nodes`](crate::node::Node) before needing to be expanded
    #[must_use]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: new_vec_with_capacity(capacity),
            children: new_vec_with_capacity(capacity),
            parents: new_vec_with_capacity(capacity),
        }
    }

    /// Adds a new unattached leaf node to the forest, and returns the [`NodeId`] of the new node
    pub(crate) fn new_leaf(&mut self, style: Style, measure: MeasureFunc) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(NodeData::new_leaf(style, measure));
        self.children.push(new_vec_with_capacity(0));
        self.parents.push(new_vec_with_capacity(1));
        id
    }

    /// Adds a new unparented node to the forest with the associated children attached, and returns the [`NodeId`] of the new node
    pub(crate) fn new_node(&mut self, style: Style, children: ChildrenVec<NodeId>) -> NodeId {
        let id = self.nodes.len();
        for child in &children {
            self.parents[*child].push(id);
        }
        self.nodes.push(NodeData::new(style));
        self.children.push(children);
        self.parents.push(new_vec_with_capacity(1));
        id
    }

    /// Adds a `child` node to the `parent` node
    pub(crate) fn add_child(&mut self, parent: NodeId, child: NodeId) {
        self.parents[child].push(parent);
        self.children[parent].push(child);
        self.mark_dirty(parent)
    }

    /// Removes all nodes and resets the data structure
    ///
    /// The capacity is retained.
    pub(crate) fn clear(&mut self) {
        self.nodes.clear();
        self.children.clear();
        self.parents.clear();
    }

    /// Removes the specified `node`
    ///
    /// The last existing node is moved to its previous position, in order to ensure compactness.
    /// Returns the previous [`NodeId`] of the moved node, if one was moved.
    pub(crate) fn swap_remove(&mut self, node: NodeId) -> Option<NodeId> {
        self.nodes.swap_remove(node);

        // Now the last element is swapped in at index `node`.
        if self.nodes.is_empty() {
            self.children.clear();
            self.parents.clear();
            return None;
        }

        // Remove old node as parent from all its chilren.
        for child in &self.children[node] {
            let parents_child = &mut self.parents[*child];
            let mut pos = 0;
            while pos < parents_child.len() {
                if parents_child[pos] == node {
                    parents_child.swap_remove(pos);
                } else {
                    pos += 1;
                }
            }
        }

        // Remove old node as child from all its parents.
        for parent in &self.parents[node] {
            let childrens_parent = &mut self.children[*parent];
            let mut pos = 0;
            while pos < childrens_parent.len() {
                if childrens_parent[pos] == node {
                    childrens_parent.swap_remove(pos);
                } else {
                    pos += 1;
                }
            }
        }

        let last = self.nodes.len();

        if last != node {
            // Update ids for every child of the swapped in node.
            for child in &self.children[last] {
                for parent in &mut self.parents[*child] {
                    if *parent == last {
                        *parent = node;
                    }
                }
            }

            // Update ids for every parent of the swapped in node.
            for parent in &self.parents[last] {
                for child in &mut self.children[*parent] {
                    if *child == last {
                        *child = node;
                    }
                }
            }

            self.children.swap_remove(node);
            self.parents.swap_remove(node);

            Some(last)
        } else {
            self.children.swap_remove(node);
            self.parents.swap_remove(node);
            None
        }
    }

    /// Breaks the link between the `parent` node and the `child` node
    ///
    /// The `child`'s data is not removed.
    pub(crate) fn remove_child(&mut self, parent: NodeId, child: NodeId) -> NodeId {
        let index = self.children[parent].iter().position(|n| *n == child).unwrap();
        self.remove_child_at_index(parent, index)
    }

    /// Breaks the link between the `parent` node and the n-th child node
    ///
    /// The child's data is not removed.
    pub(crate) fn remove_child_at_index(&mut self, parent: NodeId, child_index: usize) -> NodeId {
        let child = self.children[parent].remove(child_index);
        self.parents[child].retain(|p| *p != parent);
        self.mark_dirty(parent);
        child
    }

    /// Marks the `node` as needing layout recalculation
    ///
    /// Any cached layout information is cleared.
    pub(crate) fn mark_dirty(&mut self, node: NodeId) {
        /// Performs a recursive depth-first search up the tree until the root node is reached
        ///
        ///  WARNING: this will stack-overflow if the tree contains a cycle
        fn mark_dirty_recursive(nodes: &mut Vec<NodeData>, parents: &[ParentsVec<NodeId>], node_id: NodeId) {
            nodes[node_id].mark_dirty();

            for parent in &parents[node_id] {
                mark_dirty_recursive(nodes, parents, *parent);
            }
        }

        mark_dirty_recursive(&mut self.nodes, &self.parents, node);
    }

    /// Computes the layout of the `node` and its children
    pub(crate) fn compute_layout(&mut self, node: NodeId, size: Size<Number>) {
        // TODO: It's not clear why this method is distinct
        self.compute(node, size)
    }
}
