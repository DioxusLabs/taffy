//! Generic code that is shared between multiple layout algorithms
pub(crate) mod alignment;
mod stack_frame_cache;
pub(crate) use stack_frame_cache::StackFrameCache;
