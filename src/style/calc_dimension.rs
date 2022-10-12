//! Dimensions that are have to be calculated while resolving.

use crate::resolve::MaybeResolve;

use super::Dimension;

/// A [`Dimension`] calculation.
///
/// The values are calulated when the point values are resolved.
#[derive(Clone, PartialEq, Debug)]
pub enum CalcDimension {
    /// Add two [`Dimension`]s together.
    Add(Box<Dimension>, Box<Dimension>),

    /// Subtract the right [`Dimension`] from the left one.
    Sub(Box<Dimension>, Box<Dimension>),

    /// Multiply a [`Dimension`] with a constant.
    Mul(Box<Dimension>, f32),

    /// Divide a [`Dimension`] by a constant.
    Div(Box<Dimension>, f32),
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
