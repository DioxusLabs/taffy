//! Generic code that is shared between multiple layout algorithms
pub(crate) mod alignment;
pub(crate) mod sizing_keyword;

/// Tolerance used when checking whether boxes fit in a space along an axis, to absorb `f32`
/// rounding errors: sizes that sum to exactly the available space (percentages that add up to
/// 100%, or a container sized to the max-content sum of the very items it then wraps, summed in
/// a different order) may otherwise exceed it by an ulp and spuriously wrap
pub(crate) const FIT_TOLERANCE: f32 = 0.001;

#[cfg(feature = "content_size")]
pub(crate) mod scrollable_overflow;
