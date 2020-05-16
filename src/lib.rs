#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[cfg_attr(feature = "serde", macro_use)]
#[cfg(feature = "serde")]
extern crate serde;

pub mod geometry;
pub mod node;
pub mod number;
pub mod prelude;
pub mod result;
pub mod style;

mod algo;
mod forest;
mod id;

mod sys;

pub use crate::node::Stretch;

#[derive(Debug)]
pub enum Error {
    InvalidNode(node::Node),
}

#[cfg(feature = "std")]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::InvalidNode(ref node) => write!(f, "Invalid node {:?}", node),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidNode(_) => "The node is not part of the stretch instance",
        }
    }
}
