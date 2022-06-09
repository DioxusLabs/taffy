//! Forest - ECS like datastructure for storing node trees.
//!
//! Backing datastructure for `Sprawl` structs.
use crate::geometry::Size;
use crate::layout::{Cache, Layout};
use crate::node::{MeasureFunc, NodeId};
use crate::number::Number;
use crate::style::Style;
use crate::sys::{new_vec_with_capacity, ChildrenVec, ParentsVec, Vec};

pub(crate) struct NodeData {
    pub(crate) style: Style,
    pub(crate) measure: Option<MeasureFunc>,
    pub(crate) layout: Layout,
    pub(crate) main_size_layout_cache: Option<Cache>,
    pub(crate) other_layout_cache: Option<Cache>,
    pub(crate) is_dirty: bool,
}

impl NodeData {
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

pub(crate) struct Forest {
    pub(crate) nodes: Vec<NodeData>,
    pub(crate) children: Vec<ChildrenVec<NodeId>>,
    pub(crate) parents: Vec<ParentsVec<NodeId>>,
}

impl Forest {
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: new_vec_with_capacity(capacity),
            children: new_vec_with_capacity(capacity),
            parents: new_vec_with_capacity(capacity),
        }
    }

    pub fn new_leaf(&mut self, style: Style, measure: MeasureFunc) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(NodeData::new_leaf(style, measure));
        self.children.push(new_vec_with_capacity(0));
        self.parents.push(new_vec_with_capacity(1));
        id
    }

    pub fn new_node(&mut self, style: Style, children: ChildrenVec<NodeId>) -> NodeId {
        let id = self.nodes.len();
        for child in &children {
            self.parents[*child].push(id);
        }
        self.nodes.push(NodeData::new(style));
        self.children.push(children);
        self.parents.push(new_vec_with_capacity(1));
        id
    }

    pub fn add_child(&mut self, node: NodeId, child: NodeId) {
        self.parents[child].push(node);
        self.children[node].push(child);
        self.mark_dirty(node)
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.children.clear();
        self.parents.clear();
    }

    /// Removes a node and swaps with the last node.
    pub fn swap_remove(&mut self, node: NodeId) -> Option<NodeId> {
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

    pub fn remove_child(&mut self, node: NodeId, child: NodeId) -> NodeId {
        let index = self.children[node].iter().position(|n| *n == child).unwrap();
        self.remove_child_at_index(node, index)
    }

    pub fn remove_child_at_index(&mut self, node: NodeId, index: usize) -> NodeId {
        let child = self.children[node].remove(index);
        self.parents[child].retain(|p| *p != node);
        self.mark_dirty(node);
        child
    }

    pub fn mark_dirty(&mut self, node: NodeId) {
        // Performs a recursive depth-first search up the tree until the root node is reached
        // WARNING: this will stack-overflow if the tree contains a cycle
        fn mark_dirty_recursive(nodes: &mut Vec<NodeData>, parents: &[ParentsVec<NodeId>], node_id: NodeId) {
            nodes[node_id].mark_dirty();

            for parent in &parents[node_id] {
                mark_dirty_recursive(nodes, parents, *parent);
            }
        }

        mark_dirty_recursive(&mut self.nodes, &self.parents, node);
    }

    pub fn compute_layout(&mut self, node: NodeId, size: Size<Number>) {
        self.compute(node, size)
    }
}
