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
    ///
    /// Usually, you don't want to construct this variant yourself.
    /// Instead use the `+`, `-`, `*` and `/` operators.
    ///
    /// # Example
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// # use taffy::style::CalcDimension;
    /// #
    /// let calc = Dimension::Points(10.0) + Dimension::Percent(50.0);
    /// let expected = Dimension::Calc(
    ///     CalcDimension::Add(
    ///         Box::new(Dimension::Points(10.0)),
    ///         Box::new(Dimension::Percent(50.0))
    ///     )
    /// );
    /// assert_eq!(calc, expected);
    /// ```
    ///
    /// The actual result is then calculated when resolving the absolute values.
    /// See [`Dimension::maybe_resolve`].
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

    /// Add two [`Dimension`]s together.
    ///
    /// If the result of the calculation can already be known, it is calculated immediately.
    /// Otherwise, variants of [`Dimension::Calc`] are constructed.
    ///
    /// # Examples
    ///
    /// When the result can already be known, it is calculated immediately:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// #
    /// let result = Dimension::Points(13.0) + Dimension::Points(5.5);
    /// assert_eq!(result, Dimension::Points(18.5));
    /// ```
    ///
    /// If any of the operants is [`Dimension::Undefined`], the result will be [`Dimension::Undefined`]:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// #
    /// let result = Dimension::Points(28.0) + Dimension::Undefined;
    /// assert_eq!(result, Dimension::Undefined);
    /// ```
    ///
    /// The behavior is analogue for [`Dimension::Auto`].
    ///
    /// If the result cannot be known beforehand, variants of [`Dimension::Calc`] will be constructed:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// # use taffy::style::CalcDimension;
    /// #
    /// let result = Dimension::Points(5.0) + Dimension::Percent(80.0);
    /// let expected = Dimension::Calc(
    ///     CalcDimension::Add(
    ///         Box::new(Dimension::Points(5.0)),
    ///         Box::new(Dimension::Percent(80.0))
    ///     )
    /// );
    /// assert_eq!(result, expected);
    /// ```
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

    /// Subtract one [`Dimension`] from another.
    ///
    /// If the result of the calculation can already be known, it is calculated immediately.
    /// Otherwise, variants of [`Dimension::Calc`] are constructed.
    ///
    /// # Examples
    ///
    /// When the result can already be known, it is calculated immediately:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// #
    /// let result = Dimension::Points(13.0) - Dimension::Points(5.5);
    /// assert_eq!(result, Dimension::Points(7.5));
    /// ```
    ///
    /// If any of the operants is [`Dimension::Undefined`], the result will be [`Dimension::Undefined`]:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// #
    /// let result = Dimension::Points(28.0) - Dimension::Undefined;
    /// assert_eq!(result, Dimension::Undefined);
    /// ```
    ///
    /// The behavior is analogue for [`Dimension::Auto`].
    ///
    /// If the result cannot be known beforehand, variants of [`Dimension::Calc`] will be constructed:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// # use taffy::style::CalcDimension;
    /// #
    /// let result = Dimension::Percent(80.0) - Dimension::Points(5.0);
    /// let expected = Dimension::Calc(
    ///     CalcDimension::Sub(
    ///         Box::new(Dimension::Percent(80.0)),
    ///         Box::new(Dimension::Points(5.0))
    ///     )
    /// );
    /// assert_eq!(result, expected);
    /// ```
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

    /// Multiply a [`Dimension`] with a constant factor.
    ///
    /// If the result of the calculation can already be known, it is calculated immediately.
    /// Otherwise, variants of [`Dimension::Calc`] are constructed.
    ///
    /// # Examples
    ///
    /// When the result can already be known, it is calculated immediately:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// #
    /// let result = Dimension::Points(2.0) * 5.0;
    /// assert_eq!(result, Dimension::Points(10.0));
    /// ```
    ///
    /// If the value is [`Dimension::Undefined`], the result will be [`Dimension::Undefined`]:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// #
    /// let result = Dimension::Undefined * 3.0;
    /// assert_eq!(result, Dimension::Undefined);
    /// ```
    ///
    /// The behavior is analogue for [`Dimension::Auto`].
    ///
    /// If the result cannot be known beforehand, variants of [`Dimension::Calc`] will be constructed:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// # use taffy::style::CalcDimension;
    /// #
    /// let lhs = Dimension::Calc(
    ///     CalcDimension::Sub(
    ///         Box::new(Dimension::Percent(80.0)),
    ///         Box::new(Dimension::Points(5.0))
    ///     )
    /// );
    /// let result = lhs * 3.0;
    /// let expected = Dimension::Calc(
    ///     CalcDimension::Mul(
    ///         Box::new(
    ///             Dimension::Calc(
    ///                 CalcDimension::Sub(
    ///                     Box::new(Dimension::Percent(80.0)),
    ///                     Box::new(Dimension::Points(5.0))
    ///                 )
    ///             )
    ///         ),
    ///         3.0
    ///     )
    /// );
    /// assert_eq!(result, expected);
    /// ```
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

    /// Multiply a constant factor with a [`Dimension`].
    ///
    /// If the result of the calculation can already be known, it is calculated immediately.
    /// Otherwise, variants of [`Dimension::Calc`] are constructed.
    ///
    /// # Examples
    ///
    /// When the result can already be known, it is calculated immediately:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// #
    /// let result = 5.0 * Dimension::Points(2.0);
    /// assert_eq!(result, Dimension::Points(10.0));
    /// ```
    ///
    /// If the value is [`Dimension::Undefined`], the result will be [`Dimension::Undefined`]:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// #
    /// let result = 3.0 * Dimension::Undefined;
    /// assert_eq!(result, Dimension::Undefined);
    /// ```
    ///
    /// The behavior is analogue for [`Dimension::Auto`].
    ///
    /// If the result cannot be known beforehand, variants of [`Dimension::Calc`] will be constructed:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// # use taffy::style::CalcDimension;
    /// #
    /// let lhs = Dimension::Calc(
    ///     CalcDimension::Sub(
    ///         Box::new(Dimension::Percent(80.0)),
    ///         Box::new(Dimension::Points(5.0))
    ///     )
    /// );
    /// let result = 3.0 * lhs;
    /// let expected = Dimension::Calc(
    ///     CalcDimension::Mul(
    ///         Box::new(
    ///             Dimension::Calc(
    ///                 CalcDimension::Sub(
    ///                     Box::new(Dimension::Percent(80.0)),
    ///                     Box::new(Dimension::Points(5.0))
    ///                 )
    ///             )
    ///         ),
    ///         3.0
    ///     )
    /// );
    /// assert_eq!(result, expected);
    /// ```
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

    /// Divide a [`Dimension`] by a constant factor.
    ///
    /// If the result of the calculation can already be known, it is calculated immediately.
    /// Otherwise, variants of [`Dimension::Calc`] are constructed.
    ///
    /// # Examples
    ///
    /// When the result can already be known, it is calculated immediately:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// #
    /// let result = Dimension::Points(10.0) / 2.0;
    /// assert_eq!(result, Dimension::Points(5.0));
    /// ```
    ///
    /// If the value is [`Dimension::Undefined`], the result will be [`Dimension::Undefined`]:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// #
    /// let result = Dimension::Undefined / 3.0;
    /// assert_eq!(result, Dimension::Undefined);
    /// ```
    ///
    /// The behavior is analogue for [`Dimension::Auto`].
    ///
    /// If the result cannot be known beforehand, variants of [`Dimension::Calc`] will be constructed:
    ///
    /// ```
    /// # use taffy::style::Dimension;
    /// # use taffy::style::CalcDimension;
    /// #
    /// let lhs = Dimension::Calc(
    ///     CalcDimension::Sub(
    ///         Box::new(Dimension::Percent(80.0)),
    ///         Box::new(Dimension::Points(5.0))
    ///     )
    /// );
    /// let result = lhs / 3.0;
    /// let expected = Dimension::Calc(
    ///     CalcDimension::Div(
    ///         Box::new(
    ///             Dimension::Calc(
    ///                 CalcDimension::Sub(
    ///                     Box::new(Dimension::Percent(80.0)),
    ///                     Box::new(Dimension::Points(5.0))
    ///                 )
    ///             )
    ///         ),
    ///         3.0
    ///     )
    /// );
    /// assert_eq!(result, expected);
    /// ```
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
