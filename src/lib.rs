#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]

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
mod indexmap;
mod sys;

#[cfg(feature = "std")]
use core::fmt::{Display, Formatter, Result};

pub use crate::node::Sprawl;

#[derive(Debug)]
pub enum Error {
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
            Error::InvalidNode(_) => "The node is not part of the sprawl instance",
        }
    }
}
