//! Identifier for a Node
//!
//!
use core::sync::atomic::{AtomicUsize, Ordering};

/// Internal node id.
pub(crate) type NodeId = usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(not(any(feature = "std", feature = "alloc")), derive(hash32_derive::Hash32))]
pub(crate) struct Id(usize);

pub(crate) struct Allocator {
    new_id: AtomicUsize,
}

impl Allocator {
    pub const fn new() -> Self {
        Self { new_id: AtomicUsize::new(0) }
    }

    pub fn allocate(&self) -> Id {
        Id(self.new_id.fetch_add(1, Ordering::Relaxed))
    }

    pub fn free(&self, _ids: &[Id]) {}
}
