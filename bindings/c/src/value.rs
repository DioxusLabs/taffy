//! Values types for C FFI

use taffy::prelude as core;

use super::{TaffyFFIDefault, TaffyReturnCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum TaffyEdge {
    /// The top edge of the box
    Top,
    /// The bottom edge of the box
    Bottom,
    /// The left edge of the box
    Left,
    /// The right edge of the box
    Right,
    /// Both the top and bottom edges of the box
    Vertical,
    /// Both the left and right edges of the box
    Horizontal,
    /// All four edges of the box
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum TaffyUnit {
    /// A none value (used to unset optional fields)
    None,
    /// Fixed Length (pixel) value
    Length,
    /// Percentage value
    Percent,
    /// Min-content size
    MinContent,
    /// Max-content size
    MaxContent,
    /// fit-content() function with a pixel limit
    FitContentPx,
    /// fit-content() function with a percentage limit
    FitContentPercent,
    /// Automatic values
    Auto,
    /// fr unit
    Fr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum TaffyMeasureMode {
    /// A none value (used to unset optional fields)
    Exact,
    /// Fixed Length (pixel) value
    FitContent,
    /// Percentage value
    MinContent,
    /// Min-content size
    MaxContent,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TaffySize {
    width: f32,
    height: f32,
}
impl From<TaffySize> for core::Size<f32> {
    #[inline(always)]
    fn from(value: TaffySize) -> Self {
        core::Size { width: value.width, height: value.height }
    }
}

#[repr(C)]
pub struct TaffyLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl TaffyFFIDefault for TaffyLayout {
    fn default() -> Self {
        TaffyLayout { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct TaffyDimension {
    /// The value. If the unit is variant that doesn't require a value (e.g. Auto) then the value is ignored.
    pub value: f32,
    pub unit: TaffyUnit,
}
impl TaffyFFIDefault for TaffyDimension {
    fn default() -> Self {
        Self { unit: TaffyUnit::None, value: 0.0 }
    }
}

impl TaffyDimension {
    #[inline(always)]
    pub fn from_raw(unit: TaffyUnit, value: f32) -> Self {
        Self { unit, value }
    }
}

impl From<core::LengthPercentage> for TaffyDimension {
    fn from(value: core::LengthPercentage) -> Self {
        match value {
            core::LengthPercentage::Length(value) => Self { unit: TaffyUnit::Length, value },
            core::LengthPercentage::Percent(value) => Self { unit: TaffyUnit::Percent, value },
        }
    }
}

impl TryFrom<TaffyDimension> for core::LengthPercentage {
    type Error = TaffyReturnCode;

    fn try_from(value: TaffyDimension) -> Result<Self, Self::Error> {
        match value.unit {
            TaffyUnit::Length => Ok(core::LengthPercentage::Length(value.value)),
            TaffyUnit::Percent => Ok(core::LengthPercentage::Percent(value.value)),
            TaffyUnit::None => Err(TaffyReturnCode::InvalidNone),
            TaffyUnit::Auto => Err(TaffyReturnCode::InvalidAuto),
            TaffyUnit::MinContent => Err(TaffyReturnCode::InvalidMinContent),
            TaffyUnit::MaxContent => Err(TaffyReturnCode::InvalidMaxContent),
            TaffyUnit::FitContentPx => Err(TaffyReturnCode::InvalidFitContentPx),
            TaffyUnit::FitContentPercent => Err(TaffyReturnCode::InvalidFitContentPercent),
            TaffyUnit::Fr => Err(TaffyReturnCode::InvalidFr),
        }
    }
}

impl From<core::LengthPercentageAuto> for TaffyDimension {
    fn from(value: core::LengthPercentageAuto) -> Self {
        match value {
            core::LengthPercentageAuto::Length(value) => Self { unit: TaffyUnit::Length, value },
            core::LengthPercentageAuto::Percent(value) => Self { unit: TaffyUnit::Percent, value },
            core::LengthPercentageAuto::Auto => Self { unit: TaffyUnit::Auto, value: 0.0 },
        }
    }
}

impl TryFrom<TaffyDimension> for core::LengthPercentageAuto {
    type Error = TaffyReturnCode;

    fn try_from(value: TaffyDimension) -> Result<Self, Self::Error> {
        match value.unit {
            TaffyUnit::Auto => Ok(core::LengthPercentageAuto::Auto),
            TaffyUnit::Length => Ok(core::LengthPercentageAuto::Length(value.value)),
            TaffyUnit::Percent => Ok(core::LengthPercentageAuto::Percent(value.value)),
            TaffyUnit::None => Err(TaffyReturnCode::InvalidNone),
            TaffyUnit::MinContent => Err(TaffyReturnCode::InvalidMinContent),
            TaffyUnit::MaxContent => Err(TaffyReturnCode::InvalidMaxContent),
            TaffyUnit::FitContentPx => Err(TaffyReturnCode::InvalidFitContentPx),
            TaffyUnit::FitContentPercent => Err(TaffyReturnCode::InvalidFitContentPercent),
            TaffyUnit::Fr => Err(TaffyReturnCode::InvalidFr),
        }
    }
}

impl From<core::Dimension> for TaffyDimension {
    fn from(value: core::Dimension) -> Self {
        match value {
            core::Dimension::Length(value) => Self { unit: TaffyUnit::Length, value },
            core::Dimension::Percent(value) => Self { unit: TaffyUnit::Percent, value },
            core::Dimension::Auto => Self { unit: TaffyUnit::Auto, value: 0.0 },
        }
    }
}

impl TryFrom<TaffyDimension> for core::Dimension {
    type Error = TaffyReturnCode;

    fn try_from(value: TaffyDimension) -> Result<Self, Self::Error> {
        match value.unit {
            TaffyUnit::Auto => Ok(core::Dimension::Auto),
            TaffyUnit::Length => Ok(core::Dimension::Length(value.value)),
            TaffyUnit::Percent => Ok(core::Dimension::Percent(value.value)),
            TaffyUnit::None => Err(TaffyReturnCode::InvalidNone),
            TaffyUnit::MinContent => Err(TaffyReturnCode::InvalidMinContent),
            TaffyUnit::MaxContent => Err(TaffyReturnCode::InvalidMaxContent),
            TaffyUnit::FitContentPx => Err(TaffyReturnCode::InvalidFitContentPx),
            TaffyUnit::FitContentPercent => Err(TaffyReturnCode::InvalidFitContentPercent),
            TaffyUnit::Fr => Err(TaffyReturnCode::InvalidFr),
        }
    }
}

/// For all fields, zero represents not set
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct TaffyGridPlacement {
    pub start: i16,
    pub end: i16,
    pub span: u16,
}

impl TaffyFFIDefault for TaffyGridPlacement {
    fn default() -> Self {
        Self { start: 0, end: 0, span: 0 }
    }
}

impl From<TaffyGridPlacement> for core::Line<core::GridPlacement> {
    fn from(placement: TaffyGridPlacement) -> Self {
        Self::from_raw_parts(placement.start, placement.span, placement.end)
    }
}

impl From<core::Line<core::GridPlacement>> for TaffyGridPlacement {
    fn from(placement: core::Line<core::GridPlacement>) -> Self {
        let (start, span, end) = placement.into_raw_parts();
        Self { start, span, end }
    }
}
