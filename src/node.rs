#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

use std::collections::HashMap;
use std::ops::Drop;
use std::sync::Mutex;

use crate::geometry::Size;
use crate::id;
use crate::number::Number;
use crate::result::{Cache, Layout, Result};
use crate::style::*;

type MeasureFunc = Box<Fn(Size<Number>) -> Result<Size<f32>>>;

lazy_static! {
    /// Global stretch instance id allocator.
    static ref INSTANCE_ALLOCATOR: Mutex<id::Allocator> = Mutex::new(id::Allocator::new());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Node {
    instance: id::Id,
    local: id::Id,
}

pub(crate) type Storage<T> = HashMap<Node, T>;

pub struct Stretch {
    id: id::Id,
    nodes: id::Allocator,
    pub(crate) style: Storage<Style>,
    pub(crate) parents: Storage<Vec<Node>>,
    pub(crate) children: Storage<Vec<Node>>,
    pub(crate) measure: Storage<Option<MeasureFunc>>,
    pub(crate) layout: Storage<Layout>,
    pub(crate) layout_cache: Storage<Option<Cache>>,
    pub(crate) is_dirty: Storage<bool>,
}

impl Stretch {
    pub fn new() -> Self {
        Stretch {
            id: INSTANCE_ALLOCATOR.lock().unwrap().allocate(),
            nodes: id::Allocator::new(),
            style: Storage::new(),
            parents: Storage::new(),
            children: Storage::new(),
            measure: Storage::new(),
            layout: Storage::new(),
            layout_cache: Storage::new(),
            is_dirty: Storage::new(),
        }
    }

    fn allocate_node(&mut self) -> Node {
        let local = self.nodes.allocate();
        Node { instance: self.id, local }
    }

    pub fn new_leaf(&mut self, style: Style, measure: MeasureFunc) -> Node {
        let node = self.allocate_node();

        self.style.insert(node, style);
        self.parents.insert(node, Vec::with_capacity(1));
        self.children.insert(node, Vec::with_capacity(0));
        self.measure.insert(node, Some(measure));
        self.layout.insert(node, Layout::new());
        self.layout_cache.insert(node, None);
        self.is_dirty.insert(node, true);

        node
    }

    pub fn new_node(&mut self, style: Style, children: Vec<Node>) -> Node {
        let node = self.allocate_node();

        for child in &children {
            self.parents.get_mut(&child).unwrap().push(node);
        }

        self.style.insert(node, style);
        self.parents.insert(node, Vec::with_capacity(1));
        self.children.insert(node, children);
        self.measure.insert(node, None);
        self.layout.insert(node, Layout::new());
        self.layout_cache.insert(node, None);
        self.is_dirty.insert(node, true);

        node
    }

    pub fn set_measure(&mut self, node: Node, measure: Option<MeasureFunc>) {
        *self.measure[&node].unwrap() = measure;
        self.mark_dirty(node);
    }

    pub fn add_child(&mut self, node: Node, child: Node) {
        // child.0.borrow_mut().parents.push(Rc::downgrade(&self.0));
        // self.0.borrow_mut().children.push(Rc::clone(&child.0));
        // self.mark_dirty();

        unimplemented!()
    }

    pub fn set_children(&mut self, node: Node, children: Vec<Node>) {
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

    pub fn remove_child(&mut self, node: Node, child: Node) -> Node {
        // self.remove_child_at_index({
        //     let parent = self.0.borrow();
        //     parent.children.iter().position(|x| Rc::ptr_eq(&x, &child.0)).unwrap()
        // })

        unimplemented!()
    }

    pub fn remove_child_at_index(&mut self, node: Node, index: usize) -> Node {
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

    pub fn replace_child_at_index(&mut self, node: Node, index: usize, child: Node) -> Node {
        // child.0.borrow_mut().parents.push(Rc::downgrade(&self.0));
        // let old_child = std::mem::replace(&mut self.0.borrow_mut().children[index], Rc::clone(&child.0));

        // let position =
        //     old_child.borrow().parents.iter().position(|x| Rc::ptr_eq(&x.upgrade().unwrap(), &self.0)).unwrap();
        // old_child.borrow_mut().parents.remove(position);

        // self.mark_dirty();

        // Node(old_child)

        unimplemented!()
    }

    pub fn children(&self, node: Node) -> Vec<Node> {
        self.children[&node].clone()
    }

    pub fn child_count(&self, node: Node) -> usize {
        self.children[&node].len()
    }

    pub fn set_style(&mut self, node: Node, style: Style) {
        // self.0.borrow_mut().style = style;
        // self.mark_dirty();

        unimplemented!()
    }

    pub fn style(&self, node: Node) -> &Style {
        &self.style[&node]
    }

    pub fn layout(&self, node: Node) -> &Layout {
        &self.layout[&node]
    }

    pub fn mark_dirty(&mut self, node: Node) {
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

    pub fn dirty(&self, node: Node) -> bool {
        self.is_dirty[&node]
    }

    pub fn compute_layout(&mut self, node: Node, size: Size<Number>) -> Result<()> {
        self.compute(node, size)
    }
}

impl Drop for Stretch {
    fn drop(&mut self) {
        INSTANCE_ALLOCATOR.lock().unwrap().free(&[self.id]);
    }
}
