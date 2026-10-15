//! Length resolution with Yoga's percentage arithmetic.
//!
//! Yoga stores a percentage as its percentage number and resolves it as `value * reference * 0.01f`
//! (`StyleLength::resolve`); Taffy stores the fraction and multiplies it directly. The two differ
//! in the last bit, which Yoga's flex passes can amplify (a frozen item's factor must cancel its
//! line total exactly), so the Yoga mode resolves lengths through these traits instead of
//! [`MaybeResolve`](crate::util::MaybeResolve) and [`ResolveOrZero`](crate::util::ResolveOrZero).

use crate::geometry::{Rect, Size};
use crate::style::{CompactLength, Dimension, LengthPercentage, LengthPercentageAuto};

/// `value * referenceLength * 0.01f` with `value` the percentage number Yoga stores
#[inline]
fn yoga_percent(fraction: f32, reference: f32) -> f32 {
    (fraction * 100.0) * reference * 0.01
}

/// Resolve a length or percentage with Yoga's percentage arithmetic
fn resolve_raw(raw: CompactLength, context: Option<f32>, calc: impl Fn(*const (), f32) -> f32) -> Option<f32> {
    match raw.tag() {
        CompactLength::LENGTH_TAG => Some(raw.value()),
        CompactLength::PERCENT_TAG => context.map(|dim| yoga_percent(raw.value(), dim)),
        #[cfg(feature = "calc")]
        _ if raw.is_plain_calc() => context.map(|dim| calc(raw.calc_value(), dim)),
        _ => {
            let _ = calc;
            None
        }
    }
}

/// [`MaybeResolve`](crate::util::MaybeResolve) with Yoga's percentage arithmetic
pub(crate) trait YogaResolve<In, Out> {
    /// Resolve against `context`, returning `None` for `auto` and for percentages of an undefined reference
    fn yoga_resolve(self, context: In, calc: impl Fn(*const (), f32) -> f32) -> Out;
}

/// [`ResolveOrZero`](crate::util::ResolveOrZero) with Yoga's percentage arithmetic
pub(crate) trait YogaResolveOrZero<In, Out> {
    /// Resolve against `context`, treating anything that does not resolve as zero
    fn yoga_resolve_or_zero(self, context: In, calc: impl Fn(*const (), f32) -> f32) -> Out;
}

impl YogaResolve<Option<f32>, Option<f32>> for LengthPercentage {
    fn yoga_resolve(self, context: Option<f32>, calc: impl Fn(*const (), f32) -> f32) -> Option<f32> {
        resolve_raw(self.into_raw(), context, calc)
    }
}

impl YogaResolve<Option<f32>, Option<f32>> for LengthPercentageAuto {
    fn yoga_resolve(self, context: Option<f32>, calc: impl Fn(*const (), f32) -> f32) -> Option<f32> {
        resolve_raw(self.into_raw(), context, calc)
    }
}

impl YogaResolve<Option<f32>, Option<f32>> for Dimension {
    fn yoga_resolve(self, context: Option<f32>, calc: impl Fn(*const (), f32) -> f32) -> Option<f32> {
        resolve_raw(self.into_raw(), context, calc)
    }
}

impl<T: YogaResolve<Option<f32>, Option<f32>>> YogaResolve<f32, Option<f32>> for T {
    fn yoga_resolve(self, context: f32, calc: impl Fn(*const (), f32) -> f32) -> Option<f32> {
        self.yoga_resolve(Some(context), calc)
    }
}

impl<In, Out, T: YogaResolve<In, Out>> YogaResolve<Size<In>, Size<Out>> for Size<T> {
    fn yoga_resolve(self, context: Size<In>, calc: impl Fn(*const (), f32) -> f32) -> Size<Out> {
        Size {
            width: self.width.yoga_resolve(context.width, &calc),
            height: self.height.yoga_resolve(context.height, &calc),
        }
    }
}

impl<In, T: YogaResolve<In, Option<f32>>> YogaResolveOrZero<In, f32> for T {
    fn yoga_resolve_or_zero(self, context: In, calc: impl Fn(*const (), f32) -> f32) -> f32 {
        self.yoga_resolve(context, calc).unwrap_or(0.0)
    }
}

impl<In, Out, T: YogaResolveOrZero<In, Out>> YogaResolveOrZero<Size<In>, Size<Out>> for Size<T> {
    fn yoga_resolve_or_zero(self, context: Size<In>, calc: impl Fn(*const (), f32) -> f32) -> Size<Out> {
        Size {
            width: self.width.yoga_resolve_or_zero(context.width, &calc),
            height: self.height.yoga_resolve_or_zero(context.height, &calc),
        }
    }
}

impl<In: Copy, Out, T: YogaResolveOrZero<In, Out>> YogaResolveOrZero<In, Rect<Out>> for Rect<T> {
    fn yoga_resolve_or_zero(self, context: In, calc: impl Fn(*const (), f32) -> f32) -> Rect<Out> {
        Rect {
            left: self.left.yoga_resolve_or_zero(context, &calc),
            right: self.right.yoga_resolve_or_zero(context, &calc),
            top: self.top.yoga_resolve_or_zero(context, &calc),
            bottom: self.bottom.yoga_resolve_or_zero(context, &calc),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_calc(_: *const (), _: f32) -> f32 {
        0.0
    }

    #[test]
    fn percent_uses_yoga_arithmetic() {
        // 68 * 113 * 0.01f, which differs from 0.68f * 113 in the last bit
        let yoga = 68.0f32 * 113.0 * 0.01;
        assert_eq!(LengthPercentage::percent(0.68).yoga_resolve(Some(113.0), no_calc), Some(yoga));
        assert_eq!(Dimension::percent(0.68).yoga_resolve(Some(113.0), no_calc), Some(yoga));
        assert_eq!(LengthPercentageAuto::percent(0.5).yoga_resolve(None, no_calc), None);
    }

    #[test]
    fn lengths_and_auto() {
        assert_eq!(Dimension::length(10.0).yoga_resolve(None, no_calc), Some(10.0));
        assert_eq!(LengthPercentageAuto::auto().yoga_resolve(Some(100.0), no_calc), None);
        assert_eq!(LengthPercentageAuto::auto().yoga_resolve_or_zero(Some(100.0), no_calc), 0.0);
        assert_eq!(
            Rect {
                left: LengthPercentage::percent(0.1),
                right: LengthPercentage::length(1.0),
                top: LengthPercentage::length(2.0),
                bottom: LengthPercentage::percent(0.5)
            }
            .yoga_resolve_or_zero(Some(50.0), no_calc),
            Rect { left: 10.0 * 50.0 * 0.01, right: 1.0, top: 2.0, bottom: 50.0 * 50.0 * 0.01 }
        );
    }
}
