//! Measure function type and trait definitions

use crate::geometry::Size;
use crate::style::AvailableSpace;
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::util::sys::Box;

/// A function type that can be used in a [`MeasureFunc`]
///
/// This trait is automatically implemented for all types (including closures) that define a function with the appropriate type signature.
pub trait Measurable {
    /// A user-defined context which is passed to taffy when the `compute_layout` function is called, and which Taffy then passes
    /// into measure functions when it calls them
    type Context;

    /// Measure node
    fn measure(
        &self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        context: &mut Self::Context,
    ) -> Size<f32>;
}

/// A function that can be used to compute the intrinsic size of a node
pub enum MeasureFunc<Context = ()> {
    /// Stores an unboxed function
    #[allow(clippy::type_complexity)]
    Raw(fn(Size<Option<f32>>, Size<AvailableSpace>, context: &mut Context) -> Size<f32>),

    /// Stores a boxed function
    #[cfg(any(feature = "std", feature = "alloc"))]
    Boxed(Box<dyn Measurable<Context = Context>>),
}

impl<Context> Measurable for MeasureFunc<Context> {
    type Context = Context;

    /// Call the measure function to measure the node
    #[inline(always)]
    fn measure(
        &self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        context: &mut Context,
    ) -> Size<f32> {
        match self {
            Self::Raw(measure) => measure(known_dimensions, available_space, context),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Self::Boxed(measurable) => measurable.measure(known_dimensions, available_space, context),
        }
    }
}

/// A function that can be used to compute the intrinsic size of a node
pub enum SyncMeasureFunc<Context = ()> {
    /// Stores an unboxed function
    #[allow(clippy::type_complexity)]
    Raw(fn(Size<Option<f32>>, Size<AvailableSpace>, context: &mut Context) -> Size<f32>),

    /// Stores a boxed function
    #[cfg(any(feature = "std", feature = "alloc"))]
    Boxed(Box<dyn Measurable<Context = Context> + Send + Sync>),
}

impl<Context> Measurable for SyncMeasureFunc<Context> {
    type Context = Context;

    /// Call the measure function to measure the node
    #[inline(always)]
    fn measure(
        &self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        context: &mut Context,
    ) -> Size<f32> {
        match self {
            Self::Raw(measure) => measure(known_dimensions, available_space, context),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Self::Boxed(measurable) => measurable.measure(known_dimensions, available_space, context),
        }
    }
}

#[cfg(test)]
mod test {
    use super::SyncMeasureFunc;
    use crate::tree::Taffy;

    #[test]
    fn sync_measure_func_is_send_and_sync() {
        fn is_send_and_sync<T: Send + Sync>() {}
        is_send_and_sync::<SyncMeasureFunc>();
    }

    #[test]
    fn taffy_with_sync_measure_func_is_send_and_sync() {
        fn is_send_and_sync<T: Send + Sync>() {}
        is_send_and_sync::<Taffy<SyncMeasureFunc>>();
    }
}
