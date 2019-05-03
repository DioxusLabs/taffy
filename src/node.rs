#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

use std::collections::HashMap;

use crate::geometry::Size;
use crate::number::Number;
use crate::result::{Cache, Layout, Result};
use crate::style::*;

type MeasureFunc = Box<Fn(Size<Number>) -> Result<Size<f32>>>;

pub type NodeId = usize;
pub(crate) type Storage<T> = HashMap<NodeId, T>;

struct Allocator {
    new_id: NodeId,
    free_ids: Vec<NodeId>,
}

impl Allocator {
    pub fn new() -> Self {
        Allocator { new_id: 0, free_ids: Vec::new() }
    }

    pub fn allocate(&mut self) -> NodeId {
        match self.free_ids.pop() {
            Some(id) => id,
            None => {
                let id = self.new_id;
                self.new_id += 1;
                id
            }
        }
    }
}

pub struct Stretch {
    nodes: Allocator,
    pub(crate) style: Storage<Style>,
    pub(crate) parents: Storage<Vec<NodeId>>,
    pub(crate) children: Storage<Vec<NodeId>>,
    pub(crate) measure: Storage<Option<MeasureFunc>>,
    pub(crate) layout: Storage<Layout>,
    pub(crate) layout_cache: Storage<Option<Cache>>,
    pub(crate) is_dirty: Storage<bool>,
}

impl Stretch {
    pub fn new() -> Self {
        Stretch {
            nodes: Allocator::new(),
            style: Storage::new(),
            parents: Storage::new(),
            children: Storage::new(),
            measure: Storage::new(),
            layout: Storage::new(),
            layout_cache: Storage::new(),
            is_dirty: Storage::new(),
        }
    }

    pub fn new_leaf(&mut self, style: Style, measure: MeasureFunc) -> NodeId {
        // Node(Rc::new(RefCell::new(InternalNode {
        //     style,
        //     parents: Vec::with_capacity(1),
        //     children: Vec::with_capacity(0),
        //     measure: Some(measure),
        //     layout_cache: None,
        //     is_dirty: true,
        // })))
        unimplemented!()
    }

    pub fn new_node(&mut self, style: Style, children: Vec<NodeId>) -> NodeId {
        // let parent = Node(Rc::new(RefCell::new(InternalNode {
        //     style,
        //     parents: Vec::with_capacity(1),
        //     children: Vec::with_capacity(children.len()),
        //     measure: None,
        //     layout_cache: None,
        //     is_dirty: true,
        // })));

        // for child in children {
        //     child.0.borrow_mut().parents.push(Rc::downgrade(&parent.0));
        //     parent.0.borrow_mut().children.push(Rc::clone(&child.0));
        // }

        // parent

        unimplemented!()
    }

    pub fn set_measure(&mut self, node: NodeId, measure: Option<MeasureFunc>) {
        // self.0.borrow_mut().measure = measure;
        // self.mark_dirty();

        unimplemented!()
    }

    pub fn add_child(&mut self, node: NodeId, child: NodeId) {
        // child.0.borrow_mut().parents.push(Rc::downgrade(&self.0));
        // self.0.borrow_mut().children.push(Rc::clone(&child.0));
        // self.mark_dirty();

        unimplemented!()
    }

    pub fn set_children(&mut self, node: NodeId, children: Vec<NodeId>) {
        // for child in &self.0.borrow().children {
        //     let position =
        //         child.borrow().parents.iter().position(|x| Rc::ptr_eq(&x.upgrade().unwrap(), &self.0)).unwrap();
        //     child.borrow_mut().parents.remove(position);
        // }

        // self.0.borrow_mut().children = Vec::with_capacity(children.len());

        // for child in children {
        //     child.0.borrow_mut().parents.push(Rc::downgrade(&self.0));
        //     self.0.borrow_mut().children.push(Rc::clone(&child.0));
        // }

        // self.mark_dirty();

        unimplemented!()
    }

    pub fn remove_child(&mut self, node: NodeId, child: NodeId) -> NodeId {
        // self.remove_child_at_index({
        //     let parent = self.0.borrow();
        //     parent.children.iter().position(|x| Rc::ptr_eq(&x, &child.0)).unwrap()
        // })

        unimplemented!()
    }

    pub fn remove_child_at_index(&mut self, node: NodeId, index: usize) -> NodeId {
        // let child = {
        //     let mut parent = self.0.borrow_mut();
        //     let child = parent.children.remove(index);
        //     let position =
        //         child.borrow().parents.iter().position(|x| Rc::ptr_eq(&x.upgrade().unwrap(), &self.0)).unwrap();
        //     child.borrow_mut().parents.remove(position);
        //     child
        // };

        // self.mark_dirty();
        // Node(child)

        unimplemented!()
    }

    pub fn replace_child_at_index(&mut self, node: NodeId, index: usize, child: NodeId) -> NodeId {
        // child.0.borrow_mut().parents.push(Rc::downgrade(&self.0));
        // let old_child = std::mem::replace(&mut self.0.borrow_mut().children[index], Rc::clone(&child.0));

        // let position =
        //     old_child.borrow().parents.iter().position(|x| Rc::ptr_eq(&x.upgrade().unwrap(), &self.0)).unwrap();
        // old_child.borrow_mut().parents.remove(position);

        // self.mark_dirty();

        // Node(old_child)

        unimplemented!()
    }

    pub fn children(&self, node: NodeId) -> Vec<NodeId> {
        // let node = self.0.borrow_mut();
        // node.children.iter().map(|child| Node(Rc::clone(child))).collect()

        unimplemented!()
    }

    pub fn child_count(&self, node: NodeId) -> usize {
        // let node = self.0.borrow_mut();
        // node.children.len()

        unimplemented!()
    }

    pub fn set_style(&mut self, node: NodeId, style: Style) {
        // self.0.borrow_mut().style = style;
        // self.mark_dirty();

        unimplemented!()
    }

    pub fn style(&self, node: NodeId) -> Style {
        // self.0.borrow().style

        unimplemented!()
    }

    pub fn mark_dirty(&mut self, node: NodeId) {
        // let mut node = self.0.borrow_mut();
        // node.layout_cache = None;
        // node.is_dirty = true;

        // for parent in &node.parents {
        //     if let Some(parent) = parent.upgrade() {
        //         Node(parent).mark_dirty();
        //     }
        // }

        unimplemented!()
    }

    pub fn dirty(&self, node: NodeId) -> bool {
        // self.0.borrow().is_dirty

        unimplemented!()
    }

    pub fn compute_layout(&mut self, node: NodeId, size: Size<Number>) -> Result<()> {
        self.compute(node, size)
    }
}
