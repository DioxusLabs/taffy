//! Style types for representing lengths / sizes
use super::CompactLength;
use crate::geometry::Rect;
use crate::style_helpers::{FromLength, FromPercent, TaffyAuto, TaffyZero};

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`](crate::geometry::Size).
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LengthPercentage(pub(crate) CompactLength);
impl TaffyZero for LengthPercentage {
    const ZERO: Self = Self(CompactLength::ZERO);
}
impl FromLength for LengthPercentage {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self(CompactLength::from_length(value))
    }
}
impl FromPercent for LengthPercentage {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self(CompactLength::from_percent(percent))
    }
}
impl LengthPercentage {
    /// Get the underlying `CompactLength` representation of the value
    pub fn into_raw(self) -> CompactLength {
        self.0
    }

    /// Create a LengthPercentage from a raw `CompactLength`.
    /// # Safety
    /// CompactLength must represent a valid variant for LengthPercentage
    #[allow(unsafe_code)]
    pub unsafe fn from_raw(val: CompactLength) -> Self {
        Self(val)
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`](crate::geometry::Size).
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LengthPercentageAuto(pub(crate) CompactLength);
impl TaffyZero for LengthPercentageAuto {
    const ZERO: Self = Self(CompactLength::ZERO);
}
impl TaffyAuto for LengthPercentageAuto {
    const AUTO: Self = Self(CompactLength::AUTO);
}
impl FromLength for LengthPercentageAuto {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self(CompactLength::from_length(value))
    }
}
impl FromPercent for LengthPercentageAuto {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self(CompactLength::from_percent(percent))
    }
}
impl From<LengthPercentage> for LengthPercentageAuto {
    fn from(input: LengthPercentage) -> Self {
        Self(input.0)
    }
}

impl LengthPercentageAuto {
    /// Returns:
    ///   - Some(length) for Length variants
    ///   - Some(resolved) using the provided context for Percent variants
    ///   - None for Auto variants
    #[inline(always)]
    pub fn resolve_to_option(self, context: f32, calc_resolver: impl Fn(u64, f32) -> f32) -> Option<f32> {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => Some(self.0.value()),
            CompactLength::PERCENT_TAG => Some(context * self.0.value()),
            CompactLength::AUTO_TAG => None,
            _ if self.0.is_calc() => Some(calc_resolver(self.0.calc_value(), context)),
            _ => unreachable!("LengthPercentageAuto values cannot be constructed with other tags"),
        }
    }

    /// Returns true if value is LengthPercentageAuto::Auto
    #[inline(always)]
    pub fn is_auto(self) -> bool {
        self.0.is_auto()
    }

    /// Get the underlying `CompactLength` representation of the value
    pub fn into_raw(self) -> CompactLength {
        self.0
    }

    /// Create a LengthPercentageAuto from a raw `CompactLength`.
    /// # Safety
    /// CompactLength must represent a valid variant for LengthPercentageAuto
    #[allow(unsafe_code)]
    pub unsafe fn from_raw(val: CompactLength) -> Self {
        Self(val)
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`](crate::geometry::Size).
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Dimension(pub(crate) CompactLength);
impl TaffyZero for Dimension {
    const ZERO: Self = Self(CompactLength::ZERO);
}
impl TaffyAuto for Dimension {
    const AUTO: Self = Self(CompactLength::AUTO);
}
impl FromLength for Dimension {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self(CompactLength::from_length(value))
    }
}
impl FromPercent for Dimension {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self(CompactLength::from_percent(percent))
    }
}
impl From<LengthPercentage> for Dimension {
    fn from(input: LengthPercentage) -> Self {
        Self(input.0)
    }
}
impl From<LengthPercentageAuto> for Dimension {
    fn from(input: LengthPercentageAuto) -> Self {
        Self(input.0)
    }
}

impl Dimension {
    /// Get Length value if value is Length variant
    #[cfg(feature = "grid")]
    pub fn into_option(self) -> Option<f32> {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => Some(self.0.value()),
            _ => None,
        }
    }
    /// Returns true if value is Auto
    #[inline(always)]
    pub fn is_auto(self) -> bool {
        self.0.is_auto()
    }

    /// Get the raw `CompactLength` tag
    pub fn tag(self) -> u64 {
        self.0.tag()
    }

    /// Get the raw `CompactLength` value for non-calc variants that have a numeric parameter
    pub fn value(self) -> f32 {
        self.0.value()
    }

    /// Get the underlying `CompactLength` representation of the value
    pub fn into_raw(self) -> CompactLength {
        self.0
    }

    /// Create a Dimension from a raw `CompactLength`.
    /// # Safety
    ///  CompactLength must represent a valid variant for Dimension
    #[allow(unsafe_code)]
    pub unsafe fn from_raw(val: CompactLength) -> Self {
        Self(val)
    }
}

impl Rect<Dimension> {
    /// Create a new Rect with length values
    #[must_use]
    pub const fn from_length(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension(CompactLength::length(start)),
            right: Dimension(CompactLength::length(end)),
            top: Dimension(CompactLength::length(top)),
            bottom: Dimension(CompactLength::length(bottom)),
        }
    }

    /// Create a new Rect with percentage values
    #[must_use]
    pub const fn from_percent(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension(CompactLength::percent(start)),
            right: Dimension(CompactLength::percent(end)),
            top: Dimension(CompactLength::percent(top)),
            bottom: Dimension(CompactLength::percent(bottom)),
        }
    }
}
