#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[cfg_attr(feature = "serde", macro_use)]
#[cfg(feature = "serde")]
extern crate serde;

pub mod geometry;
pub mod layout;
pub mod node;
pub mod number;
pub mod prelude;
pub mod style;

mod flexbox;
mod forest;
#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
mod indexmap;
mod sys;

pub use crate::node::Taffy;

#[cfg(feature = "std")]
use core::fmt::{Display, Formatter, Result};

/// The [`Node`](node::Node) was not found in the [`Taffy`] instance
#[derive(Debug)]
pub struct NodeNotFoundError(pub node::Node);

#[cfg(feature = "std")]
impl Display for NodeNotFoundError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Node {:?} is not in the Taffy instance", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NodeNotFoundError {
}

/// An error that occurs while trying to access or modify a [`Node`](node::Node)'s children by index.
#[derive(Debug)]
pub enum ChildOperationError {
    /// The parent [`Node`](node::Node) does not have a child at `child_index`. It only has `child_count` children
    ChildIndexOutOfBounds {
        /// The parent node whose child was being looked up
        parent: node::Node,
        /// The index that was looked up
        child_index: usize,
        /// The total number of children the parent has
        child_count: usize,
    },
    ///The [`Node`](node::Node) was not found in the [`Taffy`] instance
    NodeNotFoundError(NodeNotFoundError),
}

#[cfg(feature = "std")]
impl Display for ChildOperationError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ChildOperationError::ChildIndexOutOfBounds { parent, child_index, child_count } => 
                write!(f, "Index (is {}) should be < child_count ({}) for parent node {:?}", child_index, child_count, parent),
            ChildOperationError::NodeNotFoundError(inner) => inner.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ChildOperationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ChildOperationError::ChildIndexOutOfBounds { .. } => None,
            ChildOperationError::NodeNotFoundError(node) => Some(node),
        }
    }
}

impl From<NodeNotFoundError> for ChildOperationError {
    fn from(node: NodeNotFoundError) -> Self {
        Self::NodeNotFoundError(node)
    }
}