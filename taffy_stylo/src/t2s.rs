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
        && matches!(input.inside(), stylo::DisplayInside::Flow | stylo::DisplayInside::FlowRoot)
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
pub(crate) fn box_sizing(input: stylo::BoxSizing) -> taffy::BoxSizing {
    match input {
        stylo::BoxSizing::BorderBox => taffy::BoxSizing::BorderBox,
        stylo::BoxSizing::ContentBox => taffy::BoxSizing::ContentBox,
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
pub(crate) fn content_alignment(input: stylo::ContentDistribution) -> Option<taffy::AlignContent> {
    match input.primary().value() {
        stylo::AlignFlags::NORMAL => None,
        stylo::AlignFlags::AUTO => None,
        stylo::AlignFlags::START => Some(taffy::AlignContent::Start),
        stylo::AlignFlags::END => Some(taffy::AlignContent::End),
        stylo::AlignFlags::FLEX_START => Some(taffy::AlignContent::FlexStart),
        stylo::AlignFlags::STRETCH => Some(taffy::AlignContent::Stretch),
        stylo::AlignFlags::FLEX_END => Some(taffy::AlignContent::FlexEnd),
        stylo::AlignFlags::CENTER => Some(taffy::AlignContent::Center),
        stylo::AlignFlags::SPACE_BETWEEN => Some(taffy::AlignContent::SpaceBetween),
        stylo::AlignFlags::SPACE_AROUND => Some(taffy::AlignContent::SpaceAround),
        stylo::AlignFlags::SPACE_EVENLY => Some(taffy::AlignContent::SpaceEvenly),
        // Should never be hit. But no real reason to panic here.
        _ => None,
    }
}

#[inline]
pub(crate) fn item_alignment(input: stylo::AlignFlags) -> Option<taffy::AlignItems> {
    match input.value() {
        stylo::AlignFlags::NORMAL => None,
        stylo::AlignFlags::AUTO => None,
        stylo::AlignFlags::STRETCH => Some(taffy::AlignItems::Stretch),
        stylo::AlignFlags::FLEX_START => Some(taffy::AlignItems::FlexStart),
        stylo::AlignFlags::FLEX_END => Some(taffy::AlignItems::FlexEnd),
        stylo::AlignFlags::START => Some(taffy::AlignItems::Start),
        stylo::AlignFlags::END => Some(taffy::AlignItems::End),
        stylo::AlignFlags::CENTER => Some(taffy::AlignItems::Center),
        stylo::AlignFlags::BASELINE => Some(taffy::AlignItems::Baseline),
        // Should never be hit. But no real reason to panic here.
        _ => None,
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
