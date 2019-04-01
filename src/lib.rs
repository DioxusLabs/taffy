#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod geometry;
pub mod result;
pub mod number;
pub mod style;
pub mod node;

mod algo;
mod ref_eq;
