use core::fmt::{Debug, Display, Write};
use core::sync::atomic::{AtomicUsize, Ordering};
use slotmap::Key;
use std::sync::{Arc, Mutex};

use crate::node::Node;
use crate::tree::LayoutTree;

#[doc(hidden)]
pub fn print_tree(tree: &impl LayoutTree, root: Node) {
    println!("");
    print_node(tree, root, 0);
}

#[doc(hidden)]
fn print_node(tree: &impl LayoutTree, node: Node, level: usize) {
    let layout = tree.layout(node);
    println!(
        "{space:leftpad$}{key:?} (x:{x}, y: {y}, width: {width}, height: {height})",
        space = " ",
        leftpad = level * 4,
        key = node.data(),
        x = layout.location.x,
        y = layout.location.y,
        width = layout.size.width,
        height = layout.size.height,
    );

    // Recurse into children
    for child in tree.children(node) {
        print_node(tree, *child, level + 1)
    }
}

pub(crate) struct DebugLogger {
    stack: Mutex<Vec<String>>,
}

static EMPTY_STRING: String = String::new();

impl DebugLogger {
    pub(crate) const fn new() -> Self {
        Self { stack: Mutex::new(Vec::new()) }
    }

    pub(crate) fn push_node(&self, new_key: impl Key) {
        let mut stack = self.stack.lock().unwrap();
        let mut key_string = String::new();
        write!(&mut key_string, "{:?}", new_key.data()).unwrap();
        stack.push(key_string);
    }

    pub(crate) fn pop_node(&self) {
        let mut stack = self.stack.lock().unwrap();
        stack.pop();
    }

    pub(crate) fn log(&self, message: impl Display) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {message}");
    }

    pub(crate) fn llog(&self, label: &str, message: impl Display) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {label} {message}");
    }

    pub(crate) fn debug_log(&self, message: impl Debug) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {message:?}");
    }

    pub(crate) fn debug_llog(&self, label: &str, message: impl Debug) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {label} {message:?}");
    }
}

pub(crate) static NODE_LOGGER: DebugLogger = DebugLogger::new();
