//! Helpful misc. utilities such as a function to debug print a tree
mod math;
mod resolve;
pub(crate) mod sys;

pub use math::MaybeMath;
pub use resolve::{MaybeResolve, ResolveOrZero};

#[doc(hidden)]
#[macro_use]
pub(crate) mod debug;

#[cfg(feature = "std")]
mod print;
#[cfg(feature = "std")]
pub use print::print_tree;
