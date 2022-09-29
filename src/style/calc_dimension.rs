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
    fn maybe_resolve(&self, context: Option<f32>) -> Option<f32> {
        match self {
            CalcDimension::Add(lhs, rhs) => lhs
                .maybe_resolve(context)
                .zip(rhs.maybe_resolve(context))
                .map(|(lhs_value, rhs_value)| lhs_value + rhs_value),

            CalcDimension::Sub(lhs, rhs) => lhs
                .maybe_resolve(context)
                .zip(rhs.maybe_resolve(context))
                .map(|(lhs_value, rhs_value)| lhs_value - rhs_value),

            CalcDimension::Mul(lhs, factor) => lhs.maybe_resolve(context).map(|lhs_value| lhs_value * factor),

            CalcDimension::Div(lhs, divisor) => lhs.maybe_resolve(context).map(|lhs_value| lhs_value / divisor),
        }
    }
}
