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
pub enum Error {
    /// The [`Node`](node::Node) was invalid
    InvalidNode(node::Node),
}

#[cfg(feature = "std")]
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Error::InvalidNode(ref node) => write!(f, "Invalid node {:?}", node),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidNode(_) => "The node is not part of the Taffy instance",
        }
    }
}
