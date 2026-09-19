//! Reusable scratch buffers for the layout algorithms
//!
//! Each layout algorithm needs a handful of temporary vectors (the list of items being laid out, the
//! list of flex lines, the list of grid tracks, etc.) for the duration of a single call. Rather than
//! allocating these afresh for every node, a tree may hold a [`LayoutScratch`] and hand it out via
//! [`LayoutPartialTree::layout_scratch`], in which case the algorithms take their buffers from it and
//! return them (cleared, but with their capacity intact) when they are done.

use crate::tree::LayoutPartialTree;
use crate::util::sys::Vec;
use core::fmt::{self, Debug};

/// A free-list of vectors of a single element type
///
/// Layout is recursive, so a buffer taken by a parent node is still in use while its children are
/// laid out. The pool therefore holds one spare vector per level of nesting that has been reached.
pub(crate) struct Pool<T>(Vec<Vec<T>>);

impl<T> Default for Pool<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> Pool<T> {
    /// Take an empty vector from the pool (or create a new one if the pool is empty)
    #[inline]
    pub(crate) fn take(&mut self) -> Vec<T> {
        self.0.pop().unwrap_or_default()
    }

    /// Take an empty vector from the pool with room for at least `capacity` elements
    #[inline]
    pub(crate) fn take_with_capacity(&mut self, capacity: usize) -> Vec<T> {
        let mut vec = self.take();
        vec.reserve(capacity);
        vec
    }

    /// Return a vector to the pool. Its contents are dropped but its allocation is kept.
    #[inline]
    pub(crate) fn give(&mut self, mut vec: Vec<T>) {
        vec.clear();
        if vec.capacity() > 0 {
            self.0.push(vec);
        }
    }

    /// Take an empty vector from the pool, reinterpreted as a vector of `U`
    ///
    /// This is used to store vectors whose element type carries a lifetime (e.g. `FlexLine<'a>`) in the
    /// pool as their `'static` counterpart. The vector is empty so no elements are ever converted; the
    /// allocation itself is carried over by the standard library's in-place `collect` specialization
    /// whenever `T` and `U` have the same size and alignment (which they do when they only differ by
    /// a lifetime). If the specialization does not apply this simply results in a fresh allocation.
    #[inline]
    pub(crate) fn take_as<U>(&mut self) -> Vec<U> {
        recycle(self.take())
    }

    /// Return a vector of `U` to the pool, reinterpreted as a vector of `T`. See [`Pool::take_as`].
    #[inline]
    pub(crate) fn give_as<U>(&mut self, mut vec: Vec<U>) {
        vec.clear();
        self.give(recycle(vec));
    }
}

/// Convert an empty `Vec<T>` into an empty `Vec<U>`, reusing the allocation where possible
#[inline]
fn recycle<T, U>(mut vec: Vec<T>) -> Vec<U> {
    vec.clear();
    vec.into_iter().map(|_| unreachable!("recycled vector is empty")).collect()
}

/// Scratch buffers which Taffy's layout algorithms can reuse across calls, avoiding a number of
/// heap allocations for each node that is laid out.
///
/// A tree implementation can hold one of these and return it from
/// [`LayoutPartialTree::layout_scratch`]. The buffers are cleared after each use, so no state leaks
/// between nodes; only the allocations are retained. Call [`LayoutScratch::clear`] to release them.
///
/// [`TaffyTree`](crate::TaffyTree) holds a `LayoutScratch` internally.
#[derive(Default)]
pub struct LayoutScratch {
    /// Buffers used by the flexbox algorithm
    #[cfg(feature = "flexbox")]
    pub(crate) flexbox: crate::compute::flexbox::FlexboxScratch,
    /// Buffers used by the block algorithm
    #[cfg(feature = "block_layout")]
    pub(crate) block: crate::compute::block::BlockScratch,
    /// Buffers used by the grid algorithm
    #[cfg(feature = "grid")]
    pub(crate) grid: crate::compute::grid::GridScratch,
}

impl LayoutScratch {
    /// Create an empty `LayoutScratch`
    pub fn new() -> Self {
        Self::default()
    }

    /// Release all retained allocations
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

impl Debug for LayoutScratch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("LayoutScratch")
    }
}

impl Clone for LayoutScratch {
    /// Cloning a `LayoutScratch` yields an empty one: the retained allocations are not copied.
    fn clone(&self) -> Self {
        Self::default()
    }
}

/// Take a buffer from the tree's scratch space (if it provides one), or else create a new one
#[inline]
pub(crate) fn take<T>(
    tree: &mut impl LayoutPartialTree,
    pool: impl FnOnce(&mut LayoutScratch) -> &mut Pool<T>,
    capacity: usize,
) -> Vec<T> {
    match tree.layout_scratch() {
        Some(scratch) => pool(scratch).take_with_capacity(capacity),
        None => Vec::with_capacity(capacity),
    }
}

/// Return a buffer to the tree's scratch space (if it provides one), or else drop it
#[inline]
pub(crate) fn give<T>(
    tree: &mut impl LayoutPartialTree,
    pool: impl FnOnce(&mut LayoutScratch) -> &mut Pool<T>,
    vec: Vec<T>,
) {
    if let Some(scratch) = tree.layout_scratch() {
        pool(scratch).give(vec);
    }
}

/// Like [`take`], but reinterpreting the element type. See [`Pool::take_as`].
#[inline]
pub(crate) fn take_as<T, U>(
    tree: &mut impl LayoutPartialTree,
    pool: impl FnOnce(&mut LayoutScratch) -> &mut Pool<T>,
) -> Vec<U> {
    match tree.layout_scratch() {
        Some(scratch) => pool(scratch).take_as(),
        None => Vec::new(),
    }
}

/// Like [`give`], but reinterpreting the element type. See [`Pool::give_as`].
#[inline]
pub(crate) fn give_as<T, U>(
    tree: &mut impl LayoutPartialTree,
    pool: impl FnOnce(&mut LayoutScratch) -> &mut Pool<T>,
    vec: Vec<U>,
) {
    if let Some(scratch) = tree.layout_scratch() {
        pool(scratch).give_as(vec);
    }
}
