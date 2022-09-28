use super::Dimension;

/// A [`Dimension`] calculation.
///
/// The values are calulated when the point values are resolved.
#[derive(Copy, Clone, PartialEq, Debug)]
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
