//! # Taffy
//!
//! [![GitHub CI](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml/badge.svg)](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml)
//! [![crates.io](https://img.shields.io/crates/v/taffy.svg)](https://crates.io/crates/taffy)
//! [![docs.rs](https://img.shields.io/docsrs/taffy)](https://docs.rs/taffy)
//!
//! Taffy is a flexible, high-performance, cross-platform UI layout library written in [Rust](https://www.rust-lang.org).
//!
//! It currently implements the **Flexbox** and **CSS Grid** layout algorithms. Support for other paradigms is planned.
//! For more information on this and other future development plans see the [roadmap issue](https://github.com/DioxusLabs/taffy/issues/345).
//!
//! ## Architecture
//!
//! Taffy is based on a tree of "UI nodes" similar to the tree of DOM nodes that one finds in web-based UI. Each node has:
//!   - A [`Style`](crate::style::Style) struct which holds a set of CSS styles which function as the primary input to the layout computations.
//!   - A [`Layout`](crate::tree::Layout) struct containing a position (x/y) and a size (width/height) which function as the output of the layout computations.
//!   - Optionally, one of:
//!       - A `Vec` set of child nodes
//!       - A [`MeasureFunc`](crate::tree::MeasureFunc) which computes the node's "intrinsic size"
//!
//! Usage of Taffy consists of constructing a tree of UI nodes (with associated styles, children and measure functions), then calling function(s)
//! from Taffy to translate those styles, parent-child relationships and measure functions into a size and position in 2d space for each node
//! in the tree.
//!
//! ## High-level API vs. Low-level API
//!
//! Taffy has two APIs: a high-level API that is simpler and easier to get started with, and a low-level API that is more flexible gives greater control.
//!
//! We would generally recommend the high-level API for users using Taffy standalone and the low-level API for users wanting to embed Taffy as part of
//! a wider layout system or as part of a UI framework that already has it's own node/widget tree representation.
//!
//! - **The high-level API** consists of the [`Taffy`](crate::Taffy) struct which contains a tree implementation and provides methods that allow you to construct
//!   a tree of UI nodes. Once constructed, you can call the [`compute_layout`](crate::Taffy::compute_layout) method to compute the layout, and then access
//!   the layout of each node using the [`layout`](crate::Taffy::compute_layout) method.
//!
//!   When using the high-level API, Taffy will take care of node storage, caching and dispatching to the correct layout algorithm for a given node for you.
//!   See the [`Taffy`](crate::Taffy) struct for more details on this API.
//!   <br /><br />
//!
//! - **The low-level API** consists of the [`LayoutTree`](crate::LayoutTree) trait which defines an interface behind which you must implement your own
//!   tree implementation, and the [`FlexboxAlgorithm`] and [`CssGridAlgorithm`] structs which contain methods which allow you to integrate Flexbox or
//!   CSS Grid layout computation into your own layout algorithm.
//!
//!   When using this API, you must handle node storage, caching, and dispatching to the correct layout algorithm for a given node yourself.
//!   See the [`LayoutTree`](crate::LayoutTree) trait for more details on this API.

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

pub mod compute;
pub mod geometry;
pub mod prelude;
pub mod style;
pub mod style_helpers;
pub mod tree;
pub mod util;

mod readme_doctest {
    #![doc = include_str!("../README.md")]
}

#[cfg(feature = "flexbox")]
pub use crate::compute::flexbox::FlexboxAlgorithm;
#[cfg(feature = "grid")]
pub use crate::compute::grid::CssGridAlgorithm;
pub use crate::compute::LayoutAlgorithm;
pub use crate::tree::{LayoutTree, Taffy, TaffyError, TaffyResult};
