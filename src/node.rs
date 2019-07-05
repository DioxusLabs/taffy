#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

use core::any::Any;

use std::collections::HashMap;
use std::ops::Drop;
use std::sync::Mutex;

use crate::forest::Forest;
use crate::geometry::Size;
use crate::id::{self, NodeId};
use crate::number::Number;
use crate::result::Layout;
use crate::style::*;
use crate::Error;

pub type MeasureFunc = Box<Fn(Size<Number>) -> Result<Size<f32>, Box<Any>>>;

lazy_static! {
    /// Global stretch instance id allocator.
    static ref INSTANCE_ALLOCATOR: Mutex<id::Allocator> = Mutex::new(id::Allocator::new());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Node {
    instance: id::Id,
    local: id::Id,
}

pub struct Stretch {
    id: id::Id,
    nodes: id::Allocator,
    nodes_to_ids: HashMap<Node, NodeId>,
    ids_to_nodes: HashMap<NodeId, Node>,
    forest: Forest,
}

impl Stretch {
    const DEFAULT_CAPACITY: usize = 16;

    pub fn new() -> Self {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Stretch {
            id: INSTANCE_ALLOCATOR.lock().unwrap().allocate(),
            nodes: id::Allocator::new(),
            nodes_to_ids: HashMap::with_capacity(capacity),
            ids_to_nodes: HashMap::with_capacity(capacity),
            forest: Forest::with_capacity(capacity),
        }
    }

    fn allocate_node(&mut self) -> Node {
        let local = self.nodes.allocate();
        Node { instance: self.id, local }
    }

    fn add_node(&mut self, node: Node, id: NodeId) {
        self.nodes_to_ids.insert(node, id);
        self.ids_to_nodes.insert(id, node);
    }

    // Find node in the forest.
    fn find_node(&self, node: Node) -> Result<NodeId, Error> {
        match self.nodes_to_ids.get(&node) {
            Some(id) => Ok(*id),
            None => Err(Error::InvalidNode(node)),
        }
    }

    pub fn new_leaf(&mut self, style: Style, measure: MeasureFunc) -> Result<Node, Error> {
        let node = self.allocate_node();
        let id = self.forest.new_leaf(style, measure);
        self.add_node(node, id);
        Ok(node)
    }

    pub fn new_node(&mut self, style: Style, children: Vec<Node>) -> Result<Node, Error> {
        let node = self.allocate_node();
        let children = children.iter().map(|child| self.find_node(*child)).collect::<Result<Vec<_>, Error>>()?;
        let id = self.forest.new_node(style, children);
        self.add_node(node, id);
        Ok(node)
    }

    /// Removes all nodes.
    ///
    /// All associated nodes will be invalid.
    pub fn clear(&mut self) {
        for node in self.nodes_to_ids.keys() {
            self.nodes.free(&[node.local]);
        }
        self.nodes_to_ids.clear();
        self.ids_to_nodes.clear();
        self.forest.clear();
    }

    /// Remove nodes.
    pub fn remove(&mut self, node: Node) {
        let id = if let Ok(id) = self.find_node(node) { id } else { return };

        self.nodes_to_ids.remove(&node);
        self.ids_to_nodes.remove(&id);

        if let Some(new_id) = self.forest.swap_remove(id) {
            let new = self.ids_to_nodes.remove(&new_id).unwrap();
            self.nodes_to_ids.insert(new, id);
        }
    }

    pub fn set_measure(&mut self, node: Node, measure: Option<MeasureFunc>) -> Result<(), Error> {
        let id = self.find_node(node)?;
        self.forest.nodes[id].measure = measure;
        self.forest.mark_dirty(id);
        Ok(())
    }

    pub fn add_child(&mut self, node: Node, child: Node) -> Result<(), Error> {
        let node_id = self.find_node(node)?;
        let child_id = self.find_node(child)?;

        self.forest.add_child(node_id, child_id);
        Ok(())
    }

    pub fn set_children(&mut self, node: Node, children: Vec<Node>) -> Result<(), Error> {
        let node_id = self.find_node(node)?;
        let children_id = children.iter().map(|child| self.find_node(*child)).collect::<Result<Vec<_>, _>>()?;

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

    pub fn remove_child(&mut self, node: Node, child: Node) -> Result<Node, Error> {
        let node_id = self.find_node(node)?;
        let child_id = self.find_node(child)?;

        let prev_id = unsafe { self.forest.remove_child(node_id, child_id) };
        Ok(self.ids_to_nodes[&prev_id])
    }

    pub fn remove_child_at_index(&mut self, node: Node, index: usize) -> Result<Node, Error> {
        let node_id = self.find_node(node)?;
        // TODO: index check

        let prev_id = self.forest.remove_child_at_index(node_id, index);
        Ok(self.ids_to_nodes[&prev_id])
    }

    pub fn replace_child_at_index(&mut self, node: Node, index: usize, child: Node) -> Result<Node, Error> {
        let node_id = self.find_node(node)?;
        let child_id = self.find_node(child)?;
        // TODO: index check

        self.forest.parents[child_id].push(node_id);
        let old_child = std::mem::replace(&mut self.forest.children[node_id][index], child_id);
        self.forest.parents[old_child].retain(|p| *p != node_id);

        self.forest.mark_dirty(node_id);

        Ok(self.ids_to_nodes[&old_child])
    }

    pub fn children(&self, node: Node) -> Result<Vec<Node>, Error> {
        let id = self.find_node(node)?;
        Ok(self.forest.children[id].iter().map(|child| self.ids_to_nodes[child]).collect())
    }

    pub fn child_count(&self, node: Node) -> Result<usize, Error> {
        let id = self.find_node(node)?;
        Ok(self.forest.children[id].len())
    }

    pub fn set_style(&mut self, node: Node, style: Style) -> Result<(), Error> {
        let id = self.find_node(node)?;
        self.forest.nodes[id].style = style;
        self.forest.mark_dirty(id);
        Ok(())
    }

    pub fn style(&self, node: Node) -> Result<&Style, Error> {
        let id = self.find_node(node)?;
        Ok(&self.forest.nodes[id].style)
    }

    pub fn layout(&self, node: Node) -> Result<&Layout, Error> {
        let id = self.find_node(node)?;
        Ok(&self.forest.nodes[id].layout)
    }

    pub fn mark_dirty(&mut self, node: Node) -> Result<(), Error> {
        let id = self.find_node(node)?;
        self.forest.mark_dirty(id);
        Ok(())
    }

    pub fn dirty(&self, node: Node) -> Result<bool, Error> {
        let id = self.find_node(node)?;
        Ok(self.forest.nodes[id].is_dirty)
    }

    pub fn compute_layout(&mut self, node: Node, size: Size<Number>) -> Result<(), Error> {
        let id = self.find_node(node)?;
        self.forest.compute_layout(id, size)
    }
}

impl Drop for Stretch {
    fn drop(&mut self) {
        INSTANCE_ALLOCATOR.lock().unwrap().free(&[self.id]);
    }
}
