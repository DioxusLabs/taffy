//! The Error types produced by Taffy.
#[cfg(feature = "std")]
use core::fmt::{Display, Formatter, Result};

use crate::node::Node;

/// The [`Node`](node::Node) was not found in the [`Taffy`] instance
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

/// An error that occurs while trying to access or modify a [`Node`](node::Node)'s children by index.
#[derive(Debug)]
pub enum InvalidChild {
    /// The parent [`Node`](node::Node) does not have a child at `child_index`. It only has `child_count` children
    ChildIndexOutOfBounds {
        /// The parent node whose child was being looked up
        parent: Node,
        /// The index that was looked up
        child_index: usize,
        /// The total number of children the parent has
        child_count: usize,
    },
    /// The [`Node`](node::Node) was not found in the [`Taffy`] instance.
    InvalidNode(InvalidNode),
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
            InvalidChild::InvalidNode(inner) => inner.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidChild {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InvalidChild::ChildIndexOutOfBounds { .. } => None,
            InvalidChild::InvalidNode(node) => Some(node),
        }
    }
}

impl From<InvalidNode> for InvalidChild {
    fn from(node: InvalidNode) -> Self {
        Self::InvalidNode(node)
    }
}
