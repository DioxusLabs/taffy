use super::resolve_and_place::CellOccupancyMatrix;
use crate::geometry::{Line, Size};
use crate::layout::AvailableSpace;
use crate::node::Node;
use crate::style::{Dimension, MaxTrackSizingFunction, MinTrackSizingFunction, TrackSizingFunction};
use crate::sys::GridTrackVec;
use core::cmp::max;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AbsoluteAxis {
    Horizontal,
    Vertical,
}

impl AbsoluteAxis {
    #[inline]
    pub fn other_axis(&self) -> Self {
        match *self {
            AbsoluteAxis::Horizontal => AbsoluteAxis::Vertical,
            AbsoluteAxis::Vertical => AbsoluteAxis::Horizontal,
        }
    }

    #[inline]
    pub fn into_column_row<T>(&self, primary: T, secondary: T) -> (T, T) {
        match *self {
            AbsoluteAxis::Horizontal => (primary, secondary),
            AbsoluteAxis::Vertical => (secondary, primary),
        }
    }

    #[inline]
    pub fn into_primary_secondary<T>(&self, row: T, column: T) -> (T, T) {
        match *self {
            AbsoluteAxis::Horizontal => (row, column),
            AbsoluteAxis::Vertical => (column, row),
        }
    }
}

/// The abstract axis in CSS Grid
#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) enum GridAxis {
    /// The axis in the inline dimension, i.e. the horizontal axis in horizontal writing modes and the vertical axis in vertical writing modes.
    Inline,
    /// The axis in the block dimension, i.e. the vertical axis in horizontal writing modes and the horizontal axis in vertical writing modes.
    Block,
}

impl GridAxis {
    pub fn other(&self) -> GridAxis {
        match *self {
            GridAxis::Inline => GridAxis::Block,
            GridAxis::Block => GridAxis::Inline,
        }
    }
}

/// Whether a GridTrack represents an actual track or a gutter.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) enum GridTrackKind {
    Uninit,
    Track,
    Gutter { name: Option<u16> },
}

/// Internal sizing information for a single grid track (row/column)
/// Gutters between tracks are sized similarly to actual tracks, so they
/// are also represented by this struct
#[derive(Debug, Clone)]
pub(super) struct GridTrack {
    pub kind: GridTrackKind,
    pub min_track_sizing_function: MinTrackSizingFunction,
    pub max_track_sizing_function: MaxTrackSizingFunction,
    pub base_size: f32,
    pub growth_limit: f32,         // Note: can be infinity
    pub infinitely_growable: bool, // https://www.w3.org/TR/css3-grid-layout/#infinitely-growable
}

impl GridTrack {
    pub fn new(
        min_track_sizing_function: MinTrackSizingFunction,
        max_track_sizing_function: MaxTrackSizingFunction,
    ) -> GridTrack {
        GridTrack {
            kind: GridTrackKind::Track,
            min_track_sizing_function,
            max_track_sizing_function,
            base_size: 0.0,
            growth_limit: 0.0,
            infinitely_growable: false,
        }
    }

    pub fn gutter(size: Dimension) -> GridTrack {
        GridTrack {
            kind: GridTrackKind::Gutter { name: None },
            min_track_sizing_function: MinTrackSizingFunction::Fixed(size),
            max_track_sizing_function: MaxTrackSizingFunction::Fixed(size),
            base_size: 0.0,
            growth_limit: 0.0,
            infinitely_growable: false,
        }
    }

    pub fn uninit() -> GridTrack {
        GridTrack {
            kind: GridTrackKind::Uninit,
            min_track_sizing_function: MinTrackSizingFunction::Auto,
            max_track_sizing_function: MaxTrackSizingFunction::Auto,
            base_size: 0.0,
            growth_limit: 0.0,
            infinitely_growable: false,
        }
    }

    #[inline]
    pub fn is_flexible(&self) -> bool {
        match self.max_track_sizing_function {
            MaxTrackSizingFunction::Flex(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn flex_factor(&self) -> f32 {
        match self.max_track_sizing_function {
            MaxTrackSizingFunction::Flex(flex_factor) => flex_factor,
            _ => 0.0,
        }
    }
}

pub(super) trait GridAxisExt {
    fn flex_factor_sum(&self) -> f32;
    fn leftover_space(&self) -> f32;
}

#[derive(Debug, Clone)]
pub(super) struct GridAxisTracks {
    pub tracks: GridTrackVec<GridTrack>,
    pub origin: u16,
    pub explicit_track_count: u16,
    pub negative_implicit_track_count: u16,
    pub positive_implicit_track_count: u16,
}

impl GridAxisTracks {
    pub fn new() -> GridAxisTracks {
        Self::with_capacity(0)
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> GridAxisTracks {
        Self::with_capacity_and_origin(capacity, 0)
    }

    #[inline]
    pub fn with_capacity_and_origin(capacity: usize, origin: u16) -> GridAxisTracks {
        GridAxisTracks {
            tracks: GridTrackVec::with_capacity(capacity),
            origin,
            explicit_track_count: 0,
            negative_implicit_track_count: 0,
            positive_implicit_track_count: 0,
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
        if self.negative_implicit_track_count > 1 {
            -(self.negative_implicit_track_count as i16)
        } else {
            1
        }
    }

    /// The highest index initialised track
    pub fn max_track(&self) -> i16 {
        (self.explicit_track_count + self.positive_implicit_track_count) as i16
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
                max(0, (self.origin + self.explicit_track_count * 2) as i16 - (index.abs() as i16 * 2 - 1))
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
        index > 0 && index <= self.explicit_track_count as i16
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
        let min_line_index = self.origin - self.negative_implicit_track_count;
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

// pub(super) enum GridPosition {
//     Auto,
//     LineIndex(i16),
//     LineName(u16),
//     // GridAreaStart(u16),
//     // GridAreaEnd(u16),
// }

pub(super) struct NamedArea {
    name: u16,
    row_start: u16,
    row_end: u16,
    column_start: u16,
    column_end: u16,
}

pub(super) struct GridItem {
    pub node: Node,
    pub min_content_contribution: Option<Size<f32>>,
    pub max_content_contribution: Option<Size<f32>>,
    pub row: Line<i16>,
    pub column: Line<i16>,
}

impl GridItem {
    // pub fn new(node: Node) -> Self {
    //     GridItem {
    //         node,
    //         min_content_contribution: None,
    //         max_content_contribution: None,
    //         row: Line { start: GridPosition::Auto, end: GridPosition::Auto },
    //         column: Line { start: GridPosition::Auto, end: GridPosition::Auto },
    //     }
    // }

    pub fn span(&self, axis: GridAxis) -> u16 {
        match axis {
            GridAxis::Block => match (&self.row.start, &self.row.end) {
                (start, end) => max(end - start, 0) as u16,
                _ => 0,
            },
            GridAxis::Inline => match (&self.column.start, &self.column.end) {
                (start, end) => max(end - start, 0) as u16,
                _ => 0,
            },
        }
    }
}

pub(super) struct CssGrid {
    pub available_space: Size<AvailableSpace>,
    pub cell_occupancy_matrix: CellOccupancyMatrix,
    pub items: Vec<GridItem>,
    pub columns: GridAxisTracks,
    pub rows: GridAxisTracks,
    pub named_areas: Vec<NamedArea>,
}
