//! Contains GridTrack used to represent a single grid track (row/column) during layout
use crate::{
    prelude::TaffyZero,
    style::{LengthPercentage, MaxTrackSizingFunction, MinTrackSizingFunction},
};
use app_units::{Au, MAX_AU};

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

    /// Whether the track is a collapsed track/gutter. Collapsed tracks are effectively treated as if
    /// they don't exist for the purposes of grid sizing. Gutters between collapsed tracks are also collapsed.
    pub is_collapsed: bool,

    /// The minimum track sizing function of the track
    pub min_track_sizing_function: MinTrackSizingFunction,

    /// The maximum track sizing function of the track
    pub max_track_sizing_function: MaxTrackSizingFunction,

    /// The distance of the start of the track from the start of the grid container
    pub offset: Au,

    /// The size (width/height as applicable) of the track
    pub base_size: Au,

    /// A temporary scratch value when sizing tracks
    /// Note: can be infinity
    pub growth_limit: Au,

    /// A temporary scratch value when sizing tracks. Is used as an additional amount to add to the
    /// estimate for the available space in the opposite axis when content sizing items
    pub content_alignment_adjustment: Au,

    /// A temporary scratch value when "distributing space" to avoid clobbering planned increase variable
    pub item_incurred_increase: Au,
    /// A temporary scratch value when "distributing space" to avoid clobbering the main variable
    pub base_size_planned_increase: Au,
    /// A temporary scratch value when "distributing space" to avoid clobbering the main variable
    pub growth_limit_planned_increase: Au,
    /// A temporary scratch value when "distributing space"
    /// See: https://www.w3.org/TR/css3-grid-layout/#infinitely-growable
    pub infinitely_growable: bool,

    /// The resolved `fit-content()` limit of the max track sizing function (infinity if it is not `fit-content()`
    /// or the limit cannot be resolved). Resolved once per track sizing pass by `resolve_fit_content_limit`.
    pub fit_content_limit: Au,
}

#[inline(always)]
fn au_min(a: Au, b: Au) -> Au {
    a.min(b)
}

impl GridTrack {
    /// GridTrack constructor with all configuration parameters for the other constructors exposed
    const fn new_with_kind(
        kind: GridTrackKind,
        min_track_sizing_function: MinTrackSizingFunction,
        max_track_sizing_function: MaxTrackSizingFunction,
    ) -> GridTrack {
        GridTrack {
            kind,
            is_collapsed: false,
            min_track_sizing_function,
            max_track_sizing_function,
            offset: Au(0),
            base_size: Au(0),
            growth_limit: Au(0),
            content_alignment_adjustment: Au(0),
            item_incurred_increase: Au(0),
            base_size_planned_increase: Au(0),
            growth_limit_planned_increase: Au(0),
            infinitely_growable: false,
            fit_content_limit: MAX_AU,
        }
    }

    /// Create new GridTrack representing an actual track (not a gutter)
    pub const fn new(
        min_track_sizing_function: MinTrackSizingFunction,
        max_track_sizing_function: MaxTrackSizingFunction,
    ) -> GridTrack {
        Self::new_with_kind(GridTrackKind::Track, min_track_sizing_function, max_track_sizing_function)
    }

    /// Create a new GridTrack representing a gutter
    pub fn gutter(size: LengthPercentage) -> GridTrack {
        Self::new_with_kind(
            GridTrackKind::Gutter,
            MinTrackSizingFunction::from(size),
            MaxTrackSizingFunction::from(size),
        )
    }

    /// Mark a GridTrack as collapsed. Also sets both of the track's sizing functions
    /// to fixed zero-sized sizing functions.
    pub fn collapse(&mut self) {
        self.is_collapsed = true;
        self.min_track_sizing_function = MinTrackSizingFunction::ZERO;
        self.max_track_sizing_function = MaxTrackSizingFunction::ZERO;
        self.fit_content_limit = MAX_AU;
    }

    #[inline(always)]
    /// Returns true if the track is flexible (has a Flex MaxTrackSizingFunction), else false.
    pub fn is_flexible(&self) -> bool {
        self.max_track_sizing_function.is_fr()
    }

    #[inline(always)]
    /// Returns true if the track is flexible (has a Flex MaxTrackSizingFunction), else false.
    pub fn uses_percentage(&self) -> bool {
        self.min_track_sizing_function.uses_percentage() || self.max_track_sizing_function.uses_percentage()
    }

    #[inline(always)]
    /// Returns true if the track has an intrinsic min and or max sizing function
    pub fn has_intrinsic_sizing_function(&self) -> bool {
        self.min_track_sizing_function.is_intrinsic() || self.max_track_sizing_function.is_intrinsic()
    }

    /// Cache the `fit-content()` limit of the max track sizing function (if any), resolved against the
    /// container's inner size in this axis. Percentage and calc() limits resolve to infinity if that size is
    /// indefinite, as do tracks which are not `fit-content()`.
    #[inline]
    pub fn resolve_fit_content_limit(
        &mut self,
        axis_inner_node_size: Option<Au>,
        calc_resolver: impl Fn(*const (), f32) -> f32,
    ) {
        self.fit_content_limit =
            self.max_track_sizing_function.fit_content_limit(axis_inner_node_size, calc_resolver).unwrap_or(MAX_AU);
    }

    #[inline(always)]
    /// Returns the resolved `fit-content()` limit of the track (infinity if the track is not `fit-content()`)
    pub fn fit_content_limit(&self) -> Au {
        self.fit_content_limit
    }

    #[inline]
    /// Returns the track's growth limit, capped by its `fit-content()` limit (if any)
    pub fn fit_content_limited_growth_limit(&self) -> Au {
        au_min(self.growth_limit, self.fit_content_limit)
    }

    #[inline]
    /// Returns the track's flex factor if it is a flex track, else 0.
    pub fn flex_factor(&self) -> f32 {
        if self.max_track_sizing_function.is_fr() {
            self.max_track_sizing_function.0.value()
        } else {
            0.0
        }
    }
}
