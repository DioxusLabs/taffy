#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::rc::Weak;

use crate::algo;
use crate::geometry::Size;
use crate::number::Number;
use crate::result::{Cache, Layout, Result};
use crate::style::*;

type MeasureFunc = Box<Fn(Size<Number>) -> Result<Size<f32>>>;

pub(crate) struct InternalNode {
    pub(crate) style: Style,
    pub(crate) parents: Vec<Weak<RefCell<InternalNode>>>,
    pub(crate) children: Vec<Rc<RefCell<InternalNode>>>,
    pub(crate) measure: Option<MeasureFunc>,
    pub(crate) layout_cache: Option<Cache>,
    pub(crate) is_dirty: bool,
}

pub struct Node(Rc<RefCell<InternalNode>>);

impl Node {
    pub fn new_leaf(style: Style, measure: MeasureFunc) -> Node {
        Node(Rc::new(RefCell::new(InternalNode {
            style,
            parents: Vec::with_capacity(1),
            children: Vec::with_capacity(0),
            measure: Some(measure),
            layout_cache: None,
            is_dirty: true,
        })))
    }

    pub fn new(style: Style, children: Vec<&Node>) -> Node {
        let parent = Node(Rc::new(RefCell::new(InternalNode {
            style,
            parents: Vec::with_capacity(1),
            children: Vec::with_capacity(children.len()),
            measure: None,
            layout_cache: None,
            is_dirty: true,
        })));

        for child in children {
            child.0.borrow_mut().parents.push(Rc::downgrade(&parent.0));
            parent.0.borrow_mut().children.push(Rc::clone(&child.0));
        }

        parent
    }

    pub fn set_measure(&mut self, measure: Option<MeasureFunc>) {
        self.0.borrow_mut().measure = measure;
        self.mark_dirty();
    }

    pub fn add_child(&mut self, child: &Node) {
        child.0.borrow_mut().parents.push(Rc::downgrade(&self.0));
        self.0.borrow_mut().children.push(Rc::clone(&child.0));
        self.mark_dirty();
    }

    pub fn set_children(&mut self, children: Vec<&Node>) {
        for child in &self.0.borrow().children {
            let position =
                child.borrow().parents.iter().position(|x| Rc::ptr_eq(&x.upgrade().unwrap(), &self.0)).unwrap();
            child.borrow_mut().parents.remove(position);
        }

        self.0.borrow_mut().children = Vec::with_capacity(children.len());

        for child in children {
            child.0.borrow_mut().parents.push(Rc::downgrade(&self.0));
            self.0.borrow_mut().children.push(Rc::clone(&child.0));
        }

        self.mark_dirty();
    }

    pub fn remove_child(&mut self, child: &Node) -> Node {
        self.remove_child_at_index({
            let parent = self.0.borrow();
            parent.children.iter().position(|x| Rc::ptr_eq(&x, &child.0)).unwrap()
        })
    }

    pub fn remove_child_at_index(&mut self, index: usize) -> Node {
        let child = {
            let mut parent = self.0.borrow_mut();
            let child = parent.children.remove(index);
            let position =
                child.borrow().parents.iter().position(|x| Rc::ptr_eq(&x.upgrade().unwrap(), &self.0)).unwrap();
            child.borrow_mut().parents.remove(position);
            child
        };

        self.mark_dirty();
        Node(child)
    }

    pub fn replace_child_at_index(&mut self, index: usize, child: &Node) -> Node {
        child.0.borrow_mut().parents.push(Rc::downgrade(&self.0));
        let old_child = std::mem::replace(&mut self.0.borrow_mut().children[index], Rc::clone(&child.0));

        let position =
            old_child.borrow().parents.iter().position(|x| Rc::ptr_eq(&x.upgrade().unwrap(), &self.0)).unwrap();
        old_child.borrow_mut().parents.remove(position);

        self.mark_dirty();

        Node(old_child)
    }

    pub fn children(&self) -> Vec<Node> {
        let node = self.0.borrow_mut();
        node.children.iter().map(|child| Node(Rc::clone(child))).collect()
    }

    pub fn child_count(&self) -> usize {
        let node = self.0.borrow_mut();
        node.children.len()
    }

    pub fn set_style(&mut self, style: Style) {
        self.0.borrow_mut().style = style;
        self.mark_dirty();
    }

    pub fn style(&self) -> Style {
        self.0.borrow().style
    }

    pub fn mark_dirty(&mut self) {
        let mut node = self.0.borrow_mut();
        node.layout_cache = None;
        node.is_dirty = true;

        for parent in &node.parents {
            if let Some(parent) = parent.upgrade() {
                Node(parent).mark_dirty();
            }
        }
    }

    pub fn dirty(&self) -> bool {
        self.0.borrow().is_dirty
    }

    pub fn compute_layout(&self, size: Size<Number>) -> Result<Layout> {
        algo::compute(&mut self.0.borrow_mut(), size)
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let style = &self.0.borrow().style;
        let children: Vec<_> = self.0.borrow().children.iter().map(|c| Node(Rc::clone(c))).collect();
        write!(f, "Node {{ style: {:?}, children: {:?} }}", style, children)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Node {}
