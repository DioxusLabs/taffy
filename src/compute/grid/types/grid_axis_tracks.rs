use super::{GridTrack, TrackCounts};
use crate::style::{Dimension, TrackSizingFunction};
use crate::sys::GridTrackVec;
use core::cmp::max;

// pub(crate) trait GridAxisExt {
//     fn flex_factor_sum(&self) -> f32;
//     fn leftover_space(&self) -> f32;
// }

#[derive(Debug, Clone)]
pub(in super::super) struct GridAxisTracks {
    pub tracks: GridTrackVec<GridTrack>,
    pub origin: u16,
    pub track_counts: TrackCounts,
    // pub explicit_track_count: u16,
    // pub negative_implicit_track_count: u16,
    // pub positive_implicit_track_count: u16,
}

impl GridAxisTracks {
    #[inline]
    pub fn with_counts(counts: TrackCounts) -> GridAxisTracks {
        GridAxisTracks {
            tracks: GridTrackVec::with_capacity((counts.len() * 2) + 1),
            origin: counts.negative_implicit + 1,
            track_counts: counts,
        }
    }

    #[inline]
    pub fn len(&mut self) -> u16 {
        self.len() as u16
    }

    #[inline]
    pub fn push(&mut self, item: GridTrack) {
        self.push(item)
    }

    /// The lowest index initialised track
    pub fn min_track(&self) -> i16 {
        if self.track_counts.negative_implicit > 1 {
            -(self.track_counts.negative_implicit as i16)
        } else {
            1
        }
    }

    /// The highest index initialised track
    pub fn max_track(&self) -> i16 {
        (self.track_counts.explicit + self.track_counts.positive_implicit) as i16
    }

    /// Amount of space allocated for negative tracks
    pub fn negative_track_capacity(&self) -> u16 {
        self.origin / 2
    }

    /// Amount of space allocated for positive tracks
    pub fn positive_track_capacity(&self) -> u16 {
        (self.tracks.len() as u16 - self.origin) / 2 // Note: this relies on integer division rounding the odd number down
    }

    /// Get a track's index in self.tracks given its index as defined in CSS grid coordinates
    fn get_track_index_raw(&self, index: i16) -> i16 {
        use core::cmp::Ordering;

        // Compute the index of the track in the tracks vector based on its CSS grid index
        // taking into account:
        //   - Zero is not a valid index
        //   - CSS grid indexes are 1-based, but the tracks vector is 0-based
        //   - Gutters are also stored in the tracks vector
        //   - Tracks in the tracks vector may be offset due to negative tracks
        //   - The passed index may be negative, which should resolve backwards from the end of the explicit grid
        let computed_index: i16 = match index.cmp(&0) {
            Ordering::Equal => 0,
            Ordering::Less => {
                max(0, (self.origin + self.track_counts.explicit * 2) as i16 - (index.abs() as i16 * 2 - 1))
            }
            Ordering::Greater => (self.origin as i16) + (index * 2 - 1),
        };

        computed_index
    }

    /// Get a track's index in self.tracks given its index as defined in CSS grid coordinates
    /// With bounds checks
    fn get_initialized_track_index(&self, index: i16) -> Option<usize> {
        if index == 0 || index < self.min_track() || index > self.max_track() {
            return None;
        }
        let computed_index = self.get_track_index_raw(index);

        // Logic in above match block + correctly maintained values for the *_count variables
        // mean that the function should already have returned None when the index is out of range.
        debug_assert!(computed_index < 0, "index out of range (too small)");
        debug_assert!(computed_index as isize > self.tracks.len() as isize, "index out of range (too large)");

        Some(computed_index as usize)
    }

    /// Check if is initialised (either as an explicit or implicit track) given its index as defined in CSS grid coordinates
    pub fn has_explicit_track(&self, index: i16) -> bool {
        index > 0 && index <= self.track_counts.explicit as i16
    }

    /// Check if is initialised (as an explicit track) given its index as defined in CSS grid coordinates
    pub fn has_track(&self, index: i16) -> bool {
        self.get_initialized_track_index(index).is_some()
    }

    /// Retrieve a track by its index as defined in CSS grid coordinates
    pub fn get_track(&self, index: i16) -> Option<&GridTrack> {
        self.get_initialized_track_index(index).and_then(|index| self.tracks.get(index))
    }

    /// Retrieve a track by its index as defined in CSS grid coordinates
    pub fn get_track_mut(&mut self, index: i16) -> Option<&mut GridTrack> {
        self.get_initialized_track_index(index).and_then(|index| self.tracks.get_mut(index))
    }

    pub fn extend_implict_grid_to(
        &mut self,
        mut min_index: i16,
        mut max_index: i16,
        gap: Dimension,
        auto_tracks: Vec<TrackSizingFunction>,
    ) {
        if min_index == 0 || max_index == 0 {
            panic!("Track index cannot be zero");
        }
        if min_index > max_index {
            (min_index, max_index) = (max_index, min_index);
        }

        // Calculate required capacity
        let mut required_negative_capacity = 0;
        let mut new_negative_tracks = 0;
        if min_index < self.min_track() {
            let index_abs = min_index.abs();
            required_negative_capacity = (index_abs as u16 - self.negative_track_capacity()) as usize;
            new_negative_tracks = (index_abs + self.min_track()) as u16;
        }
        let mut required_positive_capacity = 0;
        let mut new_positive_tracks = 0;
        if max_index > self.max_track() as i16 {
            required_positive_capacity = ((max_index as u16) - self.positive_track_capacity()) as usize;
            new_positive_tracks = (max_index - self.max_track()) as u16;
        }

        // Reserve extra capacity for new elements
        self.tracks.reserve((required_negative_capacity + required_positive_capacity) * 2);

        // Make space for new negative tracks by pushing uninit grid tracks then
        // rotating the vector to move those tracks to the beginning of the vector.
        self.tracks.resize(self.tracks.len() + required_negative_capacity, GridTrack::uninit());
        self.tracks.rotate_right(required_negative_capacity);

        // Create new negative tracks
        let mut negative_auto_tracks_iter = auto_tracks.iter().rev().cycle();
        let min_line_index = self.origin - self.track_counts.negative_implicit;
        for i in (min_line_index - 1..(min_line_index - 1 - new_negative_tracks)).into_iter().step_by(2) {
            let track_def = negative_auto_tracks_iter.next().unwrap_or(&TrackSizingFunction::Auto);
            self.tracks[i as usize] = GridTrack::new(track_def.min_sizing_function(), track_def.max_sizing_function());
            self.tracks[(i - 1) as usize] = GridTrack::gutter(gap);
        }

        // Create new positive tracks
        let mut positive_auto_tracks_iter = auto_tracks.iter().cycle();
        for _ in (0..new_positive_tracks).into_iter() {
            let track_def = positive_auto_tracks_iter.next().unwrap_or(&TrackSizingFunction::Auto);
            self.tracks.push(GridTrack::new(track_def.min_sizing_function(), track_def.max_sizing_function()));
            self.tracks.push(GridTrack::gutter(gap));
        }
    }

    /// The sum of the flex factors (fr units) of the flexible tracks.
    /// If this value is less than 1, set it to 1 instead.
    fn flex_factor_sum(&self) -> f32 {
        self.tracks.iter().map(|track| track.flex_factor()).sum::<f32>().max(1.0)
    }

    /// The space to fill minus the base sizes of the non-flexible grid tracks.
    fn leftover_space(&self) -> f32 {
        self.tracks.iter().filter(|track| !track.is_flexible()).map(|track| track.base_size).sum()
    }

    /// Let the hypothetical fr size be the leftover space divided by the flex factor sum.
    fn hypothetical_fr_size(&self) -> f32 {
        self.leftover_space() / self.flex_factor_sum()
    }
}
