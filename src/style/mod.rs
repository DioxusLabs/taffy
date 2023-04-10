//! A representation of [CSS layout properties](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) in Rust, used for flexbox layout
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
use crate::geometry::{Rect, Size};

#[cfg(feature = "grid")]
use crate::geometry::Line;
#[cfg(feature = "serde")]
use crate::style_helpers;
#[cfg(feature = "grid")]
use crate::sys::GridTrackVec;

/// Sets the layout used for the children of this node
///
/// The default values depends on on which feature flags are enabled. The order of precedence is: Flex, Grid, None.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Display {
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
    #[cfg(all(not(feature = "flexbox"), not(feature = "grid")))]
    pub const DEFAULT: Display = Display::None;
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
    #[cfg(any(feature = "flexbox", feature = "grid"))]
    pub justify_items: Option<AlignItems>,
    /// How this node should be aligned in the inline axis
    /// Falls back to the parents [`JustifyItems`] if not set
    #[cfg(any(feature = "flexbox", feature = "grid"))]
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
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        justify_items: None,
        #[cfg(any(feature = "flexbox", feature = "grid"))]
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
            position: Default::default(),
            #[cfg(feature = "flexbox")]
            flex_direction: Default::default(),
            #[cfg(feature = "flexbox")]
            flex_wrap: Default::default(),
            align_items: Default::default(),
            align_self: Default::default(),
            #[cfg(feature = "grid")]
            justify_items: Default::default(),
            justify_self: Default::default(),
            align_content: Default::default(),
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
        assert_type_size::<Style>(344);
    }
}
