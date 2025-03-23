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
    /// An allocation-backend agnostic vector type
    pub(crate) type Vec<A> = std::vec::Vec<A>;
    /// A vector of child nodes
    pub(crate) type ChildrenVec<A> = std::vec::Vec<A>;
    #[cfg(feature = "grid")]
    /// A vector of grid tracks
    pub(crate) type GridTrackVec<A> = std::vec::Vec<A>;

    /// Creates a new vector with the capacity for the specified number of items before it must be resized
    #[must_use]
    pub(crate) fn new_vec_with_capacity<A>(capacity: usize) -> Vec<A> {
        Vec::with_capacity(capacity)
    }

    /// Rounds to the nearest whole number
    #[must_use]
    #[inline(always)]
    pub(crate) fn round(value: f32) -> f32 {
        value.round()
    }

    /// Rounds up to the nearest whole number
    #[must_use]
    #[inline(always)]
    pub(crate) fn ceil(value: f32) -> f32 {
        value.ceil()
    }

    /// Rounds down to the nearest whole number
    #[must_use]
    #[inline(always)]
    pub(crate) fn floor(value: f32) -> f32 {
        value.floor()
    }

    /// Computes the absolute value
    #[must_use]
    #[inline(always)]
    pub(crate) fn abs(value: f32) -> f32 {
        value.abs()
    }

    /// Returns the largest of two f32 values
    #[inline(always)]
    pub(crate) fn f32_max(a: f32, b: f32) -> f32 {
        a.max(b)
    }

    /// Returns the smallest of two f32 values
    #[inline(always)]
    pub(crate) fn f32_min(a: f32, b: f32) -> f32 {
        a.min(b)
    }
}

/// For when `alloc` but not `std` is enabled
#[cfg(all(feature = "alloc", not(feature = "std")))]
mod alloc {
    extern crate alloc;
    use core::cmp::Ordering;

    /// An allocation-backend agnostic vector type
    pub(crate) type Vec<A> = alloc::vec::Vec<A>;
    /// A vector of child nodes
    pub(crate) type ChildrenVec<A> = alloc::vec::Vec<A>;
    #[cfg(feature = "grid")]
    /// A vector of grid tracks
    pub(crate) type GridTrackVec<A> = alloc::vec::Vec<A>;

    /// Creates a new vector with the capacity for the specified number of items before it must be resized
    #[must_use]
    pub(crate) fn new_vec_with_capacity<A>(capacity: usize) -> Vec<A> {
        Vec::with_capacity(capacity)
    }

    /// Rounds to the nearest whole number
    pub(crate) use super::polyfill::round;

    /// Rounds up to the nearest whole number
    pub(crate) use super::polyfill::ceil;

    /// Rounds down to the nearest whole number
    pub(crate) use super::polyfill::floor;

    /// Computes the absolute value
    pub(crate) use super::polyfill::abs;

    /// Returns the largest of two f32 values
    #[inline(always)]
    pub(crate) fn f32_max(a: f32, b: f32) -> f32 {
        a.max(b)
    }

    /// Returns the smallest of two f32 values
    #[inline(always)]
    pub(crate) fn f32_min(a: f32, b: f32) -> f32 {
        a.min(b)
    }
}

/// For when neither `alloc` nor `std` is enabled
#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
mod core {
    use core::cmp::Ordering;

    /// The maximum number of nodes in the tree
    pub const MAX_NODE_COUNT: usize = 256;
    /// The maximum number of children of any given node
    pub const MAX_CHILD_COUNT: usize = 16;
    #[cfg(feature = "grid")]
    /// The maximum number of children of any given node
    pub const MAX_GRID_TRACKS: usize = 16;

    /// An allocation-backend agnostic vector type
    pub(crate) type Vec<A> = arrayvec::ArrayVec<A, MAX_NODE_COUNT>;
    /// A vector of child nodes, whose length cannot exceed [`MAX_CHILD_COUNT`]
    pub(crate) type ChildrenVec<A> = arrayvec::ArrayVec<A, MAX_CHILD_COUNT>;
    #[cfg(feature = "grid")]
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
    pub(crate) use super::polyfill::round;

    /// Computes the absolute value
    pub(crate) use super::polyfill::abs;

    /// Returns the largest of two f32 values
    #[inline(always)]
    pub(crate) fn f32_max(a: f32, b: f32) -> f32 {
        a.max(b)
    }

    /// Returns the smallest of two f32 values
    #[inline(always)]
    pub(crate) fn f32_min(a: f32, b: f32) -> f32 {
        a.min(b)
    }
}

/// Implementations of float functions for no_std and alloc builds
/// Copied from `num-traits` crate
#[cfg(not(feature = "std"))]
mod polyfill {
    #[must_use]
    #[inline(always)]
    fn fract(value: f32) -> f32 {
        if value == 0.0 {
            0.0
        } else {
            value % 1.0
        }
    }

    #[must_use]
    #[inline(always)]
    pub(crate) fn round(value: f32) -> f32 {
        let f = fract(value);
        if f.is_nan() || f == 0.0 {
            value
        } else if value > 0.0 {
            if f < 0.5 {
                value - f
            } else {
                value - f + 1.0
            }
        } else if -f < 0.5 {
            value - f
        } else {
            value - f - 1.0
        }
    }

    #[must_use]
    #[inline(always)]
    pub(crate) fn floor(value: f32) -> f32 {
        let f = fract(value);
        if f.is_nan() || f == 0.0 {
            value
        } else if value < 0.0 {
            value - f - 1.0
        } else {
            value - f
        }
    }

    #[must_use]
    #[inline(always)]
    pub(crate) fn ceil(value: f32) -> f32 {
        let f = fract(value);
        if f.is_nan() || f == 0.0 {
            value
        } else if value > 0.0 {
            value - f + 1.0
        } else {
            value - f
        }
    }

    /// Computes the absolute value
    #[must_use]
    #[inline(always)]
    pub(crate) fn abs(value: f32) -> f32 {
        if value.is_sign_positive() {
            return value;
        } else if value.is_sign_negative() {
            return -value;
        } else {
            f32::NAN
        }
    }
}
