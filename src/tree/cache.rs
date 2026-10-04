//! A cache for storing the results of layout computation

#![allow(clippy::unusual_byte_groupings)]

use crate::geometry::Size;
use crate::style::AvailableSpace;
use crate::tree::{CollapsibleMarginSet, LayoutInput, LayoutOutput, RunMode};
use crate::RequestedAxis;

/// The number of cache entries for each node in the tree
const CACHE_SIZE: usize = 9;

// Manually written-out results of float to u32 bit casts because
// `f32::to_bits` is not yet const at our MSRV.

/// `f32::INFINITY` as a u32
const INFINITY_BITS: u32 = 0b_0_11111111_00000000000000000000000_u32;
/// `f32::NEG_INFINITY` as a u32
const NEG_INFINITY_BITS: u32 = 0b_1_11111111_00000000000000000000000_u32;

// The `CacheKey` encodes two f32s as a u64. We know that the f32s will always be
// non-negative, so we pack two extra bits encoding the `RequestedAxis` into the
// sign bits of the f32s. These constants help to encode and decode those bits.

/// The sign bit of the first f32
const SIGN_BIT_1: u64 = 1u64 << 63;
/// The sign bit of the second f32
const SIGN_BIT_2: u64 = 1u64 << 31;
/// Mask of both sign bits (used to compute NON_SIGN_BITS_MASK)
const BOTH_SIGN_BITS_MASK: u64 = SIGN_BIT_1 | SIGN_BIT_2;
/// Mask of excluding the sign bits (used when setting/getting the size excluding the packed bits)
const NON_SIGN_BITS_MASK: u64 = !BOTH_SIGN_BITS_MASK;

/// Mask which includes only the bits which encode the x-axis value that we can use to ignore the
/// y-axis value when comparing a cache key.
const X_AXIS_VALUE_MASK: u64 = (u32::MAX as u64) << 32;

/// Pack `Option<f32>` into `u32`
#[inline(always)]
fn option_cache_key(input: Option<f32>) -> u32 {
    match input {
        Some(value) => value.to_bits(),
        None => INFINITY_BITS,
    }
}

/// Pack `Size<Option<f32>>` into `u64`
#[inline(always)]
fn size_option_cache_key(input: Size<Option<f32>>) -> u64 {
    (option_cache_key(input.width) as u64) << 32 | option_cache_key(input.height) as u64
}

/// The bits of a quiet NaN, used to encode the absence of a min or max size. Unlike infinity (which is
/// a meaningful value for a min or max size) no resolved style value has this bit pattern.
const NO_BOUND_BITS: u32 = u32::MAX;

/// Pack a `Size<Option<f32>>` that holds a min or max size into `u64`
#[inline(always)]
fn size_bound_cache_key(input: Size<Option<f32>>) -> u64 {
    let bits = |bound: Option<f32>| match bound {
        Some(value) => value.to_bits(),
        None => NO_BOUND_BITS,
    };
    (bits(input.width) as u64) << 32 | bits(input.height) as u64
}

/// The min and max sizes that were passed to a node ([`LayoutInput::min_size`] and [`LayoutInput::max_size`]),
/// packed into two `u64`s.
///
/// Most nodes have neither a min nor a max size, and the ones that do are nearly always passed the same
/// sizes every time that they are laid out (the sizes only vary if they are percentages and the size that
/// they resolve against varies). So rather than storing the sizes in every [`CacheKey`], each key only
/// records *whether* it was computed with any min or max size ([`CacheKey::is_bounded`]) and the [`Cache`]
/// records the single set of sizes that all of its bounded entries were computed with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
struct CacheBounds {
    /// The min size that was passed to the node
    min_size: u64,
    /// The max size that was passed to the node
    max_size: u64,
}

impl CacheBounds {
    /// Neither a min nor a max size in either axis
    const NONE: Self = Self { min_size: u64::MAX, max_size: u64::MAX };

    /// Whether there is a min or a max size in either axis
    #[inline(always)]
    fn is_bounded(&self) -> bool {
        (self.min_size & self.max_size) != u64::MAX
    }
}

impl From<&LayoutInput> for CacheBounds {
    #[inline(always)]
    fn from(input: &LayoutInput) -> Self {
        Self { min_size: size_bound_cache_key(input.min_size), max_size: size_bound_cache_key(input.max_size) }
    }
}

/// Pack `AvailableSpace` into `u32`
#[inline(always)]
fn available_space_cache_key(input: AvailableSpace) -> u32 {
    match input {
        AvailableSpace::Definite(value) => (-value).to_bits(),
        AvailableSpace::MinContent => NEG_INFINITY_BITS,
        AvailableSpace::MaxContent => INFINITY_BITS,
    }
}

/// Pack `Size<AvailableSpace>` into `u64`
#[inline(always)]
#[allow(dead_code)]
fn size_available_space_cache_key(input: Size<AvailableSpace>) -> u64 {
    (available_space_cache_key(input.width) as u64) << 32 | available_space_cache_key(input.height) as u64
}

/// Encodes combination of a `known_dimension` (Option<f32>) and `AvailableSpace` in
/// a single dimension into a cache key in a single dimension.
#[inline(always)]
fn mixed_cache_key(kd: Option<f32>, avs: AvailableSpace) -> u32 {
    kd.map(|kd| kd.to_bits()).unwrap_or_else(|| available_space_cache_key(avs))
}

/// Encodes combination of a `known_dimension` (Option<f32>) and `AvailableSpace` in
/// two dimensions into a cache key in a single dimension.
#[inline(always)]
fn size_mixed_cache_key(kd: Size<Option<f32>>, avs: Size<AvailableSpace>) -> u64 {
    (mixed_cache_key(kd.width, avs.width) as u64) << 32 | mixed_cache_key(kd.height, avs.height) as u64
}

/// Space-optimised cache key that packs bits into as small a size as possible
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
struct CacheKey {
    /// The initial cached size of the node itself
    kd_available_space: u64,
    /// The initial cached size of the parent's node
    parent_size: u64,
    /// Whether the node was passed a min or max size in either axis. If it was, then the sizes that it
    /// was passed are the [`Cache::bounds`] of the cache that the entry is stored in.
    is_bounded: bool,
    /// Whether each known dimension is definite. Normalized such that an axis
    /// without a known dimension is always `true`.
    known_dimensions_are_definite: Size<bool>,
}

impl CacheKey {
    #[inline(always)]
    #[allow(dead_code)]
    /// Return the parent size with the extra bits that encode the requested axis masked out
    fn parent_size(&self) -> u64 {
        self.parent_size & NON_SIGN_BITS_MASK
    }

    /// Return the parent size with the extra bits that encode the requested axis masked out
    /// And the y-axis value masked out
    fn x_axis_parent_size(&self) -> u64 {
        self.parent_size & (X_AXIS_VALUE_MASK & NON_SIGN_BITS_MASK)
    }

    /// Return the bits that encode the requested axis
    fn requested_axis_bits(&self) -> u64 {
        self.parent_size & BOTH_SIGN_BITS_MASK
    }

    /// Whether a cached entry with this key contains a valid size for the axis requested by `other`.
    /// Sizes computed for a single axis may contain garbage values in the other axis, so an entry
    /// is only usable if it was computed for the same axis (or for both axes).
    fn size_is_valid_for(&self, other: &CacheKey) -> bool {
        let entry_axis = self.requested_axis_bits();
        entry_axis == BOTH_SIGN_BITS_MASK || entry_axis == other.requested_axis_bits()
    }
}

impl CacheKey {
    /// Create the key for `input`, whose min and max sizes are `bounds`
    #[inline(always)]
    fn new(input: &LayoutInput, bounds: &CacheBounds) -> Self {
        // Pack axis enum into spare bits in the known_dimensions and available_space values
        let extra_bits = match input.axis {
            RequestedAxis::Horizontal => SIGN_BIT_1,
            RequestedAxis::Vertical => SIGN_BIT_2,
            RequestedAxis::Both => SIGN_BIT_1 | SIGN_BIT_2,
        };

        Self {
            kd_available_space: size_mixed_cache_key(input.known_dimensions, input.available_space),
            parent_size: (size_option_cache_key(input.parent_size) & NON_SIGN_BITS_MASK) | extra_bits,
            is_bounded: bounds.is_bounded(),
            known_dimensions_are_definite: input
                .known_dimensions_are_definite
                .zip_map(input.known_dimensions, |is_definite, kd| is_definite || kd.is_none()),
        }
    }
}

/// The layout inputs that affect a node's layout but are not part of the [`CacheKey`]:
/// whether each of the node's vertical margins is collapsible, packed into a `u8`.
///
/// These are determined by the layout algorithm of the node's parent and by the node's own style,
/// so they only change for a given node when one of those changes. Rather than keying every entry
/// on them, a [`Cache`] records the single mode that all of its entries were computed with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
struct CacheMode(u8);

impl CacheMode {
    /// Set if the node's top margin is collapsible
    const COLLAPSIBLE_START_MARGIN_BIT: u8 = 0b01;
    /// Set if the node's bottom margin is collapsible
    const COLLAPSIBLE_END_MARGIN_BIT: u8 = 0b10;

    /// The mode of a cache that has never been stored to. Which mode this is does not matter,
    /// as such a cache has no entries.
    const INITIAL: Self = Self(0);
}

impl From<&LayoutInput> for CacheMode {
    #[inline(always)]
    fn from(input: &LayoutInput) -> Self {
        let collapsible = input.vertical_margins_are_collapsible;
        Self(
            (collapsible.start as u8 * Self::COLLAPSIBLE_START_MARGIN_BIT)
                | (collapsible.end as u8 * Self::COLLAPSIBLE_END_MARGIN_BIT),
        )
    }
}

#[cfg(all(debug_assertions, feature = "std"))]
std::thread_local! {
    /// See [`cache_mode_change_evictions`]
    static MODE_CHANGE_EVICTIONS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

/// The number of times that the current thread has stored a result into a non-empty [`Cache`]
/// whose entries were computed with a different `vertical_margins_are_collapsible`,
/// dropping those entries.
///
/// This is expected when the layout algorithm of a node's parent has changed since the node was
/// last laid out. It must not happen while laying out a tree whose caches started out empty: that
/// would mean that Taffy lays a node out with more than one mode in a single pass, with each
/// change of mode throwing away the node's cache. Taffy's test suite uses this function to check
/// that. Only available in debug builds.
#[doc(hidden)]
#[cfg(all(debug_assertions, feature = "std"))]
pub fn cache_mode_change_evictions() -> usize {
    MODE_CHANGE_EVICTIONS.with(|count| count.get())
}

#[cfg(all(debug_assertions, feature = "std"))]
std::thread_local! {
    /// See [`cache_bounds_change_evictions`]
    static BOUNDS_CHANGE_EVICTIONS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

/// The number of times that the current thread has stored a result into a [`Cache`] which held
/// entries that were computed with a different (non-empty) set of min and max sizes, dropping those entries.
///
/// Unlike [`cache_mode_change_evictions`] this can legitimately happen within a single layout pass:
/// a node whose min or max size is a percentage is passed a different size whenever the size that
/// the percentage resolves against changes. It is exposed so that how often it happens can be measured.
/// Only available in debug builds.
#[doc(hidden)]
#[cfg(all(debug_assertions, feature = "std"))]
pub fn cache_bounds_change_evictions() -> usize {
    BOUNDS_CHANGE_EVICTIONS.with(|count| count.get())
}

/// Cached intermediate layout results
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub(crate) struct CacheEntry<T> {
    /// The key for the cache entry
    key: CacheKey,
    /// The cached size and baselines of the item
    content: T,
}

/// A cache for caching the results of a sizing a Grid Item or Flexbox Item
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Cache {
    /// The cache entry for the node's final layout
    final_layout_entry: Option<CacheEntry<LayoutOutput>>,
    /// The cache entries for the node's preliminary size measurements
    measure_entries: [Option<CacheEntry<Size<f32>>>; CACHE_SIZE],
    /// Tracks which measure entries have been used since the eviction cursor last passed them
    recently_used_entries: u16,
    /// The next measure entry to consider replacing
    next_measure_entry: u8,
    /// Tracks if all cache entries are empty
    is_empty: bool,
    /// The `vertical_margins_are_collapsible` input that all of the cache's
    /// entries were computed with. Results are only retrieved for inputs that have the same mode,
    /// and storing a result that was computed with a different mode drops the existing entries.
    mode: CacheMode,
    /// The min and max sizes that all of the cache's bounded entries (see [`CacheKey::is_bounded`])
    /// were computed with. Results for bounded inputs are only retrieved if the input has the same
    /// min and max sizes, and storing a result that was computed with different ones drops the
    /// existing bounded entries (but not the unbounded ones).
    bounds: CacheBounds,
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    /// Create a new empty cache
    pub const fn new() -> Self {
        /// Workaround for `Option<CacheEntry<_>>` not being `Copy` (required for array repeat expressions)
        const NONE_MEASURE_ENTRY: Option<CacheEntry<Size<f32>>> = None;
        Self {
            final_layout_entry: None,
            measure_entries: [NONE_MEASURE_ENTRY; CACHE_SIZE],
            recently_used_entries: 0,
            next_measure_entry: 0,
            is_empty: true,
            mode: CacheMode::INITIAL,
            bounds: CacheBounds::NONE,
        }
    }

    /// Try to retrieve a cached result from the cache
    #[inline]
    pub fn get(&mut self, input: &LayoutInput) -> Option<LayoutOutput> {
        if self.mode != CacheMode::from(input) {
            return None;
        }
        let bounds = CacheBounds::from(input);
        let key = CacheKey::new(input, &bounds);
        if key.is_bounded && bounds != self.bounds {
            return None;
        }
        match input.run_mode {
            RunMode::PerformLayout => {
                self.final_layout_entry.as_ref().filter(|entry| entry.key == key).map(|e| e.content.clone())
            }
            RunMode::ComputeSize => {
                for (index, entry) in self.measure_entries.iter().enumerate() {
                    let Some(entry) = entry else { continue };
                    if entry.key.kd_available_space == key.kd_available_space
                        && entry.key.is_bounded == key.is_bounded
                        && entry.key.known_dimensions_are_definite == key.known_dimensions_are_definite
                        && (entry.key.x_axis_parent_size() == key.x_axis_parent_size())
                        && entry.key.size_is_valid_for(&key)
                    {
                        self.recently_used_entries |= 1 << index;
                        return Some(LayoutOutput::from_outer_size(entry.content));
                    }
                }

                None
            }
            RunMode::PerformHiddenLayout => None,
        }
    }

    /// Store a computed size in the cache
    pub fn store(&mut self, input: &LayoutInput, layout_output: LayoutOutput) {
        if input.run_mode == RunMode::PerformHiddenLayout {
            return;
        }
        let mode = CacheMode::from(input);
        if self.mode != mode {
            #[cfg(all(debug_assertions, feature = "std"))]
            if !self.is_empty {
                MODE_CHANGE_EVICTIONS.with(|count| count.set(count.get() + 1));
            }
            self.clear();
            self.mode = mode;
        }
        let bounds = CacheBounds::from(input);
        let key = CacheKey::new(input, &bounds);
        if key.is_bounded && bounds != self.bounds {
            self.clear_bounded_entries();
            self.bounds = bounds;
        }
        match input.run_mode {
            RunMode::PerformLayout => {
                self.is_empty = false;
                self.final_layout_entry = Some(CacheEntry { key, content: layout_output })
            }
            RunMode::ComputeSize => {
                // Measure entries only store the size, and cache hits are reconstructed with
                // `LayoutOutput::from_outer_size`, which resets the margin-collapse metadata
                // (`top_margin`, `bottom_margin`, `margins_can_collapse_through`). Results that
                // carry such metadata cannot be reconstructed from their size, so don't cache them.
                if layout_output.margins_can_collapse_through
                    || layout_output.top_margin != CollapsibleMarginSet::ZERO
                    || layout_output.bottom_margin != CollapsibleMarginSet::ZERO
                {
                    return;
                }
                self.is_empty = false;
                if let Some(index) =
                    self.measure_entries.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.key == key))
                {
                    self.measure_entries[index].as_mut().unwrap().content = layout_output.size;
                    self.recently_used_entries |= 1 << index;
                    return;
                }
                while self.recently_used_entries & (1 << self.next_measure_entry) != 0 {
                    self.recently_used_entries &= !(1 << self.next_measure_entry);
                    self.next_measure_entry += 1;
                    if self.next_measure_entry == CACHE_SIZE as u8 {
                        self.next_measure_entry = 0;
                    }
                }
                let entry_index = self.next_measure_entry as usize;
                self.measure_entries[entry_index] = Some(CacheEntry { key, content: layout_output.size });
                self.recently_used_entries |= 1 << entry_index;
                self.next_measure_entry += 1;
                if self.next_measure_entry == CACHE_SIZE as u8 {
                    self.next_measure_entry = 0;
                }
            }
            RunMode::PerformHiddenLayout => {}
        }
    }

    /// Drop the entries that were computed with a min or max size
    #[cold]
    fn clear_bounded_entries(&mut self) {
        let mut evicted = false;
        if self.final_layout_entry.as_ref().is_some_and(|entry| entry.key.is_bounded) {
            self.final_layout_entry = None;
            evicted = true;
        }
        for (index, entry) in self.measure_entries.iter_mut().enumerate() {
            if entry.as_ref().is_some_and(|entry| entry.key.is_bounded) {
                *entry = None;
                self.recently_used_entries &= !(1 << index);
                evicted = true;
            }
        }
        #[cfg(all(debug_assertions, feature = "std"))]
        if evicted {
            BOUNDS_CHANGE_EVICTIONS.with(|count| count.set(count.get() + 1));
        }
        #[cfg(not(all(debug_assertions, feature = "std")))]
        let _ = evicted;
    }

    /// Clear all cache entries and reports clear operation outcome ([`ClearState`])
    pub fn clear(&mut self) -> ClearState {
        if self.is_empty {
            return ClearState::AlreadyEmpty;
        }
        self.is_empty = true;
        self.final_layout_entry = None;
        /// Workaround for `Option<CacheEntry<_>>` not being `Copy` (required for array repeat expressions)
        const NONE_MEASURE_ENTRY: Option<CacheEntry<Size<f32>>> = None;
        self.measure_entries = [NONE_MEASURE_ENTRY; CACHE_SIZE];
        self.recently_used_entries = 0;
        self.next_measure_entry = 0;
        ClearState::Cleared
    }

    /// Returns true if all cache entries are None, else false
    pub fn is_empty(&self) -> bool {
        self.final_layout_entry.is_none() && !self.measure_entries.iter().any(|entry| entry.is_some())
    }
}

/// Clear operation outcome. See [`Cache::clear`]
pub enum ClearState {
    /// Cleared some values
    Cleared,
    /// Everything was already cleared
    AlreadyEmpty,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Line;

    fn input(width: f32) -> LayoutInput {
        LayoutInput {
            run_mode: RunMode::ComputeSize,
            axis: RequestedAxis::Both,
            known_dimensions: Size { width: Some(width), height: None },
            known_dimensions_are_definite: Size { width: true, height: true },
            min_size: Size::NONE,
            max_size: Size::NONE,
            parent_size: Size::NONE,
            available_space: Size { width: AvailableSpace::MaxContent, height: AvailableSpace::MaxContent },
            vertical_margins_are_collapsible: Line::FALSE,
        }
    }

    fn output(width: f32) -> LayoutOutput {
        LayoutOutput::from_outer_size(Size { width, height: width })
    }

    #[test]
    fn results_are_not_shared_between_inputs_that_differ_in_min_or_max_size() {
        let unbounded = input(1.0);
        let variants = [
            LayoutInput { min_size: Size { width: None, height: Some(10.0) }, ..unbounded },
            LayoutInput { min_size: Size { width: Some(10.0), height: None }, ..unbounded },
            LayoutInput { max_size: Size { width: None, height: Some(10.0) }, ..unbounded },
            LayoutInput { max_size: Size { width: Some(10.0), height: None }, ..unbounded },
            LayoutInput { max_size: Size { width: None, height: Some(20.0) }, ..unbounded },
            // An infinite bound is not the same input as an absent one
            LayoutInput { min_size: Size { width: None, height: Some(f32::INFINITY) }, ..unbounded },
            LayoutInput { max_size: Size { width: None, height: Some(f32::INFINITY) }, ..unbounded },
        ];

        for run_mode in [RunMode::ComputeSize, RunMode::PerformLayout] {
            let with_run_mode = |input: &LayoutInput| LayoutInput { run_mode, ..*input };
            for (index, variant) in variants.iter().enumerate() {
                let mut cache = Cache::new();
                cache.store(&with_run_mode(&unbounded), output(1.0));
                assert_eq!(cache.get(&with_run_mode(variant)), None);

                let mut cache = Cache::new();
                cache.store(&with_run_mode(variant), output(2.0));
                assert_eq!(cache.get(&with_run_mode(&unbounded)), None);
                assert_eq!(cache.get(&with_run_mode(variant)), Some(output(2.0)));
                for (other_index, other) in variants.iter().enumerate() {
                    if other_index != index {
                        assert_eq!(cache.get(&with_run_mode(other)), None);
                    }
                }
            }
        }
    }

    #[test]
    fn measurements_with_different_min_or_max_sizes_are_cached_side_by_side() {
        let bounded = LayoutInput { max_size: Size { width: None, height: Some(10.0) }, ..input(1.0) };
        let mut cache = Cache::new();
        cache.store(&input(1.0), output(1.0));
        cache.store(&bounded, output(2.0));

        assert_eq!(cache.get(&input(1.0)), Some(output(1.0)));
        assert_eq!(cache.get(&bounded), Some(output(2.0)));
    }

    #[test]
    fn storing_a_result_with_different_min_or_max_sizes_only_drops_the_bounded_entries() {
        let bounded = |max_height: f32, width: f32| LayoutInput {
            max_size: Size { width: None, height: Some(max_height) },
            ..input(width)
        };
        let mut cache = Cache::new();
        cache.store(&input(1.0), output(1.0));
        cache.store(&bounded(10.0, 1.0), output(2.0));
        cache.store(&bounded(10.0, 2.0), output(3.0));
        cache.store(&LayoutInput { run_mode: RunMode::PerformLayout, ..bounded(10.0, 1.0) }, output(4.0));

        // Nothing is dropped by a lookup
        assert_eq!(cache.get(&bounded(20.0, 1.0)), None);
        assert_eq!(cache.get(&bounded(10.0, 1.0)), Some(output(2.0)));

        cache.store(&bounded(20.0, 1.0), output(5.0));
        assert_eq!(cache.get(&bounded(20.0, 1.0)), Some(output(5.0)));
        assert_eq!(cache.get(&bounded(10.0, 1.0)), None);
        assert_eq!(cache.get(&bounded(10.0, 2.0)), None);
        assert_eq!(cache.get(&LayoutInput { run_mode: RunMode::PerformLayout, ..bounded(10.0, 1.0) }), None);
        assert_eq!(cache.get(&input(1.0)), Some(output(1.0)));
        assert!(!cache.is_empty());
    }

    #[test]
    fn recently_used_measure_entries_get_a_second_chance() {
        let mut cache = Cache::new();
        for width in 0..CACHE_SIZE {
            cache.store(&input(width as f32), output(width as f32));
        }
        cache.store(&input(CACHE_SIZE as f32), output(CACHE_SIZE as f32));

        assert_eq!(cache.get(&input(1.0)), Some(output(1.0)));
        cache.store(&input((CACHE_SIZE + 1) as f32), output((CACHE_SIZE + 1) as f32));

        assert_eq!(cache.get(&input(1.0)), Some(output(1.0)));
        assert_eq!(cache.get(&input(2.0)), None);
    }

    #[test]
    fn storing_an_existing_measurement_updates_it_in_place() {
        let mut cache = Cache::new();
        cache.store(&input(1.0), output(1.0));
        cache.store(&input(2.0), output(2.0));
        cache.store(&input(1.0), output(3.0));

        assert_eq!(cache.measure_entries.iter().flatten().count(), 2);
        assert_eq!(cache.get(&input(1.0)), Some(output(3.0)));
    }

    #[test]
    fn measurements_with_margin_collapse_metadata_are_not_cached() {
        let mut cache = Cache::new();

        let mut collapse_through = output(1.0);
        collapse_through.margins_can_collapse_through = true;
        cache.store(&input(1.0), collapse_through);
        assert_eq!(cache.get(&input(1.0)), None);

        let mut carried_margin = output(2.0);
        carried_margin.top_margin = CollapsibleMarginSet::from_margin(10.0);
        cache.store(&input(2.0), carried_margin);
        assert_eq!(cache.get(&input(2.0)), None);
    }

    #[test]
    fn retrieving_a_measurement_only_marks_its_slot_as_used() {
        let mut cache = Cache::new();
        cache.store(&input(1.0), output(1.0));
        cache.store(&input(2.0), output(2.0));
        cache.recently_used_entries = 0;
        let entries = cache.measure_entries.clone();

        assert_eq!(cache.get(&input(1.0)), Some(output(1.0)));
        assert_eq!(cache.measure_entries, entries);
        assert_ne!(cache.recently_used_entries, 0);
    }

    fn perform_layout_input(width: f32, height: f32) -> LayoutInput {
        LayoutInput {
            run_mode: RunMode::PerformLayout,
            known_dimensions: Size { width: Some(width), height: Some(height) },
            parent_size: Size { width: Some(width), height: Some(height) },
            ..input(width)
        }
    }

    /// Inputs that differ from `input`/`perform_layout_input` only in their mode
    fn other_modes(input: LayoutInput) -> [LayoutInput; 3] {
        [
            LayoutInput { vertical_margins_are_collapsible: Line::TRUE, ..input },
            LayoutInput { vertical_margins_are_collapsible: Line { start: true, end: false }, ..input },
            LayoutInput { vertical_margins_are_collapsible: Line { start: false, end: true }, ..input },
        ]
    }

    #[test]
    fn cache_mode_distinguishes_every_combination_of_inputs() {
        let mut modes = [CacheMode::INITIAL; 4];
        for (index, mode) in modes.iter_mut().enumerate() {
            *mode = CacheMode::from(&LayoutInput {
                vertical_margins_are_collapsible: Line { start: index & 1 != 0, end: index & 2 != 0 },
                ..input(1.0)
            });
        }
        for (index, mode) in modes.iter().enumerate() {
            assert!(!modes[..index].contains(mode));
        }
    }

    #[test]
    fn retrieving_with_a_different_mode_misses_and_keeps_the_entries() {
        let mut cache = Cache::new();
        cache.store(&perform_layout_input(100.0, 50.0), output(1.0));
        cache.store(&input(1.0), output(2.0));

        for other in other_modes(perform_layout_input(100.0, 50.0)) {
            assert_eq!(cache.get(&other), None);
        }
        for other in other_modes(input(1.0)) {
            assert_eq!(cache.get(&other), None);
        }

        assert_eq!(cache.get(&perform_layout_input(100.0, 50.0)), Some(output(1.0)));
        assert_eq!(cache.get(&input(1.0)), Some(output(2.0)));
    }

    #[test]
    fn storing_with_a_different_mode_drops_the_existing_entries() {
        for other in other_modes(input(3.0)) {
            let mut cache = Cache::new();
            cache.store(&perform_layout_input(100.0, 50.0), output(1.0));
            cache.store(&input(1.0), output(2.0));

            cache.store(&other, output(3.0));
            assert_eq!(cache.get(&other), Some(output(3.0)));
            assert!(cache.final_layout_entry.is_none());
            assert_eq!(cache.measure_entries.iter().flatten().count(), 1);

            // The old entries are gone, even for inputs that have the mode that the cache had before
            cache.store(&input(4.0), output(4.0));
            assert_eq!(cache.get(&perform_layout_input(100.0, 50.0)), None);
            assert_eq!(cache.get(&input(1.0)), None);
            assert_eq!(cache.get(&other), None);
            assert_eq!(cache.get(&input(4.0)), Some(output(4.0)));
        }

        for other in other_modes(perform_layout_input(100.0, 50.0)) {
            let mut cache = Cache::new();
            cache.store(&perform_layout_input(100.0, 50.0), output(1.0));
            cache.store(&input(1.0), output(2.0));

            cache.store(&other, output(3.0));
            assert_eq!(cache.get(&other), Some(output(3.0)));
            assert!(cache.measure_entries.iter().all(|entry| entry.is_none()));
            assert_eq!(cache.get(&perform_layout_input(100.0, 50.0)), None);
        }
    }

    #[test]
    fn hidden_layouts_do_not_affect_the_cache() {
        let mut cache = Cache::new();
        cache.store(&input(1.0), output(1.0));
        cache.store(&LayoutInput::HIDDEN, output(2.0));
        assert_eq!(cache.get(&input(1.0)), Some(output(1.0)));
    }

    #[test]
    #[cfg(all(debug_assertions, feature = "std"))]
    fn mode_change_evictions_are_counted_for_non_empty_caches_only() {
        let collapsible = LayoutInput { vertical_margins_are_collapsible: Line::TRUE, ..input(1.0) };
        let evictions = cache_mode_change_evictions();

        let mut cache = Cache::new();
        cache.store(&collapsible, output(1.0));
        cache.store(&LayoutInput { vertical_margins_are_collapsible: Line::TRUE, ..input(2.0) }, output(2.0));
        assert_eq!(cache_mode_change_evictions(), evictions);

        cache.store(&input(1.0), output(1.0));
        assert_eq!(cache_mode_change_evictions(), evictions + 1);

        cache.clear();
        cache.store(&collapsible, output(1.0));
        assert_eq!(cache_mode_change_evictions(), evictions + 1);
    }
}
