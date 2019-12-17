//! Identifier for a Node
//!
//!
use core::sync::atomic;

/// Internal node id.
pub(crate) type NodeId = usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(not(any(feature = "std", feature = "alloc")), derive(hash32_derive::Hash32))]
pub(crate) struct Id(u64);

pub(crate) struct Allocator {
    new_id: atomic::AtomicU64,
}

impl Allocator {
    pub const fn new() -> Self {
        Allocator { new_id: atomic::AtomicU64::new(0) }
    }

    pub fn allocate(&self) -> Id {
        Id(self.new_id.fetch_add(1, atomic::Ordering::Relaxed))
    }

    pub fn free(&self, _ids: &[Id]) {}
}
