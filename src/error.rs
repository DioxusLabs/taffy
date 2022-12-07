//! The Error types produced by Taffy.
#[cfg(feature = "std")]
use core::fmt::{Display, Formatter, Result};

use crate::node::Node;

/// The error Taffy generates on invalid operations
pub type TaffyResult<T> = core::result::Result<T, TaffyError>;

/// An error that occurs while trying to access or modify a [`Node`]'s children by index.
#[derive(Debug)]
pub enum TaffyError {
    /// The parent [`Node`] does not have a child at `child_index`. It only has `child_count` children
    ChildIndexOutOfBounds {
        /// The parent node whose child was being looked up
        parent: Node,
        /// The index that was looked up
        child_index: usize,
        /// The total number of children the parent has
        child_count: usize,
    },
    /// The parent [`Node`] was not found in the [`Taffy`](crate::Taffy) instance.
    InvalidParentNode(Node),
    /// The child [`Node`] was not found in the [`Taffy`](crate::Taffy) instance.
    InvalidChildNode(Node),
    /// The supplied [`Node`] was not found in the [`Taffy`](crate::Taffy) instance.
    InvalidInputNode(Node),
}

#[cfg(feature = "std")]
impl Display for TaffyError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TaffyError::ChildIndexOutOfBounds { parent, child_index, child_count } => {
                write!(f, "Index (is {child_index}) should be < child_count ({child_count}) for parent node {parent:?}")
            }
            TaffyError::InvalidParentNode(parent) => {
                write!(f, "Parent Node {parent:?} is not in the Taffy instance")
            }
            TaffyError::InvalidChildNode(child) => write!(f, "Child Node {child:?} is not in the Taffy instance"),
            TaffyError::InvalidInputNode(node) => write!(f, "Supplied Node {node:?} is not in the Taffy instance"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TaffyError {}
