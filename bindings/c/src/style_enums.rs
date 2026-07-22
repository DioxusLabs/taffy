use taffy::prelude as core;

/// Sets the layout used for the children of this node
///
/// The default values depends on on which feature flags are enabled. The order of precedence is: Flex, Grid, Block, None.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum TaffyDisplay {
    /// The children will follow the block layout algorithm
    Block,
    /// The children will follow the flexbox layout algorithm
    Flex,
    /// The children will follow the CSS Grid layout algorithm
    Grid,
    /// The children will not be laid out, and will follow absolute positioning
    None,
}
impl From<TaffyDisplay> for core::Display {
    fn from(input: TaffyDisplay) -> core::Display {
        match input {
            TaffyDisplay::Block => core::Display::Block,
            TaffyDisplay::Flex => core::Display::Flex,
            TaffyDisplay::Grid => core::Display::Grid,
            TaffyDisplay::None => core::Display::None,
        }
    }
}
impl From<core::Display> for TaffyDisplay {
    fn from(input: core::Display) -> TaffyDisplay {
        match input {
            core::Display::Block => TaffyDisplay::Block,
            core::Display::Flex => TaffyDisplay::Flex,
            core::Display::Grid => TaffyDisplay::Grid,
            core::Display::None => TaffyDisplay::None,
        }
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
#[repr(C)]
pub enum TaffyPosition {
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
impl From<TaffyPosition> for core::Position {
    fn from(input: TaffyPosition) -> core::Position {
        match input {
            TaffyPosition::Relative => core::Position::Relative,
            TaffyPosition::Absolute => core::Position::Absolute,
        }
    }
}
impl From<core::Position> for TaffyPosition {
    fn from(input: core::Position) -> TaffyPosition {
        match input {
            core::Position::Relative => TaffyPosition::Relative,
            core::Position::Absolute => TaffyPosition::Absolute,
        }
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
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum TaffyOverflow {
    /// The automatic minimum size of this node as a flexbox/grid item should be based on the size of it's content.
    Visible,
    /// The automatic minimum size of this node as a flexbox/grid item should be based on the size of its content.
    /// Content that overflows this node should *not* contribute to the scroll region of its parent.
    Clip,
    /// The automatic minimum size of this node as a flexbox/grid item should be `0`.
    Hidden,
    /// The automatic minimum size of this node as a flexbox/grid item should be `0`. Additionally, space should be reserved
    /// for a scrollbar. The amount of space reserved is controlled by the `scrollbar_width` property.
    Scroll,
}
impl From<TaffyOverflow> for core::Overflow {
    fn from(input: TaffyOverflow) -> core::Overflow {
        match input {
            TaffyOverflow::Visible => core::Overflow::Visible,
            TaffyOverflow::Clip => core::Overflow::Clip,
            TaffyOverflow::Hidden => core::Overflow::Hidden,
            TaffyOverflow::Scroll => core::Overflow::Scroll,
        }
    }
}
impl From<core::Overflow> for TaffyOverflow {
    fn from(input: core::Overflow) -> TaffyOverflow {
        match input {
            core::Overflow::Visible => TaffyOverflow::Visible,
            core::Overflow::Clip => TaffyOverflow::Clip,
            core::Overflow::Hidden => TaffyOverflow::Hidden,
            core::Overflow::Scroll => TaffyOverflow::Scroll,
        }
    }
}

/// Used to control how child nodes are aligned.
/// For Flexbox it controls alignment in the cross axis
/// For Grid it controls alignment in the block axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-items)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum TaffyAlignItems {
    /// Items are aligned according to their algorithm-specific default value
    /// This is equivalent to not setting a value in CSS
    Normal,
    /// Items are packed toward the start of the axis
    Start,
    /// Items are packed toward the end of the axis
    End,
    /// Items are packed towards the flex-relative start of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to End. In all other cases it is equivalent to Start.
    FlexStart,
    /// Items are packed towards the flex-relative end of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to Start. In all other cases it is equivalent to End.
    FlexEnd,
    /// Items are packed along the center of the cross axis
    Center,
    /// Items are aligned such as their baselines align
    Baseline,
    /// Stretch to fill the container
    Stretch,
}
impl From<TaffyAlignItems> for Option<core::AlignItems> {
    fn from(input: TaffyAlignItems) -> Option<core::AlignItems> {
        match input {
            TaffyAlignItems::Normal => None,
            TaffyAlignItems::Start => Some(core::AlignItems::Start),
            TaffyAlignItems::End => Some(core::AlignItems::End),
            TaffyAlignItems::FlexStart => Some(core::AlignItems::FlexStart),
            TaffyAlignItems::FlexEnd => Some(core::AlignItems::FlexEnd),
            TaffyAlignItems::Center => Some(core::AlignItems::Center),
            TaffyAlignItems::Baseline => Some(core::AlignItems::Baseline),
            TaffyAlignItems::Stretch => Some(core::AlignItems::Stretch),
        }
    }
}
impl From<Option<core::AlignItems>> for TaffyAlignItems {
    fn from(input: Option<core::AlignItems>) -> TaffyAlignItems {
        match input {
            None => TaffyAlignItems::Normal,
            Some(core::AlignItems::Start) => TaffyAlignItems::Start,
            Some(core::AlignItems::End) => TaffyAlignItems::End,
            Some(core::AlignItems::FlexStart) => TaffyAlignItems::FlexStart,
            Some(core::AlignItems::FlexEnd) => TaffyAlignItems::FlexEnd,
            Some(core::AlignItems::Center) => TaffyAlignItems::Center,
            Some(core::AlignItems::Baseline) => TaffyAlignItems::Baseline,
            Some(core::AlignItems::Stretch) => TaffyAlignItems::Stretch,
        }
    }
}

/// Sets the distribution of space between and around content items
/// For Flexbox it controls alignment in the cross axis
/// For Grid it controls alignment in the block axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-content)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum TaffyAlignContent {
    /// Items are aligned according to their algorithm-specific default value
    /// This is equivalent to not setting a value in CSS
    Normal,
    /// Items are packed toward the start of the axis
    Start,
    /// Items are packed toward the end of the axis
    End,
    /// Items are packed towards the flex-relative start of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to End. In all other cases it is equivalent to Start.
    FlexStart,
    /// Items are packed towards the flex-relative end of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to Start. In all other cases it is equivalent to End.
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
impl From<TaffyAlignContent> for Option<core::AlignContent> {
    fn from(input: TaffyAlignContent) -> Option<core::AlignContent> {
        match input {
            TaffyAlignContent::Normal => None,
            TaffyAlignContent::Start => Some(core::AlignContent::Start),
            TaffyAlignContent::End => Some(core::AlignContent::End),
            TaffyAlignContent::FlexStart => Some(core::AlignContent::FlexStart),
            TaffyAlignContent::FlexEnd => Some(core::AlignContent::FlexEnd),
            TaffyAlignContent::Center => Some(core::AlignContent::Center),
            TaffyAlignContent::Stretch => Some(core::AlignContent::Stretch),
            TaffyAlignContent::SpaceBetween => Some(core::AlignContent::SpaceBetween),
            TaffyAlignContent::SpaceEvenly => Some(core::AlignContent::SpaceEvenly),
            TaffyAlignContent::SpaceAround => Some(core::AlignContent::SpaceAround),
        }
    }
}
impl From<Option<core::AlignContent>> for TaffyAlignContent {
    fn from(input: Option<core::AlignContent>) -> TaffyAlignContent {
        match input {
            None => TaffyAlignContent::Normal,
            Some(core::AlignContent::Start) => TaffyAlignContent::Start,
            Some(core::AlignContent::End) => TaffyAlignContent::End,
            Some(core::AlignContent::FlexStart) => TaffyAlignContent::FlexStart,
            Some(core::AlignContent::FlexEnd) => TaffyAlignContent::FlexEnd,
            Some(core::AlignContent::Center) => TaffyAlignContent::Center,
            Some(core::AlignContent::Stretch) => TaffyAlignContent::Stretch,
            Some(core::AlignContent::SpaceBetween) => TaffyAlignContent::SpaceBetween,
            Some(core::AlignContent::SpaceAround) => TaffyAlignContent::SpaceAround,
            Some(core::AlignContent::SpaceEvenly) => TaffyAlignContent::SpaceEvenly,
        }
    }
}

/// Controls whether flex items are forced onto one line or can wrap onto multiple lines.
///
/// Defaults to [`FlexWrap::NoWrap`]
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum TaffyFlexWrap {
    /// Items will not wrap and stay on a single line
    NoWrap,
    /// Items will wrap according to this item's [`FlexDirection`]
    Wrap,
    /// Items will wrap in the opposite direction to this item's [`FlexDirection`]
    WrapReverse,
}
impl From<TaffyFlexWrap> for core::FlexWrap {
    fn from(input: TaffyFlexWrap) -> core::FlexWrap {
        match input {
            TaffyFlexWrap::NoWrap => core::FlexWrap::NoWrap,
            TaffyFlexWrap::Wrap => core::FlexWrap::Wrap,
            TaffyFlexWrap::WrapReverse => core::FlexWrap::WrapReverse,
        }
    }
}
impl From<core::FlexWrap> for TaffyFlexWrap {
    fn from(input: core::FlexWrap) -> TaffyFlexWrap {
        match input {
            core::FlexWrap::NoWrap => TaffyFlexWrap::NoWrap,
            core::FlexWrap::Wrap => TaffyFlexWrap::Wrap,
            core::FlexWrap::WrapReverse => TaffyFlexWrap::WrapReverse,
        }
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
#[repr(C)]
pub enum TaffyFlexDirection {
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
impl From<TaffyFlexDirection> for core::FlexDirection {
    fn from(input: TaffyFlexDirection) -> core::FlexDirection {
        match input {
            TaffyFlexDirection::Row => core::FlexDirection::Row,
            TaffyFlexDirection::Column => core::FlexDirection::Column,
            TaffyFlexDirection::RowReverse => core::FlexDirection::RowReverse,
            TaffyFlexDirection::ColumnReverse => core::FlexDirection::ColumnReverse,
        }
    }
}
impl From<core::FlexDirection> for TaffyFlexDirection {
    fn from(input: core::FlexDirection) -> TaffyFlexDirection {
        match input {
            core::FlexDirection::Row => TaffyFlexDirection::Row,
            core::FlexDirection::Column => TaffyFlexDirection::Column,
            core::FlexDirection::RowReverse => TaffyFlexDirection::RowReverse,
            core::FlexDirection::ColumnReverse => TaffyFlexDirection::ColumnReverse,
        }
    }
}

/// Controls whether grid items are placed row-wise or column-wise. And whether the sparse or dense packing algorithm is used.
///
/// The "dense" packing algorithm attempts to fill in holes earlier in the grid, if smaller items come up later.
/// This may cause items to appear out-of-order, when doing so would fill in holes left by larger items.
///
/// Defaults to [`GridAutoFlow::Row`]
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum TaffyGridAutoFlow {
    /// Items are placed by filling each row in turn, adding new rows as necessary
    Row,
    /// Items are placed by filling each column in turn, adding new columns as necessary.
    Column,
    /// Combines `Row` with the dense packing algorithm.
    RowDense,
    /// Combines `Column` with the dense packing algorithm.
    ColumnDense,
}
impl From<TaffyGridAutoFlow> for core::GridAutoFlow {
    fn from(input: TaffyGridAutoFlow) -> core::GridAutoFlow {
        match input {
            TaffyGridAutoFlow::Row => core::GridAutoFlow::Row,
            TaffyGridAutoFlow::Column => core::GridAutoFlow::Column,
            TaffyGridAutoFlow::RowDense => core::GridAutoFlow::RowDense,
            TaffyGridAutoFlow::ColumnDense => core::GridAutoFlow::ColumnDense,
        }
    }
}
impl From<core::GridAutoFlow> for TaffyGridAutoFlow {
    fn from(input: core::GridAutoFlow) -> TaffyGridAutoFlow {
        match input {
            core::GridAutoFlow::Row => TaffyGridAutoFlow::Row,
            core::GridAutoFlow::Column => TaffyGridAutoFlow::Column,
            core::GridAutoFlow::RowDense => TaffyGridAutoFlow::RowDense,
            core::GridAutoFlow::ColumnDense => TaffyGridAutoFlow::ColumnDense,
        }
    }
}
