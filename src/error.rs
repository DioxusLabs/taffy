//! The Error types produced by Taffy.
#[cfg(feature = "std")]
use core::fmt::{Display, Formatter, Result};

use crate::node::Node;

/// The [`Node`] was not found in the [`Taffy`](crate::Taffy) instance
#[derive(Debug)]
pub struct InvalidNode(pub Node);

#[cfg(feature = "std")]
impl Display for InvalidNode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Node {:?} is not in the Taffy instance", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidNode {}

/// An error that occurs while trying to access or modify a [`Node`]'s children by index.
#[derive(Debug)]
pub enum InvalidChild {
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
}

#[cfg(feature = "std")]
impl Display for InvalidChild {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InvalidChild::ChildIndexOutOfBounds { parent, child_index, child_count } => write!(
                f,
                "Index (is {}) should be < child_count ({}) for parent node {:?}",
                child_index, child_count, parent
            ),
            InvalidChild::InvalidParentNode(parent) => {
                write!(f, "Parent Node {:?} is not in the Taffy instance", parent)
            }
            InvalidChild::InvalidChildNode(child) => write!(f, "Child Node {:?} is not in the Taffy instance", child),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidChild {}
