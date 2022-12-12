//! Style types for representing lengths / sizes

use crate::geometry::Rect;

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
