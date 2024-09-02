use taffy::FlexDirection;
use taffy::TextAlign;
use taffy::Display;
use taffy::Position;
use taffy::BoxSizing;
use taffy::BoxGenerationMode;
use taffy::AlignItems;
use taffy::AbsoluteAxis;
use taffy::GridAutoFlow;
use taffy::FlexWrap;
use taffy::Overflow;
use taffy::AlignContent;
use crate::traits::FromJavaEnum;

impl FromJavaEnum<AlignContent> for AlignContent {
    const JAVA_CLASS: &'static str = "Lcom/dioxuslabs/taffy/enums/AlignContent;";

    fn from_ordinal(internal: i32) -> Option<AlignContent> {
        Some(match internal {
            0 => AlignContent::Start,
            1 => AlignContent::End,
            2 => AlignContent::FlexStart,
            3 => AlignContent::FlexEnd,
            4 => AlignContent::Center,
            5 => AlignContent::Stretch,
            6 => AlignContent::SpaceBetween,
            7 => AlignContent::SpaceEvenly,
            8 => AlignContent::SpaceAround,
            _ => panic!("Invalid value: {internal}"),
        })
    }
}

impl FromJavaEnum<Overflow> for Overflow {
    const JAVA_CLASS: &'static str = "Lcom/dioxuslabs/taffy/enums/Overflow;";

    fn from_ordinal(internal: i32) -> Option<Overflow> {
        Some(match internal {
            0 => Overflow::Visible,
            1 => Overflow::Clip,
            2 => Overflow::Hidden,
            3 => Overflow::Scroll,
            _ => panic!("Invalid value: {internal}"),
        })
    }
}

impl FromJavaEnum<FlexWrap> for FlexWrap {
    const JAVA_CLASS: &'static str = "Lcom/dioxuslabs/taffy/enums/FlexWrap;";

    fn from_ordinal(internal: i32) -> Option<FlexWrap> {
        Some(match internal {
            0 => FlexWrap::NoWrap,
            1 => FlexWrap::Wrap,
            2 => FlexWrap::WrapReverse,
            _ => panic!("Invalid value: {internal}"),
        })
    }
}

impl FromJavaEnum<GridAutoFlow> for GridAutoFlow {
    const JAVA_CLASS: &'static str = "Lcom/dioxuslabs/taffy/enums/GridAutoFlow;";

    fn from_ordinal(internal: i32) -> Option<GridAutoFlow> {
        Some(match internal {
            0 => GridAutoFlow::Row,
            1 => GridAutoFlow::Column,
            2 => GridAutoFlow::RowDense,
            3 => GridAutoFlow::ColumnDense,
            _ => panic!("Invalid value: {internal}"),
        })
    }
}

impl FromJavaEnum<AbsoluteAxis> for AbsoluteAxis {
    const JAVA_CLASS: &'static str = "Lcom/dioxuslabs/taffy/enums/AbsoluteAxis;";

    fn from_ordinal(internal: i32) -> Option<AbsoluteAxis> {
        Some(match internal {
            0 => AbsoluteAxis::Horizontal,
            1 => AbsoluteAxis::Vertical,
            _ => panic!("Invalid value: {internal}"),
        })
    }
}

impl FromJavaEnum<AlignItems> for AlignItems {
    const JAVA_CLASS: &'static str = "Lcom/dioxuslabs/taffy/enums/AlignItems;";

    fn from_ordinal(internal: i32) -> Option<AlignItems> {
        Some(match internal {
            0 => AlignItems::Start,
            1 => AlignItems::End,
            2 => AlignItems::FlexStart,
            3 => AlignItems::FlexEnd,
            4 => AlignItems::Center,
            5 => AlignItems::Baseline,
            6 => AlignItems::Stretch,
            _ => panic!("Invalid value: {internal}"),
        })
    }
}

impl FromJavaEnum<BoxGenerationMode> for BoxGenerationMode {
    const JAVA_CLASS: &'static str = "Lcom/dioxuslabs/taffy/enums/BoxGenerationMode;";

    fn from_ordinal(internal: i32) -> Option<BoxGenerationMode> {
        Some(match internal {
            0 => BoxGenerationMode::Normal,
            1 => BoxGenerationMode::None,
            _ => panic!("Invalid value: {internal}"),
        })
    }
}

impl FromJavaEnum<BoxSizing> for BoxSizing {
    const JAVA_CLASS: &'static str = "Lcom/dioxuslabs/taffy/enums/BoxSizing;";

    fn from_ordinal(internal: i32) -> Option<BoxSizing> {
        Some(match internal {
            0 => BoxSizing::BorderBox,
            1 => BoxSizing::ContentBox,
            _ => panic!("Invalid value: {internal}"),
        })
    }
}

impl FromJavaEnum<Position> for Position {
    const JAVA_CLASS: &'static str = "Lcom/dioxuslabs/taffy/enums/Position;";

    fn from_ordinal(internal: i32) -> Option<Position> {
        Some(match internal {
            0 => Position::Relative,
            1 => Position::Absolute,
            _ => panic!("Invalid value: {internal}"),
        })
    }
}

impl FromJavaEnum<Display> for Display {
    const JAVA_CLASS: &'static str = "Lcom/dioxuslabs/taffy/enums/Display;";

    fn from_ordinal(internal: i32) -> Option<Display> {
        Some(match internal {
            0 => Display::Block,
            1 => Display::Flex,
            2 => Display::Grid,
            3 => Display::None,
            _ => panic!("Invalid value: {internal}"),
        })
    }
}

impl FromJavaEnum<TextAlign> for TextAlign {
    const JAVA_CLASS: &'static str = "Lcom/dioxuslabs/taffy/enums/TextAlign;";

    fn from_ordinal(internal: i32) -> Option<TextAlign> {
        Some(match internal {
            0 => TextAlign::Auto,
            1 => TextAlign::LegacyLeft,
            2 => TextAlign::LegacyRight,
            3 => TextAlign::LegacyCenter,
            _ => panic!("Invalid value: {internal}"),
        })
    }
}

impl FromJavaEnum<FlexDirection> for FlexDirection {
    const JAVA_CLASS: &'static str = "Lcom/dioxuslabs/taffy/enums/FlexDirection;";

    fn from_ordinal(internal: i32) -> Option<FlexDirection> {
        Some(match internal {
            0 => FlexDirection::Row,
            1 => FlexDirection::Column,
            2 => FlexDirection::RowReverse,
            3 => FlexDirection::ColumnReverse,
            _ => panic!("Invalid value: {internal}"),
        })
    }
}