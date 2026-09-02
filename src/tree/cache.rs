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

impl From<&LayoutInput> for CacheKey {
    fn from(input: &LayoutInput) -> Self {
        // Pack axis enum into spare bits in the known_dimensions and available_space values
        let extra_bits = match input.axis {
            RequestedAxis::Horizontal => SIGN_BIT_1,
            RequestedAxis::Vertical => SIGN_BIT_2,
            RequestedAxis::Both => SIGN_BIT_1 | SIGN_BIT_2,
        };

        Self {
            kd_available_space: size_mixed_cache_key(input.known_dimensions, input.available_space),
            parent_size: (size_option_cache_key(input.parent_size) & NON_SIGN_BITS_MASK) | extra_bits,
            known_dimensions_are_definite: input
                .known_dimensions_are_definite
                .zip_map(input.known_dimensions, |is_definite, kd| is_definite || kd.is_none()),
        }
    }
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
        }
    }

    /// Try to retrieve a cached result from the cache
    #[inline]
    pub fn get(&mut self, input: &LayoutInput) -> Option<LayoutOutput> {
        let key = CacheKey::from(input);
        match input.run_mode {
            RunMode::PerformLayout => {
                self.final_layout_entry.as_ref().filter(|entry| entry.key == key).map(|e| e.content.clone())
            }
            RunMode::ComputeSize => {
                for (index, entry) in self.measure_entries.iter().enumerate() {
                    let Some(entry) = entry else { continue };
                    if entry.key.kd_available_space == key.kd_available_space
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
        let key = CacheKey::from(input);
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
    use crate::tree::SizingMode;

    fn input(width: f32) -> LayoutInput {
        LayoutInput {
            run_mode: RunMode::ComputeSize,
            sizing_mode: SizingMode::InherentSize,
            axis: RequestedAxis::Both,
            known_dimensions: Size { width: Some(width), height: None },
            known_dimensions_are_definite: Size { width: true, height: true },
            parent_size: Size::NONE,
            available_space: Size { width: AvailableSpace::MaxContent, height: AvailableSpace::MaxContent },
            vertical_margins_are_collapsible: Line::FALSE,
        }
    }

    fn output(width: f32) -> LayoutOutput {
        LayoutOutput::from_outer_size(Size { width, height: width })
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
}
