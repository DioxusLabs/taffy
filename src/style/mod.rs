//! A typed representation of [CSS style properties](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) in Rust. Used as input to layout computation.
mod alignment;
mod dimension;

#[cfg(feature = "flexbox")]
mod flex;

pub use self::alignment::{AlignContent, AlignItems, AlignSelf, JustifyContent, JustifyItems, JustifySelf};
pub use self::dimension::{AvailableSpace, Dimension, LengthPercentage, LengthPercentageAuto};

#[cfg(feature = "flexbox")]
pub use self::flex::{FlexDirection, FlexWrap, FlexboxContainerStyle, FlexboxItemStyle};

#[cfg(feature = "grid")]
mod grid;
#[cfg(feature = "grid")]
pub(crate) use self::grid::{GenericGridPlacement, OriginZeroGridPlacement};
#[cfg(feature = "grid")]
pub use self::grid::{
    GridAutoFlow, GridContainerStyle, GridItemStyle, GridPlacement, GridTrackRepetition, MaxTrackSizingFunction,
    MinTrackSizingFunction, NonRepeatedTrackSizingFunction, TrackSizingFunction,
};
use crate::geometry::{Point, Rect, Size};

#[cfg(feature = "grid")]
use crate::geometry::Line;
#[cfg(feature = "serde")]
use crate::style_helpers;
#[cfg(feature = "grid")]
use crate::util::sys::GridTrackVec;

/// The core set of styles that are shared between all CSS layout nodes
///
/// Note that all methods come with a default implementation which simply returns the default value for that style property
/// but this is a just a convenience to save on boilerplate for styles that your implementation doesn't support. You will need
/// to override the default implementation for each style property that your style type actually supports.
pub trait CoreStyle {
    /// Which box generation mode should be used
    #[inline(always)]
    fn box_generation_mode(&self) -> BoxGenerationMode {
        BoxGenerationMode::DEFAULT
    }

    // Overflow properties
    /// How children overflowing their container should affect layout
    #[inline(always)]
    fn overflow(&self) -> Point<Overflow> {
        Style::DEFAULT.overflow
    }
    /// How much space (in points) should be reserved for the scrollbars of `Overflow::Scroll` and `Overflow::Auto` nodes.
    #[inline(always)]
    fn scrollbar_width(&self) -> f32 {
        0.0
    }

    // Position properties
    /// What should the `position` value of this struct use as a base offset?
    #[inline(always)]
    fn position(&self) -> Position {
        Style::DEFAULT.position
    }
    /// How should the position of this element be tweaked relative to the layout defined?
    #[inline(always)]
    fn inset(&self) -> Rect<LengthPercentageAuto> {
        Style::DEFAULT.inset
    }

    // Size properies
    /// Sets the initial size of the item
    #[inline(always)]
    fn size(&self) -> Size<Dimension> {
        Style::DEFAULT.size
    }
    /// Controls the minimum size of the item
    #[inline(always)]
    fn min_size(&self) -> Size<Dimension> {
        Style::DEFAULT.min_size
    }
    /// Controls the maximum size of the item
    #[inline(always)]
    fn max_size(&self) -> Size<Dimension> {
        Style::DEFAULT.max_size
    }
    /// Sets the preferred aspect ratio for the item
    /// The ratio is calculated as width divided by height.
    #[inline(always)]
    fn aspect_ratio(&self) -> Option<f32> {
        Style::DEFAULT.aspect_ratio
    }

    // Spacing Properties
    /// How large should the margin be on each side?
    #[inline(always)]
    fn margin(&self) -> Rect<LengthPercentageAuto> {
        Style::DEFAULT.margin
    }
    /// How large should the padding be on each side?
    #[inline(always)]
    fn padding(&self) -> Rect<LengthPercentage> {
        Style::DEFAULT.padding
    }
    /// How large should the border be on each side?
    #[inline(always)]
    fn border(&self) -> Rect<LengthPercentage> {
        Style::DEFAULT.border
    }
}

/// Sets the layout used for the children of this node
///
/// The default values depends on on which feature flags are enabled. The order of precedence is: Flex, Grid, Block, None.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Display {
    /// The children will follow the block layout algorithm
    #[cfg(feature = "block_layout")]
    Block,
    /// The children will follow the flexbox layout algorithm
    #[cfg(feature = "flexbox")]
    Flex,
    /// The children will follow the CSS Grid layout algorithm
    #[cfg(feature = "grid")]
    Grid,
    /// The node is hidden, and it's children will also be hidden
    None,
}

impl Display {
    /// The default Display mode
    #[cfg(feature = "flexbox")]
    pub const DEFAULT: Display = Display::Flex;

    /// The default Display mode
    #[cfg(all(feature = "grid", not(feature = "flexbox")))]
    pub const DEFAULT: Display = Display::Grid;

    /// The default Display mode
    #[cfg(all(feature = "block_layout", not(feature = "flexbox"), not(feature = "grid")))]
    pub const DEFAULT: Display = Display::Block;

    /// The default Display mode
    #[cfg(all(not(feature = "flexbox"), not(feature = "grid"), not(feature = "block_layout")))]
    pub const DEFAULT: Display = Display::None;
}

impl Default for Display {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl core::fmt::Display for Display {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Display::None => write!(f, "NONE"),
            #[cfg(feature = "block_layout")]
            Display::Block => write!(f, "BLOCK"),
            #[cfg(feature = "flexbox")]
            Display::Flex => write!(f, "FLEX"),
            #[cfg(feature = "grid")]
            Display::Grid => write!(f, "GRID"),
        }
    }
}

/// An abstracted version of the CSS `display` property where any value other than "none" is represented by "normal"
/// See: <https://www.w3.org/TR/css-display-3/#box-generation>
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BoxGenerationMode {
    /// The node generates a box in the regular way
    Normal,
    /// The node and it's descendants generate no boxes (they are hidden)
    None,
}

impl BoxGenerationMode {
    /// The default of BoxGenerationMode
    pub const DEFAULT: BoxGenerationMode = BoxGenerationMode::Normal;
}

impl Default for BoxGenerationMode {
    fn default() -> Self {
        Self::DEFAULT
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
/// [`Position::Relative`] is the default value, in contrast to the default behavior in CSS.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Position {
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

impl Default for Position {
    fn default() -> Self {
        Self::Relative
    }
}

/// How children overflowing their container should affect layout
///
/// In CSS the primary effect of this property is to control whether contents of a parent container that overflow that container should
/// be displayed anyway, be clipped, or trigger the container to become a scroll container. However it also has secondary effects on layout,
/// the main ones being:
///
///   - The automatic minimum size Flexbox/CSS Grid items with non-`Visible` overflow is `0` rather than being content based
///   - `Overflow::Scroll` nodes have space in the layout reserved for a scrollbar (width controlled by the `scrollbar_width` property)
///
/// In Taffy, we only implement the layout related secondary effects as we are not concerned with drawing/painting. The amount of space reserved for
/// a scrollbar is controlled by the `scrollbar_width` property. If this is `0` then `Scroll` behaves identically to `Hidden`.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Overflow {
    /// The automatic minimum size of this node as a flexbox/grid item should be based on the size of it's content.
    #[default]
    Visible,
    /// The automatic minimum size of this node as a flexbox/grid item should be `0`.
    Hidden,
    /// The automatic minimum size of this node as a flexbox/grid item should be `0`. Additionally, space should be reserved
    /// for a scrollbar. The amount of space reserved is controlled by the `scrollbar_width` property.
    Scroll,
}

impl Overflow {
    /// Returns true for overflow modes that contain their contents (`Overflow::Hidden`, `Overflow::Scroll`, `Overflow::Auto`)
    /// or else false for overflow modes that allow their contains to spill (`Overflow::Visible`).
    #[inline(always)]
    pub(crate) fn is_scroll_container(self) -> bool {
        self != Overflow::Visible
    }

    /// Returns `Some(0.0)` if the overflow mode would cause the automatic minimum size of a Flexbox or CSS Grid item
    /// to be `0`. Else returns None.
    #[inline(always)]
    pub(crate) fn maybe_into_automatic_min_size(self) -> Option<f32> {
        match self.is_scroll_container() {
            true => Some(0.0),
            false => None,
        }
    }
}

/// The flexbox layout information for a single node.
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

    // Overflow properties
    /// How children overflowing their container should affect layout
    pub overflow: Point<Overflow>,
    /// How much space (in points) should be reserved for the scrollbars of `Overflow::Scroll` and `Overflow::Auto` nodes.
    pub scrollbar_width: f32,

    // Position properties
    /// What should the `position` value of this struct use as a base offset?
    pub position: Position,
    /// How should the position of this element be tweaked relative to the layout defined?
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::auto"))]
    pub inset: Rect<LengthPercentageAuto>,

    // Size properies
    /// Sets the initial size of the item
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::auto"))]
    pub size: Size<Dimension>,
    /// Controls the minimum size of the item
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::auto"))]
    pub min_size: Size<Dimension>,
    /// Controls the maximum size of the item
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::auto"))]
    pub max_size: Size<Dimension>,
    /// Sets the preferred aspect ratio for the item
    ///
    /// The ratio is calculated as width divided by height.
    pub aspect_ratio: Option<f32>,

    // Spacing Properties
    /// How large should the margin be on each side?
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::zero"))]
    pub margin: Rect<LengthPercentageAuto>,
    /// How large should the padding be on each side?
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::zero"))]
    pub padding: Rect<LengthPercentage>,
    /// How large should the border be on each side?
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::zero"))]
    pub border: Rect<LengthPercentage>,

    // Alignment properties
    /// How this node's children aligned in the cross/block axis?
    #[cfg(any(feature = "flexbox", feature = "grid"))]
    pub align_items: Option<AlignItems>,
    /// How this node should be aligned in the cross/block axis
    /// Falls back to the parents [`AlignItems`] if not set
    #[cfg(any(feature = "flexbox", feature = "grid"))]
    pub align_self: Option<AlignSelf>,
    /// How this node's children should be aligned in the inline axis
    #[cfg(feature = "grid")]
    pub justify_items: Option<AlignItems>,
    /// How this node should be aligned in the inline axis
    /// Falls back to the parents [`JustifyItems`] if not set
    #[cfg(feature = "grid")]
    pub justify_self: Option<AlignSelf>,
    /// How should content contained within this item be aligned in the cross/block axis
    #[cfg(any(feature = "flexbox", feature = "grid"))]
    pub align_content: Option<AlignContent>,
    /// How should contained within this item be aligned in the main/inline axis
    #[cfg(any(feature = "flexbox", feature = "grid"))]
    pub justify_content: Option<JustifyContent>,
    /// How large should the gaps between items in a grid or flex container be?
    #[cfg(any(feature = "flexbox", feature = "grid"))]
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::zero"))]
    pub gap: Size<LengthPercentage>,

    // Flexbox properies
    /// Which direction does the main axis flow in?
    #[cfg(feature = "flexbox")]
    pub flex_direction: FlexDirection,
    /// Should elements wrap, or stay in a single line?
    #[cfg(feature = "flexbox")]
    pub flex_wrap: FlexWrap,
    /// Sets the initial main axis size of the item
    #[cfg(feature = "flexbox")]
    pub flex_basis: Dimension,
    /// The relative rate at which this item grows when it is expanding to fill space
    ///
    /// 0.0 is the default value, and this value must be positive.
    #[cfg(feature = "flexbox")]
    pub flex_grow: f32,
    /// The relative rate at which this item shrinks when it is contracting to fit into space
    ///
    /// 1.0 is the default value, and this value must be positive.
    #[cfg(feature = "flexbox")]
    pub flex_shrink: f32,

    // Grid container properies
    /// Defines the track sizing functions (heights) of the grid rows
    #[cfg(feature = "grid")]
    pub grid_template_rows: GridTrackVec<TrackSizingFunction>,
    /// Defines the track sizing functions (widths) of the grid columns
    #[cfg(feature = "grid")]
    pub grid_template_columns: GridTrackVec<TrackSizingFunction>,
    /// Defines the size of implicitly created rows
    #[cfg(feature = "grid")]
    pub grid_auto_rows: GridTrackVec<NonRepeatedTrackSizingFunction>,
    /// Defined the size of implicitly created columns
    #[cfg(feature = "grid")]
    pub grid_auto_columns: GridTrackVec<NonRepeatedTrackSizingFunction>,
    /// Controls how items get placed into the grid for auto-placed items
    #[cfg(feature = "grid")]
    pub grid_auto_flow: GridAutoFlow,

    // Grid child properties
    /// Defines which row in the grid the item should start and end at
    #[cfg(feature = "grid")]
    pub grid_row: Line<GridPlacement>,
    /// Defines which column in the grid the item should start and end at
    #[cfg(feature = "grid")]
    pub grid_column: Line<GridPlacement>,
}

impl Style {
    /// The [`Default`] layout, in a form that can be used in const functions
    pub const DEFAULT: Style = Style {
        display: Display::DEFAULT,
        overflow: Point { x: Overflow::Visible, y: Overflow::Visible },
        scrollbar_width: 0.0,
        position: Position::Relative,
        inset: Rect::auto(),
        margin: Rect::zero(),
        padding: Rect::zero(),
        border: Rect::zero(),
        size: Size::auto(),
        min_size: Size::auto(),
        max_size: Size::auto(),
        aspect_ratio: None,
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        gap: Size::zero(),
        // Aligment
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        align_items: None,
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        align_self: None,
        #[cfg(feature = "grid")]
        justify_items: None,
        #[cfg(feature = "grid")]
        justify_self: None,
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        align_content: None,
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        justify_content: None,
        // Flexbox
        #[cfg(feature = "flexbox")]
        flex_direction: FlexDirection::Row,
        #[cfg(feature = "flexbox")]
        flex_wrap: FlexWrap::NoWrap,
        #[cfg(feature = "flexbox")]
        flex_grow: 0.0,
        #[cfg(feature = "flexbox")]
        flex_shrink: 1.0,
        #[cfg(feature = "flexbox")]
        flex_basis: Dimension::Auto,
        // Grid
        #[cfg(feature = "grid")]
        grid_template_rows: GridTrackVec::new(),
        #[cfg(feature = "grid")]
        grid_template_columns: GridTrackVec::new(),
        #[cfg(feature = "grid")]
        grid_auto_rows: GridTrackVec::new(),
        #[cfg(feature = "grid")]
        grid_auto_columns: GridTrackVec::new(),
        #[cfg(feature = "grid")]
        grid_auto_flow: GridAutoFlow::Row,
        #[cfg(feature = "grid")]
        grid_row: Line { start: GridPlacement::Auto, end: GridPlacement::Auto },
        #[cfg(feature = "grid")]
        grid_column: Line { start: GridPlacement::Auto, end: GridPlacement::Auto },
    };
}

impl Default for Style {
    fn default() -> Self {
        Style::DEFAULT
    }
}

impl CoreStyle for Style {
    #[inline(always)]
    fn box_generation_mode(&self) -> BoxGenerationMode {
        match self.display {
            Display::None => BoxGenerationMode::None,
            _ => BoxGenerationMode::Normal,
        }
    }
    #[inline(always)]
    fn overflow(&self) -> Point<Overflow> {
        self.overflow
    }
    #[inline(always)]
    fn scrollbar_width(&self) -> f32 {
        self.scrollbar_width
    }
    #[inline(always)]
    fn position(&self) -> Position {
        self.position
    }
    #[inline(always)]
    fn inset(&self) -> Rect<LengthPercentageAuto> {
        self.inset
    }
    #[inline(always)]
    fn size(&self) -> Size<Dimension> {
        self.size
    }
    #[inline(always)]
    fn min_size(&self) -> Size<Dimension> {
        self.min_size
    }
    #[inline(always)]
    fn max_size(&self) -> Size<Dimension> {
        self.max_size
    }
    #[inline(always)]
    fn aspect_ratio(&self) -> Option<f32> {
        self.aspect_ratio
    }
    #[inline(always)]
    fn margin(&self) -> Rect<LengthPercentageAuto> {
        self.margin
    }
    #[inline(always)]
    fn padding(&self) -> Rect<LengthPercentage> {
        self.padding
    }
    #[inline(always)]
    fn border(&self) -> Rect<LengthPercentage> {
        self.border
    }
}

#[cfg(feature = "flexbox")]
impl FlexboxContainerStyle for Style {
    #[inline(always)]
    fn flex_direction(&self) -> FlexDirection {
        self.flex_direction
    }
    #[inline(always)]
    fn flex_wrap(&self) -> FlexWrap {
        self.flex_wrap
    }
    #[inline(always)]
    fn gap(&self) -> Size<LengthPercentage> {
        self.gap
    }
    #[inline(always)]
    fn align_content(&self) -> Option<AlignContent> {
        self.align_content
    }
    #[inline(always)]
    fn align_items(&self) -> Option<AlignItems> {
        self.align_items
    }
    #[inline(always)]
    fn justify_content(&self) -> Option<JustifyContent> {
        self.justify_content
    }
}
#[cfg(feature = "flexbox")]
impl FlexboxItemStyle for Style {
    #[inline(always)]
    fn flex_basis(&self) -> Dimension {
        self.flex_basis
    }
    #[inline(always)]
    fn flex_grow(&self) -> f32 {
        self.flex_grow
    }
    #[inline(always)]
    fn flex_shrink(&self) -> f32 {
        self.flex_shrink
    }
    #[inline(always)]
    fn align_self(&self) -> Option<AlignSelf> {
        self.align_self
    }
}

#[cfg(feature = "grid")]
impl GridContainerStyle for Style {
    #[inline(always)]
    fn grid_template_rows(&self) -> &[TrackSizingFunction] {
        &self.grid_template_rows
    }
    #[inline(always)]
    fn grid_template_columns(&self) -> &[TrackSizingFunction] {
        &self.grid_template_columns
    }
    #[inline(always)]
    fn grid_auto_rows(&self) -> &[NonRepeatedTrackSizingFunction] {
        &self.grid_auto_rows
    }
    #[inline(always)]
    fn grid_auto_columns(&self) -> &[NonRepeatedTrackSizingFunction] {
        &self.grid_auto_columns
    }
    #[inline(always)]
    fn grid_auto_flow(&self) -> GridAutoFlow {
        self.grid_auto_flow
    }
    #[inline(always)]
    fn gap(&self) -> Size<LengthPercentage> {
        self.gap
    }
    #[inline(always)]
    fn align_content(&self) -> Option<AlignContent> {
        self.align_content
    }
    #[inline(always)]
    fn justify_content(&self) -> Option<JustifyContent> {
        self.justify_content
    }
    #[inline(always)]
    fn align_items(&self) -> Option<AlignItems> {
        self.align_items
    }
    #[inline(always)]
    fn justify_items(&self) -> Option<AlignItems> {
        self.justify_items
    }
}
#[cfg(feature = "grid")]
impl GridItemStyle for Style {
    #[inline(always)]
    fn grid_row(&self) -> Line<GridPlacement> {
        self.grid_row
    }
    #[inline(always)]
    fn grid_column(&self) -> Line<GridPlacement> {
        self.grid_column
    }
    #[inline(always)]
    fn align_self(&self) -> Option<AlignSelf> {
        self.align_self
    }
    #[inline(always)]
    fn justify_self(&self) -> Option<AlignSelf> {
        self.justify_self
    }
}

#[cfg(test)]
mod tests {
    use super::Style;
    use crate::geometry::*;

    #[test]
    fn defaults_match() {
        #[cfg(feature = "grid")]
        use super::GridPlacement;

        let old_defaults = Style {
            display: Default::default(),
            overflow: Default::default(),
            scrollbar_width: 0.0,
            position: Default::default(),
            #[cfg(feature = "flexbox")]
            flex_direction: Default::default(),
            #[cfg(feature = "flexbox")]
            flex_wrap: Default::default(),
            #[cfg(any(feature = "flexbox", feature = "grid"))]
            align_items: Default::default(),
            #[cfg(any(feature = "flexbox", feature = "grid"))]
            align_self: Default::default(),
            #[cfg(feature = "grid")]
            justify_items: Default::default(),
            #[cfg(feature = "grid")]
            justify_self: Default::default(),
            #[cfg(any(feature = "flexbox", feature = "grid"))]
            align_content: Default::default(),
            #[cfg(any(feature = "flexbox", feature = "grid"))]
            justify_content: Default::default(),
            inset: Rect::auto(),
            margin: Rect::zero(),
            padding: Rect::zero(),
            border: Rect::zero(),
            gap: Size::zero(),
            #[cfg(feature = "flexbox")]
            flex_grow: 0.0,
            #[cfg(feature = "flexbox")]
            flex_shrink: 1.0,
            #[cfg(feature = "flexbox")]
            flex_basis: super::Dimension::Auto,
            size: Size::auto(),
            min_size: Size::auto(),
            max_size: Size::auto(),
            aspect_ratio: Default::default(),
            #[cfg(feature = "grid")]
            grid_template_rows: Default::default(),
            #[cfg(feature = "grid")]
            grid_template_columns: Default::default(),
            #[cfg(feature = "grid")]
            grid_auto_rows: Default::default(),
            #[cfg(feature = "grid")]
            grid_auto_columns: Default::default(),
            #[cfg(feature = "grid")]
            grid_auto_flow: Default::default(),
            #[cfg(feature = "grid")]
            grid_row: Line { start: GridPlacement::Auto, end: GridPlacement::Auto },
            #[cfg(feature = "grid")]
            grid_column: Line { start: GridPlacement::Auto, end: GridPlacement::Auto },
        };

        assert_eq!(Style::DEFAULT, Style::default());
        assert_eq!(Style::DEFAULT, old_defaults);
    }

    // NOTE: Please feel free the update the sizes in this test as required. This test is here to prevent unintentional size changes
    // and to serve as accurate up-to-date documentation on the sizes.
    #[test]
    fn style_sizes() {
        use super::*;

        fn assert_type_size<T>(expected_size: usize) {
            let name = ::core::any::type_name::<T>();
            let name = name.replace("taffy::geometry::", "");
            let name = name.replace("taffy::style::dimension::", "");
            let name = name.replace("taffy::style::alignment::", "");
            let name = name.replace("taffy::style::flex::", "");
            let name = name.replace("taffy::style::grid::", "");

            assert_eq!(
                ::core::mem::size_of::<T>(),
                expected_size,
                "Expected {} for be {} byte(s) but it was {} byte(s)",
                name,
                expected_size,
                ::core::mem::size_of::<T>(),
            );
        }

        // Display and Position
        assert_type_size::<Display>(1);
        assert_type_size::<Position>(1);
        assert_type_size::<Overflow>(1);

        // Dimensions and aggregations of Dimensions
        assert_type_size::<f32>(4);
        assert_type_size::<LengthPercentage>(8);
        assert_type_size::<LengthPercentageAuto>(8);
        assert_type_size::<Dimension>(8);
        assert_type_size::<Size<LengthPercentage>>(16);
        assert_type_size::<Size<LengthPercentageAuto>>(16);
        assert_type_size::<Size<Dimension>>(16);
        assert_type_size::<Rect<LengthPercentage>>(32);
        assert_type_size::<Rect<LengthPercentageAuto>>(32);
        assert_type_size::<Rect<Dimension>>(32);

        // Alignment
        assert_type_size::<AlignContent>(1);
        assert_type_size::<AlignItems>(1);
        assert_type_size::<Option<AlignItems>>(1);

        // Flexbox Container
        assert_type_size::<FlexDirection>(1);
        assert_type_size::<FlexWrap>(1);

        // CSS Grid Container
        assert_type_size::<GridAutoFlow>(1);
        assert_type_size::<MinTrackSizingFunction>(8);
        assert_type_size::<MaxTrackSizingFunction>(12);
        assert_type_size::<NonRepeatedTrackSizingFunction>(20);
        assert_type_size::<TrackSizingFunction>(32);
        assert_type_size::<Vec<NonRepeatedTrackSizingFunction>>(24);
        assert_type_size::<Vec<TrackSizingFunction>>(24);

        // CSS Grid Item
        assert_type_size::<GridPlacement>(4);
        assert_type_size::<Line<GridPlacement>>(8);

        // Overall
        assert_type_size::<Style>(352);
    }
}
