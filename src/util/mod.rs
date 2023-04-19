//! Helpful misc. utilities such as a function to debug print a tree

mod math;
pub(crate) use math::MaybeMath;
mod resolve;
pub(crate) use resolve::{MaybeResolve, ResolveOrZero};
pub(crate) mod sys;

#[cfg(feature = "std")]
#[doc(hidden)]
pub(crate) mod debug;
#[cfg(feature = "std")]
pub use debug::print_tree;
