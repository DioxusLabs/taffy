//! Generic code that is shared between multiple layout algorithms
pub(crate) mod alignment;
#[cfg(any(feature = "block_layout", feature = "flexbox", feature = "grid"))]
pub(crate) mod containment;
pub(crate) mod sizing_keyword;

#[cfg(feature = "content_size")]
pub(crate) mod content_size;
