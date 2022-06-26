//! Forest - a struct-of-arrays data structure for storing node trees.
//!
//! Backing data structure for `Taffy` structs.
use crate::layout::{Cache, Layout};
use crate::node::{MeasureFunc, NodeId};
use crate::style::FlexboxLayout;
use crate::sys::{new_vec_with_capacity, ChildrenVec, ParentsVec, Vec};

/// Layout information for a given [`Node`](crate::node::Node)
///
/// Stored in a [`Forest`].
pub(crate) struct NodeData {
    /// The layout strategy used by this node
    pub(crate) style: FlexboxLayout,
    /// The mapping from the Size<Option<f32>> (in real units) to Size<f32> (in points) for this node
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
    /// Create the data for a new node with a [`MeasureFunc`]
    #[must_use]
    fn new_with_measure(style: FlexboxLayout, measure: MeasureFunc) -> Self {
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
    #[must_use]
    fn new(style: FlexboxLayout) -> Self {
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

    /// Creates and adds a new unattached leaf node to the forest, and returns the [`NodeId`] of the new node
    pub(crate) fn new_leaf(&mut self, layout: FlexboxLayout) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(NodeData::new(layout));
        self.children.push(new_vec_with_capacity(0));
        self.parents.push(new_vec_with_capacity(1));
        id
    }

    /// Creates and adds a new unattached leaf node to the forest, and returns the [`NodeId`] of the new node
    ///
    /// The node must have a [`MeasureFunc`] supplied
    pub(crate) fn new_leaf_with_measure(&mut self, layout: FlexboxLayout, measure: MeasureFunc) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(NodeData::new_with_measure(layout, measure));
        self.children.push(new_vec_with_capacity(0));
        self.parents.push(new_vec_with_capacity(1));
        id
    }

    /// Creates and adds a new unparented node to the forest with the associated children attached, and returns the [`NodeId`] of the new node
    pub(crate) fn new_with_children(&mut self, layout: FlexboxLayout, children: ChildrenVec<NodeId>) -> NodeId {
        let id = self.nodes.len();
        for child in &children {
            self.parents[*child].push(id);
        }
        self.nodes.push(NodeData::new(layout));
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

        // Remove old node as parent from all its children.
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
}

#[cfg(test)]
mod tests {
    use super::{Forest, NodeData};
    use crate::geometry::Size;
    use crate::node::{MeasureFunc, NodeId};
    use crate::style::FlexboxLayout;
    use crate::sys::new_vec_with_capacity;

    fn assert_forest_size(forest: &Forest, size: usize) {
        assert_eq!(forest.nodes.len(), size);
        assert_eq!(forest.children.len(), size);
        assert_eq!(forest.parents.len(), size);
    }

    fn node_measure_eq(node: &NodeData, measure_fn: fn(Size<Option<f32>>) -> Size<f32>) -> bool {
        match node.measure.as_ref().unwrap() {
            MeasureFunc::Raw(m) => measure_fn(Size::NONE) == m(Size::NONE),
            _ => false,
        }
    }

    fn add_default_leaf(forest: &mut Forest) -> NodeId {
        forest.new_leaf(FlexboxLayout::default())
    }

    #[test]
    fn new_leaf_first_leaf() {
        let mut forest = Forest::with_capacity(1);
        let s1 = FlexboxLayout { flex_grow: 1.0, ..Default::default() };

        let id = forest.new_leaf(s1);

        let node = &forest.nodes[id];
        assert_eq!(id, 0);
        assert_eq!(node.style, s1);
        assert_forest_size(&forest, 1);
    }

    #[test]
    fn new_leaf_second_leaf() {
        let mut forest = Forest::with_capacity(2);
        let s1 = FlexboxLayout { flex_grow: 1.0, ..Default::default() };
        let s2 = FlexboxLayout { flex_grow: 2.0, ..Default::default() };

        forest.new_leaf(s1);
        let id = forest.new_leaf(s2);

        let node = &forest.nodes[id];
        assert_eq!(id, 1);
        assert_eq!(node.style, s2);
        assert_forest_size(&forest, 2);
    }

    #[test]
    fn new_leaf_with_measure_first_leaf() {
        let mut forest = Forest::with_capacity(1);
        let s1 = FlexboxLayout { flex_grow: 1.0, ..Default::default() };
        let measure_fn1 = |_| Size { width: 1.0, height: 1.0 };

        let id = forest.new_leaf_with_measure(s1, MeasureFunc::Raw(measure_fn1));

        let node = &forest.nodes[id];
        assert_eq!(id, 0);
        assert_eq!(node.style, s1);
        assert!(node_measure_eq(&node, measure_fn1));
        assert_forest_size(&forest, 1);
    }

    #[test]
    fn new_leaf_with_measure_second_leaf() {
        let mut forest = Forest::with_capacity(2);
        let s1 = FlexboxLayout { flex_grow: 1.0, ..Default::default() };
        let s2 = FlexboxLayout { flex_grow: 2.0, ..Default::default() };
        let measure_fn1 = |_| Size { width: 1.0, height: 1.0 };
        let measure_fn2 = |_| Size { width: 2.0, height: 2.0 };

        forest.new_leaf_with_measure(s1, MeasureFunc::Raw(measure_fn1));
        let id = forest.new_leaf_with_measure(s2, MeasureFunc::Raw(measure_fn2));

        let node = &forest.nodes[id];
        assert_eq!(id, 1);
        assert_eq!(node.style, s2);
        assert!(node_measure_eq(&node, measure_fn2));
        assert_forest_size(&forest, 2);
    }

    #[test]
    fn new_with_children_single() {
        let mut forest = Forest::with_capacity(2);
        let style = FlexboxLayout { flex_grow: 1.0, ..Default::default() };
        let c1_id = add_default_leaf(&mut forest);
        let mut children = new_vec_with_capacity(1);
        children.push(c1_id);

        let id = forest.new_with_children(style, children);
        let new_node = &forest.nodes[id];

        assert_eq!(id, 1);
        assert_eq!(new_node.style, style);
        assert_eq!(forest.parents[c1_id][0], id);
        assert_eq!(forest.children[id][0], c1_id);
        assert_forest_size(&forest, 2);
    }

    #[test]
    fn new_with_children_multiple() {
        let mut forest = Forest::with_capacity(3);
        let style = FlexboxLayout { flex_grow: 1.0, ..Default::default() };
        let c1_id = add_default_leaf(&mut forest);
        let c2_id = add_default_leaf(&mut forest);
        let mut children = new_vec_with_capacity(2);
        children.push(c1_id);
        children.push(c2_id);

        let id = forest.new_with_children(style, children);
        let new_node = &forest.nodes[id];

        assert_eq!(id, 2);
        assert_eq!(new_node.style, style);
        assert_eq!(forest.parents[c1_id][0], id);
        assert_eq!(forest.parents[c2_id][0], id);
        assert_eq!(forest.children[id][0], c1_id);
        assert_eq!(forest.children[id][1], c2_id);
        assert_forest_size(&forest, 3);
    }

    #[test]
    fn add_child() {
        let mut forest = Forest::with_capacity(2);
        let parent_id = add_default_leaf(&mut forest);
        let child_id = add_default_leaf(&mut forest);
        forest.add_child(parent_id, child_id);

        let parent = &forest.nodes[parent_id];

        assert_eq!(forest.parents[child_id][0], parent_id);
        assert_eq!(forest.children[parent_id][0], child_id);
        assert!(parent.is_dirty);
    }

    #[test]
    fn add_child_multiple() {
        let mut forest = Forest::with_capacity(3);
        let parent_id = add_default_leaf(&mut forest);
        let c1_id = add_default_leaf(&mut forest);
        let c2_id = add_default_leaf(&mut forest);
        forest.add_child(parent_id, c1_id);
        forest.add_child(parent_id, c2_id);

        let parent = &forest.nodes[parent_id];

        assert_eq!(forest.parents[c1_id][0], parent_id);
        assert_eq!(forest.parents[c2_id][0], parent_id);
        assert_eq!(forest.children[parent_id][0], c1_id);
        assert_eq!(forest.children[parent_id][1], c2_id);
        assert!(parent.is_dirty);
    }

    #[test]
    fn clear() {
        let mut forest = Forest::with_capacity(1);
        add_default_leaf(&mut forest);
        forest.clear();
        assert_forest_size(&forest, 0);
    }

    #[test]
    fn swap_remove_single() {
        let mut forest = Forest::with_capacity(1);
        let parent_id = add_default_leaf(&mut forest);

        let moved_id = forest.swap_remove(parent_id);

        assert_eq!(moved_id, None);
        assert_forest_size(&forest, 0);
    }

    #[test]
    fn swap_remove_parent() {
        let mut forest = Forest::with_capacity(3);
        let parent_id = add_default_leaf(&mut forest);
        let c1_id = add_default_leaf(&mut forest);
        let c2_id = add_default_leaf(&mut forest);
        forest.add_child(parent_id, c1_id);
        forest.add_child(parent_id, c2_id);

        let moved_id = forest.swap_remove(parent_id);
        let new_c1_id = parent_id.clone();
        let new_c2_id = c1_id.clone();

        assert_eq!(moved_id, Some(c2_id));
        assert_eq!(forest.parents[new_c1_id].len(), 0);
        assert_eq!(forest.parents[new_c2_id].len(), 0);
        assert_forest_size(&forest, 2);
    }

    #[test]
    fn swap_remove_parent_nested() {
        let mut forest = Forest::with_capacity(3);
        let parent_id = add_default_leaf(&mut forest);
        let c1_id = add_default_leaf(&mut forest);
        let c2_id = add_default_leaf(&mut forest);
        forest.add_child(parent_id, c1_id);
        forest.add_child(c1_id, c2_id);

        let moved_id = forest.swap_remove(parent_id);
        let new_c1_id = c1_id.clone();
        let new_c2_id = parent_id.clone();

        assert_eq!(moved_id, Some(c2_id));
        assert_eq!(forest.parents[new_c1_id].len(), 0);
        assert_eq!(forest.parents[new_c2_id].len(), 1);
        assert_eq!(forest.parents[new_c2_id][0], new_c1_id);
        assert_forest_size(&forest, 2);
    }

    #[test]
    fn swap_remove_first_child_nested() {
        let mut forest = Forest::with_capacity(3);
        let parent_id = add_default_leaf(&mut forest);
        let c1_id = add_default_leaf(&mut forest);
        let c2_id = add_default_leaf(&mut forest);
        forest.add_child(parent_id, c1_id);
        forest.add_child(parent_id, c2_id);

        let moved_id = forest.swap_remove(c1_id);
        let new_c2_id = c1_id.clone();

        assert_eq!(moved_id, Some(c2_id));
        assert_eq!(forest.children[parent_id].len(), 1);
        assert_eq!(forest.children[parent_id][0], new_c2_id);
        assert_eq!(forest.parents[new_c2_id].len(), 1);
        assert_eq!(forest.parents[new_c2_id][0], parent_id);
        assert_forest_size(&forest, 2);
    }

    #[test]
    fn swap_remove_last_child_nested() {
        let mut forest = Forest::with_capacity(3);
        let parent_id = add_default_leaf(&mut forest);
        let c1_id = add_default_leaf(&mut forest);
        let c2_id = add_default_leaf(&mut forest);
        forest.add_child(parent_id, c1_id);
        forest.add_child(parent_id, c2_id);

        let moved_id = forest.swap_remove(c2_id);

        assert_eq!(moved_id, None);
        assert_eq!(forest.children[parent_id].len(), 1);
        assert_eq!(forest.parents[c1_id].len(), 1);
        assert_forest_size(&forest, 2);
    }

    #[test]
    fn swap_remove_disjoint() {
        let mut forest = Forest::with_capacity(2);
        let n1_id = add_default_leaf(&mut forest);
        let n2_id = add_default_leaf(&mut forest);

        let moved_id = forest.swap_remove(n1_id);

        assert_eq!(moved_id, Some(n2_id));
        assert_forest_size(&forest, 1);
    }

    #[test]
    fn remove_child() {
        let mut forest = Forest::with_capacity(3);
        let parent_id = add_default_leaf(&mut forest);
        let c1_id = add_default_leaf(&mut forest);
        let c2_id = add_default_leaf(&mut forest);
        forest.add_child(parent_id, c1_id);
        forest.add_child(parent_id, c2_id);

        let removed_id = forest.remove_child(parent_id, c1_id);
        let parent = &forest.nodes[parent_id];

        assert!(parent.is_dirty);
        assert_eq!(forest.children[parent_id].len(), 1);
        assert_eq!(forest.parents[c1_id].len(), 0);
        assert_eq!(forest.parents[c2_id].len(), 1);
        assert_eq!(removed_id, c1_id);
    }

    #[test]
    fn remove_child_at_index() {
        let mut forest = Forest::with_capacity(3);
        let parent_id = add_default_leaf(&mut forest);
        let c1_id = add_default_leaf(&mut forest);
        let c2_id = add_default_leaf(&mut forest);
        forest.add_child(parent_id, c1_id);
        forest.add_child(parent_id, c2_id);

        let removed_id = forest.remove_child_at_index(parent_id, 0);
        let parent = &forest.nodes[parent_id];

        assert!(parent.is_dirty);
        assert_eq!(forest.children[parent_id].len(), 1);
        assert_eq!(forest.parents[c1_id].len(), 0);
        assert_eq!(forest.parents[c2_id].len(), 1);
        assert_eq!(removed_id, c1_id);
    }

    #[test]
    fn mark_dirty() {
        let mut forest = Forest::with_capacity(3);
        let parent_id = add_default_leaf(&mut forest);
        let c1_id = add_default_leaf(&mut forest);
        let c2_id = add_default_leaf(&mut forest);
        forest.add_child(parent_id, c1_id);
        forest.add_child(c1_id, c2_id);

        forest.mark_dirty(c2_id);

        assert!(forest.nodes[0].is_dirty);
        assert!(forest.nodes[1].is_dirty);
        assert!(forest.nodes[2].is_dirty);
    }
}
