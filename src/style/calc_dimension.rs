//! Dimensions that are have to be calculated while resolving.

use crate::resolve::MaybeResolve;

use super::Dimension;

/// An addition, subtraction, multiplication or division between [`Dimension`]s.
///
/// The values are calulated when the point values are resolved.
///
/// This type is needed, because not all calculations can be determined before resolving the actual values.
/// For example, when adding [`Dimension::Points`] and [`Dimension::Percent`] together,
/// the percentage first needs to be resolved to an absolute value to get the final result.
#[derive(Clone, PartialEq, Debug)]
pub enum CalcDimension {
    /// Add two [`Dimension`]s together.
    Add(Dimension, Dimension),

    /// Subtract the right [`Dimension`] from the left one.
    Sub(Dimension, Dimension),

    /// Multiply a [`Dimension`] with a constant.
    Mul(Dimension, f32),

    /// Divide a [`Dimension`] by a constant.
    Div(Dimension, f32),
}

impl MaybeResolve<Option<f32>> for CalcDimension {
    /// Resolve the calculations to concrete values.
    ///
    /// If any side resolves to [`None`], [`None`] is also returned.
    fn maybe_resolve(&self, context: Option<f32>) -> Option<f32> {
        match self {
            // Resolve both sides and add them together
            // If either side resolves to `None`, return `None`
            CalcDimension::Add(lhs, rhs) => lhs
                .maybe_resolve(context)
                .zip(rhs.maybe_resolve(context))
                .map(|(lhs_value, rhs_value)| lhs_value + rhs_value),

            // Resolve both sides and subtract them
            // If either side resolves to `None`, return `None`
            CalcDimension::Sub(lhs, rhs) => lhs
                .maybe_resolve(context)
                .zip(rhs.maybe_resolve(context))
                .map(|(lhs_value, rhs_value)| lhs_value - rhs_value),

            // Resolve the left side and multiply it by the factor
            // If the left side resolves to `None`, return `None`
            CalcDimension::Mul(lhs, factor) => lhs.maybe_resolve(context).map(|lhs_value| lhs_value * factor),

            // Resolve the left side and divide it by the factor
            // If the left side resolves to `None`, return `None`
            CalcDimension::Div(lhs, divisor) => lhs.maybe_resolve(context).map(|lhs_value| lhs_value / divisor),
        }
    }
}
