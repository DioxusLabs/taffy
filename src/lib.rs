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

pub mod error;
pub mod geometry;
pub mod layout;
pub mod math;
pub mod node;
pub mod prelude;
pub mod style;

mod flexbox;
mod forest;
#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
mod indexmap;
mod sys;

pub use crate::node::Taffy;
