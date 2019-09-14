#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

#[macro_use]
extern crate lazy_static;

#[cfg_attr(feature = "serde", macro_use)]
#[cfg(feature = "serde")]
extern crate serde;

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
    Measure(Box<dyn Any>),
}

#[cfg(feature = "std")]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::InvalidNode(ref node) => write!(f, "Invalid node {:?}", node),
            Error::Measure(_) => write!(f, "Error during measurement"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidNode(_) => "The node is not part of the stretch instance",
            Error::Measure(_) => "Error occurred inside a measurement function",
        }
    }
}
