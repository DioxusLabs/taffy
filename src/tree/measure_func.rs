//! Measure function type and trait definitions

use crate::geometry::Size;
use crate::style::AvailableSpace;
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::util::sys::Box;

/// Represents a node that can be sized / laid out, where the sizing/layout logic exists outside of Taffy. For example,
/// a text node where the text layout logic is implemented by the user.
///
/// This trait is implemented by the `MeasureFunc` and `SyncMeasureFunc` types which provide a bridge between this trait
/// and raw functions / `FnMut` closures.
pub trait Measurable {
    /// A user-defined context which is passed to taffy when the `compute_layout` function is called, and which Taffy then passes
    /// into measure functions when it calls them
    type Context;

    /// Measure node
    fn measure(
        &mut self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        context: &mut Self::Context,
    ) -> Size<f32>;
}

/// A function that can be used to compute the intrinsic size of a node
#[allow(clippy::type_complexity)]
pub enum MeasureFunc<Context = ()> {
    /// Stores an unboxed function
    Raw(fn(Size<Option<f32>>, Size<AvailableSpace>, &mut Context) -> Size<f32>),

    /// Stores a boxed function
    #[cfg(any(feature = "std", feature = "alloc"))]
    Boxed(Box<dyn FnMut(Size<Option<f32>>, Size<AvailableSpace>, &mut Context) -> Size<f32>>),
}

/// A function that can be used to compute the intrinsic size of a node
#[allow(clippy::type_complexity)]
pub enum SyncMeasureFunc<Context = ()> {
    /// Stores an unboxed function
    Raw(fn(Size<Option<f32>>, Size<AvailableSpace>, &mut Context) -> Size<f32>),

    /// Stores a boxed function
    #[cfg(any(feature = "std", feature = "alloc"))]
    Boxed(Box<dyn FnMut(Size<Option<f32>>, Size<AvailableSpace>, &mut Context) -> Size<f32> + Send + Sync>),
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<Context> MeasureFunc<Context> {
    /// Constructor for the Boxed variant that takes a plain closure and handles the actual boxing
    pub fn boxed(
        measure: impl FnMut(Size<Option<f32>>, Size<AvailableSpace>, &mut Context) -> Size<f32> + 'static,
    ) -> Self {
        Self::Boxed(Box::new(measure))
    }
}

impl<Context> Measurable for MeasureFunc<Context> {
    type Context = Context;

    /// Call the measure function to measure the node
    #[inline(always)]
    fn measure(
        &mut self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        context: &mut Context,
    ) -> Size<f32> {
        match self {
            Self::Raw(measure) => measure(known_dimensions, available_space, context),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Self::Boxed(measure) => measure(known_dimensions, available_space, context),
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<Context> SyncMeasureFunc<Context> {
    /// Constructor for the Boxed variant that takes a plain closure and handles the actual boxing
    pub fn boxed(
        measure: impl FnMut(Size<Option<f32>>, Size<AvailableSpace>, &mut Context) -> Size<f32> + Send + Sync + 'static,
    ) -> Self {
        Self::Boxed(Box::new(measure))
    }
}

impl<Context> Measurable for SyncMeasureFunc<Context> {
    type Context = Context;

    /// Call the measure function to measure the node
    #[inline(always)]
    fn measure(
        &mut self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        context: &mut Context,
    ) -> Size<f32> {
        match self {
            Self::Raw(measure) => measure(known_dimensions, available_space, context),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Self::Boxed(measure) => measure(known_dimensions, available_space, context),
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
