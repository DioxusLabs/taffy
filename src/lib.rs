#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod geometry;
pub mod node;
pub mod number;
pub mod result;
pub mod style;

mod algo;
