//! Forest - ECS like datastructure for storing node trees.
//!
//! Backing datastructure for `Stretch` structs.

use crate::geometry::Size;
use crate::id::NodeId;
use crate::node::MeasureFunc;
use crate::number::Number;
use crate::result::{Cache, Layout};
use crate::style::Style;
use crate::Error;

pub(crate) struct NodeData {
    pub(crate) style: Style,
    pub(crate) measure: Option<MeasureFunc>,
    pub(crate) layout: Layout,
    pub(crate) layout_cache: Option<Cache>,
    pub(crate) is_dirty: bool,
}

impl NodeData {
    fn new_leaf(style: Style, measure: MeasureFunc) -> Self {
        NodeData { style, measure: Some(measure), layout_cache: None, layout: Layout::new(), is_dirty: true }
    }

    fn new(style: Style) -> Self {
        NodeData { style, measure: None, layout_cache: None, layout: Layout::new(), is_dirty: true }
    }
}

pub(crate) struct Forest {
    pub(crate) nodes: Vec<NodeData>,
    pub(crate) children: Vec<Vec<NodeId>>,
    pub(crate) parents: Vec<Vec<NodeId>>,
}

impl Forest {
    pub fn with_capacity(capacity: usize) -> Self {
        Forest {
            nodes: Vec::with_capacity(capacity),
            children: Vec::with_capacity(capacity),
            parents: Vec::with_capacity(capacity),
        }
    }

    pub fn new_leaf(&mut self, style: Style, measure: MeasureFunc) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(NodeData::new_leaf(style, measure));
        self.children.push(Vec::with_capacity(0));
        self.parents.push(Vec::with_capacity(1));
        id
    }

    pub fn new_node(&mut self, style: Style, children: Vec<NodeId>) -> NodeId {
        let id = self.nodes.len();
        for child in &children {
            self.parents[*child].push(id);
        }
        self.nodes.push(NodeData::new(style));
        self.children.push(children);
        self.parents.push(Vec::with_capacity(1));
        id
    }

    pub fn add_child(&mut self, node: NodeId, child: NodeId) {
        self.parents[child].push(node);
        self.children[node].push(child);
        self.mark_dirty(node)
    }

    pub unsafe fn remove_child(&mut self, node: NodeId, child: NodeId) -> NodeId {
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
        fn mark_dirty_impl(nodes: &mut Vec<NodeData>, parents: &Vec<Vec<NodeId>>, node_id: NodeId) {
            let node = &mut nodes[node_id];
            node.layout_cache = None;
            node.is_dirty = true;

            for parent in &parents[node_id] {
                mark_dirty_impl(nodes, parents, *parent);
            }
        }

        mark_dirty_impl(&mut self.nodes, &self.parents, node);
    }

    pub fn compute_layout(&mut self, node: NodeId, size: Size<Number>) -> Result<(), Error> {
        self.compute(node, size).map_err(|err| Error::Measure(err))
    }
}
