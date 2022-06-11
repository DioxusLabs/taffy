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

/// An error that can occur when performing layout
#[derive(Debug)]
pub enum TaffyError {
    /// The [`Node`](node::Node) is not part of the [`Taffy`](Taffy) instance.
    InvalidNode(node::Node),
    IndexOutOfBounds {
        /// The parent node whose child was being looked up
        parent: node::Node,
        /// The index that was looked up
        child_index: usize,
        /// The total number of children the parent has
        child_count: usize,
    }
}

#[cfg(feature = "std")]
impl Display for TaffyError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            TaffyError::InvalidNode(ref node) => write!(f, "Invalid node {:?}", node),
            TaffyError::IndexOutOfBounds { parent, child_index, child_count } => 
                write!(f, "Index (is {}) should be < len ({}) for parent node {:?}", child_index, child_count, parent),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TaffyError {
    fn description(&self) -> &str {
        match *self {
            TaffyError::InvalidNode(_) => "The node is not part of the Taffy instance",
            TaffyError::IndexOutOfBounds { .. } => todo!(),
        }
    }
}