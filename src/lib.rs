#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod geometry;
pub mod layout;
pub mod number;
pub mod style;

mod algo;
mod ref_eq;
pub use crate::algo::compute;
