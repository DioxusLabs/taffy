//! Parse/Deserialize raw CSS strings into Taffy style types

use crate::{
    prelude::*,
    style::{FlexDirection, Position},
};
use lightningcss::properties::border::LineStyle;
use lightningcss::properties::{align, border, display, flex, position, size};
use lightningcss::{
    properties::{Property, PropertyId},
    stylesheet::ParserOptions,
    traits::Parse,
    values::{
        length::{Length, LengthPercentageOrAuto, LengthValue},
        percentage::DimensionPercentage,
        ratio::Ratio,
    },
};

/// applies the entire html namespace defined in dioxus-html with the specified configeration
pub fn apply_layout_attributes(name: &str, value: &str, style: &mut Style) {
    if let Ok(property) = Property::parse_string(PropertyId::from(name), value, ParserOptions::default()) {
        match property {
            Property::Display(display) => match display {
                display::Display::Keyword(display::DisplayKeyword::None) => style.display = Display::None,
                display::Display::Pair(pair) => match pair.inside {
                    display::DisplayInside::Flex(_) => style.display = Display::Flex,
                    #[cfg(feature = "grid")]
                    display::DisplayInside::Grid => style.display = Display::Grid,
                    _ => {}
                },
                _ => (),
            },
            Property::Position(position) => {
                style.position = match position {
                    position::Position::Relative => Position::Relative,
                    position::Position::Absolute => Position::Absolute,
                    _ => return,
                }
            }
            Property::Top(top) => style.inset.top = convert_length_percentage_or_auto(top),
            Property::Bottom(bottom) => style.inset.bottom = convert_length_percentage_or_auto(bottom),
            Property::Left(left) => style.inset.left = convert_length_percentage_or_auto(left),
            Property::Right(right) => style.inset.right = convert_length_percentage_or_auto(right),
            Property::Inset(inset) => {
                style.inset.top = convert_length_percentage_or_auto(inset.top);
                style.inset.bottom = convert_length_percentage_or_auto(inset.bottom);
                style.inset.left = convert_length_percentage_or_auto(inset.left);
                style.inset.right = convert_length_percentage_or_auto(inset.right);
            }

            // Borders
            Property::BorderTopWidth(width) => {
                style.border.top = convert_border_side_width(width);
            }
            Property::BorderBottomWidth(width) => {
                style.border.bottom = convert_border_side_width(width);
            }
            Property::BorderLeftWidth(width) => {
                style.border.left = convert_border_side_width(width);
            }
            Property::BorderRightWidth(width) => {
                style.border.right = convert_border_side_width(width);
            }
            Property::BorderWidth(width) => {
                style.border.top = convert_border_side_width(width.top);
                style.border.bottom = convert_border_side_width(width.bottom);
                style.border.left = convert_border_side_width(width.left);
                style.border.right = convert_border_side_width(width.right);
            }
            Property::Border(border) => {
                let width = convert_border_side_width(border.width);
                style.border.top = width;
                style.border.bottom = width;
                style.border.left = width;
                style.border.right = width;
            }
            Property::BorderTop(top) => {
                style.border.top = convert_border_side_width(top.width);
            }
            Property::BorderBottom(bottom) => {
                style.border.bottom = convert_border_side_width(bottom.width);
            }
            Property::BorderLeft(left) => {
                style.border.left = convert_border_side_width(left.width);
            }
            Property::BorderRight(right) => {
                style.border.right = convert_border_side_width(right.width);
            }
            Property::BorderTopStyle(line_style) => {
                if line_style != LineStyle::None {
                    style.border.top = LengthPercentage::Length(3.0);
                }
            }
            Property::BorderBottomStyle(line_style) => {
                if line_style != LineStyle::None {
                    style.border.bottom = LengthPercentage::Length(3.0);
                }
            }
            Property::BorderLeftStyle(line_style) => {
                if line_style != LineStyle::None {
                    style.border.left = LengthPercentage::Length(3.0);
                }
            }
            Property::BorderRightStyle(line_style) => {
                if line_style != LineStyle::None {
                    style.border.right = LengthPercentage::Length(3.0);
                }
            }
            Property::BorderStyle(styles) => {
                if styles.top != LineStyle::None {
                    style.border.top = LengthPercentage::Length(3.0);
                }
                if styles.bottom != LineStyle::None {
                    style.border.bottom = LengthPercentage::Length(3.0);
                }
                if styles.left != LineStyle::None {
                    style.border.left = LengthPercentage::Length(3.0);
                }
                if styles.right != LineStyle::None {
                    style.border.right = LengthPercentage::Length(3.0);
                }
            }

            // Flexbox properties
            Property::FlexDirection(flex_direction, _) => {
                use FlexDirection::*;
                style.flex_direction = match flex_direction {
                    flex::FlexDirection::Row => Row,
                    flex::FlexDirection::RowReverse => RowReverse,
                    flex::FlexDirection::Column => Column,
                    flex::FlexDirection::ColumnReverse => ColumnReverse,
                }
            }
            Property::FlexWrap(wrap, _) => {
                use FlexWrap::*;
                style.flex_wrap = match wrap {
                    flex::FlexWrap::NoWrap => NoWrap,
                    flex::FlexWrap::Wrap => Wrap,
                    flex::FlexWrap::WrapReverse => WrapReverse,
                }
            }
            Property::FlexGrow(grow, _) => {
                style.flex_grow = grow;
            }
            Property::FlexShrink(shrink, _) => {
                style.flex_shrink = shrink;
            }
            Property::FlexBasis(basis, _) => {
                style.flex_basis = convert_length_percentage_or_auto(basis).into();
            }
            Property::Flex(flex, _) => {
                style.flex_grow = flex.grow;
                style.flex_shrink = flex.shrink;
                style.flex_basis = convert_length_percentage_or_auto(flex.basis).into();
            }

            // Alignment properties
            Property::AlignContent(align, _) => {
                use AlignContent::*;
                style.align_content = match align {
                    align::AlignContent::ContentDistribution(distribution) => match distribution {
                        align::ContentDistribution::SpaceBetween => Some(SpaceBetween),
                        align::ContentDistribution::SpaceAround => Some(SpaceAround),
                        align::ContentDistribution::SpaceEvenly => Some(SpaceEvenly),
                        align::ContentDistribution::Stretch => Some(Stretch),
                    },
                    align::AlignContent::ContentPosition { value: position, .. } => match position {
                        align::ContentPosition::Center => Some(Center),
                        align::ContentPosition::Start => Some(Start),
                        align::ContentPosition::FlexStart => Some(FlexStart),
                        align::ContentPosition::End => Some(End),
                        align::ContentPosition::FlexEnd => Some(FlexEnd),
                    },
                    _ => return,
                };
            }
            Property::JustifyContent(justify, _) => {
                use AlignContent::*;
                style.justify_content = match justify {
                    align::JustifyContent::ContentDistribution(distribution) => match distribution {
                        align::ContentDistribution::SpaceBetween => Some(SpaceBetween),
                        align::ContentDistribution::SpaceAround => Some(SpaceAround),
                        align::ContentDistribution::SpaceEvenly => Some(SpaceEvenly),
                        _ => return,
                    },
                    align::JustifyContent::ContentPosition { value: position, .. } => match position {
                        align::ContentPosition::Center => Some(Center),
                        align::ContentPosition::Start => Some(Start),
                        align::ContentPosition::FlexStart => Some(FlexStart),
                        align::ContentPosition::End => Some(End),
                        align::ContentPosition::FlexEnd => Some(FlexEnd),
                    },
                    _ => return,
                };
            }
            Property::AlignSelf(align, _) => {
                use AlignItems::*;
                style.align_self = match align {
                    align::AlignSelf::Auto => None,
                    align::AlignSelf::Stretch => Some(Stretch),
                    align::AlignSelf::BaselinePosition(_) => Some(Baseline),
                    align::AlignSelf::SelfPosition { value: position, .. } => match position {
                        align::SelfPosition::Center => Some(Center),
                        align::SelfPosition::Start | align::SelfPosition::SelfStart => Some(Start),
                        align::SelfPosition::FlexStart => Some(FlexStart),
                        align::SelfPosition::End | align::SelfPosition::SelfEnd => Some(End),
                        align::SelfPosition::FlexEnd => Some(FlexEnd),
                    },
                    _ => return,
                };
            }
            Property::AlignItems(align, _) => {
                use AlignItems::*;
                style.align_items = match align {
                    align::AlignItems::BaselinePosition(_) => Some(Baseline),
                    align::AlignItems::Stretch => Some(Stretch),
                    align::AlignItems::SelfPosition { value: position, .. } => match position {
                        align::SelfPosition::Center => Some(Center),
                        align::SelfPosition::FlexStart => Some(FlexStart),
                        align::SelfPosition::FlexEnd => Some(FlexEnd),
                        align::SelfPosition::Start | align::SelfPosition::SelfStart => Some(FlexEnd),
                        align::SelfPosition::End | align::SelfPosition::SelfEnd => Some(FlexEnd),
                    },
                    _ => return,
                };
            }
            Property::RowGap(row_gap) => {
                style.gap.width = convert_gap_value(row_gap);
            }
            Property::ColumnGap(column_gap) => {
                style.gap.height = convert_gap_value(column_gap);
            }
            Property::Gap(gap) => {
                style.gap = Size { width: convert_gap_value(gap.row), height: convert_gap_value(gap.column) };
            }
            Property::MarginTop(margin) => {
                style.margin.top = convert_length_percentage_or_auto(margin);
            }
            Property::MarginBottom(margin) => {
                style.margin.bottom = convert_length_percentage_or_auto(margin);
            }
            Property::MarginLeft(margin) => {
                style.margin.left = convert_length_percentage_or_auto(margin);
            }
            Property::MarginRight(margin) => {
                style.margin.right = convert_length_percentage_or_auto(margin);
            }
            Property::Margin(margin) => {
                style.margin = Rect {
                    top: convert_length_percentage_or_auto(margin.top),
                    bottom: convert_length_percentage_or_auto(margin.bottom),
                    left: convert_length_percentage_or_auto(margin.left),
                    right: convert_length_percentage_or_auto(margin.right),
                };
            }
            Property::PaddingTop(padding) => {
                style.padding.top = convert_padding(padding);
            }
            Property::PaddingBottom(padding) => {
                style.padding.bottom = convert_padding(padding);
            }
            Property::PaddingLeft(padding) => {
                style.padding.left = convert_padding(padding);
            }
            Property::PaddingRight(padding) => {
                style.padding.right = convert_padding(padding);
            }
            Property::Padding(padding) => {
                style.padding = Rect {
                    top: convert_padding(padding.top),
                    bottom: convert_padding(padding.bottom),
                    left: convert_padding(padding.left),
                    right: convert_padding(padding.right),
                };
            }
            Property::Width(width) => {
                style.size.width = convert_size(width);
            }
            Property::Height(height) => {
                style.size.height = convert_size(height);
            }
            _ => (),
        }
        // currently not implemented in lightningcss
        if name == "aspect-ratio" {
            if let Ok(ratio) = Ratio::parse_string(value) {
                style.aspect_ratio = Some(ratio.0 / ratio.1);
            }
        }
    }
}

fn extract_px_value(length_value: LengthValue) -> f32 {
    match length_value {
        LengthValue::Px(value) => value,
        _ => todo!(),
    }
}

fn convert_length_percentage(dimension_percentage: DimensionPercentage<LengthValue>) -> LengthPercentage {
    match dimension_percentage {
        DimensionPercentage::Dimension(value) => LengthPercentage::Length(extract_px_value(value)),
        DimensionPercentage::Percentage(percentage) => LengthPercentage::Percent(percentage.0),
        DimensionPercentage::Calc(_) => todo!(),
    }
}

fn convert_padding(dimension_percentage: LengthPercentageOrAuto) -> LengthPercentage {
    match dimension_percentage {
        LengthPercentageOrAuto::Auto => unimplemented!(),
        LengthPercentageOrAuto::LengthPercentage(lp) => match lp {
            DimensionPercentage::Dimension(value) => LengthPercentage::Length(extract_px_value(value)),
            DimensionPercentage::Percentage(percentage) => LengthPercentage::Percent(percentage.0),
            DimensionPercentage::Calc(_) => unimplemented!(),
        },
    }
}

fn convert_length_percentage_or_auto(dimension_percentage: LengthPercentageOrAuto) -> LengthPercentageAuto {
    match dimension_percentage {
        LengthPercentageOrAuto::Auto => LengthPercentageAuto::Auto,
        LengthPercentageOrAuto::LengthPercentage(lp) => match lp {
            DimensionPercentage::Dimension(value) => LengthPercentageAuto::Length(extract_px_value(value)),
            DimensionPercentage::Percentage(percentage) => LengthPercentageAuto::Percent(percentage.0),
            DimensionPercentage::Calc(_) => todo!(),
        },
    }
}

fn convert_dimension(dimension_percentage: DimensionPercentage<LengthValue>) -> Dimension {
    match dimension_percentage {
        DimensionPercentage::Dimension(value) => Dimension::Length(extract_px_value(value)),
        DimensionPercentage::Percentage(percentage) => Dimension::Percent(percentage.0),
        DimensionPercentage::Calc(_) => todo!(),
    }
}

fn convert_border_side_width(border_side_width: border::BorderSideWidth) -> LengthPercentage {
    match border_side_width {
        border::BorderSideWidth::Length(Length::Value(value)) => LengthPercentage::Length(extract_px_value(value)),
        border::BorderSideWidth::Thick => LengthPercentage::Length(1.0),
        border::BorderSideWidth::Medium => LengthPercentage::Length(3.0),
        border::BorderSideWidth::Thin => LengthPercentage::Length(5.0),
        border::BorderSideWidth::Length(_) => unimplemented!(),
    }
}

fn convert_gap_value(gap_value: align::GapValue) -> LengthPercentage {
    match gap_value {
        align::GapValue::LengthPercentage(dim) => convert_length_percentage(dim),
        align::GapValue::Normal => LengthPercentage::Length(0.0),
    }
}

fn convert_size(size: size::Size) -> Dimension {
    match size {
        size::Size::Auto => Dimension::Auto,
        size::Size::LengthPercentage(length) => convert_dimension(length),
        size::Size::MinContent(_) => Dimension::Auto, // Unimplemented, so default auto
        size::Size::MaxContent(_) => Dimension::Auto, // Unimplemented, so default auto
        size::Size::FitContent(_) => Dimension::Auto, // Unimplemented, so default auto
        size::Size::FitContentFunction(_) => Dimension::Auto, // Unimplemented, so default auto
        size::Size::Stretch(_) => Dimension::Auto,    // Unimplemented, so default auto
        size::Size::Contain => Dimension::Auto,       // Unimplemented, so default auto
    }
}

/// parse relative or absolute value
pub fn parse_value(value: &str) -> Option<Dimension> {
    if value.ends_with("px") {
        if let Ok(px) = value.trim_end_matches("px").parse::<f32>() {
            Some(Dimension::Length(px))
        } else {
            None
        }
    } else if value.ends_with('%') {
        if let Ok(pct) = value.trim_end_matches('%').parse::<f32>() {
            Some(Dimension::Percent(pct / 100.0))
        } else {
            None
        }
    } else {
        None
    }
}
