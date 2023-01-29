//! Helper trait to calculate dimensions during layout resolution

use crate::geometry::{Rect, Size};
use crate::style::{Dimension, LengthPercentage, LengthPercentageAuto};
use crate::style_helpers::TaffyZero;

/// Trait to encapsulate behaviour where we need to resolve from a
/// potentially context-dependent size or dimension into
/// a context-independent size or dimension.
///
/// Will return a `None` if it unable to resolve.
pub(crate) trait MaybeResolve<In, Out> {
    /// Resolve a dimension that might be dependent on a context, with `None` as fallback value
    fn maybe_resolve(self, context: In) -> Out;
}

/// Trait to encapsulate behaviour where we need to resolve from a
/// potentially context-dependent size or dimension into
/// a context-independent size or dimension.
///
/// Will return a default value if it unable to resolve.
pub(crate) trait ResolveOrZero<TContext, TOutput: TaffyZero> {
    /// Resolve a dimension that might be dependent on a context, with a default fallback value
    fn resolve_or_zero(self, context: TContext) -> TOutput;
}

impl MaybeResolve<Option<f32>, Option<f32>> for LengthPercentage {
    /// Converts the given [`LengthPercentage`] into a concrete value of points
    /// Can return `None`
    fn maybe_resolve(self, context: Option<f32>) -> Option<f32> {
        match self {
            LengthPercentage::Points(points) => Some(points),
            LengthPercentage::Percent(percent) => context.map(|dim| dim * percent),
        }
    }
}

impl MaybeResolve<Option<f32>, Option<f32>> for LengthPercentageAuto {
    /// Converts the given [`LengthPercentageAuto`] into a concrete value of points
    /// Can return `None`
    fn maybe_resolve(self, context: Option<f32>) -> Option<f32> {
        match self {
            LengthPercentageAuto::Points(points) => Some(points),
            LengthPercentageAuto::Percent(percent) => context.map(|dim| dim * percent),
            LengthPercentageAuto::Auto => None,
        }
    }
}

impl MaybeResolve<Option<f32>, Option<f32>> for Dimension {
    /// Converts the given [`Dimension`] into a concrete value of points
    ///
    /// Can return `None`
    fn maybe_resolve(self, context: Option<f32>) -> Option<f32> {
        match self {
            Dimension::Points(points) => Some(points),
            Dimension::Percent(percent) => context.map(|dim| dim * percent),
            Dimension::Auto => None,
        }
    }
}

// Generic implementation of MaybeResolve for f32 context where MaybeResolve is implemented
// for Option<f32> context
impl<T: MaybeResolve<Option<f32>, Option<f32>>> MaybeResolve<f32, Option<f32>> for T {
    /// Converts the given MaybeResolve value into a concrete value of points
    /// Can return `None`
    fn maybe_resolve(self, context: f32) -> Option<f32> {
        self.maybe_resolve(Some(context))
    }
}

// Generic MaybeResolve for Size
impl<In, Out, T: MaybeResolve<In, Out>> MaybeResolve<Size<In>, Size<Out>> for Size<T> {
    /// Converts any `parent`-relative values for size into an absolute size
    fn maybe_resolve(self, context: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_resolve(context.width), height: self.height.maybe_resolve(context.height) }
    }
}

impl ResolveOrZero<Option<f32>, f32> for LengthPercentage {
    /// Will return a default value of result is evaluated to `None`
    fn resolve_or_zero(self, context: Option<f32>) -> f32 {
        self.maybe_resolve(context).unwrap_or(0.0)
    }
}

impl ResolveOrZero<Option<f32>, f32> for LengthPercentageAuto {
    /// Will return a default value of result is evaluated to `None`
    fn resolve_or_zero(self, context: Option<f32>) -> f32 {
        self.maybe_resolve(context).unwrap_or(0.0)
    }
}

impl ResolveOrZero<Option<f32>, f32> for Dimension {
    /// Will return a default value of result is evaluated to `None`
    fn resolve_or_zero(self, context: Option<f32>) -> f32 {
        self.maybe_resolve(context).unwrap_or(0.0)
    }
}

// Generic ResolveOrZero for Size
impl<In, Out: TaffyZero, T: ResolveOrZero<In, Out>> ResolveOrZero<Size<In>, Size<Out>> for Size<T> {
    /// Converts any `parent`-relative values for size into an absolute size
    fn resolve_or_zero(self, context: Size<In>) -> Size<Out> {
        Size { width: self.width.resolve_or_zero(context.width), height: self.height.resolve_or_zero(context.height) }
    }
}

// Generic ResolveOrZero for resolving Rect against Size
impl<In: Copy, Out: TaffyZero, T: ResolveOrZero<In, Out>> ResolveOrZero<Size<In>, Rect<Out>> for Rect<T> {
    /// Converts any `parent`-relative values for Rect into an absolute Rect
    fn resolve_or_zero(self, context: Size<In>) -> Rect<Out> {
        Rect {
            left: self.left.resolve_or_zero(context.width),
            right: self.right.resolve_or_zero(context.width),
            top: self.top.resolve_or_zero(context.height),
            bottom: self.bottom.resolve_or_zero(context.height),
        }
    }
}

// Generic ResolveOrZero for resolving Rect against Option
impl<Out: TaffyZero, T: ResolveOrZero<Option<f32>, Out>> ResolveOrZero<Option<f32>, Rect<Out>> for Rect<T> {
    /// Converts any `parent`-relative values for Rect into an absolute Rect
    fn resolve_or_zero(self, context: Option<f32>) -> Rect<Out> {
        Rect {
            left: self.left.resolve_or_zero(context),
            right: self.right.resolve_or_zero(context),
            top: self.top.resolve_or_zero(context),
            bottom: self.bottom.resolve_or_zero(context),
        }
    }
}

#[cfg(test)]
mod tests {
    mod maybe_resolve_dimension {

        use crate::resolve::MaybeResolve;
        use crate::style::Dimension;
        use rstest::rstest;

        /// `Dimension::Auto` should always return `None`
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(Dimension::Auto, None, None)]
        #[case(Dimension::Auto, Some(5.0), None)]
        #[case(Dimension::Auto, Some(-5.0), None)]
        #[case(Dimension::Auto, Some(0.), None)]
        fn resolve_auto(#[case] input: Dimension, #[case] context: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(input.maybe_resolve(context), expected);
        }

        /// `Dimension::Points` should always return `Some(f32)`
        /// where the f32 value is the inner value of the points.
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(Dimension::Points(1.0), None, Some(1.0))]
        #[case(Dimension::Points(1.0), Some(5.0), Some(1.0))]
        #[case(Dimension::Points(1.0), Some(-5.0), Some(1.0))]
        #[case(Dimension::Points(1.0), Some(0.), Some(1.0))]
        fn resolve_points(#[case] input: Dimension, #[case] context: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(input.maybe_resolve(context), expected);
        }

        /// `Dimension::Percent` should return `None` if context is  `None`.
        /// Otherwise it should return `Some(f32)`
        /// where the f32 value is the inner value of the percent * context value.
        ///
        /// The parent / context __should__ affect the outcome.
        #[rstest]
        #[case(Dimension::Percent(1.0), None, None)]
        #[case(Dimension::Percent(1.0), Some(5.0), Some(5.0))]
        #[case(Dimension::Percent(1.0), Some(-5.0), Some(-5.0))]
        #[case(Dimension::Percent(1.0), Some(50.0), Some(50.0))]
        fn resolve_percent(#[case] input: Dimension, #[case] context: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(input.maybe_resolve(context), expected);
        }
    }

    mod maybe_resolve_size_dimension {
        use crate::{prelude::Size, resolve::MaybeResolve, style::Dimension};
        use rstest::rstest;

        /// Size<Dimension::Auto> should always return Size<None>
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(Size::auto(), Size::NONE, Size::NONE)]
        #[case(Size::auto(), Size::new(5.0, 5.0), Size::NONE)]
        #[case(Size::auto(), Size::new(-5.0, -5.0), Size::NONE)]
        #[case(Size::auto(), Size::new(0.0, 0.0), Size::NONE)]
        fn maybe_resolve_auto(
            #[case] input: Size<Dimension>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Size<Option<f32>>,
        ) {
            assert_eq!(input.maybe_resolve(context), expected);
        }

        /// Size<Dimension::Points> should always return a Size<Some(f32)>
        /// where the f32 values are the inner value of the points.
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(Size::from_points(5.0, 5.0), Size::NONE, Size::new(5.0, 5.0))]
        #[case(Size::from_points(5.0, 5.0), Size::new(5.0, 5.0), Size::new(5.0, 5.0))]
        #[case(Size::from_points(5.0, 5.0), Size::new(-5.0, -5.0), Size::new(5.0, 5.0))]
        #[case(Size::from_points(5.0, 5.0), Size::new(0.0, 0.0), Size::new(5.0, 5.0))]
        fn maybe_resolve_points(
            #[case] input: Size<Dimension>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Size<Option<f32>>,
        ) {
            assert_eq!(input.maybe_resolve(context), expected);
        }

        /// `Size<Dimension::Percent>` should return `Size<None>` if context is `Size<None>`.
        /// Otherwise it should return `Size<Some(f32)>`
        /// where the f32 value is the inner value of the percent * context value.
        ///
        /// The context __should__ affect the outcome.
        #[rstest]
        #[case(Size::from_percent(5.0, 5.0), Size::NONE, Size::NONE)]
        #[case(Size::from_percent(5.0, 5.0), Size::new(5.0, 5.0), Size::new(25.0, 25.0))]
        #[case(Size::from_percent(5.0, 5.0), Size::new(-5.0, -5.0), Size::new(-25.0, -25.0))]
        #[case(Size::from_percent(5.0, 5.0), Size::new(0.0, 0.0), Size::new(0.0, 0.0))]
        fn maybe_resolve_percent(
            #[case] input: Size<Dimension>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Size<Option<f32>>,
        ) {
            assert_eq!(input.maybe_resolve(context), expected);
        }
    }

    mod resolve_or_zero_dimension_to_option_f32 {
        use crate::resolve::ResolveOrZero;
        use crate::style::Dimension;
        use rstest::rstest;

        #[rstest]
        #[case(Dimension::Auto, None, 0.0)]
        #[case(Dimension::Auto, Some(5.0), 0.0)]
        #[case(Dimension::Auto, Some(-5.0), 0.0)]
        #[case(Dimension::Auto, Some(0.0), 0.0)]
        fn resolve_or_zero_auto(#[case] input: Dimension, #[case] context: Option<f32>, #[case] expected: f32) {
            assert_eq!(input.resolve_or_zero(context), expected);
        }
        #[rstest]
        #[case(Dimension::Points(5.0), None, 5.0)]
        #[case(Dimension::Points(5.0), Some(5.0), 5.0)]
        #[case(Dimension::Points(5.0), Some(-5.0), 5.0)]
        #[case(Dimension::Points(5.0), Some(0.0), 5.0)]
        fn resolve_or_zero_points(#[case] input: Dimension, #[case] context: Option<f32>, #[case] expected: f32) {
            assert_eq!(input.resolve_or_zero(context), expected);
        }
        #[rstest]
        #[case(Dimension::Percent(5.0), None, 0.0)]
        #[case(Dimension::Percent(5.0), Some(5.0), 25.0)]
        #[case(Dimension::Percent(5.0), Some(-5.0), -25.0)]
        #[case(Dimension::Percent(5.0), Some(0.0), 0.0)]
        fn resolve_or_zero_percent(#[case] input: Dimension, #[case] context: Option<f32>, #[case] expected: f32) {
            assert_eq!(input.resolve_or_zero(context), expected);
        }
    }

    mod resolve_or_zero_rect_dimension_to_rect {
        use crate::geometry::{Rect, Size};
        use crate::resolve::ResolveOrZero;
        use crate::style::Dimension;
        use rstest::rstest;

        #[rstest]
        #[case(Rect::auto(), Size::NONE, Rect::zero())]
        #[case(Rect::auto(), Size::new(5.0, 5.0), Rect::zero())]
        #[case(Rect::auto(), Size::new(-5.0, -5.0), Rect::zero())]
        #[case(Rect::auto(), Size::new(0.0, 0.0), Rect::zero())]
        fn resolve_or_zero_auto(
            #[case] input: Rect<Dimension>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_zero(context), expected);
        }

        #[rstest]
        #[case(Rect::from_points(5.0, 5.0, 5.0, 5.0), Size::NONE, Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::from_points(5.0, 5.0, 5.0, 5.0), Size::new(5.0, 5.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::from_points(5.0, 5.0, 5.0, 5.0), Size::new(-5.0, -5.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::from_points(5.0, 5.0, 5.0, 5.0), Size::new(0.0, 0.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        fn resolve_or_zero_points(
            #[case] input: Rect<Dimension>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_zero(context), expected);
        }

        #[rstest]
        #[case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), Size::NONE, Rect::zero())]
        #[case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), Size::new(5.0, 5.0), Rect::new(25.0, 25.0, 25.0, 25.0))]
        #[case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), Size::new(-5.0, -5.0), Rect::new(-25.0, -25.0, -25.0, -25.0))]
        #[case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), Size::new(0.0, 0.0), Rect::zero())]
        fn resolve_or_zero_percent(
            #[case] input: Rect<Dimension>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_zero(context), expected);
        }
    }

    mod resolve_or_zero_rect_dimension_to_rect_f32_via_option {
        use crate::geometry::Rect;
        use crate::resolve::ResolveOrZero;
        use crate::style::Dimension;
        use rstest::rstest;

        #[rstest]
        #[case(Rect::auto(), None, Rect::zero())]
        #[case(Rect::auto(), Some(5.0), Rect::zero())]
        #[case(Rect::auto(), Some(-5.0), Rect::zero())]
        #[case(Rect::auto(), Some(0.0), Rect::zero())]
        fn resolve_or_zero_auto(
            #[case] input: Rect<Dimension>,
            #[case] context: Option<f32>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_zero(context), expected);
        }

        #[rstest]
        #[case(Rect::from_points(5.0, 5.0, 5.0, 5.0), None, Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::from_points(5.0, 5.0, 5.0, 5.0), Some(5.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::from_points(5.0, 5.0, 5.0, 5.0), Some(-5.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::from_points(5.0, 5.0, 5.0, 5.0), Some(0.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        fn resolve_or_zero_points(
            #[case] input: Rect<Dimension>,
            #[case] context: Option<f32>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_zero(context), expected);
        }

        #[rstest]
        #[case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), None, Rect::zero())]
        #[case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), Some(5.0), Rect::new(25.0, 25.0, 25.0, 25.0))]
        #[case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), Some(-5.0), Rect::new(-25.0, -25.0, -25.0, -25.0))]
        #[case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), Some(0.0), Rect::zero())]
        fn resolve_or_zero_percent(
            #[case] input: Rect<Dimension>,
            #[case] context: Option<f32>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_zero(context), expected);
        }
    }
}
