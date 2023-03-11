#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

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

pub mod compute;
pub mod geometry;
pub mod prelude;
pub mod style;
pub mod style_helpers;
pub mod tree;
pub mod util;

#[cfg(feature = "css")]
pub mod css;

#[cfg(feature = "flexbox")]
pub use crate::compute::flexbox::FlexboxAlgorithm;
#[cfg(feature = "grid")]
pub use crate::compute::grid::CssGridAlgorithm;
pub use crate::compute::LayoutAlgorithm;
pub use crate::tree::LayoutTree;
#[cfg(feature = "taffy_tree")]
pub use crate::tree::{Taffy, TaffyError, TaffyResult};
