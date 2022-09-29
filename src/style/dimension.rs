//! A unit of linear measurement.

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
