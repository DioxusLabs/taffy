//! Measure function type and trait definitions

use crate::geometry::Size;
use crate::style::AvailableSpace;
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::sys::Box;

/// A function type that can be used in a [`MeasureFunc`]
///
/// This trait is automatically implemented for all types (including closures) that define a function with the appropriate type signature.
pub trait Measurable: Send + Sync + Fn(Size<Option<f32>>, Size<AvailableSpace>) -> Size<f32> {}
impl<F: Send + Sync + Fn(Size<Option<f32>>, Size<AvailableSpace>) -> Size<f32>> Measurable for F {}

/// A function that can be used to compute the intrinsic size of a node
pub enum MeasureFunc {
    /// Stores an unboxed function
    Raw(fn(Size<Option<f32>>, Size<AvailableSpace>) -> Size<f32>),

    /// Stores a boxed function
    #[cfg(any(feature = "std", feature = "alloc"))]
    Boxed(Box<dyn Measurable>),
}

impl MeasureFunc {
    /// Call the measure function to measure to the node
    #[inline(always)]
    pub fn measure(&self, known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> Size<f32> {
        match self {
            MeasureFunc::Raw(measure) => measure(known_dimensions, available_space),
            #[cfg(any(feature = "std", feature = "alloc"))]
            MeasureFunc::Boxed(measure) => (measure as &dyn Fn(_, _) -> _)(known_dimensions, available_space),
        }
    }
}
