//! Generic code that is shared between multiple layout algorithms
pub(crate) mod alignment;
pub(crate) mod sizing_keyword;

#[cfg(feature = "content_size")]
pub(crate) mod content_size;
