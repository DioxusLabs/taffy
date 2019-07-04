#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#[cfg(not(feature = "std"))]
extern crate alloc;

#[macro_use]
extern crate lazy_static;

pub mod geometry;
pub mod node;
pub mod number;
pub mod result;
pub mod style;

mod algo;
mod forest;
mod id;

pub use crate::node::Stretch;

use core::any::Any;

#[derive(Debug)]
pub enum Error {
    InvalidNode(node::Node),
    Measure(Box<Any>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::InvalidNode(ref node) => write!(f, "Invalid node {:?}", node),
            Error::Measure(_) => write!(f, "Error during measurement"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidNode(_) => "The node is not part of the stretch instance",
            Error::Measure(_) => "Error occurred inside a measurement function",
        }
    }
}
