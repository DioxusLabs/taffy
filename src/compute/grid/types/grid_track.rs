//! Contains GridTrack used to represent a single grid track (row/column) during layout
use crate::style::{LengthPercentage, MaxTrackSizingFunction, MinTrackSizingFunction};

/// Whether a GridTrack represents an actual track or a gutter.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(in super::super) enum GridTrackKind {
    /// Track is an actual track
    Track,
    /// Track is a gutter (aka grid line) (aka gap)
    Gutter, // { name: Option<u16> },
}

/// Internal sizing information for a single grid track (row/column)
/// Gutters between tracks are sized similarly to actual tracks, so they
/// are also represented by this struct
#[derive(Debug, Clone)]
pub(in super::super) struct GridTrack {
    #[allow(dead_code)] // Used in tests + may be useful in future
    /// Whether the track is a full track, a gutter, or a placeholder that has not yet been initialised
    pub kind: GridTrackKind,

    /// The minimum track sizing function of the track
    pub min_track_sizing_function: MinTrackSizingFunction,

    /// The maximum track sizing function of the track
    pub max_track_sizing_function: MaxTrackSizingFunction,

    /// The distance of the start of the track from the start of the grid container
    pub offset: f32,

    /// The size (width/height as applicable) of the track
    pub base_size: f32,

    /// A temporary scratch value when sizing tracks
    /// Note: can be infinity
    pub growth_limit: f32,

    /// A temporary scratch value when "distributing space" to avoid clobbering planned increase variable
    pub item_incurred_increase: f32,
    /// A temporary scratch value when "distributing space" to avoid clobbering the main variable
    pub base_size_planned_increase: f32,
    /// A temporary scratch value when "distributing space" to avoid clobbering the main variable
    pub growth_limit_planned_increase: f32,
    /// A temporary scratch value when "distributing space"
    /// See: https://www.w3.org/TR/css3-grid-layout/#infinitely-growable
    pub infinitely_growable: bool,
}

impl GridTrack {
    /// Create new GridTrack representing an actual track (not a gutter)
    pub fn new(
        min_track_sizing_function: MinTrackSizingFunction,
        max_track_sizing_function: MaxTrackSizingFunction,
    ) -> GridTrack {
        GridTrack {
            kind: GridTrackKind::Track,
            min_track_sizing_function,
            max_track_sizing_function,
            offset: 0.0,
            base_size: 0.0,
            growth_limit: 0.0,
            item_incurred_increase: 0.0,
            base_size_planned_increase: 0.0,
            growth_limit_planned_increase: 0.0,
            infinitely_growable: false,
        }
    }

    /// Create a new GridTrack representing a gutter
    pub fn gutter(size: LengthPercentage) -> GridTrack {
        GridTrack {
            kind: GridTrackKind::Gutter, // { name: None },
            min_track_sizing_function: MinTrackSizingFunction::Fixed(size),
            max_track_sizing_function: MaxTrackSizingFunction::Fixed(size),
            offset: 0.0,
            base_size: 0.0,
            growth_limit: 0.0,
            item_incurred_increase: 0.0,
            base_size_planned_increase: 0.0,
            growth_limit_planned_increase: 0.0,
            infinitely_growable: false,
        }
    }

    #[inline]
    /// Returns true if the track is flexible (has a Flex MaxTrackSizingFunction), else false.
    pub fn is_flexible(&self) -> bool {
        matches!(self.max_track_sizing_function, MaxTrackSizingFunction::Flex(_))
    }

    #[inline]
    /// Returns the track's flex factor if it is a flex track, else 0.
    pub fn flex_factor(&self) -> f32 {
        match self.max_track_sizing_function {
            MaxTrackSizingFunction::Flex(flex_factor) => flex_factor,
            _ => 0.0,
        }
    }
}
