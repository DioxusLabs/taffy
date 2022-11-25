#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![forbid(unsafe_code)]

// We always need std for the tests
// See <https://github.com/la10736/rstest/issues/149#issuecomment-1156402989>
#[cfg(all(test, not(feature = "std")))]
#[macro_use]
extern crate std;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[cfg_attr(feature = "serde", macro_use)]
#[cfg(feature = "serde")]
extern crate serde;

#[doc(hidden)]
pub mod debug;
pub mod error;
pub mod geometry;
pub mod layout;
pub mod math;
pub mod node;
pub mod prelude;
pub mod style;
pub mod tree;

#[cfg(feature = "random")]
pub mod randomizable;

mod compute;
mod data;
mod resolve;
mod sys;

pub use crate::compute::compute_layout;
pub use crate::node::Taffy;
