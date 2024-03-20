//! A typed representation of [CSS style properties](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) in Rust. Used as input to layout computation.
mod alignment;
mod dimension;

#[cfg(feature = "flexbox")]
mod flex;

pub use self::alignment::{AlignContent, AlignItems, AlignSelf, JustifyContent, JustifyItems, JustifySelf};
pub use self::dimension::{AvailableSpace, Dimension, LengthPercentage, LengthPercentageAuto};

#[cfg(feature = "flexbox")]
pub use self::flex::{FlexDirection, FlexWrap};

#[cfg(feature = "grid")]
mod grid;
#[cfg(feature = "grid")]
pub(crate) use self::grid::{GenericGridPlacement, OriginZeroGridPlacement};
#[cfg(feature = "grid")]
pub use self::grid::{
    GridAutoFlow, GridPlacement, GridTrackRepetition, MaxTrackSizingFunction, MinTrackSizingFunction,
    NonRepeatedTrackSizingFunction, TrackSizingFunction,
};
use crate::geometry::{Point, Rect, Size};

#[cfg(feature = "grid")]
use crate::geometry::Line;
#[cfg(feature = "serde")]
use crate::style_helpers;
#[cfg(feature = "grid")]
use crate::util::sys::GridTrackVec;

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
    /// The children will not be laid out, and will follow absolute positioning
    None,
}

impl Display {
    /// The default of Display.
    #[cfg(feature = "flexbox")]
    pub const DEFAULT: Display = Display::Flex;

    /// The default of Display.
    #[cfg(all(feature = "grid", not(feature = "flexbox")))]
    pub const DEFAULT: Display = Display::Grid;

    /// The default of Display.
    #[cfg(all(feature = "block_layout", not(feature = "flexbox"), not(feature = "grid")))]
    pub const DEFAULT: Display = Display::Block;

    /// The default of Display.
    #[cfg(all(not(feature = "flexbox"), not(feature = "grid"), not(feature = "block_layout")))]
    pub const DEFAULT: Display = Display::None;
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

impl Default for Display {
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
    /// The automatic minimum size of this node as a flexbox/grid item should be based on the size of its content.
    /// Content that overflows this node *should* contribute to the scroll region of its parent.
    #[default]
    Visible,
    /// The automatic minimum size of this node as a flexbox/grid item should be based on the size of its content.
    /// Content that overflows this node should *not* contribute to the scroll region of its parent.
    Clip,
    /// The automatic minimum size of this node as a flexbox/grid item should be `0`.
    /// Content that overflows this node should *not* contribute to the scroll region of its parent.
    Hidden,
    /// The automatic minimum size of this node as a flexbox/grid item should be `0`. Additionally, space should be reserved
    /// for a scrollbar. The amount of space reserved is controlled by the `scrollbar_width` property.
    /// Content that overflows this node should *not* contribute to the scroll region of its parent.
    Scroll,
}

impl Overflow {
    /// Returns true for overflow modes that contain their contents (`Overflow::Hidden`, `Overflow::Scroll`, `Overflow::Auto`)
    /// or else false for overflow modes that allow their contains to spill (`Overflow::Visible`).
    #[inline(always)]
    pub(crate) fn is_scroll_container(self) -> bool {
        match self {
            Self::Visible | Self::Clip => false,
            Self::Hidden | Self::Scroll => true,
        }
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

/// A typed representation of the CSS style information for a single node.
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

    // Size properties
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

    // Flexbox properties
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

    // Grid container properties
    /// Defines the track sizing functions (widths) of the grid rows
    #[cfg(feature = "grid")]
    pub grid_template_rows: GridTrackVec<TrackSizingFunction>,
    /// Defines the track sizing functions (heights) of the grid columns
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
        // Alignment
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

        fn assert_type_size_and_align<T>(expected_size: usize, expected_align: usize) {
            let name = ::core::any::type_name::<T>();
            let name = name.replace("taffy::geometry::", "");
            let name = name.replace("taffy::style::dimension::", "");
            let name = name.replace("taffy::style::alignment::", "");
            let name = name.replace("taffy::style::flex::", "");
            let name = name.replace("taffy::style::grid::", "");
            let name = name.replace("alloc::vec::", "");
            let name = name.replace("taffy::compute::grid::types::coordinates::", "");

            assert_eq!(
                ::core::mem::size_of::<T>(),
                expected_size,
                "Expected size_of {} to be {} byte(s) but it was {} byte(s)",
                name,
                expected_size,
                ::core::mem::size_of::<T>(),
            );

            assert_eq!(
                ::core::mem::align_of::<T>(),
                expected_align,
                "Expected align_of {} to be {} byte(s) but it was {} byte(s)",
                name,
                expected_align,
                ::core::mem::align_of::<T>(),
            );
        }

        // Display and Position
        assert_type_size_and_align::<Display>(1, 1);
        assert_type_size_and_align::<Position>(1, 1);
        assert_type_size_and_align::<Overflow>(1, 1);

        // Dimensions and aggregations of Dimensions
        assert_type_size_and_align::<f32>(4, 4);
        assert_type_size_and_align::<LengthPercentage>(8, 4);
        assert_type_size_and_align::<LengthPercentageAuto>(8, 4);
        assert_type_size_and_align::<Dimension>(8, 4);
        assert_type_size_and_align::<Size<LengthPercentage>>(16, 4);
        assert_type_size_and_align::<Size<LengthPercentageAuto>>(16, 4);
        assert_type_size_and_align::<Size<Dimension>>(16, 4);
        assert_type_size_and_align::<Rect<LengthPercentage>>(32, 4);
        assert_type_size_and_align::<Rect<LengthPercentageAuto>>(32, 4);
        assert_type_size_and_align::<Rect<Dimension>>(32, 4);

        // Alignment
        assert_type_size_and_align::<AlignContent>(1, 1);
        assert_type_size_and_align::<AlignItems>(1, 1);
        assert_type_size_and_align::<Option<AlignItems>>(1, 1);

        // Flexbox Container
        assert_type_size_and_align::<FlexDirection>(1, 1);
        assert_type_size_and_align::<FlexWrap>(1, 1);

        // CSS Grid Container
        assert_type_size_and_align::<GridAutoFlow>(1, 1);
        assert_type_size_and_align::<MinTrackSizingFunction>(8, 4);
        assert_type_size_and_align::<MaxTrackSizingFunction>(12, 4);
        assert_type_size_and_align::<NonRepeatedTrackSizingFunction>(20, 4);
        assert_type_size_and_align::<TrackSizingFunction>(32, 8);
        assert_type_size_and_align::<Vec<NonRepeatedTrackSizingFunction>>(24, 8);
        assert_type_size_and_align::<Vec<TrackSizingFunction>>(24, 8);

        // CSS Grid Item
        assert_type_size_and_align::<GridPlacement>(4, 2);
        assert_type_size_and_align::<Line<GridPlacement>>(8, 2);

        // Overall
        assert_type_size_and_align::<Style>(352, 8);
    }
}
