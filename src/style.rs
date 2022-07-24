//! A representation of [CSS layout properties](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) in Rust, used for flexbox layout

use crate::geometry::{Rect, Size};
use crate::sys::GridTrackVec;

/// How [`Nodes`](crate::node::Node) are aligned relative to the cross axis
///
/// The default behavior is [`AlignItems::Stretch`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#align-items-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AlignItems {
    /// Items are packed toward the start of the cross axis
    FlexStart,
    /// Items are packed toward the end of the cross axis
    FlexEnd,
    /// Items are packed along the center of the cross axis
    Center,
    /// Items are aligned such as their baselines align
    Baseline,
    /// Stretch to fill the container
    Stretch,
}

impl Default for AlignItems {
    fn default() -> Self {
        Self::Stretch
    }
}

/// Overrides the inherited [`AlignItems`] behavior for this node.
///
/// The behavior of any child nodes will be controlled by this node's [`AlignItems`] value.
///
/// The default behavior is [`AlignSelf::Auto`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#align-items-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AlignSelf {
    /// Inherits the [`AlignItems`] behavior of the parent
    Auto,
    /// Items are packed toward the start of the cross axis
    FlexStart,
    /// Items are packed toward the end of the cross axis
    FlexEnd,
    /// Items are packed along the center of the cross axis
    Center,
    /// Items are aligned such as their baselines align
    Baseline,
    /// Distribute items evenly, but stretch them to fill the container
    Stretch,
}

impl Default for AlignSelf {
    fn default() -> Self {
        Self::Auto
    }
}

/// Sets the distribution of space between and around content items along the cross-axis
///
/// The default value is [`AlignContent::Stretch`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#align-content-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AlignContent {
    /// Items are packed toward the start of the axis
    FlexStart,
    /// Items are packed toward the end of the axis
    FlexEnd,
    /// Items are centered around the middle of the axis
    Center,
    /// Items are stretched to fill the container
    Stretch,
    /// The first and last items are aligned flush with the edges of the container (no gap)
    /// The gap between items is distributed evenly.
    SpaceBetween,
    /// The gap between the first and last items is exactly THE SAME as the gap between items.
    /// The gaps are distributed evenly
    SpaceEvenly,
    /// The gap between the first and last items is exactly HALF the gap between items.
    /// The gaps are distributed evenly in proportion to these ratios.
    SpaceAround,
}

impl Default for AlignContent {
    fn default() -> Self {
        Self::Stretch
    }
}

/// Sets the layout used for the children of this node
///
/// [`Display::Flex`] is the default value.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Display {
    /// The children will follow the flexbox layout algorithm
    Flex,
    /// The children will follow the CSS Grid layout algorithm
    Grid,
    /// The children will not be laid out, and will follow absolute positioning
    None,
}

impl Default for Display {
    fn default() -> Self {
        Self::Flex
    }
}

/// The direction of the flexbox layout main axis.
///
/// There are always two perpendicular layout axes: main (or primary) and cross (or secondary).
/// Adding items will cause them to be positioned adjacent to each other along the main axis.
/// By varying this value throughout your tree, you can create complex axis-aligned layouts.
///
/// Items are always aligned relative to the cross axis, and justified relative to the main axis.
///
/// The default behavior is [`FlexDirection::Row`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-direction-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FlexDirection {
    /// Defines +x as the main axis
    ///
    /// Items will be added from left to right in a row.
    Row,
    /// Defines +y as the main axis
    ///
    /// Items will be added from top to bottom in a column.
    Column,
    /// Defines -x as the main axis
    ///
    /// Items will be added from right to left in a row.
    RowReverse,
    /// Defines -y as the main axis
    ///
    /// Items will be added from bottom to top in a column.
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        Self::Row
    }
}

impl FlexDirection {
    #[inline]
    /// Is the direction [`FlexDirection::Row`] or [`FlexDirection::RowReverse`]?
    pub(crate) fn is_row(self) -> bool {
        matches!(self, Self::Row | Self::RowReverse)
    }

    #[inline]
    /// Is the direction [`FlexDirection::Column`] or [`FlexDirection::ColumnReverse`]?
    pub(crate) fn is_column(self) -> bool {
        matches!(self, Self::Column | Self::ColumnReverse)
    }

    #[inline]
    /// Is the direction [`FlexDirection::RowReverse`] or [`FlexDirection::ColumnReverse`]?
    pub(crate) fn is_reverse(self) -> bool {
        matches!(self, Self::RowReverse | Self::ColumnReverse)
    }
}

/// Sets the distribution of space between and around content items along the main-axis
///
/// The default value is [`JustifyContent::FlexStart`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#justify-content-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JustifyContent {
    /// Items are packed toward the start of the main axis
    FlexStart,
    /// Items are packed toward the end of the main axis
    FlexEnd,
    /// Items are packed along the center of the main axis
    Center,
    /// The first and last items are aligned flush with the edges of the container (no gap)
    /// The gaps between items are distributed evenly.
    SpaceBetween,
    /// The gap between the first and last items is exactly THE SAME as the gap between items.
    /// The gaps are distributed evenly
    SpaceEvenly,
    /// The gap between the first and last items is exactly HALF the gap between items.
    /// The gaps are distributed evenly in proportion to these ratios.
    SpaceAround,
}

impl Default for JustifyContent {
    fn default() -> Self {
        Self::FlexStart
    }
}

/// The positioning strategy for this item.
///
/// This controls both how the origin is determined for the [`Style::position`] field,
/// and whether or not the item will be controlled by flexbox's layout algorithm.
///
/// WARNING: this enum follows the behavior of [CSS's `position` property](https://developer.mozilla.org/en-US/docs/Web/CSS/position),
/// which can be unintuitive.
///
/// [`PositionType::Relative`] is the default value, in contrast to the default behavior in CSS.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PositionType {
    /// The offset is computed relative to the final position given by the layout algorithm.
    /// Offsets do not affect the position of any other items; they are effectively a correction factor applied at the end.
    Relative,
    /// The offset is computed relative to this item's closest positioned ancestor, if any.
    /// Otherwise, it is placed relative to the origin.
    /// No space is created for the item in the page layout, and its size will not be altered.
    ///
    /// WARNING: to opt-out of layouting entirely, you must use [`Display::None`] instead on your [`Style`] object.
    Absolute,
}

impl Default for PositionType {
    fn default() -> Self {
        Self::Relative
    }
}

/// Controls whether flex items are forced onto one line or can wrap onto multiple lines.
///
/// Defaults to [`FlexWrap::NoWrap`]
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FlexWrap {
    /// Items will not wrap and stay on a single line
    NoWrap,
    /// Items will wrap according to this item's [`FlexDirection`]
    Wrap,
    /// Items will wrap in the opposite direction to this item's [`FlexDirection`]
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> Self {
        Self::NoWrap
    }
}

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
    ///
    Dense,
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

/// A track placement specicification. Used for grid-[row/column]-[start/end]. Named tracks are not implemented.
///
/// Defaults to [`GridLine::Auto`]
///
/// [Specification](https://www.w3.org/TR/css3-grid-layout/#typedef-grid-row-start-grid-line)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GridLine {
    /// Place item according to the auto-placement algorithm, and the parent's grid_auto_flow property
    Auto,
    /// Place item at specified track (column or row) index
    Track(i16),
    /// Item should span specified number of tracks (columns or rows)
    Span(u16),
}

impl Default for GridLine {
    fn default() -> Self {
        Self::Auto
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LengthPercentage {
    /// Points are abstract absolute units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Points(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LengthPercentageAuto {
    /// Points are abstract absolute units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Points(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
    /// The dimension should be automatically computed
    Auto,
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Dimension {
    /// Points are abstract absolute units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Points(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
    /// The dimension should be automatically computed
    Auto,
}

impl Dimension {
    /// Is this value defined?
    pub(crate) fn is_defined(self) -> bool {
        matches!(self, Dimension::Points(_) | Dimension::Percent(_))
    }
}

impl Rect<Dimension> {
    /// Generates a [`Rect<Dimension>`] using [`Dimension::Points`] values for `start` and `top`
    #[must_use]
    pub const fn top_from_points(start: f32, top: f32) -> Rect<Dimension> {
        Rect { left: Dimension::Points(start), top: Dimension::Points(top), ..Rect::AUTO }
    }

    /// Generates a [`Rect<Dimension>`] using [`Dimension::Points`] values for `end` and `bottom`
    #[must_use]
    pub const fn bot_from_points(end: f32, bottom: f32) -> Rect<Dimension> {
        Rect { right: Dimension::Points(end), bottom: Dimension::Points(bottom), ..Rect::AUTO }
    }

    /// Generates a [`Rect<Dimension>`] using [`Dimension::Percent`] values for `start` and `top`
    #[must_use]
    pub const fn top_from_percent(start: f32, top: f32) -> Rect<Dimension> {
        Rect { left: Dimension::Percent(start), top: Dimension::Percent(top), ..Rect::AUTO }
    }

    /// Generates a [`Rect<Dimension>`] using [`Dimension::Percent`] values for `end` and `bottom`
    #[must_use]
    pub const fn bot_from_percent(end: f32, bottom: f32) -> Rect<Dimension> {
        Rect { right: Dimension::Percent(end), bottom: Dimension::Percent(bottom), ..Rect::AUTO }
    }

    /// Generates a [`Rect<Dimension>`] using [`Dimension::Auto`] for all values
    pub const AUTO: Rect<Dimension> =
        Self { left: Dimension::Auto, right: Dimension::Auto, top: Dimension::Auto, bottom: Dimension::Auto };

    /// Create a new Rect with [`Dimension::Points`]
    #[must_use]
    pub const fn from_points(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension::Points(start),
            right: Dimension::Points(end),
            top: Dimension::Points(top),
            bottom: Dimension::Points(bottom),
        }
    }

    /// Create a new Rect with [`Dimension::Percent`]
    #[must_use]
    pub const fn from_percent(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension::Percent(start),
            right: Dimension::Percent(end),
            top: Dimension::Percent(top),
            bottom: Dimension::Percent(bottom),
        }
    }
}

/// The flexbox layout information for a single [`Node`](crate::node::Node).
///
/// The most important idea in flexbox is the notion of a "main" and "cross" axis, which are always perpendicular to each other.
/// The orientation of these axes are controlled via the [`FlexDirection`] field of this struct.
///
/// This struct follows the [CSS equivalent](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout/Basic_Concepts_of_Flexbox) directly;
/// information about the behavior on the web should transfer directly.
///
/// Detailed information about the exact behavior of each of these fields
/// can be found on [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS) by searching for the field name.
/// The distinction between margin, padding and border is explained well in
/// this [introduction to the box model](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Box_Model/Introduction_to_the_CSS_box_model).
///
/// If the behavior does not match the flexbox layout algorithm on the web, please file a bug!
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Style {
    /// What layout strategy should be used?
    pub display: Display,

    // Position properties
    /// What should the `position` value of this struct use as a base offset?
    pub position_type: PositionType,
    /// How should the position of this element be tweaked relative to the layout defined?
    pub position: Rect<LengthPercentageAuto>,

    // Size properies
    /// Sets the initial size of the item
    pub size: Size<Dimension>,
    /// Controls the minimum size of the item
    pub min_size: Size<Dimension>,
    /// Controls the maximum size of the item
    pub max_size: Size<Dimension>,
    /// Sets the preferred aspect ratio for the item
    ///
    /// The ratio is calculated as width divided by height.
    pub aspect_ratio: Option<f32>,

    // Spacing Properties
    /// How large should the margin be on each side?
    pub margin: Rect<LengthPercentageAuto>,
    /// How large should the padding be on each side?
    pub padding: Rect<LengthPercentage>,
    /// How large should the border be on each side?
    pub border: Rect<LengthPercentage>,

    // Alignment properties
    /// How should items be aligned relative to the cross axis?
    pub align_items: AlignItems,
    /// Should this item violate the cross axis alignment specified by its parent's [`AlignItems`]?
    pub align_self: AlignSelf,
    /// How should content contained within this item be aligned relative to the cross axis?
    pub align_content: AlignContent,
    /// How should items be aligned relative to the main axis?
    pub justify_content: JustifyContent,
    /// How large should the gaps between items in a grid or flex container be?
    pub gap: Size<LengthPercentage>,

    // Flexbox properies
    /// Which direction does the main axis flow in?
    pub flex_direction: FlexDirection,
    /// Should elements wrap, or stay in a single line?
    pub flex_wrap: FlexWrap,
    /// Sets the initial main axis size of the item
    pub flex_basis: Dimension,
    /// The relative rate at which this item grows when it is expanding to fill space
    ///
    /// 0.0 is the default value, and this value must be positive.
    pub flex_grow: f32,
    /// The relative rate at which this item shrinks when it is contracting to fit into space
    ///
    /// 1.0 is the default value, and this value must be positive.
    pub flex_shrink: f32,

    // Grid container properies
    /// Defines the track sizing functions (widths) of the grid rows
    pub grid_template_rows: GridTrackVec<Dimension>,
    /// Defines the track sizing functions (heights) of the grid columns
    pub grid_template_columns: GridTrackVec<Dimension>,
    /// Defines the size of implicitly created rows
    pub grid_auto_rows: Dimension,
    /// Defined the size of implicitly created columns
    pub grid_auto_columns: Dimension,
    /// Controls how items get placed into the grid for auto-placed items
    pub grid_auto_flow: GridAutoFlow,

    // Grid child properties
    /// Defines which row in the grid the item should start at
    pub grid_row_start: GridLine,
    /// Defines which row in the grid the item should end at
    pub grid_row_end: GridLine,
    /// Defines which column in the grid the item should start at
    pub grid_column_start: GridLine,
    /// Defines which column in the grid the item should end at
    pub grid_column_end: GridLine,
}

impl Style {
    /// The [`Default`] layout, in a form that can be used in const functions
    pub const DEFAULT: Style = Style {
        display: Display::Flex,
        position_type: PositionType::Relative,
        flex_direction: FlexDirection::Row,
        flex_wrap: FlexWrap::NoWrap,
        align_items: AlignItems::Stretch,
        align_self: AlignSelf::Auto,
        align_content: AlignContent::Stretch,
        justify_content: JustifyContent::FlexStart,
        position: Rect::auto(),
        margin: Rect::zero(),
        padding: Rect::zero(),
        border: Rect::zero(),
        gap: Size::zero(),
        flex_grow: 0.0,
        flex_shrink: 1.0,
        flex_basis: Dimension::Auto,
        size: Size::auto(),
        min_size: Size::auto(),
        max_size: Size::auto(),
        aspect_ratio: None,
        grid_template_rows: Vec::new(),
        grid_template_columns: Vec::new(),
        grid_auto_rows: Dimension::Auto,
        grid_auto_columns: Dimension::Auto,
        grid_auto_flow: GridAutoFlow::Row,
        grid_row_start: GridLine::Auto,
        grid_row_end: GridLine::Auto,
        grid_column_start: GridLine::Auto,
        grid_column_end: GridLine::Auto,
    };
}

impl Default for Style {
    fn default() -> Self {
        Style::DEFAULT
    }
}

impl Style {
    /// If the `direction` is row-oriented, the min width. Otherwise the min height
    pub(crate) fn min_main_size(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.min_size.width
        } else {
            self.min_size.height
        }
    }

    /// If the `direction` is row-oriented, the max width. Otherwise the max height
    pub(crate) fn max_main_size(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.max_size.width
        } else {
            self.max_size.height
        }
    }

    /// If the `direction` is row-oriented, the margin start. Otherwise the margin top
    pub(crate) fn main_margin_start(&self, direction: FlexDirection) -> LengthPercentageAuto {
        if direction.is_row() {
            self.margin.left
        } else {
            self.margin.top
        }
    }

    /// If the `direction` is row-oriented, the margin end. Otherwise the margin bottom
    pub(crate) fn main_margin_end(&self, direction: FlexDirection) -> LengthPercentageAuto {
        if direction.is_row() {
            self.margin.right
        } else {
            self.margin.bottom
        }
    }

    /// If the `direction` is row-oriented, the height. Otherwise the width
    pub(crate) fn cross_size(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.size.height
        } else {
            self.size.width
        }
    }

    /// If the `direction` is row-oriented, the min height. Otherwise the min width
    pub(crate) fn min_cross_size(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.min_size.height
        } else {
            self.min_size.width
        }
    }

    /// If the `direction` is row-oriented, the max height. Otherwise the max width
    pub(crate) fn max_cross_size(&self, direction: FlexDirection) -> Dimension {
        if direction.is_row() {
            self.max_size.height
        } else {
            self.max_size.width
        }
    }

    /// If the `direction` is row-oriented, the margin top. Otherwise the margin start
    pub(crate) fn cross_margin_start(&self, direction: FlexDirection) -> LengthPercentageAuto {
        if direction.is_row() {
            self.margin.top
        } else {
            self.margin.left
        }
    }

    /// If the `direction` is row-oriented, the margin bottom. Otherwise the margin end
    pub(crate) fn cross_margin_end(&self, direction: FlexDirection) -> LengthPercentageAuto {
        if direction.is_row() {
            self.margin.bottom
        } else {
            self.margin.right
        }
    }

    /// Computes the final alignment of this item based on the parent's [`AlignItems`] and this item's [`AlignSelf`]
    pub(crate) fn align_self(&self, parent: &Style) -> AlignSelf {
        // FUTURE WARNING: This function should never return AlignSelf::Auto
        // See #169 https://github.com/DioxusLabs/taffy/pull/169#issuecomment-1157698840

        if self.align_self == AlignSelf::Auto {
            match parent.align_items {
                AlignItems::FlexStart => AlignSelf::FlexStart,
                AlignItems::FlexEnd => AlignSelf::FlexEnd,
                AlignItems::Center => AlignSelf::Center,
                AlignItems::Baseline => AlignSelf::Baseline,
                AlignItems::Stretch => AlignSelf::Stretch,
            }
        } else {
            self.align_self
        }
    }
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    use super::Style;
    use crate::geometry::{Rect, Size};

    #[test]
    fn defaults_match() {
        let old_defaults = Style {
            display: Default::default(),
            position_type: Default::default(),
            flex_direction: Default::default(),
            flex_wrap: Default::default(),
            align_items: Default::default(),
            align_self: Default::default(),
            align_content: Default::default(),
            justify_content: Default::default(),
            position: Rect::auto(),
            margin: Rect::zero(),
            padding: Rect::zero(),
            border: Rect::zero(),
            gap: Size::zero(),
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: super::Dimension::Auto,
            size: Size::auto(),
            min_size: Size::auto(),
            max_size: Size::auto(),
            aspect_ratio: Default::default(),
            grid_template_rows: Default::default(),
            grid_template_columns: Default::default(),
            grid_auto_rows: super::Dimension::Auto,
            grid_auto_columns: super::Dimension::Auto,
            grid_auto_flow: Default::default(),
            grid_row_start: Default::default(),
            grid_row_end: Default::default(),
            grid_column_start: Default::default(),
            grid_column_end: Default::default(),
        };

        assert_eq!(Style::DEFAULT, Style::default());
        assert_eq!(Style::DEFAULT, old_defaults);
    }

    mod test_flex_direction {
        use crate::style::*;

        #[test]
        fn flex_direction_is_row() {
            assert_eq!(FlexDirection::Row.is_row(), true);
            assert_eq!(FlexDirection::RowReverse.is_row(), true);
            assert_eq!(FlexDirection::Column.is_row(), false);
            assert_eq!(FlexDirection::ColumnReverse.is_row(), false);
        }

        #[test]
        fn flex_direction_is_column() {
            assert_eq!(FlexDirection::Row.is_column(), false);
            assert_eq!(FlexDirection::RowReverse.is_column(), false);
            assert_eq!(FlexDirection::Column.is_column(), true);
            assert_eq!(FlexDirection::ColumnReverse.is_column(), true);
        }

        #[test]
        fn flex_direction_is_reverse() {
            assert_eq!(FlexDirection::Row.is_reverse(), false);
            assert_eq!(FlexDirection::RowReverse.is_reverse(), true);
            assert_eq!(FlexDirection::Column.is_reverse(), false);
            assert_eq!(FlexDirection::ColumnReverse.is_reverse(), true);
        }
    }

    mod test_flexbox_layout {
        use crate::style::*;
        use crate::style_helpers::*;

        fn layout_from_align_items(align: AlignItems) -> Style {
            Style { align_items: align, ..Default::default() }
        }

        fn layout_from_align_self(align: AlignSelf) -> Style {
            Style { align_self: align, ..Default::default() }
        }

        #[test]
        fn flexbox_layout_min_main_size() {
            let layout = Style { min_size: Size::from_points(1.0, 2.0), ..Default::default() };
            assert_eq!(layout.min_main_size(FlexDirection::Row), Dimension::Points(1.0));
            assert_eq!(layout.min_main_size(FlexDirection::Column), Dimension::Points(2.0));
        }

        #[test]
        fn flexbox_layout_max_main_size() {
            let layout = Style { max_size: Size::from_points(1.0, 2.0), ..Default::default() };
            assert_eq!(layout.max_main_size(FlexDirection::Row), Dimension::Points(1.0));
            assert_eq!(layout.max_main_size(FlexDirection::Column), Dimension::Points(2.0));
        }

        #[test]
        fn flexbox_layout_main_margin_start() {
            let layout = Style {
                margin: Rect { top: points(1.0), bottom: auto(), left: points(2.0), right: auto() },
                ..Default::default()
            };
            assert_eq!(layout.main_margin_start(FlexDirection::Row), points(2.0));
            assert_eq!(layout.main_margin_start(FlexDirection::Column), points(1.0));
        }

        #[test]
        fn flexbox_layout_main_margin_end() {
            let layout = Style {
                margin: Rect { top: auto(), bottom: points(1.0), left: auto(), right: points(2.0) },
                ..Default::default()
            };
            assert_eq!(layout.main_margin_end(FlexDirection::Row), points(2.0));
            assert_eq!(layout.main_margin_end(FlexDirection::Column), points(1.0));
        }

        #[test]
        fn flexbox_layout_cross_size() {
            let layout = Style { size: Size::from_points(1.0, 2.0), ..Default::default() };
            assert_eq!(layout.cross_size(FlexDirection::Row), Dimension::Points(2.0));
            assert_eq!(layout.cross_size(FlexDirection::Column), Dimension::Points(1.0));
        }

        #[test]
        fn flexbox_layout_min_cross_size() {
            let layout = Style { min_size: Size::from_points(1.0, 2.0), ..Default::default() };
            assert_eq!(layout.min_cross_size(FlexDirection::Row), Dimension::Points(2.0));
            assert_eq!(layout.min_cross_size(FlexDirection::Column), Dimension::Points(1.0));
        }

        #[test]
        fn flexbox_layout_max_cross_size() {
            let layout = Style { max_size: Size::from_points(1.0, 2.0), ..Default::default() };
            assert_eq!(layout.max_cross_size(FlexDirection::Row), Dimension::Points(2.0));
            assert_eq!(layout.max_cross_size(FlexDirection::Column), Dimension::Points(1.0));
        }

        #[test]
        fn flexbox_layout_cross_margin_start() {
            let layout = Style {
                margin: Rect { top: points(1.0), bottom: auto(), left: points(2.0), right: auto() },
                ..Default::default()
            };
            assert_eq!(layout.cross_margin_start(FlexDirection::Row), points(1.0));
            assert_eq!(layout.cross_margin_start(FlexDirection::Column), points(2.0));
        }

        #[test]
        fn flexbox_layout_cross_margin_end() {
            let layout = Style {
                margin: Rect { top: auto(), bottom: points(1.0), left: auto(), right: points(2.0) },
                ..Default::default()
            };
            assert_eq!(layout.cross_margin_end(FlexDirection::Row), points(1.0));
            assert_eq!(layout.cross_margin_end(FlexDirection::Column), points(2.0));
        }

        #[test]
        fn flexbox_layout_align_self_auto() {
            let parent = layout_from_align_items(AlignItems::FlexStart);
            let layout = layout_from_align_self(AlignSelf::Auto);
            assert_eq!(layout.align_self(&parent), AlignSelf::FlexStart);

            let parent = layout_from_align_items(AlignItems::FlexEnd);
            let layout = layout_from_align_self(AlignSelf::Auto);
            assert_eq!(layout.align_self(&parent), AlignSelf::FlexEnd);

            let parent = layout_from_align_items(AlignItems::Center);
            let layout = layout_from_align_self(AlignSelf::Auto);
            assert_eq!(layout.align_self(&parent), AlignSelf::Center);

            let parent = layout_from_align_items(AlignItems::Baseline);
            let layout = layout_from_align_self(AlignSelf::Auto);
            assert_eq!(layout.align_self(&parent), AlignSelf::Baseline);

            let parent = layout_from_align_items(AlignItems::Stretch);
            let layout = layout_from_align_self(AlignSelf::Auto);
            assert_eq!(layout.align_self(&parent), AlignSelf::Stretch);
        }

        #[test]
        fn align_self() {
            let parent = layout_from_align_items(AlignItems::FlexEnd);
            let layout = layout_from_align_self(AlignSelf::FlexStart);
            assert_eq!(layout.align_self(&parent), AlignSelf::FlexStart);

            let parent = layout_from_align_items(AlignItems::FlexStart);
            let layout = layout_from_align_self(AlignSelf::FlexEnd);
            assert_eq!(layout.align_self(&parent), AlignSelf::FlexEnd);

            let parent = layout_from_align_items(AlignItems::FlexStart);
            let layout = layout_from_align_self(AlignSelf::Center);
            assert_eq!(layout.align_self(&parent), AlignSelf::Center);

            let parent = layout_from_align_items(AlignItems::FlexStart);
            let layout = layout_from_align_self(AlignSelf::Baseline);
            assert_eq!(layout.align_self(&parent), AlignSelf::Baseline);

            let parent = layout_from_align_items(AlignItems::FlexStart);
            let layout = layout_from_align_self(AlignSelf::Stretch);
            assert_eq!(layout.align_self(&parent), AlignSelf::Stretch);
        }
    }
}
