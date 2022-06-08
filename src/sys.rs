// When std is enabled, prefer those types
#[cfg(feature = "std")]
pub use std_data_types::*;

// When alloc but not std is enabled, use those types
#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc_data_types::*;

// When neither alloc or std is enabled, use a heapless fallback
#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
pub use core_data_types::*;

#[cfg(feature = "std")]
mod std_data_types {
    pub type Box<A> = ::std::boxed::Box<A>;
    pub type Map<K, V> = ::std::collections::HashMap<K, V>;
    pub type Vec<A> = ::std::vec::Vec<A>;
    pub type ChildrenVec<A> = ::std::vec::Vec<A>;
    pub type ParentsVec<A> = ::std::vec::Vec<A>;

    pub fn new_map_with_capacity<K, V>(capacity: usize) -> Map<K, V>
    where
        K: Eq + ::std::hash::Hash,
    {
        Map::with_capacity(capacity)
    }

    pub fn new_vec_with_capacity<A>(capacity: usize) -> Vec<A> {
        Vec::with_capacity(capacity)
    }

    #[inline]
    pub fn round(value: f32) -> f32 {
        value.round()
    }

    #[inline]
    pub fn abs(value: f32) -> f32 {
        value.abs()
    }
}

#[cfg(feature = "alloc")]
mod alloc_data_types {
    extern crate alloc;

    type Box<A> = alloc::boxed::Box<A>;
    type Map<K, V> = hashbrown::HashMap<K, V>;
    type Vec<A> = alloc::vec::Vec<A>;
    type ChildrenVec<A> = alloc::vec::Vec<A>;
    type ParentsVec<A> = alloc::vec::Vec<A>;

    fn new_map_with_capacity<K, V>(capacity: usize) -> Map<K, V> {
        Map::with_capacity(capacity)
    }

    fn new_vec_with_capacity<A>(capacity: usize) -> Vec<A> {
        Vec::with_capacity(capacity)
    }

    #[inline]
    fn round(value: f32) -> f32 {
        num_traits::float::FloatCore::round(value)
    }

    #[inline]
    fn abs(value: f32) -> f32 {
        num_traits::float::FloatCore::abs(value)
    }
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
mod core_data_types {
    const MAX_NODE_COUNT: usize = 256;
    const MAX_CHILD_COUNT: usize = 16;
    const MAX_PARENTS_COUNT: usize = 1;

    type Map<K, V> = heapless::FnvIndexMap<K, V, MAX_NODE_COUNT>;
    type Vec<A> = arrayvec::ArrayVec<A, MAX_NODE_COUNT>;
    type ChildrenVec<A> = arrayvec::ArrayVec<A, MAX_CHILD_COUNT>;
    type ParentsVec<A> = arrayvec::ArrayVec<A, MAX_PARENTS_COUNT>;

    fn new_map_with_capacity<K, V>(_capacity: usize) -> Map<K, V>
    where
        K: Eq + ::hash32::Hash,
    {
        Map::new()
    }

    fn new_vec_with_capacity<A, const CAP: usize>(_capacity: usize) -> arrayvec::ArrayVec<A, CAP> {
        arrayvec::ArrayVec::new()
    }

    #[inline]
    fn round(value: f32) -> f32 {
        num_traits::float::FloatCore::round(value)
    }

    #[inline]
    pub fn abs(value: f32) -> f32 {
        num_traits::float::FloatCore::abs(value)
    }
}
