#![allow(dead_code)]

use crate::tree::NodeId;
use crate::{style, LayoutTree};

#[cfg(any(feature = "debug", feature = "profile"))]
use core::fmt::{Debug, Display, Write};
#[cfg(any(feature = "debug", feature = "profile"))]
use std::sync::Mutex;

/// Prints a debug representation of the computed layout for a tree of nodes, starting with the passed root node.
#[cfg(feature = "std")]
pub fn print_tree(tree: &impl LayoutTree, root: NodeId) {
    println!("TREE");
    print_node(tree, root, false, String::new());
}

#[cfg(feature = "std")]
fn print_node(tree: &impl LayoutTree, node: NodeId, has_sibling: bool, lines_string: String) {
    let layout = &tree.get_final_layout(node);
    let style = &tree.get_style(node);
    let num_children = tree.child_count(node);

    let display = match (num_children, style.display) {
        (_, style::Display::None) => "NONE",
        (0, _) => "LEAF",
        #[cfg(feature = "block_layout")]
        (_, style::Display::Block) => "BLOCK",
        #[cfg(feature = "flexbox")]
        (_, style::Display::Flex) => "FLEX",
        #[cfg(feature = "grid")]
        (_, style::Display::Grid) => "GRID",
    };

    let fork_string = if has_sibling { "├── " } else { "└── " };
    println!(
        "{lines}{fork} {display} [x: {x:<4} y: {y:<4} width: {width:<4} height: {height:<4} content_width: {content_width:<4} content_height: {content_height:<4}] ({key:?})",
        lines = lines_string,
        fork = fork_string,
        display = display,
        x = layout.location.x,
        y = layout.location.y,
        width = layout.size.width,
        height = layout.size.height,
        content_width = layout.content_size.width,
        content_height = layout.content_size.height,
        key = node,
    );
    let bar = if has_sibling { "│   " } else { "    " };
    let new_string = lines_string + bar;

    // Recurse into children
    for (index, child) in tree.child_ids(node).enumerate() {
        let has_sibling = index < num_children - 1;
        print_node(tree, child, has_sibling, new_string.clone());
    }
}

#[doc(hidden)]
#[cfg(any(feature = "debug", feature = "profile"))]
pub struct DebugLogger {
    stack: Mutex<Vec<String>>,
}

#[cfg(any(feature = "debug", feature = "profile"))]
static EMPTY_STRING: String = String::new();
#[cfg(any(feature = "debug", feature = "profile"))]
impl DebugLogger {
    pub const fn new() -> Self {
        Self { stack: Mutex::new(Vec::new()) }
    }

    pub fn push_node(&self, new_key: NodeId) {
        let mut stack = self.stack.lock().unwrap();
        let mut key_string = String::new();
        write!(&mut key_string, "{:?}", new_key).unwrap();
        stack.push(key_string);
    }

    pub fn pop_node(&self) {
        let mut stack = self.stack.lock().unwrap();
        stack.pop();
    }

    pub fn log(&self, message: impl Display) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {message}");
    }

    pub fn labelled_log(&self, label: &str, message: impl Display) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {label} {message}");
    }

    pub fn debug_log(&self, message: impl Debug) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {message:?}");
    }

    pub fn labelled_debug_log(&self, label: &str, message: impl Debug) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {label} {message:?}");
    }
}

#[cfg(any(feature = "debug", feature = "profile"))]
pub(crate) static NODE_LOGGER: DebugLogger = DebugLogger::new();

macro_rules! debug_log {
    // String literal label with debug printing
    ($label:literal, dbg:$item:expr) => {
        #[cfg(feature = "debug")]
        $crate::util::debug::NODE_LOGGER.labelled_debug_log($label, $item);
    };
    // String literal label with display printing
    ($label:literal, $item:expr) => {
        #[cfg(feature = "debug")]
        $crate::util::debug::NODE_LOGGER.labelled_log($label, $item);
    };
    // Debug printing
    (dbg:$item:expr) => {
        #[cfg(feature = "debug")]
        $crate::util::debug::NODE_LOGGER.debug_log($item);
    };
    // Display printing
    ($item:expr) => {
        #[cfg(feature = "debug")]
        $crate::util::debug::NODE_LOGGER.log($item);
    };
    // Blank newline
    () => {
        #[cfg(feature = "debug")]
        println!();
    };
}

macro_rules! debug_log_node {
    ($known_dimensions: expr, $parent_size: expr, $available_space: expr, $run_mode: expr, $sizing_mode: expr) => {
        debug_log!(dbg:$run_mode);
        debug_log!("sizing_mode", dbg:$sizing_mode);
        debug_log!("known_dimensions", dbg:$known_dimensions);
        debug_log!("parent_size", dbg:$parent_size);
        debug_log!("available_space", dbg:$available_space);
    };
}

macro_rules! debug_push_node {
    ($node_id:expr) => {
        #[cfg(any(feature = "debug", feature = "profile"))]
        $crate::util::debug::NODE_LOGGER.push_node($node_id);
        debug_log!("");
    };
}

macro_rules! debug_pop_node {
    () => {
        #[cfg(any(feature = "debug", feature = "profile"))]
        $crate::util::debug::NODE_LOGGER.pop_node();
    };
}

#[cfg(feature = "profile")]
#[allow(unused_macros)]
macro_rules! time {
    ($label:expr, $($code:tt)*) => {
        let start = ::std::time::Instant::now();
        $($code)*
        let duration = ::std::time::Instant::now().duration_since(start);
        crate::util::debug::NODE_LOGGER.log(format_args!("Performed {} in {}ms", $label, duration.as_millis()));
    };
}

#[cfg(not(feature = "profile"))]
#[allow(unused_macros)]
macro_rules! time {
    ($label:expr, $($code:tt)*) => {
        $($code)*
    };
}

#[allow(unused_imports)]
pub(crate) use {debug_log, debug_log_node, debug_pop_node, debug_push_node, time};
