//! Allocator-flexible data types

// When std is enabled, prefer those types
#[cfg(feature = "std")]
pub(crate) use self::std::*;

// When alloc but not std is enabled, use those types
#[cfg(all(feature = "alloc", not(feature = "std")))]
pub(crate) use self::alloc::*;

// When neither alloc or std is enabled, use a heapless fallback
#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
pub(crate) use self::core::*;

/// For when `std` is enabled
#[cfg(feature = "std")]
mod std {
    /// An allocation-backend agnostic [`Box`] type
    pub(crate) type Box<A> = std::boxed::Box<A>;
    /// An allocation-backend agnostic map type
    pub(crate) type Map<K, V> = std::collections::HashMap<K, V>;
    /// An allocation-backend agnostic vector type
    pub(crate) type Vec<A> = std::vec::Vec<A>;
    /// A vector of child nodes
    pub(crate) type ChildrenVec<A> = std::vec::Vec<A>;
    /// A vector of parent nodes
    pub(crate) type ParentsVec<A> = std::vec::Vec<A>;

    /// Creates a new map with the capacity for the specified number of items before it must be resized
    #[must_use]
    pub(crate) fn new_map_with_capacity<K, V>(capacity: usize) -> Map<K, V>
    where
        K: Eq + std::hash::Hash,
    {
        Map::with_capacity(capacity)
    }

    /// Creates a new vector with the capacity for the specified number of items before it must be resized
    #[must_use]
    pub(crate) fn new_vec_with_capacity<A>(capacity: usize) -> Vec<A> {
        Vec::with_capacity(capacity)
    }

    /// Rounds to the nearest whole number
    #[must_use]
    pub(crate) fn round(value: f32) -> f32 {
        value.round()
    }

    /// Computes the absolute value
    #[must_use]
    pub(crate) fn abs(value: f32) -> f32 {
        value.abs()
    }
}

/// For when `alloc` but not `std` is enabled
#[cfg(all(feature = "alloc", not(feature = "std")))]
mod alloc {
    extern crate alloc;

    /// An allocation-backend agnostic `Box` type
    pub(crate) type Box<A> = alloc::boxed::Box<A>;
    /// An allocation-backend agnostic map type
    pub(crate) type Map<K, V> = hashbrown::HashMap<K, V>;
    /// An allocation-backend agnostic vector type
    pub(crate) type Vec<A> = alloc::vec::Vec<A>;
    /// A vector of child nodes
    pub(crate) type ChildrenVec<A> = alloc::vec::Vec<A>;
    /// A vector of parent nodes
    pub(crate) type ParentsVec<A> = alloc::vec::Vec<A>;

    /// Creates a new map with the capacity for the specified number of items before it must be resized
    #[must_use]
    pub(crate) fn new_map_with_capacity<K, V>(capacity: usize) -> Map<K, V> {
        Map::with_capacity(capacity)
    }

    /// Creates a new vector with the capacity for the specified number of items before it must be resized
    #[must_use]
    pub(crate) fn new_vec_with_capacity<A>(capacity: usize) -> Vec<A> {
        Vec::with_capacity(capacity)
    }

    /// Rounds to the nearest whole number
    #[must_use]
    pub(crate) fn round(value: f32) -> f32 {
        num_traits::float::FloatCore::round(value)
    }

    /// Computes the absolute value
    #[must_use]
    pub(crate) fn abs(value: f32) -> f32 {
        num_traits::float::FloatCore::abs(value)
    }
}

/// For when neither `alloc` nor `std` is enabled
#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
mod core {
    /// The maximum number of nodes in the forest
    pub const MAX_NODE_COUNT: usize = 256;
    /// The maximum number of children of any given node
    pub const MAX_CHILD_COUNT: usize = 16;
    /// The maximum number of parents for each child
    pub const MAX_PARENTS_COUNT: usize = 1;

    /// An allocation-backend agnostic map type
    pub(crate) type Map<K, V> = crate::indexmap::FnvIndexMap<K, V, MAX_NODE_COUNT>;
    /// An allocation-backend agnostic vector type
    pub(crate) type Vec<A> = arrayvec::ArrayVec<A, MAX_NODE_COUNT>;
    /// A vector of child nodes, whose length cannot exceed [`MAX_CHILD_COUNT`]
    pub(crate) type ChildrenVec<A> = arrayvec::ArrayVec<A, MAX_CHILD_COUNT>;
    /// A vector of parent nodes, whose length cannot exceed [`MAX_PARENTS_COUNT`]
    pub(crate) type ParentsVec<A> = arrayvec::ArrayVec<A, MAX_PARENTS_COUNT>;

    /// Creates a new map with the capacity for the specified number of items
    ///
    /// This map cannot be resized.
    #[must_use]
    pub(crate) fn new_map_with_capacity<K, V>(_capacity: usize) -> Map<K, V>
    where
        K: Eq + ::hash32::Hash,
    {
        Map::new()
    }

    /// Creates a new map with the capacity for the specified number of items before it must be resized
    ///
    /// This vector cannot be resized.
    #[must_use]
    pub(crate) fn new_vec_with_capacity<A, const CAP: usize>(_capacity: usize) -> arrayvec::ArrayVec<A, CAP> {
        arrayvec::ArrayVec::new()
    }

    /// Rounds to the nearest whole number
    #[inline]
    #[must_use]
    pub(crate) fn round(value: f32) -> f32 {
        num_traits::float::FloatCore::round(value)
    }

    /// Computes the absolute value
    #[inline]
    #[must_use]
    pub(crate) fn abs(value: f32) -> f32 {
        num_traits::float::FloatCore::abs(value)
    }
}
