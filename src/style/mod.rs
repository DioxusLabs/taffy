//! A representation of [CSS layout properties](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) in Rust, used for flexbox layout
mod alignment;
mod dimension;
mod flex;

pub use self::alignment::{AlignContent, AlignItems, AlignSelf, JustifyContent, JustifyItems, JustifySelf};
pub use self::dimension::{AvailableSpace, Dimension, LengthPercentage, LengthPercentageAuto};
pub use self::flex::{FlexDirection, FlexWrap};

#[cfg(feature = "experimental_grid")]
mod grid;
#[cfg(feature = "experimental_grid")]
pub use self::grid::{
    GridAutoFlow, GridPlacement, GridTrackRepetition, MaxTrackSizingFunction, MinTrackSizingFunction,
    NonRepeatedTrackSizingFunction, TrackSizingFunction,
};
use crate::geometry::{Rect, Size};

#[cfg(feature = "experimental_grid")]
use crate::geometry::Line;
#[cfg(feature = "experimental_grid")]
use crate::sys::GridTrackVec;

/// Sets the layout used for the children of this node
///
/// [`Display::Flex`] is the default value.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Display {
    /// The children will follow the flexbox layout algorithm
    Flex,
    /// The children will follow the CSS Grid layout algorithm
    #[cfg(feature = "experimental_grid")]
    Grid,
    /// The children will not be laid out, and will follow absolute positioning
    None,
}

impl Default for Display {
    fn default() -> Self {
        Self::Flex
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
    /// How this node's children aligned in the cross/block axis?
    pub align_items: Option<AlignItems>,
    /// How this node should be aligned in the cross/block axis
    /// Falls back to the parents [`AlignItems`] if not set
    pub align_self: Option<AlignSelf>,
    /// How this node's children should be aligned in the inline axis
    #[cfg(feature = "experimental_grid")]
    pub justify_items: Option<AlignItems>,
    /// How this node should be aligned in the inline axis
    /// Falls back to the parents [`JustifyItems`] if not set
    pub justify_self: Option<AlignSelf>,
    /// How should content contained within this item be aligned in the cross/block axis
    pub align_content: Option<AlignContent>,
    /// How should contained within this item be aligned in the main/inline axis
    pub justify_content: Option<JustifyContent>,
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
    #[cfg(feature = "experimental_grid")]
    pub grid_template_rows: GridTrackVec<TrackSizingFunction>,
    /// Defines the track sizing functions (heights) of the grid columns
    #[cfg(feature = "experimental_grid")]
    pub grid_template_columns: GridTrackVec<TrackSizingFunction>,
    /// Defines the size of implicitly created rows
    #[cfg(feature = "experimental_grid")]
    pub grid_auto_rows: GridTrackVec<NonRepeatedTrackSizingFunction>,
    /// Defined the size of implicitly created columns
    #[cfg(feature = "experimental_grid")]
    pub grid_auto_columns: GridTrackVec<NonRepeatedTrackSizingFunction>,
    /// Controls how items get placed into the grid for auto-placed items
    #[cfg(feature = "experimental_grid")]
    pub grid_auto_flow: GridAutoFlow,

    // Grid child properties
    /// Defines which row in the grid the item should start and end at
    #[cfg(feature = "experimental_grid")]
    pub grid_row: Line<GridPlacement>,
    /// Defines which column in the grid the item should start and end at
    #[cfg(feature = "experimental_grid")]
    pub grid_column: Line<GridPlacement>,
}

impl Style {
    /// The [`Default`] layout, in a form that can be used in const functions
    pub const DEFAULT: Style = Style {
        display: Display::Flex,
        position_type: PositionType::Relative,
        flex_direction: FlexDirection::Row,
        flex_wrap: FlexWrap::NoWrap,
        align_items: None,
        align_self: None,
        #[cfg(feature = "experimental_grid")]
        justify_items: None,
        justify_self: None,
        align_content: None,
        justify_content: None,
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
        #[cfg(feature = "experimental_grid")]
        grid_template_rows: GridTrackVec::new(),
        #[cfg(feature = "experimental_grid")]
        grid_template_columns: GridTrackVec::new(),
        #[cfg(feature = "experimental_grid")]
        grid_auto_rows: GridTrackVec::new(),
        #[cfg(feature = "experimental_grid")]
        grid_auto_columns: GridTrackVec::new(),
        #[cfg(feature = "experimental_grid")]
        grid_auto_flow: GridAutoFlow::Row,
        #[cfg(feature = "experimental_grid")]
        grid_row: Line { start: GridPlacement::Auto, end: GridPlacement::Auto },
        #[cfg(feature = "experimental_grid")]
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
        #[cfg(feature = "experimental_grid")]
        use super::GridPlacement;

        let old_defaults = Style {
            display: Default::default(),
            position_type: Default::default(),
            flex_direction: Default::default(),
            flex_wrap: Default::default(),
            align_items: Default::default(),
            align_self: Default::default(),
            #[cfg(feature = "experimental_grid")]
            justify_items: Default::default(),
            justify_self: Default::default(),
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
            #[cfg(feature = "experimental_grid")]
            grid_template_rows: Default::default(),
            #[cfg(feature = "experimental_grid")]
            grid_template_columns: Default::default(),
            #[cfg(feature = "experimental_grid")]
            grid_auto_rows: Default::default(),
            #[cfg(feature = "experimental_grid")]
            grid_auto_columns: Default::default(),
            #[cfg(feature = "experimental_grid")]
            grid_auto_flow: Default::default(),
            #[cfg(feature = "experimental_grid")]
            grid_row: Line { start: GridPlacement::Auto, end: GridPlacement::Auto },
            #[cfg(feature = "experimental_grid")]
            grid_column: Line { start: GridPlacement::Auto, end: GridPlacement::Auto },
        };

        assert_eq!(Style::DEFAULT, Style::default());
        assert_eq!(Style::DEFAULT, old_defaults);
    }
}
