//! Style types for representing lengths / sizes

use crate::geometry::Rect;
use crate::style_helpers::{FromLength, FromPercent, TaffyAuto, TaffyZero};

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LengthPercentage {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Length(f32),
    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    Percent(f32),
}
impl TaffyZero for LengthPercentage {
    const ZERO: Self = Self::Length(0.0);
}
impl FromLength for LengthPercentage {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::Length(value.into())
    }
}
impl FromPercent for LengthPercentage {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::Percent(percent.into())
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LengthPercentageAuto {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Length(f32),
    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    Percent(f32),
    /// The dimension should be automatically computed
    Auto,
}
impl TaffyZero for LengthPercentageAuto {
    const ZERO: Self = Self::Length(0.0);
}
impl TaffyAuto for LengthPercentageAuto {
    const AUTO: Self = Self::Auto;
}
impl FromLength for LengthPercentageAuto {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::Length(value.into())
    }
}
impl FromPercent for LengthPercentageAuto {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::Percent(percent.into())
    }
}

impl From<LengthPercentage> for LengthPercentageAuto {
    fn from(input: LengthPercentage) -> Self {
        match input {
            LengthPercentage::Length(value) => Self::Length(value),
            LengthPercentage::Percent(value) => Self::Percent(value),
        }
    }
}

impl LengthPercentageAuto {
    /// Returns:
    ///   - Some(length) for Length variants
    ///   - Some(resolved) using the provided context for Percent variants
    ///   - None for Auto variants
    #[inline(always)]
    pub fn resolve_to_option(self, context: f32) -> Option<f32> {
        match self {
            Self::Length(length) => Some(length),
            Self::Percent(percent) => Some(context * percent),
            Self::Auto => None,
        }
    }

    /// Returns true if value is LengthPercentageAuto::Auto
    #[inline(always)]
    pub fn is_auto(self) -> bool {
        self == Self::Auto
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Dimension {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Length(f32),
    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    Percent(f32),
    /// The dimension should be automatically computed
    Auto,
}
impl TaffyZero for Dimension {
    const ZERO: Self = Self::Length(0.0);
}
impl TaffyAuto for Dimension {
    const AUTO: Self = Self::Auto;
}
impl FromLength for Dimension {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::Length(value.into())
    }
}
impl FromPercent for Dimension {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::Percent(percent.into())
    }
}

impl From<LengthPercentage> for Dimension {
    fn from(input: LengthPercentage) -> Self {
        match input {
            LengthPercentage::Length(value) => Self::Length(value),
            LengthPercentage::Percent(value) => Self::Percent(value),
        }
    }
}

impl From<LengthPercentageAuto> for Dimension {
    fn from(input: LengthPercentageAuto) -> Self {
        match input {
            LengthPercentageAuto::Length(value) => Self::Length(value),
            LengthPercentageAuto::Percent(value) => Self::Percent(value),
            LengthPercentageAuto::Auto => Self::Auto,
        }
    }
}

impl Dimension {
    /// Get Length value if value is Length variant
    #[cfg(feature = "grid")]
    pub fn into_option(self) -> Option<f32> {
        match self {
            Dimension::Length(value) => Some(value),
            _ => None,
        }
    }
}

impl Rect<Dimension> {
    /// Create a new Rect with [`Dimension::Length`]
    #[must_use]
    pub const fn from_length(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension::Length(start),
            right: Dimension::Length(end),
            top: Dimension::Length(top),
            bottom: Dimension::Length(bottom),
        }
    }

    /// Create a new Rect with [`Dimension::Percent`]
    #[must_use]
    pub const fn from_percent(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension::Percent(start),
            right: Dimension::Percent(end),
            top: Dimension::Percent(top),
            bottom: Dimension::Percent(bottom),
        }
    }
}
