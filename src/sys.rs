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

/// Returns the largest of two f32 values
pub(crate) fn f32_max(a: f32, b: f32) -> f32 {
    core::cmp::max_by(a, b, |a, b| a.total_cmp(b))
}

/// Returns the smallest of two f32 values
pub(crate) fn f32_min(a: f32, b: f32) -> f32 {
    core::cmp::min_by(a, b, |a, b| a.total_cmp(b))
}

/// For when `std` is enabled
#[cfg(feature = "std")]
mod std {
    /// An allocation-backend agnostic [`Box`] type
    pub(crate) type Box<A> = std::boxed::Box<A>;
    /// An allocation-backend agnostic vector type
    pub(crate) type Vec<A> = std::vec::Vec<A>;
    /// A vector of child nodes
    pub(crate) type ChildrenVec<A> = std::vec::Vec<A>;
    /// A vector of grid tracks
    pub(crate) type GridTrackVec<A> = std::vec::Vec<A>;

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
    /// An allocation-backend agnostic vector type
    pub(crate) type Vec<A> = alloc::vec::Vec<A>;
    /// A vector of child nodes
    pub(crate) type ChildrenVec<A> = alloc::vec::Vec<A>;
    /// A vector of grid tracks
    pub(crate) type GridTrackVec<A> = alloc::vec::Vec<A>;

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
    /// The maximum number of nodes in the tree
    pub const MAX_NODE_COUNT: usize = 256;
    /// The maximum number of children of any given node
    pub const MAX_CHILD_COUNT: usize = 16;
    /// The maximum number of children of any given node
    pub const MAX_GRID_TRACKS: usize = 16;

    /// An allocation-backend agnostic vector type
    pub(crate) type Vec<A> = arrayvec::ArrayVec<A, MAX_NODE_COUNT>;
    /// A vector of child nodes, whose length cannot exceed [`MAX_CHILD_COUNT`]
    pub(crate) type ChildrenVec<A> = arrayvec::ArrayVec<A, MAX_CHILD_COUNT>;
    /// A vector of grid tracks
    pub(crate) type GridTrackVec<A> = arrayvec::ArrayVec<A, MAX_GRID_TRACKS>;

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
