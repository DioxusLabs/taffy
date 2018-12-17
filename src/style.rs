use crate::number::Number;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Default for AlignItems {
    fn default() -> AlignItems {
        AlignItems::Stretch
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Default for AlignSelf {
    fn default() -> AlignSelf {
        AlignSelf::Auto
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

impl Default for AlignContent {
    fn default() -> AlignContent {
        AlignContent::Stretch
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    Inherit,
    LTR,
    RTL,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Inherit
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Display {
    Flex,
    None,
}

impl Default for Display {
    fn default() -> Display {
        Display::Flex
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> FlexDirection {
        FlexDirection::Row
    }
}

impl FlexDirection {
    pub(crate) fn is_row(self) -> bool {
        self == FlexDirection::Row || self == FlexDirection::RowReverse
    }

    pub(crate) fn is_column(self) -> bool {
        self == FlexDirection::Column || self == FlexDirection::ColumnReverse
    }

    pub(crate) fn is_reverse(self) -> bool {
        self == FlexDirection::RowReverse || self == FlexDirection::ColumnReverse
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> JustifyContent {
        JustifyContent::FlexStart
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

impl Default for Overflow {
    fn default() -> Overflow {
        Overflow::Visible
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Position {
    Relative,
    Absolute,
}

impl Default for Position {
    fn default() -> Position {
        Position::Relative
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> FlexWrap {
        FlexWrap::NoWrap
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dimension {
    Auto,
    Points(f32),
    Percent(f32),
}

impl Default for Dimension {
    fn default() -> Dimension {
        Dimension::Auto
    }
}

impl Dimension {
    pub(crate) fn resolve(self, parent_width: Number) -> Number {
        match self {
            Dimension::Points(points) => Number::Defined(points),
            Dimension::Percent(percent) => parent_width * percent,
            _ => Number::Undefined,
        }
    }
}

#[derive(Debug)]
pub struct Edges {
    pub start: Dimension,
    pub end: Dimension,
    pub top: Dimension,
    pub bottom: Dimension,
}

impl Default for Edges {
    fn default() -> Edges {
        Edges {
            start: Dimension::Points(0.0),
            end: Dimension::Points(0.0),
            top: Dimension::Points(0.0),
            bottom: Dimension::Points(0.0),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub position: Position,
    pub direction: Direction,
    pub flex_direction: FlexDirection,

    pub flex_wrap: FlexWrap,
    pub overflow: Overflow,

    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub align_content: AlignContent,

    pub justify_content: JustifyContent,

    pub margin: Edges,
    pub padding: Edges,
    pub border: Edges,

    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,

    pub start: Dimension,
    pub end: Dimension,
    pub top: Dimension,
    pub bottom: Dimension,

    pub width: Dimension,
    pub min_width: Dimension,
    pub max_width: Dimension,

    pub height: Dimension,
    pub min_height: Dimension,
    pub max_height: Dimension,

    pub aspect_ratio: Number,

    pub children: Vec<Node>,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            position: Default::default(),
            direction: Default::default(),
            flex_direction: Default::default(),

            flex_wrap: Default::default(),
            overflow: Default::default(),

            align_items: Default::default(),
            align_self: Default::default(),
            align_content: Default::default(),

            justify_content: Default::default(),

            margin: Default::default(),
            padding: Default::default(),
            border: Default::default(),

            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Default::default(),

            start: Default::default(),
            end: Default::default(),
            top: Default::default(),
            bottom: Default::default(),

            width: Default::default(),
            min_width: Default::default(),
            max_width: Default::default(),

            height: Default::default(),
            min_height: Default::default(),
            max_height: Default::default(),

            aspect_ratio: Default::default(),

            children: vec![],
        }
    }
}

impl Node {
    pub(crate) fn min_main_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.min_width,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.min_height,
        }
    }

    pub(crate) fn max_main_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.max_width,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.max_height,
        }
    }

    pub(crate) fn main_margin_start(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.start,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.top,
        }
    }

    pub(crate) fn main_margin_end(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.end,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.bottom,
        }
    }

    pub(crate) fn cross_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.width,
        }
    }

    pub(crate) fn min_cross_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.min_height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.min_width,
        }
    }

    pub(crate) fn max_cross_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.max_height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.max_width,
        }
    }

    pub(crate) fn cross_margin_start(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.top,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.start,
        }
    }

    pub(crate) fn cross_margin_end(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.bottom,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.end,
        }
    }

    pub(crate) fn align_self(&self, parent: &Node) -> AlignSelf {
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
