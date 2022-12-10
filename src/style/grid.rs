//! Style types for CSS Grid layout
use super::LengthPercentage;
use crate::axis::AbsoluteAxis;
use crate::geometry::Line;
use crate::layout::AvailableSpace;
use core::cmp::{max, min};

/// Controls whether grid items are placed row-wise or column-wise. And whether the sparse or dense packing algorithm is used.
/// The "dense" packing algorithm attempts to fill in holes earlier in the grid, if smaller items come up later. This may cause items to appear out-of-order, when doing so would fill in holes left by larger items.
///
/// Defaults to [`GridAutoFlow::Row`]
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GridAutoFlow {
    /// Items are placed by filling each row in turn, adding new rows as necessary
    Row,
    /// Items are placed by filling each column in turn, adding new columns as necessary.
    Column,
    /// Combines `Row` with the dense packing algorithm.
    RowDense,
    /// Combines `Column` with the dense packing algorithm.
    ColumnDense,
}

impl Default for GridAutoFlow {
    fn default() -> Self {
        Self::Row
    }
}

impl GridAutoFlow {
    /// Whether grid auto placement uses the sparse placement algorithm or the dense placement algorithm
    /// See: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow#values
    pub fn is_dense(&self) -> bool {
        match self {
            Self::Row | Self::Column => false,
            Self::RowDense | Self::ColumnDense => true,
        }
    }

    /// Whether grid auto placement fills areas row-wise or column-wise
    /// See: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow#values
    pub fn primary_axis(&self) -> AbsoluteAxis {
        match self {
            Self::Row | Self::RowDense => AbsoluteAxis::Horizontal,
            Self::Column | Self::ColumnDense => AbsoluteAxis::Vertical,
        }
    }
}

/// A track placement specicification. Used for grid-[row/column]-[start/end]. Named tracks are not implemented.
///
/// Defaults to [`GridLine::Auto`]
///
/// [Specification](https://www.w3.org/TR/css3-grid-layout/#typedef-grid-row-start-grid-line)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GridPlacement {
    /// Place item according to the auto-placement algorithm, and the parent's grid_auto_flow property
    Auto,
    /// Place item at specified track (column or row) index
    Track(i16),
    /// Item should span specified number of tracks (columns or rows)
    Span(u16),
}

impl Default for GridPlacement {
    fn default() -> Self {
        Self::Auto
    }
}

impl GridPlacement {
    /// Apply a mapping function if the GridPlacement is a Track. Otherwise return `self` unmodified.
    pub fn map_track(&self, map_fn: impl FnOnce(i16) -> i16) -> Self {
        use GridPlacement::*;
        match *self {
            Auto => Auto,
            Span(span) => Span(span),
            Track(track) => Track(map_fn(track)),
        }
    }
}

/// Represents the start and end points of a GridItem within a given axis
impl Line<GridPlacement> {
    #[inline]
    /// Whether the track position is definite in this axis (or the item will need auto placement)
    /// The track position is definite if least one of the start and end positions is a track index
    pub fn is_definite(&self) -> bool {
        use GridPlacement::*;
        matches!((self.start, self.end), (Track(_), _) | (_, Track(_)))
    }

    /// If at least one of the of the start and end positions is a track index then the other end can be resolved
    /// into a track index purely based on the information contained with the placement specification
    pub fn resolve_definite_grid_tracks(&self) -> Line<i16> {
        use GridPlacement::*;
        match (self.start, self.end) {
            (Track(track1), Track(track2)) => {
                if track1 == track2 {
                    Line { start: track1, end: track1 + 1 }
                } else {
                    Line { start: min(track1, track2), end: max(track1, track2) }
                }
            }
            (Track(track), Span(span)) => Line { start: track, end: track + span as i16 },
            (Track(track), Auto) => Line { start: track, end: track + 1_i16 },
            (Span(span), Track(track)) => Line { start: track - span as i16, end: track },
            (Auto, Track(track)) => Line { start: track - 1_i16, end: track },
            _ => panic!("resolve_definite_grid_tracks should only be called on definite grid tracks"),
        }
    }

    /// If neither of the start and end positions is a track index then the other end can be resolved
    /// into a track index if a definite start position is supplied externally
    pub fn resolve_indefinite_grid_tracks(&self, start: i16) -> Line<i16> {
        use GridPlacement::*;
        match (self.start, self.end) {
            (Auto, Auto) => Line { start, end: start + 1 },
            (Span(span), Auto) => Line { start, end: start + span as i16 },
            (Auto, Span(span)) => Line { start, end: start + span as i16 },
            (Span(span), Span(_)) => Line { start, end: start + span as i16 },
            _ => panic!("resolve_indefinite_grid_tracks should only be called on indefinite grid tracks"),
        }
    }

    /// Resolves the span for an indefinite placement (a placement that does not consist of two `Track`s).
    /// Panics if called on a definite placement
    pub fn indefinite_span(&self) -> u16 {
        use GridPlacement::*;
        match (self.start, self.end) {
            (Track(_), Auto) => 1,
            (Auto, Track(_)) => 1,
            (Auto, Auto) => 1,
            (Track(_), Span(span)) => span,
            (Span(span), Track(_)) => span,
            (Span(span), Auto) => span,
            (Auto, Span(span)) => span,
            (Span(span), Span(_)) => span,
            (Track(_), Track(_)) => panic!("indefinite_span should only be called on indefinite grid tracks"),
        }
    }
}

/// Represents the start and end points of a GridItem within a given axis
impl Default for Line<GridPlacement> {
    fn default() -> Self {
        Line { start: GridPlacement::Auto, end: GridPlacement::Auto }
    }
}

/// Minimum track sizing function
/// Specifies the maximum size of a grid track
/// See https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MaxTrackSizingFunction {
    /// Track maximum size should be a fixed points or percentage value
    Fixed(LengthPercentage),
    /// Track maximum size should be content sized under a min-content constraint
    MinContent,
    /// Track maximum size should be content sized under a max-content constraint
    MaxContent,
    /// Track maximum size should be automatically sized
    Auto,
    /// The dimension as a fraction of the total available grid space.
    /// Specified value is the numerator of the fraction. Denominator is the sum of all fraction specified in that grid dimension
    /// Spec: https://www.w3.org/TR/css3-grid-layout/#fr-unit
    Flex(f32),
}

impl MaxTrackSizingFunction {
    /// Returns true if the max track sizing function is `MinContent`, `MaxContent` or `Auto`, else false.
    #[inline(always)]
    pub fn is_intrinsic(&self) -> bool {
        use MaxTrackSizingFunction::*;
        match self {
            MinContent | MaxContent | Auto => true,
            Fixed(_) | Flex(_) => false,
        }
    }

    /// Returns true if the max track sizing function is `MaxContent`, else false.
    #[inline(always)]
    pub fn is_max_content(&self) -> bool {
        use MaxTrackSizingFunction::*;
        match self {
            MaxContent => true,
            Auto | MinContent | Fixed(_) | Flex(_) => false,
        }
    }

    /// Returns true if the max track sizing function is `Flex`, else false.
    #[inline(always)]
    pub fn is_flexible(&self) -> bool {
        matches!(self, MaxTrackSizingFunction::Flex(_))
    }

    /// Returns fixed point values directly. Attempts to resolve percentage values against
    /// the passed available_space and returns if this results in a concrete value (which it
    /// will if the available_space is `Some`). Otherwise returns None.
    #[inline(always)]
    pub fn definite_value(self, available_space: AvailableSpace) -> Option<f32> {
        use MaxTrackSizingFunction::{Auto, *};
        match self {
            Fixed(LengthPercentage::Points(size)) => Some(size),
            Fixed(LengthPercentage::Percent(fraction)) => match available_space {
                AvailableSpace::Definite(available_size) => Some(fraction * available_size),
                _ => None,
            },
            MinContent | MaxContent | Auto | Flex(_) => None,
        }
    }
}

/// Minimum track sizing function
/// Specifies the minimum size of a grid track
/// See https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MinTrackSizingFunction {
    /// Track minimum size should be a fixed points or percentage value
    Fixed(LengthPercentage),
    /// Track minimum size should be content sized under a min-content constraint
    MinContent,
    /// Track minimum size should be content sized under a max-content constraint
    MaxContent,
    /// Track minimum size should be automatically sized
    Auto,
}

impl MinTrackSizingFunction {
    /// Returns fixed point values directly. Attempts to resolve percentage values against
    /// the passed available_space and returns if this results in a concrete value (which it
    /// will if the available_space is `Some`). Otherwise returns None.
    #[inline(always)]
    pub fn definite_value(self, available_space: AvailableSpace) -> Option<f32> {
        use MinTrackSizingFunction::{Auto, *};
        match self {
            Fixed(LengthPercentage::Points(size)) => Some(size),
            Fixed(LengthPercentage::Percent(fraction)) => match available_space {
                AvailableSpace::Definite(available_size) => Some(fraction * available_size),
                _ => None,
            },
            MinContent | MaxContent | Auto => None,
        }
    }
}

/// The sizing function for a grid track (row/column).
/// May either a MinMax variant which specifies separate values for the min-/max- track sizing functions
/// or a scalar value which applies to both track sizing functions.
/// See https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TrackSizingFunction {
    /// Track should be a fixed points or percentage value
    Fixed(LengthPercentage),
    /// Track should be content sized under a min-content constraint
    MinContent,
    /// Track should be content sized under a max-content constraint
    MaxContent,
    /// Track should be automatically sized
    Auto,
    /// The dimension as a fraction of the total available grid space.
    /// Specified value is the numerator of the fraction. Denominator is the sum of all fraction specified in that grid dimension
    /// Spec: https://www.w3.org/TR/css3-grid-layout/#fr-unit
    Flex(f32),
    /// Specify the min tracking sizing function and the max sizing function separately
    MinMax {
        /// The min tracking sizing function
        min: MinTrackSizingFunction,
        /// The max tracking sizing function
        max: MaxTrackSizingFunction,
    },
}

impl TrackSizingFunction {
    /// Getter for the min_track_sizing_function. This is either the `min` property of the MinMax Variant,
    /// or else another variant converted to the same variant in the MinTrackSizingFunction enum
    /// Flex is not valid MinTrackingSizingFunction, and thus gets converted to Auto
    pub fn min_sizing_function(&self) -> MinTrackSizingFunction {
        match self {
            Self::MinMax { min, .. } => *min,
            Self::Fixed(value) => MinTrackSizingFunction::Fixed(*value),
            Self::MinContent => MinTrackSizingFunction::MinContent,
            Self::MaxContent => MinTrackSizingFunction::MaxContent,
            Self::Auto => MinTrackSizingFunction::Auto,
            Self::Flex(_) => MinTrackSizingFunction::Auto,
        }
    }

    /// Getter for the max_track_sizing_function. This is either the `max` property of the MinMax Variant,
    /// or else another variant converted to the same variant in the MaxTrackSizingFunction enum
    pub fn max_sizing_function(&self) -> MaxTrackSizingFunction {
        match self {
            Self::MinMax { max, .. } => *max,
            Self::Fixed(value) => MaxTrackSizingFunction::Fixed(*value),
            Self::MinContent => MaxTrackSizingFunction::MinContent,
            Self::MaxContent => MaxTrackSizingFunction::MaxContent,
            Self::Auto => MaxTrackSizingFunction::Auto,
            Self::Flex(value) => MaxTrackSizingFunction::Flex(*value),
        }
    }
}
