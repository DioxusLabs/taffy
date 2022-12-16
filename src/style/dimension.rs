//! Style types for representing lengths / sizes

use crate::geometry::{Rect, Size};
use crate::style_helpers::{FromPercent, FromPoints, TaffyAuto, TaffyMaxContent, TaffyMinContent, TaffyZero};
use crate::sys::abs;

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LengthPercentage {
    /// Points are abstract absolute units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Points(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
}
impl TaffyZero for LengthPercentage {
    const ZERO: Self = Self::Points(0.0);
}
impl FromPoints for LengthPercentage {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Self::Points(points.into())
    }
}
impl FromPercent for LengthPercentage {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::Points(percent.into())
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LengthPercentageAuto {
    /// Points are abstract absolute units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Points(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
    /// The dimension should be automatically computed
    Auto,
}
impl TaffyZero for LengthPercentageAuto {
    const ZERO: Self = Self::Points(0.0);
}
impl TaffyAuto for LengthPercentageAuto {
    const AUTO: Self = Self::Auto;
}
impl FromPoints for LengthPercentageAuto {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Self::Points(points.into())
    }
}
impl FromPercent for LengthPercentageAuto {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::Points(percent.into())
    }
}

impl From<LengthPercentage> for LengthPercentageAuto {
    fn from(input: LengthPercentage) -> Self {
        match input {
            LengthPercentage::Points(value) => Self::Points(value),
            LengthPercentage::Percent(value) => Self::Percent(value),
        }
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Dimension {
    /// Points are abstract absolute units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Points(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
    /// The dimension should be automatically computed
    Auto,
}
impl TaffyZero for Dimension {
    const ZERO: Self = Self::Points(0.0);
}
impl TaffyAuto for Dimension {
    const AUTO: Self = Self::Auto;
}
impl FromPoints for Dimension {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Self::Points(points.into())
    }
}
impl FromPercent for Dimension {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::Points(percent.into())
    }
}

impl From<LengthPercentage> for Dimension {
    fn from(input: LengthPercentage) -> Self {
        match input {
            LengthPercentage::Points(value) => Self::Points(value),
            LengthPercentage::Percent(value) => Self::Percent(value),
        }
    }
}

impl From<LengthPercentageAuto> for Dimension {
    fn from(input: LengthPercentageAuto) -> Self {
        match input {
            LengthPercentageAuto::Points(value) => Self::Points(value),
            LengthPercentageAuto::Percent(value) => Self::Percent(value),
            LengthPercentageAuto::Auto => Self::Auto,
        }
    }
}

impl Dimension {
    /// Is this value defined?
    pub(crate) fn is_defined(self) -> bool {
        matches!(self, Dimension::Points(_) | Dimension::Percent(_))
    }

    /// Get Points value if value is Points variant
    #[cfg(feature = "experimental_grid")]
    pub(crate) fn into_option(self) -> Option<f32> {
        match self {
            Dimension::Points(value) => Some(value),
            _ => None,
        }
    }
}

impl Rect<Dimension> {
    /// Generates a [`Rect<Dimension>`] using [`Dimension::Points`] values for `start` and `top`
    #[must_use]
    pub const fn top_from_points(start: f32, top: f32) -> Rect<Dimension> {
        Rect { left: Dimension::Points(start), top: Dimension::Points(top), ..Rect::AUTO }
    }

    /// Generates a [`Rect<Dimension>`] using [`Dimension::Points`] values for `end` and `bottom`
    #[must_use]
    pub const fn bot_from_points(end: f32, bottom: f32) -> Rect<Dimension> {
        Rect { right: Dimension::Points(end), bottom: Dimension::Points(bottom), ..Rect::AUTO }
    }

    /// Generates a [`Rect<Dimension>`] using [`Dimension::Percent`] values for `start` and `top`
    #[must_use]
    pub const fn top_from_percent(start: f32, top: f32) -> Rect<Dimension> {
        Rect { left: Dimension::Percent(start), top: Dimension::Percent(top), ..Rect::AUTO }
    }

    /// Generates a [`Rect<Dimension>`] using [`Dimension::Percent`] values for `end` and `bottom`
    #[must_use]
    pub const fn bot_from_percent(end: f32, bottom: f32) -> Rect<Dimension> {
        Rect { right: Dimension::Percent(end), bottom: Dimension::Percent(bottom), ..Rect::AUTO }
    }

    /// Generates a [`Rect<Dimension>`] using [`Dimension::Auto`] for all values
    pub const AUTO: Rect<Dimension> =
        Self { left: Dimension::Auto, right: Dimension::Auto, top: Dimension::Auto, bottom: Dimension::Auto };

    /// Create a new Rect with [`Dimension::Points`]
    #[must_use]
    pub const fn from_points(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension::Points(start),
            right: Dimension::Points(end),
            top: Dimension::Points(top),
            bottom: Dimension::Points(bottom),
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

/// The amount of space available to a node in a given axis
/// https://www.w3.org/TR/css-sizing-3/#available
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AvailableSpace {
    /// The amount of space available is the specified number of pixels
    Definite(f32),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}
impl TaffyZero for AvailableSpace {
    const ZERO: Self = Self::Definite(0.0);
}
impl TaffyMaxContent for AvailableSpace {
    const MAX_CONTENT: Self = Self::MaxContent;
}
impl TaffyMinContent for AvailableSpace {
    const MIN_CONTENT: Self = Self::MinContent;
}
impl FromPoints for AvailableSpace {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Self::Definite(points.into())
    }
}

impl AvailableSpace {
    /// Returns true for definite values, else false
    pub fn is_definite(self) -> bool {
        matches!(self, AvailableSpace::Definite(_))
    }

    /// Convert to Option
    /// Definite values become Some(value). Contraints become None.
    pub fn into_option(self) -> Option<f32> {
        match self {
            AvailableSpace::Definite(value) => Some(value),
            _ => None,
        }
    }

    /// Return the definite value or a default value
    pub fn unwrap_or(self, default: f32) -> f32 {
        self.into_option().unwrap_or(default)
    }

    /// Return the definite value. Panic is the value is not definite.
    #[track_caller]
    pub fn unwrap(self) -> f32 {
        self.into_option().unwrap()
    }

    /// If passed value is Some then return AvailableSpace::Definite containing that value, else return self
    pub fn maybe_set(self, value: Option<f32>) -> AvailableSpace {
        match value {
            Some(value) => AvailableSpace::Definite(value),
            None => self,
        }
    }

    /// If passed value is Some then return AvailableSpace::Definite containing that value, else return self
    pub fn map_definite_value(self, map_function: impl FnOnce(f32) -> f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(value) => AvailableSpace::Definite(map_function(value)),
            _ => self,
        }
    }

    /// Compute free_space given the passed used_space
    pub fn compute_free_space(&self, used_space: f32) -> f32 {
        match self {
            AvailableSpace::MaxContent => f32::INFINITY,
            AvailableSpace::MinContent => 0.0,
            AvailableSpace::Definite(available_space) => available_space - used_space,
        }
    }

    /// Compare equality with another AvailableSpace, treating definite values
    /// that are within f32::EPSILON of each other as equal
    pub fn is_roughly_equal(self, other: AvailableSpace) -> bool {
        use AvailableSpace::*;
        match (self, other) {
            (Definite(a), Definite(b)) => abs(a - b) < f32::EPSILON,
            (MinContent, MinContent) => true,
            (MaxContent, MaxContent) => true,
            _ => false,
        }
    }
}

impl From<f32> for AvailableSpace {
    fn from(value: f32) -> Self {
        Self::Definite(value)
    }
}

impl From<Option<f32>> for AvailableSpace {
    fn from(option: Option<f32>) -> Self {
        match option {
            Some(value) => Self::Definite(value),
            None => Self::MaxContent,
        }
    }
}

impl Size<AvailableSpace> {
    /// Convert Size<AvailableSpace> into Size<Option<f32>>
    pub fn into_options(self) -> Size<Option<f32>> {
        Size { width: self.width.into_option(), height: self.height.into_option() }
    }

    /// If passed value is Some then return AvailableSpace::Definite containing that value, else return self
    pub fn maybe_set(self, value: Size<Option<f32>>) -> Size<AvailableSpace> {
        Size { width: self.width.maybe_set(value.width), height: self.height.maybe_set(value.height) }
    }
}
