//! A unit of linear measurement.

use core::ops::{Add, Div, Mul, Sub};

use super::CalcDimension;

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
/// The default value is [`Dimension::Undefined`].
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Dimension {
    /// The dimension is not given
    Undefined,

    /// The dimension should be automatically computed
    Auto,

    /// The dimension is stored in [points](https://en.wikipedia.org/wiki/Point_(typography))
    ///
    /// Each point is about 0.353 mm in size.
    Points(f32),

    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),

    /// A calculation of dimensions, similar to CSS's `calc()`.
    ///
    /// One use-case of this is to add a percentage value to a points value.
    Calc(CalcDimension),
}

impl Default for Dimension {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Dimension {
    /// Is this value defined?
    pub(crate) fn is_defined(&self) -> bool {
        matches!(self, Dimension::Points(_) | Dimension::Percent(_))
    }
}

impl Add<Dimension> for Dimension {
    type Output = Dimension;

    fn add(self, rhs: Dimension) -> Self::Output {
        match (self, rhs) {
            (Dimension::Undefined, _) | (_, Dimension::Undefined) => Dimension::Undefined,
            (Dimension::Auto, _) | (_, Dimension::Auto) => Dimension::Auto,
            (Dimension::Points(lhs), Dimension::Points(rhs)) => Dimension::Points(lhs + rhs),
            (Dimension::Percent(lhs), Dimension::Percent(rhs)) => Dimension::Percent(lhs + rhs),
            // Result can't be known in advance, defer calculation until resolving the actual values
            (lhs, rhs) => Dimension::Calc(CalcDimension::Add(Box::new(lhs), Box::new(rhs))),
        }
    }
}

impl Sub<Dimension> for Dimension {
    type Output = Dimension;

    fn sub(self, rhs: Dimension) -> Self::Output {
        match (self, rhs) {
            (Dimension::Undefined, _) | (_, Dimension::Undefined) => Dimension::Undefined,
            (Dimension::Auto, _) | (_, Dimension::Auto) => Dimension::Auto,
            (Dimension::Points(lhs), Dimension::Points(rhs)) => Dimension::Points(lhs - rhs),
            (Dimension::Percent(lhs), Dimension::Percent(rhs)) => Dimension::Percent(lhs - rhs),
            // Result can't be known in advance, defer calculation until resolving the actual values
            (lhs, rhs) => Dimension::Calc(CalcDimension::Sub(Box::new(lhs), Box::new(rhs))),
        }
    }
}

impl Mul<f32> for Dimension {
    type Output = Dimension;

    fn mul(self, factor: f32) -> Self::Output {
        match self {
            Dimension::Undefined => Dimension::Undefined,
            Dimension::Auto => Dimension::Auto,
            Dimension::Points(lhs) => Dimension::Points(lhs * factor),
            Dimension::Percent(lhs) => Dimension::Percent(lhs * factor),
            // Result can't be known in advance, defer calculation until resolving the actual values
            lhs => Dimension::Calc(CalcDimension::Mul(Box::new(lhs), factor)),
        }
    }
}

impl Mul<Dimension> for f32 {
    type Output = Dimension;

    fn mul(self, rhs: Dimension) -> Self::Output {
        match rhs {
            Dimension::Undefined => Dimension::Undefined,
            Dimension::Auto => Dimension::Auto,
            Dimension::Points(lhs) => Dimension::Points(lhs * self),
            Dimension::Percent(lhs) => Dimension::Percent(lhs * self),
            // Result can't be known in advance, defer calculation until resolving the actual values
            rhs => Dimension::Calc(CalcDimension::Mul(Box::new(rhs), self)),
        }
    }
}

impl Div<f32> for Dimension {
    type Output = Dimension;

    fn div(self, divisor: f32) -> Self::Output {
        match self {
            Dimension::Undefined => Dimension::Undefined,
            Dimension::Auto => Dimension::Auto,
            Dimension::Points(lhs) => Dimension::Points(lhs / divisor),
            Dimension::Percent(lhs) => Dimension::Percent(lhs / divisor),
            // Result can't be known in advance, defer calculation until resolving the actual values
            lhs => Dimension::Calc(CalcDimension::Div(Box::new(lhs), divisor)),
        }
    }
}
