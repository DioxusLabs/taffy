//! Conversion functions from Stylo types to Taffy types

use servo_arc::Arc;

// Module of type aliases so we can refer to stylo types with nicer names
mod stylo {
    pub(crate) use style::computed_values::align_content::T as AlignContent;
    pub(crate) use style::computed_values::align_items::T as AlignItems;
    pub(crate) use style::computed_values::align_self::T as AlignSelf;
    pub(crate) use style::computed_values::flex_direction::T as FlexDirection;
    pub(crate) use style::computed_values::flex_wrap::T as FlexWrap;
    pub(crate) use style::computed_values::justify_content::T as JustifyContent;
    // pub(crate) use style::computed_values::justify_items::T as JustifyItems;
    // pub(crate) use style::computed_values::justify_self::T as JustifySelf;
    pub(crate) use style::properties::longhands::aspect_ratio::computed_value::T as AspectRatio;
    pub(crate) use style::properties::longhands::position::computed_value::T as Position;
    // pub(crate) use style::properties::style_structs::{Margin, Padding};
    pub(crate) use style::values::computed::LengthPercentage;
    pub(crate) use style::values::generics::flex::GenericFlexBasis;
    pub(crate) use style::values::generics::length::GenericLengthPercentageOrAuto;
    pub(crate) use style::values::generics::length::GenericLengthPercentageOrNormal;
    pub(crate) use style::values::generics::length::GenericMaxSize;
    pub(crate) use style::values::generics::length::GenericSize;
    pub(crate) use style::values::generics::position::PreferredRatio;
    pub(crate) use style::values::generics::NonNegative;
    pub(crate) use style::values::specified::box_::Display;
    pub(crate) use style::values::specified::box_::DisplayInside;
    pub(crate) use style::values::specified::box_::DisplayOutside;
    pub(crate) use style::values::specified::box_::Overflow;
    pub(crate) type LengthPercentageAuto = GenericLengthPercentageOrAuto<LengthPercentage>;
    pub(crate) type Size = GenericSize<NonNegative<LengthPercentage>>;
    pub(crate) type MaxSize = GenericMaxSize<NonNegative<LengthPercentage>>;
    pub(crate) type FlexBasis = GenericFlexBasis<Size>;
    pub(crate) type Gap = GenericLengthPercentageOrNormal<NonNegative<LengthPercentage>>;
    pub(crate) use style::properties::generated::ComputedValues;
}

pub struct TaffyStyloStyle(pub Arc<stylo::ComputedValues>);

impl From<Arc<stylo::ComputedValues>> for TaffyStyloStyle {
    fn from(value: Arc<stylo::ComputedValues>) -> Self {
        Self(Arc::clone(&value))
    }
}

impl taffy::CoreStyle for TaffyStyloStyle {
    #[inline]
    fn box_generation_mode(&self) -> taffy::BoxGenerationMode {
        t2s::box_generation_mode(self.0.get_box().display)
    }

    #[inline]
    fn is_block(&self) -> bool {
        t2s::is_block(self.0.get_box().display)
    }

    #[inline]
    fn overflow(&self) -> taffy::Point<taffy::Overflow> {
        let box_styles = self.0.get_box();
        taffy::Point { x: t2s::overflow(box_styles.overflow_x), y: t2s::overflow(box_styles.overflow_y) }
    }

    #[inline]
    fn scrollbar_width(&self) -> f32 {
        0.0
    }

    #[inline]
    fn position(&self) -> taffy::Position {
        t2s::position(self.0.get_box().position)
    }

    #[inline]
    fn inset(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        let position_styles = self.0.get_position();
        taffy::Rect {
            left: t2s::length_percentage_auto(&position_styles.left),
            right: t2s::length_percentage_auto(&position_styles.right),
            top: t2s::length_percentage_auto(&position_styles.top),
            bottom: t2s::length_percentage_auto(&position_styles.bottom),
        }
    }

    #[inline]
    fn size(&self) -> taffy::Size<taffy::Dimension> {
        let position_styles = self.0.get_position();
        taffy::Size { width: t2s::dimension(&position_styles.width), height: t2s::dimension(&position_styles.height) }
    }

    #[inline]
    fn min_size(&self) -> taffy::Size<taffy::Dimension> {
        let position_styles = self.0.get_position();
        taffy::Size {
            width: t2s::dimension(&position_styles.min_width),
            height: t2s::dimension(&position_styles.min_height),
        }
    }

    #[inline]
    fn max_size(&self) -> taffy::Size<taffy::Dimension> {
        let position_styles = self.0.get_position();
        taffy::Size {
            width: t2s::max_size_dimension(&position_styles.max_width),
            height: t2s::max_size_dimension(&position_styles.max_height),
        }
    }

    #[inline]
    fn aspect_ratio(&self) -> Option<f32> {
        t2s::aspect_ratio(self.0.get_position().aspect_ratio)
    }

    #[inline]
    fn margin(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        let margin_styles = self.0.get_margin();
        taffy::Rect {
            left: t2s::length_percentage_auto(&margin_styles.margin_left),
            right: t2s::length_percentage_auto(&margin_styles.margin_right),
            top: t2s::length_percentage_auto(&margin_styles.margin_top),
            bottom: t2s::length_percentage_auto(&margin_styles.margin_bottom),
        }
    }

    #[inline]
    fn padding(&self) -> taffy::Rect<taffy::LengthPercentage> {
        let padding_styles = self.0.get_padding();
        taffy::Rect {
            left: t2s::length_percentage(&padding_styles.padding_left.0),
            right: t2s::length_percentage(&padding_styles.padding_right.0),
            top: t2s::length_percentage(&padding_styles.padding_top.0),
            bottom: t2s::length_percentage(&padding_styles.padding_bottom.0),
        }
    }

    #[inline]
    fn border(&self) -> taffy::Rect<taffy::LengthPercentage> {
        let border_styles = self.0.get_border();
        taffy::Rect {
            left: taffy::LengthPercentage::Length(border_styles.border_left_width.to_f32_px()),
            right: taffy::LengthPercentage::Length(border_styles.border_right_width.to_f32_px()),
            top: taffy::LengthPercentage::Length(border_styles.border_top_width.to_f32_px()),
            bottom: taffy::LengthPercentage::Length(border_styles.border_bottom_width.to_f32_px()),
        }
    }
}

impl taffy::FlexboxContainerStyle for TaffyStyloStyle {
    #[inline]
    fn flex_direction(&self) -> taffy::FlexDirection {
        t2s::flex_direction(self.0.get_position().flex_direction)
    }

    #[inline]
    fn flex_wrap(&self) -> taffy::FlexWrap {
        t2s::flex_wrap(self.0.get_position().flex_wrap)
    }

    #[inline]
    fn gap(&self) -> taffy::Size<taffy::LengthPercentage> {
        let position_styles = self.0.get_position();
        taffy::Size {
            width: t2s::gap(&position_styles.column_gap),
            height: taffy::LengthPercentage::Length(0.0), // TODO: enable row_gap in stylo
        }
    }

    #[inline]
    fn align_content(&self) -> Option<taffy::AlignContent> {
        t2s::align_content(self.0.get_position().align_content)
    }

    #[inline]
    fn align_items(&self) -> Option<taffy::AlignItems> {
        t2s::align_items(self.0.get_position().align_items)
    }

    #[inline]
    fn justify_content(&self) -> Option<taffy::JustifyContent> {
        t2s::justify_content(self.0.get_position().justify_content)
    }
}

impl taffy::FlexboxItemStyle for TaffyStyloStyle {
    #[inline]
    fn flex_basis(&self) -> taffy::Dimension {
        t2s::flex_basis(&self.0.get_position().flex_basis)
    }

    #[inline]
    fn flex_grow(&self) -> f32 {
        self.0.get_position().flex_grow.0
    }

    #[inline]
    fn flex_shrink(&self) -> f32 {
        self.0.get_position().flex_grow.0
    }

    #[inline]
    fn align_self(&self) -> Option<taffy::AlignSelf> {
        t2s::align_self(self.0.get_position().align_self)
    }
}

mod t2s {
    use super::stylo;

    #[inline]
    pub(crate) fn length_percentage(val: &stylo::LengthPercentage) -> taffy::LengthPercentage {
        if let Some(length) = val.to_length() {
            taffy::LengthPercentage::Length(length.px())
        } else if let Some(val) = val.to_percentage() {
            taffy::LengthPercentage::Percent(val.0)
        } else {
            // TODO: Support calc
            taffy::LengthPercentage::Percent(0.0)
        }
    }

    #[inline]
    pub(crate) fn length_percentage_auto(val: &stylo::LengthPercentageAuto) -> taffy::LengthPercentageAuto {
        match val {
            stylo::LengthPercentageAuto::Auto => taffy::LengthPercentageAuto::Auto,
            stylo::LengthPercentageAuto::LengthPercentage(val) => length_percentage(val).into(),
        }
    }

    #[inline]
    pub(crate) fn dimension(val: &stylo::Size) -> taffy::Dimension {
        match val {
            stylo::Size::LengthPercentage(val) => length_percentage(&val.0).into(),
            stylo::Size::Auto => taffy::Dimension::Auto,
            // TODO: implement other values in Taffy (and servo configuration of stylo)
            // _ => taffy::Dimension::Auto,
        }
    }

    #[inline]
    pub(crate) fn max_size_dimension(val: &stylo::MaxSize) -> taffy::Dimension {
        match val {
            stylo::MaxSize::LengthPercentage(val) => length_percentage(&val.0).into(),
            stylo::MaxSize::None => taffy::Dimension::Auto,
            // TODO: implement other values in Taffy (and servo configuration of stylo)
            // _ => taffy::Dimension::Auto,
        }
    }

    #[inline]
    pub(crate) fn is_block(input: stylo::Display) -> bool {
        matches!(input.outside(), stylo::DisplayOutside::Block)
        // && matches!(input.inside(), stylo::DisplayInside::Flow | stylo::DisplayInside::FlowRoot)
    }

    #[inline]
    pub(crate) fn box_generation_mode(input: stylo::Display) -> taffy::BoxGenerationMode {
        match input.inside() {
            stylo::DisplayInside::None => taffy::BoxGenerationMode::None,
            // stylo::DisplayInside::Contents => display = taffy::BoxGenerationMode::Contents,
            _ => taffy::BoxGenerationMode::Normal,
        }
    }

    #[inline]
    pub(crate) fn position(input: stylo::Position) -> taffy::Position {
        match input {
            // TODO: support position:static
            stylo::Position::Relative => taffy::Position::Relative,
            stylo::Position::Static => taffy::Position::Relative,

            // TODO: support position:fixed and sticky
            stylo::Position::Absolute => taffy::Position::Absolute,
            stylo::Position::Fixed => taffy::Position::Absolute,
            stylo::Position::Sticky => taffy::Position::Absolute,
        }
    }

    #[inline]
    pub(crate) fn overflow(input: stylo::Overflow) -> taffy::Overflow {
        // TODO: Enable Overflow::Clip in servo configuration of stylo
        match input {
            stylo::Overflow::Visible => taffy::Overflow::Visible,
            stylo::Overflow::Hidden => taffy::Overflow::Hidden,
            stylo::Overflow::Scroll => taffy::Overflow::Scroll,
            // TODO: Support Overflow::Auto in Taffy
            stylo::Overflow::Auto => taffy::Overflow::Scroll,
        }
    }

    #[inline]
    pub(crate) fn aspect_ratio(input: stylo::AspectRatio) -> Option<f32> {
        match input.ratio {
            stylo::PreferredRatio::None => None,
            stylo::PreferredRatio::Ratio(val) => Some(val.0 .0 / val.1 .0),
        }
    }

    #[inline]
    pub(crate) fn gap(input: &stylo::Gap) -> taffy::LengthPercentage {
        match input {
            // For Flexbox and CSS Grid the "normal" value is 0px. This may need to be updated
            // if we ever implement multi-column layout.
            stylo::Gap::Normal => taffy::LengthPercentage::Length(0.0),
            stylo::Gap::LengthPercentage(val) => length_percentage(&val.0),
        }
    }

    #[inline]
    pub(crate) fn flex_basis(input: &stylo::FlexBasis) -> taffy::Dimension {
        // TODO: Support flex-basis: content in Taffy
        match input {
            stylo::FlexBasis::Content => taffy::Dimension::Auto,
            stylo::FlexBasis::Size(size) => dimension(&size).into(),
        }
    }

    #[inline]
    pub(crate) fn flex_direction(input: stylo::FlexDirection) -> taffy::FlexDirection {
        match input {
            stylo::FlexDirection::Row => taffy::FlexDirection::Row,
            stylo::FlexDirection::RowReverse => taffy::FlexDirection::RowReverse,
            stylo::FlexDirection::Column => taffy::FlexDirection::Column,
            stylo::FlexDirection::ColumnReverse => taffy::FlexDirection::ColumnReverse,
        }
    }

    #[inline]
    pub(crate) fn flex_wrap(input: stylo::FlexWrap) -> taffy::FlexWrap {
        match input {
            stylo::FlexWrap::Wrap => taffy::FlexWrap::Wrap,
            stylo::FlexWrap::WrapReverse => taffy::FlexWrap::WrapReverse,
            stylo::FlexWrap::Nowrap => taffy::FlexWrap::NoWrap,
        }
    }

    #[inline]
    pub(crate) fn justify_content(input: stylo::JustifyContent) -> Option<taffy::JustifyContent> {
        match input {
            stylo::JustifyContent::Start => Some(taffy::JustifyContent::Start),
            stylo::JustifyContent::End => Some(taffy::JustifyContent::End),
            stylo::JustifyContent::FlexStart => Some(taffy::JustifyContent::FlexStart),
            stylo::JustifyContent::FlexEnd => Some(taffy::JustifyContent::FlexEnd),
            stylo::JustifyContent::Stretch => Some(taffy::JustifyContent::Stretch),
            stylo::JustifyContent::Center => Some(taffy::JustifyContent::Center),
            stylo::JustifyContent::SpaceBetween => Some(taffy::JustifyContent::SpaceBetween),
            stylo::JustifyContent::SpaceAround => Some(taffy::JustifyContent::SpaceAround),
            stylo::JustifyContent::SpaceEvenly => Some(taffy::JustifyContent::SpaceEvenly),
        }
    }

    #[inline]
    pub(crate) fn align_content(input: stylo::AlignContent) -> Option<taffy::AlignContent> {
        match input {
            stylo::AlignContent::Start => Some(taffy::AlignContent::Start),
            stylo::AlignContent::End => Some(taffy::AlignContent::End),
            stylo::AlignContent::FlexStart => Some(taffy::AlignContent::FlexStart),
            stylo::AlignContent::FlexEnd => Some(taffy::AlignContent::FlexEnd),
            stylo::AlignContent::Center => Some(taffy::AlignContent::Center),
            stylo::AlignContent::Stretch => Some(taffy::AlignContent::Stretch),
            stylo::AlignContent::SpaceBetween => Some(taffy::AlignContent::SpaceBetween),
            stylo::AlignContent::SpaceAround => Some(taffy::AlignContent::SpaceAround),
            stylo::AlignContent::SpaceEvenly => Some(taffy::AlignContent::SpaceEvenly),
        }
    }

    #[inline]
    pub(crate) fn align_items(input: stylo::AlignItems) -> Option<taffy::AlignItems> {
        match input {
            stylo::AlignItems::Stretch => Some(taffy::AlignItems::Stretch),
            stylo::AlignItems::FlexStart => Some(taffy::AlignItems::FlexStart),
            stylo::AlignItems::FlexEnd => Some(taffy::AlignItems::FlexEnd),
            stylo::AlignItems::Center => Some(taffy::AlignItems::Center),
            stylo::AlignItems::Baseline => Some(taffy::AlignItems::Baseline),
        }
    }

    #[inline]
    pub(crate) fn align_self(input: stylo::AlignSelf) -> Option<taffy::AlignSelf> {
        match input {
            stylo::AlignSelf::Auto => None,
            stylo::AlignSelf::Stretch => Some(taffy::AlignSelf::Stretch),
            stylo::AlignSelf::FlexStart => Some(taffy::AlignSelf::FlexStart),
            stylo::AlignSelf::FlexEnd => Some(taffy::AlignSelf::FlexEnd),
            stylo::AlignSelf::Center => Some(taffy::AlignSelf::Center),
            stylo::AlignSelf::Baseline => Some(taffy::AlignSelf::Baseline),
        }
    }

    // pub(crate) fn justify_items(input: stylo::JustifyItems) -> Option<taffy::JustifyItems> {
    //     match input {
    //         stylo::JustifyItems::Stretch => Some(taffy::JustifyItems::Stretch),
    //         stylo::JustifyItems::FlexStart => Some(taffy::JustifyItems::FlexStart),
    //         stylo::JustifyItems::FlexEnd => Some(taffy::JustifyItems::FlexEnd),
    //         stylo::JustifyItems::Center => Some(taffy::JustifyItems::Center),
    //         stylo::JustifyItems::Baseline => Some(taffy::JustifyItems::Baseline),
    //     }
    // }

    // pub(crate) fn justify_self(input: stylo::JustifySelf) -> Option<taffy::JustifySelf> {
    //     match input {
    //         stylo::JustifySelf::Auto => None,
    //         stylo::JustifySelf::Stretch => Some(taffy::JustifySelf::Stretch),
    //         stylo::JustifySelf::FlexStart => Some(taffy::JustifySelf::FlexStart),
    //         stylo::JustifySelf::FlexEnd => Some(taffy::JustifySelf::FlexEnd),
    //         stylo::JustifySelf::Center => Some(taffy::JustifySelf::Center),
    //         stylo::JustifySelf::Baseline => Some(taffy::JustifySelf::Baseline),
    //     }
    // }
}
