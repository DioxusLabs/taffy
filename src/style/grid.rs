//! Style types for CSS Grid layout
use super::{AlignContent, LengthPercentage, Style};
use crate::axis::{AbsoluteAxis, AbstractAxis};
use crate::compute::grid::{GridCoordinate, GridLine, OriginZeroLine};
use crate::geometry::{Line, MinMax};
use crate::style::AvailableSpace;
use crate::style_helpers::*;
use crate::sys::GridTrackVec;
use core::cmp::{max, min};

/// Controls whether grid items are placed row-wise or column-wise. And whether the sparse or dense packing algorithm is used.
///
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

/// A grid line placement specification which is generic over the coordinate system that it uses to define
/// grid line positions.
///
/// GenericGridPlacement<GridLine> is aliased as GridPlacement and is exposed to users of Taffy to define styles.
/// GenericGridPlacement<OriginZeroLine> is aliased as OriginZeroGridPlacement and is used internally for placement computations.
///
/// See [`crate::compute::grid::type::coordinates`] for documentation on the different coordinate systems.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GenericGridPlacement<LineType: GridCoordinate> {
    /// Place item according to the auto-placement algorithm, and the parent's grid_auto_flow property
    Auto,
    /// Place item at specified line (column or row) index
    Line(LineType),
    /// Item should span specified number of tracks (columns or rows)
    Span(u16),
}

/// A grid line placement using the normalized OriginZero coordinates to specify line positions.
pub(crate) type OriginZeroGridPlacement = GenericGridPlacement<OriginZeroLine>;

/// A grid line placement specification. Used for grid-[row/column]-[start/end]. Named tracks are not implemented.
///
/// Defaults to [`GridLine::Auto`]
///
/// [Specification](https://www.w3.org/TR/css3-grid-layout/#typedef-grid-row-start-grid-line)
pub type GridPlacement = GenericGridPlacement<GridLine>;
impl TaffyAuto for GridPlacement {
    const AUTO: Self = Self::Auto;
}
impl TaffyGridLine for GridPlacement {
    fn from_line_index(index: i16) -> Self {
        GridPlacement::Line(GridLine::from(index))
    }
}
impl TaffyGridLine for Line<GridPlacement> {
    fn from_line_index(index: i16) -> Self {
        Line { start: GridPlacement::from_line_index(index), end: GridPlacement::Auto }
    }
}
impl TaffyGridSpan for GridPlacement {
    fn from_span(span: u16) -> Self {
        GridPlacement::Span(span)
    }
}
impl TaffyGridSpan for Line<GridPlacement> {
    fn from_span(span: u16) -> Self {
        Line { start: GridPlacement::from_span(span), end: GridPlacement::Auto }
    }
}

impl Default for GridPlacement {
    fn default() -> Self {
        Self::Auto
    }
}

impl GridPlacement {
    /// Apply a mapping function if the [`GridPlacement`] is a `Track`. Otherwise return `self` unmodified.
    pub fn into_origin_zero_placement(self, explicit_track_count: u16) -> OriginZeroGridPlacement {
        match self {
            Self::Auto => OriginZeroGridPlacement::Auto,
            Self::Span(span) => OriginZeroGridPlacement::Span(span),
            // Grid line zero is an invalid index, so it gets treated as Auto
            // See: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row-start#values
            Self::Line(line) => match line.as_i16() {
                0 => OriginZeroGridPlacement::Auto,
                _ => OriginZeroGridPlacement::Line(line.into_origin_zero_line(explicit_track_count)),
            },
        }
    }
}

impl<T: GridCoordinate> Line<GenericGridPlacement<T>> {
    #[inline]
    /// Whether the track position is definite in this axis (or the item will need auto placement)
    /// The track position is definite if least one of the start and end positions is a track index
    pub fn is_definite(&self) -> bool {
        matches!((self.start, self.end), (GenericGridPlacement::Line(_), _) | (_, GenericGridPlacement::Line(_)))
    }

    /// Resolves the span for an indefinite placement (a placement that does not consist of two `Track`s).
    /// Panics if called on a definite placement
    pub fn indefinite_span(&self) -> u16 {
        use GenericGridPlacement as GP;
        match (self.start, self.end) {
            (GP::Line(_), GP::Auto) => 1,
            (GP::Auto, GP::Line(_)) => 1,
            (GP::Auto, GP::Auto) => 1,
            (GP::Line(_), GP::Span(span)) => span,
            (GP::Span(span), GP::Line(_)) => span,
            (GP::Span(span), GP::Auto) => span,
            (GP::Auto, GP::Span(span)) => span,
            (GP::Span(span), GP::Span(_)) => span,
            (GP::Line(_), GP::Line(_)) => panic!("indefinite_span should only be called on indefinite grid tracks"),
        }
    }
}

impl Line<GridPlacement> {
    /// Apply a mapping function if the [`GridPlacement`] is a `Track`. Otherwise return `self` unmodified.
    pub fn into_origin_zero(&self, explicit_track_count: u16) -> Line<OriginZeroGridPlacement> {
        Line {
            start: self.start.into_origin_zero_placement(explicit_track_count),
            end: self.end.into_origin_zero_placement(explicit_track_count),
        }
    }
}

impl Line<OriginZeroGridPlacement> {
    /// If at least one of the of the start and end positions is a track index then the other end can be resolved
    /// into a track index purely based on the information contained with the placement specification
    pub fn resolve_definite_grid_lines(&self) -> Line<OriginZeroLine> {
        use OriginZeroGridPlacement as GP;
        match (self.start, self.end) {
            (GP::Line(line1), GP::Line(line2)) => {
                if line1 == line2 {
                    Line { start: line1, end: line1 + 1 }
                } else {
                    Line { start: min(line1, line2), end: max(line1, line2) }
                }
            }
            (GP::Line(line), GP::Span(span)) => Line { start: line, end: line + span },
            (GP::Line(line), GP::Auto) => Line { start: line, end: line + 1 },
            (GP::Span(span), GP::Line(line)) => Line { start: line - span, end: line },
            (GP::Auto, GP::Line(line)) => Line { start: line - 1, end: line },
            _ => panic!("resolve_definite_grid_tracks should only be called on definite grid tracks"),
        }
    }

    /// For absolutely positioned items:
    ///   - Tracks resolve to definite tracks
    ///   - For Spans:
    ///      - If the other position is a Track, they resolve to a definite track relative to the other track
    ///      - Else resolve to None
    ///   - Auto resolves to None
    ///
    /// When finally positioning the item, a value of None means that the item's grid area is bounded by the grid
    /// container's border box on that side.
    pub fn resolve_absolutely_positioned_grid_tracks(&self) -> Line<Option<OriginZeroLine>> {
        use OriginZeroGridPlacement as GP;
        match (self.start, self.end) {
            (GP::Line(track1), GP::Line(track2)) => {
                if track1 == track2 {
                    Line { start: Some(track1), end: Some(track1 + 1) }
                } else {
                    Line { start: Some(min(track1, track2)), end: Some(max(track1, track2)) }
                }
            }
            (GP::Line(track), GP::Span(span)) => Line { start: Some(track), end: Some(track + span) },
            (GP::Line(track), GP::Auto) => Line { start: Some(track), end: None },
            (GP::Span(span), GP::Line(track)) => Line { start: Some(track - span), end: Some(track) },
            (GP::Auto, GP::Line(track)) => Line { start: None, end: Some(track) },
            _ => Line { start: None, end: None },
        }
    }

    /// If neither of the start and end positions is a track index then the other end can be resolved
    /// into a track index if a definite start position is supplied externally
    pub fn resolve_indefinite_grid_tracks(&self, start: OriginZeroLine) -> Line<OriginZeroLine> {
        use OriginZeroGridPlacement as GP;
        match (self.start, self.end) {
            (GP::Auto, GP::Auto) => Line { start, end: start + 1 },
            (GP::Span(span), GP::Auto) => Line { start, end: start + span },
            (GP::Auto, GP::Span(span)) => Line { start, end: start + span },
            (GP::Span(span), GP::Span(_)) => Line { start, end: start + span },
            _ => panic!("resolve_indefinite_grid_tracks should only be called on indefinite grid tracks"),
        }
    }
}

/// Represents the start and end points of a GridItem within a given axis
impl Default for Line<GridPlacement> {
    fn default() -> Self {
        Line { start: GridPlacement::Auto, end: GridPlacement::Auto }
    }
}

/// Maximum track sizing function
///
/// Specifies the maximum size of a grid track. A grid track will automatically size between it's minimum and maximum size based
/// on the size of it's contents, the amount of available space, and the sizing constraint the grid is being size under.
/// See https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MaxTrackSizingFunction {
    /// Track maximum size should be a fixed points or percentage value
    Fixed(LengthPercentage),
    /// Track maximum size should be content sized under a min-content constraint
    MinContent,
    /// Track maximum size should be content sized under a max-content constraint
    MaxContent,
    /// Track maximum size should be sized according to the fit-content formula
    FitContent(LengthPercentage),
    /// Track maximum size should be automatically sized
    Auto,
    /// The dimension as a fraction of the total available grid space.
    /// Specified value is the numerator of the fraction. Denominator is the sum of all fraction specified in that grid dimension
    /// Spec: https://www.w3.org/TR/css3-grid-layout/#fr-unit
    Flex(f32),
}
impl TaffyAuto for MaxTrackSizingFunction {
    const AUTO: Self = Self::Auto;
}
impl TaffyMinContent for MaxTrackSizingFunction {
    const MIN_CONTENT: Self = Self::MinContent;
}
impl TaffyMaxContent for MaxTrackSizingFunction {
    const MAX_CONTENT: Self = Self::MaxContent;
}
impl TaffyFitContent for MaxTrackSizingFunction {
    fn fit_content(argument: LengthPercentage) -> Self {
        Self::FitContent(argument)
    }
}
impl TaffyZero for MaxTrackSizingFunction {
    const ZERO: Self = Self::Fixed(LengthPercentage::ZERO);
}
impl FromPoints for MaxTrackSizingFunction {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Self::Fixed(LengthPercentage::from_points(points))
    }
}
impl FromPercent for MaxTrackSizingFunction {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::Fixed(LengthPercentage::from_percent(percent))
    }
}
impl FromFlex for MaxTrackSizingFunction {
    fn from_flex<Input: Into<f32> + Copy>(flex: Input) -> Self {
        Self::Flex(flex.into())
    }
}

impl MaxTrackSizingFunction {
    /// Returns true if the max track sizing function is `MinContent`, `MaxContent` or `Auto`, else false.
    #[inline(always)]
    pub fn is_intrinsic(&self) -> bool {
        matches!(self, Self::MinContent | Self::MaxContent | Self::FitContent(_) | Self::Auto)
    }

    /// Returns true if the max track sizing function is `MaxContent`, `FitContent` or `Auto` else false.
    /// "In all cases, treat auto and fit-content() as max-content, except where specified otherwise for fit-content()."
    /// See: https://www.w3.org/TR/css-grid-1/#algo-terms
    #[inline(always)]
    pub fn is_max_content_alike(&self) -> bool {
        matches!(self, Self::MaxContent | Self::FitContent(_) | Self::Auto)
    }

    /// Returns true if the max track sizing function is `Flex`, else false.
    #[inline(always)]
    pub fn is_flexible(&self) -> bool {
        matches!(self, Self::Flex(_))
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
            MinContent | MaxContent | FitContent(_) | Auto | Flex(_) => None,
        }
    }

    pub fn definite_limit(self, available_space: AvailableSpace) -> Option<f32> {
        use MaxTrackSizingFunction::FitContent;
        match self {
            FitContent(LengthPercentage::Points(size)) => Some(size),
            FitContent(LengthPercentage::Percent(fraction)) => match available_space {
                AvailableSpace::Definite(available_size) => Some(fraction * available_size),
                _ => None,
            },
            _ => self.definite_value(available_space),
        }
    }

    /// Resolve percentage values against the passed parent_size, returning Some(value)
    /// Non-percentage values always return None.
    #[inline(always)]
    pub fn resolved_percentage_size(self, parent_size: f32) -> Option<f32> {
        use MaxTrackSizingFunction::{Auto, *};
        match self {
            Fixed(LengthPercentage::Percent(fraction)) => Some(fraction * parent_size),
            Fixed(LengthPercentage::Points(_)) | MinContent | MaxContent | FitContent(_) | Auto | Flex(_) => None,
        }
    }
}

/// Minimum track sizing function
///
/// Specifies the minimum size of a grid track. A grid track will automatically size between it's minimum and maximum size based
/// on the size of it's contents, the amount of available space, and the sizing constraint the grid is being size under.
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
impl TaffyAuto for MinTrackSizingFunction {
    const AUTO: Self = Self::Auto;
}
impl TaffyMinContent for MinTrackSizingFunction {
    const MIN_CONTENT: Self = Self::MinContent;
}
impl TaffyMaxContent for MinTrackSizingFunction {
    const MAX_CONTENT: Self = Self::MaxContent;
}
impl TaffyZero for MinTrackSizingFunction {
    const ZERO: Self = Self::Fixed(LengthPercentage::ZERO);
}
impl FromPoints for MinTrackSizingFunction {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Self::Fixed(LengthPercentage::from_points(points))
    }
}
impl FromPercent for MinTrackSizingFunction {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::Fixed(LengthPercentage::from_percent(percent))
    }
}

impl MinTrackSizingFunction {
    /// Returns fixed point values directly. Attempts to resolve percentage values against
    /// the passed available_space and returns if this results in a concrete value (which it
    /// will if the available_space is `Some`). Otherwise returns `None`.
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

    /// Resolve percentage values against the passed parent_size, returning Some(value)
    /// Non-percentage values always return None.
    #[inline(always)]
    pub fn resolved_percentage_size(self, parent_size: f32) -> Option<f32> {
        use MinTrackSizingFunction::{Auto, *};
        match self {
            Fixed(LengthPercentage::Percent(fraction)) => Some(fraction * parent_size),
            Fixed(LengthPercentage::Points(_)) | MinContent | MaxContent | Auto => None,
        }
    }
}

/// The sizing function for a grid track (row/column) (either auto-track or template track)
/// May either be a MinMax variant which specifies separate values for the min-/max- track sizing functions
/// or a scalar value which applies to both track sizing functions.
pub type NonRepeatedTrackSizingFunction = MinMax<MinTrackSizingFunction, MaxTrackSizingFunction>;
impl NonRepeatedTrackSizingFunction {
    /// Extract the min track sizing function
    pub fn min_sizing_function(&self) -> MinTrackSizingFunction {
        self.min
    }
    /// Extract the max track sizing function
    pub fn max_sizing_function(&self) -> MaxTrackSizingFunction {
        self.max
    }
    /// Determine whether at least one of the components ("min" and "max") are fixed sizing function
    pub fn has_fixed_component(&self) -> bool {
        matches!(self.min, MinTrackSizingFunction::Fixed(_)) || matches!(self.max, MaxTrackSizingFunction::Fixed(_))
    }
}
impl TaffyAuto for NonRepeatedTrackSizingFunction {
    const AUTO: Self = Self { min: MinTrackSizingFunction::AUTO, max: MaxTrackSizingFunction::AUTO };
}
impl TaffyMinContent for NonRepeatedTrackSizingFunction {
    const MIN_CONTENT: Self =
        Self { min: MinTrackSizingFunction::MIN_CONTENT, max: MaxTrackSizingFunction::MIN_CONTENT };
}
impl TaffyMaxContent for NonRepeatedTrackSizingFunction {
    const MAX_CONTENT: Self =
        Self { min: MinTrackSizingFunction::MAX_CONTENT, max: MaxTrackSizingFunction::MAX_CONTENT };
}
impl TaffyFitContent for NonRepeatedTrackSizingFunction {
    fn fit_content(argument: LengthPercentage) -> Self {
        Self { min: MinTrackSizingFunction::AUTO, max: MaxTrackSizingFunction::FitContent(argument) }
    }
}
impl TaffyZero for NonRepeatedTrackSizingFunction {
    const ZERO: Self = Self { min: MinTrackSizingFunction::ZERO, max: MaxTrackSizingFunction::ZERO };
}
impl FromPoints for NonRepeatedTrackSizingFunction {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Self { min: MinTrackSizingFunction::from_points(points), max: MaxTrackSizingFunction::from_points(points) }
    }
}
impl FromPercent for NonRepeatedTrackSizingFunction {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self { min: MinTrackSizingFunction::from_percent(percent), max: MaxTrackSizingFunction::from_percent(percent) }
    }
}
impl FromFlex for NonRepeatedTrackSizingFunction {
    fn from_flex<Input: Into<f32> + Copy>(flex: Input) -> Self {
        Self { min: MinTrackSizingFunction::AUTO, max: MaxTrackSizingFunction::from_flex(flex) }
    }
}

/// The first argument to a repeated track definition. This type represents the type of automatic repetition to perform.
///
/// See https://www.w3.org/TR/css-grid-1/#auto-repeat for an explanation of how auto-repeated track definitions work
/// and the difference between AutoFit and AutoFill.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridTrackRepetition {
    /// Auto-repeating track should be generated to fit the container
    /// See: https://developer.mozilla.org/en-US/docs/Web/CSS/repeat#auto-fill
    AutoFill,
    /// Auto-repeating track should be generated to fit the container
    /// See: https://developer.mozilla.org/en-US/docs/Web/CSS/repeat#auto-fit
    AutoFit,
}

/// The sizing function for a grid track (row/column)
/// See https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns
#[derive(Clone, PartialEq, Debug)]
pub enum TrackSizingFunction {
    /// A single non-repeated track
    Single(NonRepeatedTrackSizingFunction),
    /// Automatically generate grid tracks to fit the available space using the specified definite track lengths
    /// Only valid if every track in template (not just the repitition) has a fixed size.
    AutoRepeat(GridTrackRepetition, GridTrackVec<NonRepeatedTrackSizingFunction>),
}
impl TrackSizingFunction {
    /// Whether the track definition is a auto-repeated fragment
    pub fn is_auto_repetition(&self) -> bool {
        matches!(self, Self::AutoRepeat(_, _))
    }
}
impl TaffyAuto for TrackSizingFunction {
    const AUTO: Self = Self::Single(NonRepeatedTrackSizingFunction::AUTO);
}
impl TaffyMinContent for TrackSizingFunction {
    const MIN_CONTENT: Self = Self::Single(NonRepeatedTrackSizingFunction::MIN_CONTENT);
}
impl TaffyMaxContent for TrackSizingFunction {
    const MAX_CONTENT: Self = Self::Single(NonRepeatedTrackSizingFunction::MAX_CONTENT);
}
impl TaffyFitContent for TrackSizingFunction {
    fn fit_content(argument: LengthPercentage) -> Self {
        Self::Single(NonRepeatedTrackSizingFunction::fit_content(argument))
    }
}
impl TaffyZero for TrackSizingFunction {
    const ZERO: Self = Self::Single(NonRepeatedTrackSizingFunction::ZERO);
}
impl FromPoints for TrackSizingFunction {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Self::Single(NonRepeatedTrackSizingFunction::from_points(points))
    }
}
impl FromPercent for TrackSizingFunction {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::Single(NonRepeatedTrackSizingFunction::from_percent(percent))
    }
}
impl FromFlex for TrackSizingFunction {
    fn from_flex<Input: Into<f32> + Copy>(flex: Input) -> Self {
        Self::Single(NonRepeatedTrackSizingFunction::from_flex(flex))
    }
}
impl From<MinMax<MinTrackSizingFunction, MaxTrackSizingFunction>> for TrackSizingFunction {
    fn from(input: MinMax<MinTrackSizingFunction, MaxTrackSizingFunction>) -> Self {
        Self::Single(input)
    }
}

// Grid extensions to the Style struct
impl Style {
    /// Get a grid item's row or column placement depending on the axis passed
    pub(crate) fn grid_template_tracks(&self, axis: AbsoluteAxis) -> &GridTrackVec<TrackSizingFunction> {
        match axis {
            AbsoluteAxis::Horizontal => &self.grid_template_columns,
            AbsoluteAxis::Vertical => &self.grid_template_rows,
        }
    }

    /// Get a grid item's row or column placement depending on the axis passed
    pub(crate) fn grid_placement(&self, axis: AbsoluteAxis) -> Line<GridPlacement> {
        match axis {
            AbsoluteAxis::Horizontal => self.grid_column,
            AbsoluteAxis::Vertical => self.grid_row,
        }
    }

    /// Get a grid container's align-content or justify-content alignment depending on the axis passed
    pub(crate) fn grid_align_content(&self, axis: AbstractAxis) -> AlignContent {
        match axis {
            AbstractAxis::Inline => self.justify_content.unwrap_or(AlignContent::Stretch),
            AbstractAxis::Block => self.align_content.unwrap_or(AlignContent::Stretch),
        }
    }
}
