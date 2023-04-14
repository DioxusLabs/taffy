//! A cache for storing the results of layout computation
use crate::geometry::Size;
use crate::style::AvailableSpace;
use crate::tree::{RunMode, SizeAndBaselines};

/// The number of cache entries for each node in the tree
const CACHE_SIZE: usize = 7;

/// Cached intermediate layout results
#[derive(Debug, Clone, Copy)]
pub struct CacheEntry {
    /// The initial cached size of the node itself
    known_dimensions: Size<Option<f32>>,
    /// The initial cached size of the parent's node
    available_space: Size<AvailableSpace>,
    /// Whether or not layout should be recomputed
    run_mode: RunMode,

    /// The cached size and baselines of the item
    cached_size_and_baselines: SizeAndBaselines,
}

/// A cache for caching the results of a sizing a Grid Item or Flexbox Item
pub struct Cache {
    /// An array of entries in the cache
    entries: [Option<CacheEntry>; CACHE_SIZE],
}

impl Cache {
    /// Create a new empty cache
    pub const fn new() -> Self {
        Self { entries: [None; CACHE_SIZE] }
    }

    /// Return the cache slot to cache the current computed result in
    ///
    /// ## Caching Strategy
    ///
    /// We need multiple cache slots, because a node's size is often queried by it's parent multiple times in the course of the layout
    /// process, and we don't want later results to clobber earlier ones.
    ///
    /// The two variables that we care about when determining cache slot are:
    ///
    ///   - How many "known_dimensions" are set. In the worst case, a node may be called first with neither dimension known, then with one
    ///     dimension known (either width of height - which doesn't matter for our purposes here), and then with both dimensions known.
    ///   - Whether unknown dimensions are being sized under a min-content or a max-content available space constraint (definite available space
    ///     shares a cache slot with max-content because a node will generally be sized under one or the other but not both).
    ///
    /// ## Cache slots:
    ///
    /// - Slot 0: Both known_dimensions were set
    /// - Slot 1: width but not height known_dimension was set and the other dimension was either a MaxContent or Definite available space constraintraint
    /// - Slot 2: width but not height known_dimension was set and the other dimension was a MinContent constraint
    /// - Slot 3: height but not width known_dimension was set and the other dimension was either a MaxContent or Definite available space constraintable space constraint
    /// - Slot 4: height but not width known_dimension was set and the other dimension was a MinContent constraint
    /// - Slot 5: Neither known_dimensions were set and we are sizing under a MaxContent or Definite available space constraint
    /// - Slot 6: Neither known_dimensions were set and we are sizing under a MinContent constraint
    #[inline]
    fn compute_cache_slot(known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> usize {
        let has_known_width = known_dimensions.width.is_some();
        let has_known_height = known_dimensions.height.is_some();

        // Slot 0: Both known_dimensions were set
        if has_known_width && has_known_height {
            return 0;
        }

        // Slot 1: width but not height known_dimension was set and the other dimension was either a MaxContent or Definite available space constraint
        // Slot 2: width but not height known_dimension was set and the other dimension was a MinContent constraint
        if has_known_width && !has_known_height {
            return 1 + (available_space.height == AvailableSpace::MinContent) as usize;
        }

        // Slot 3: height but not width known_dimension was set and the other dimension was either a MaxContent or Definite available space constraint
        // Slot 4: height but not width known_dimension was set and the other dimension was a MinContent constraint
        if !has_known_width && has_known_height {
            return 3 + (available_space.width == AvailableSpace::MinContent) as usize;
        }

        // Slot 5: Neither known_dimensions were set and we are sizing under a MaxContent or Definite available space constraint
        // Slot 6: Neither known_dimensions were set and we are sizing under a MinContent constraint
        5 + (available_space.width == AvailableSpace::MinContent) as usize
    }

    /// Try to retrieve a cached result from the cache
    #[inline]
    pub fn get(
        &self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: RunMode,
    ) -> Option<SizeAndBaselines> {
        for entry in self.entries.iter().flatten() {
            // Cached ComputeSize results are not valid if we are running in PerformLayout mode
            if entry.run_mode == RunMode::ComputeSize && run_mode == RunMode::PeformLayout {
                return None;
            }

            let cached_size = entry.cached_size_and_baselines.size;

            if (known_dimensions.width == entry.known_dimensions.width
                || known_dimensions.width == Some(cached_size.width))
                && (known_dimensions.height == entry.known_dimensions.height
                    || known_dimensions.height == Some(cached_size.height))
                && (known_dimensions.width.is_some()
                    || entry.available_space.width.is_roughly_equal(available_space.width))
                && (known_dimensions.height.is_some()
                    || entry.available_space.height.is_roughly_equal(available_space.height))
            {
                return Some(entry.cached_size_and_baselines);
            }
        }

        None
    }

    /// Store a computed size in the cache
    pub fn store(
        &mut self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: RunMode,
        cached_size_and_baselines: SizeAndBaselines,
    ) {
        let cache_slot = Self::compute_cache_slot(known_dimensions, available_space);
        self.entries[cache_slot] =
            Some(CacheEntry { known_dimensions, available_space, run_mode, cached_size_and_baselines });
    }

    /// Clear all cache entries
    pub fn clear(&mut self) {
        self.entries = [None; CACHE_SIZE];
    }

    /// Returns true if all cache entries are None, else false
    pub fn is_empty(&self) -> bool {
        !self.entries.iter().any(|entry| entry.is_some())
    }
}
