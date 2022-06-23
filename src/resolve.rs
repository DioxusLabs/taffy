//! Helper trait to calculate dimensions during layout resolution

use crate::prelude::{Dimension, Rect, Size};

/// Trait to encapsulate behaviour where we need to resolve from a
/// potentially context-dependent size or dimension into
/// a context-independent size or dimension.
///
/// Will return a default value if it unable to resolve.
pub(crate) trait ResolveOrDefault<TContext, TOutput> {
    /// Resolve a dimension that might be dependent on a context, with a default fallback value
    fn resolve_or_default(self, context: TContext) -> TOutput;
}

/// Trait to encapsulate behaviour where we need to resolve from a
/// potentially context-dependent size or dimension into
/// a context-independent size or dimension.
///
/// Will return a `None` if it unable to resolve.
pub(crate) trait MaybeResolve<T> {
    /// Resolve a dimension that might be dependent on a context, with `None` as fallback value
    fn maybe_resolve(self, context: T) -> T;
}

impl MaybeResolve<Option<f32>> for Dimension {
    /// Converts the given [`Dimension`] into a concrete value of points
    ///
    /// Can return `None`
    fn maybe_resolve(self, context: Option<f32>) -> Option<f32> {
        match self {
            Dimension::Points(points) => Some(points),
            // parent_dim * percent
            Dimension::Percent(percent) => context.map(|dim| dim * percent),
            _ => None,
        }
    }
}

impl MaybeResolve<Option<f32>> for Option<Dimension> {
    fn maybe_resolve(self, context: Option<f32>) -> Option<f32> {
        self.and_then(|d| d.maybe_resolve(context))
    }
}

impl MaybeResolve<Size<Option<f32>>> for Size<Dimension> {
    /// Converts any `parent`-relative values for size into an absolute size
    fn maybe_resolve(self, context: Size<Option<f32>>) -> Size<Option<f32>> {
        Size { width: self.width.maybe_resolve(context.width), height: self.height.maybe_resolve(context.height) }
    }
}

impl ResolveOrDefault<Option<f32>, f32> for Dimension {
    /// Will return a default value of result is evaluated to `None`
    fn resolve_or_default(self, context: Option<f32>) -> f32 {
        self.maybe_resolve(context).unwrap_or(0.0)
    }
}
impl ResolveOrDefault<Option<f32>, f32> for Option<Dimension> {
    /// Will return a default value of result is evaluated to `None`
    fn resolve_or_default(self, context: Option<f32>) -> f32 {
        self.and_then(|a| a.maybe_resolve(context)).unwrap_or(0.0)
    }
}

impl ResolveOrDefault<Size<Option<f32>>, Rect<f32>> for Rect<Dimension> {
    fn resolve_or_default(self, context: Size<Option<f32>>) -> Rect<f32> {
        Rect {
            start: self.start.resolve_or_default(context.width),
            end: self.end.resolve_or_default(context.width),
            top: self.top.resolve_or_default(context.height),
            bottom: self.bottom.resolve_or_default(context.height),
        }
    }
}

impl ResolveOrDefault<Size<Option<f32>>, Rect<f32>> for Rect<Option<Dimension>> {
    fn resolve_or_default(self, context: Size<Option<f32>>) -> Rect<f32> {
        Rect {
            start: self.start.resolve_or_default(context.width),
            end: self.end.resolve_or_default(context.width),
            top: self.top.resolve_or_default(context.height),
            bottom: self.bottom.resolve_or_default(context.height),
        }
    }
}

impl ResolveOrDefault<Option<f32>, Rect<f32>> for Rect<Dimension> {
    fn resolve_or_default(self, context: Option<f32>) -> Rect<f32> {
        Rect {
            start: self.start.resolve_or_default(context),
            end: self.end.resolve_or_default(context),
            top: self.top.resolve_or_default(context),
            bottom: self.bottom.resolve_or_default(context),
        }
    }
}
impl ResolveOrDefault<Option<f32>, Rect<f32>> for Rect<Option<Dimension>> {
    fn resolve_or_default(self, context: Option<f32>) -> Rect<f32> {
        Rect {
            start: self.start.resolve_or_default(context),
            end: self.end.resolve_or_default(context),
            top: self.top.resolve_or_default(context),
            bottom: self.bottom.resolve_or_default(context),
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

    mod maybe_resolve_option_dimension {

        use crate::resolve::MaybeResolve;
        use crate::style::Dimension;
        use rstest::rstest;

        /// `None` should always return `None`
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(None, None, None)]
        #[case(None, Some(5.0), None)]
        #[case(None, Some(-5.0), None)]
        #[case(None, Some(0.), None)]
        fn resolve_none(#[case] input: Option<Dimension>, #[case] context: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(input.maybe_resolve(context), expected);
        }

        /// `Dimension::Auto` should always return `None`
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(Some(Dimension::Auto), None, None)]
        #[case(Some(Dimension::Auto), Some(5.0), None)]
        #[case(Some(Dimension::Auto), Some(-5.0), None)]
        #[case(Some(Dimension::Auto), Some(0.), None)]
        fn resolve_auto(#[case] input: Option<Dimension>, #[case] context: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(input.maybe_resolve(context), expected);
        }

        /// `Dimension::Points` should always return `Some(f32)`
        /// where the f32 value is the inner value of the points.
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(Some(Dimension::Points(5.0)), None, Some(5.0))]
        #[case(Some(Dimension::Points(5.0)), Some(5.0), Some(5.0))]
        #[case(Some(Dimension::Points(5.0)), Some(-5.0), Some(5.0))]
        #[case(Some(Dimension::Points(5.0)), Some(0.), Some(5.0))]
        fn resolve_points(
            #[case] input: Option<Dimension>,
            #[case] context: Option<f32>,
            #[case] expected: Option<f32>,
        ) {
            assert_eq!(input.maybe_resolve(context), expected);
        }

        /// `Dimension::Percent` should return `None` if context is  `None`.
        /// Otherwise it should return `Some(f32)`
        /// where the f32 value is the inner value of the percent * context value.
        ///
        /// The parent / context __should__ affect the outcome.
        #[rstest]
        #[case(Some(Dimension::Percent(5.0)), None, None)]
        #[case(Some(Dimension::Percent(5.0)), Some(5.0), Some(25.0))]
        #[case(Some(Dimension::Percent(5.0)), Some(-5.0), Some(-25.0))]
        #[case(Some(Dimension::Percent(5.0)), Some(0.0), Some(0.0))]
        fn resolve_percent(
            #[case] input: Option<Dimension>,
            #[case] context: Option<f32>,
            #[case] expected: Option<f32>,
        ) {
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
        #[case(Size::AUTO, Size::NONE, Size::NONE)]
        #[case(Size::AUTO, Size::new(5.0, 5.0), Size::NONE)]
        #[case(Size::AUTO, Size::new(-5.0, -5.0), Size::NONE)]
        #[case(Size::AUTO, Size::new(0.0, 0.0), Size::NONE)]
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

    mod resolve_or_default_dimension_via_option_f32 {
        use crate::resolve::ResolveOrDefault;
        use crate::style::Dimension;
        use rstest::rstest;

        #[rstest]
        #[case(Dimension::Auto, None, 0.0)]
        #[case(Dimension::Auto, Some(5.0), 0.0)]
        #[case(Dimension::Auto, Some(-5.0), 0.0)]
        #[case(Dimension::Auto, Some(0.0), 0.0)]
        fn resolve_or_default_auto(#[case] input: Dimension, #[case] context: Option<f32>, #[case] expected: f32) {
            assert_eq!(input.resolve_or_default(context), expected);
        }
        #[rstest]
        #[case(Dimension::Points(5.0), None, 5.0)]
        #[case(Dimension::Points(5.0), Some(5.0), 5.0)]
        #[case(Dimension::Points(5.0), Some(-5.0), 5.0)]
        #[case(Dimension::Points(5.0), Some(0.0), 5.0)]
        fn resolve_or_default_points(#[case] input: Dimension, #[case] context: Option<f32>, #[case] expected: f32) {
            assert_eq!(input.resolve_or_default(context), expected);
        }
        #[rstest]
        #[case(Dimension::Percent(5.0), None, 0.0)]
        #[case(Dimension::Percent(5.0), Some(5.0), 25.0)]
        #[case(Dimension::Percent(5.0), Some(-5.0), -25.0)]
        #[case(Dimension::Percent(5.0), Some(0.0), 0.0)]
        fn resolve_or_default_percent(#[case] input: Dimension, #[case] context: Option<f32>, #[case] expected: f32) {
            assert_eq!(input.resolve_or_default(context), expected);
        }
    }

    mod resolve_or_default_option_dimension_via_option_f32 {
        use crate::resolve::ResolveOrDefault;
        use crate::style::Dimension;
        use rstest::rstest;

        /// `None` should always return `None`
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(None, None, 0.0)]
        #[case(None, Some(5.0), 0.0)]
        #[case(None, Some(-5.0), 0.0)]
        #[case(None, Some(0.), 0.0)]
        fn resolve_none(#[case] input: Option<Dimension>, #[case] context: Option<f32>, #[case] expected: f32) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        /// `Dimension::Auto` should always return `None`
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(Some(Dimension::Auto), None, 0.0)]
        #[case(Some(Dimension::Auto), Some(5.0), 0.0)]
        #[case(Some(Dimension::Auto), Some(-5.0), 0.0)]
        #[case(Some(Dimension::Auto), Some(0.), 0.0)]
        fn resolve_auto(#[case] input: Option<Dimension>, #[case] context: Option<f32>, #[case] expected: f32) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        /// `Dimension::Points` should always return `Some(f32)`
        /// where the f32 value is the inner value of the points.
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(Some(Dimension::Points(5.0)), None, 5.0)]
        #[case(Some(Dimension::Points(5.0)), Some(5.0), 5.0)]
        #[case(Some(Dimension::Points(5.0)), Some(-5.0), 5.0)]
        #[case(Some(Dimension::Points(5.0)), Some(0.), 5.0)]
        fn resolve_points(#[case] input: Option<Dimension>, #[case] context: Option<f32>, #[case] expected: f32) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        /// `Dimension::Percent` should return `None` if context is  `None`.
        /// Otherwise it should return `Some(f32)`
        /// where the f32 value is the inner value of the percent * context value.
        ///
        /// The parent / context __should__ affect the outcome.
        #[rstest]
        #[case(Some(Dimension::Percent(5.0)), None, 0.0)]
        #[case(Some(Dimension::Percent(5.0)), Some(5.0), 25.0)]
        #[case(Some(Dimension::Percent(5.0)), Some(-5.0), -25.0)]
        #[case(Some(Dimension::Percent(5.0)), Some(0.0), 0.0)]
        fn resolve_percent(#[case] input: Option<Dimension>, #[case] context: Option<f32>, #[case] expected: f32) {
            assert_eq!(input.resolve_or_default(context), expected);
        }
    }

    mod resolve_or_default_rect_dimension_via_size {
        use crate::geometry::{Rect, Size};
        use crate::resolve::ResolveOrDefault;
        use crate::style::Dimension;
        use rstest::rstest;

        #[rstest]
        #[case(Rect::<Dimension>::AUTO, Size::NONE, Rect::ZERO)]
        #[case(Rect::<Dimension>::AUTO, Size::new(5.0, 5.0), Rect::ZERO)]
        #[case(Rect::<Dimension>::AUTO, Size::new(-5.0, -5.0), Rect::ZERO)]
        #[case(Rect::<Dimension>::AUTO, Size::new(0.0, 0.0), Rect::ZERO)]
        fn resolve_or_default_auto(
            #[case] input: Rect<Dimension>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        #[rstest]
        #[case(Rect::<Dimension>::from_points(5.0, 5.0, 5.0, 5.0), Size::NONE, Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::<Dimension>::from_points(5.0, 5.0, 5.0, 5.0), Size::new(5.0, 5.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::<Dimension>::from_points(5.0, 5.0, 5.0, 5.0), Size::new(-5.0, -5.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::<Dimension>::from_points(5.0, 5.0, 5.0, 5.0), Size::new(0.0, 0.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        fn resolve_or_default_points(
            #[case] input: Rect<Dimension>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        #[rstest]
        #[case(Rect::<Dimension>::from_percent(5.0, 5.0, 5.0, 5.0), Size::NONE, Rect::ZERO)]
        #[case(Rect::<Dimension>::from_percent(5.0, 5.0, 5.0, 5.0), Size::new(5.0, 5.0), Rect::new(25.0, 25.0, 25.0, 25.0))]
        #[case(Rect::<Dimension>::from_percent(5.0, 5.0, 5.0, 5.0), Size::new(-5.0, -5.0), Rect::new(-25.0, -25.0, -25.0, -25.0))]
        #[case(Rect::<Dimension>::from_percent(5.0, 5.0, 5.0, 5.0), Size::new(0.0, 0.0), Rect::ZERO)]
        fn resolve_or_default_percent(
            #[case] input: Rect<Dimension>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }
    }

    mod resolve_or_default_rect_option_dimension_via_size {
        use crate::geometry::{Rect, Size};
        use crate::resolve::ResolveOrDefault;
        use crate::style::Dimension;
        use rstest::rstest;

        #[rstest]
        #[case(Rect::<Option<Dimension>>::NONE, Size::NONE, Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::NONE, Size::new(5.0, 5.0), Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::NONE, Size::new(-5.0, -5.0), Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::NONE, Size::new(0.0, 0.0), Rect::ZERO)]
        fn resolve_or_default_none(
            #[case] input: Rect<Option<Dimension>>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        #[rstest]
        #[case(Rect::<Option<Dimension>>::AUTO, Size::NONE, Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::AUTO, Size::new(5.0, 5.0), Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::AUTO, Size::new(-5.0, -5.0), Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::AUTO, Size::new(0.0, 0.0), Rect::ZERO)]
        fn resolve_or_default_auto(
            #[case] input: Rect<Option<Dimension>>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        #[rstest]
        #[case(Rect::<Option<Dimension>>::from_points(5.0, 5.0, 5.0, 5.0), Size::NONE, Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::<Option<Dimension>>::from_points(5.0, 5.0, 5.0, 5.0), Size::new(5.0, 5.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::<Option<Dimension>>::from_points(5.0, 5.0, 5.0, 5.0), Size::new(-5.0, -5.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::<Option<Dimension>>::from_points(5.0, 5.0, 5.0, 5.0), Size::new(0.0, 0.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        fn resolve_or_default_points(
            #[case] input: Rect<Option<Dimension>>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        #[rstest]
        #[case(Rect::<Option<Dimension>>::from_percent(5.0, 5.0, 5.0, 5.0), Size::NONE, Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::from_percent(5.0, 5.0, 5.0, 5.0), Size::new(5.0, 5.0), Rect::new(25.0, 25.0, 25.0, 25.0))]
        #[case(Rect::<Option<Dimension>>::from_percent(5.0, 5.0, 5.0, 5.0), Size::new(-5.0, -5.0), Rect::new(-25.0, -25.0, -25.0, -25.0))]
        #[case(Rect::<Option<Dimension>>::from_percent(5.0, 5.0, 5.0, 5.0), Size::new(0.0, 0.0), Rect::ZERO)]
        fn resolve_or_default_percent(
            #[case] input: Rect<Option<Dimension>>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }
    }

    mod resolve_or_default_rect_dimension_to_rect_f32_via_option {
        use crate::geometry::Rect;
        use crate::resolve::ResolveOrDefault;
        use crate::style::Dimension;
        use rstest::rstest;

        #[rstest]
        #[case(Rect::<Dimension>::AUTO, None, Rect::ZERO)]
        #[case(Rect::<Dimension>::AUTO, Some(5.0), Rect::ZERO)]
        #[case(Rect::<Dimension>::AUTO, Some(-5.0), Rect::ZERO)]
        #[case(Rect::<Dimension>::AUTO, Some(0.0), Rect::ZERO)]
        fn resolve_or_default_auto(
            #[case] input: Rect<Dimension>,
            #[case] context: Option<f32>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        #[rstest]
        #[case(Rect::<Dimension>::from_points(5.0, 5.0, 5.0, 5.0), None, Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::<Dimension>::from_points(5.0, 5.0, 5.0, 5.0), Some(5.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::<Dimension>::from_points(5.0, 5.0, 5.0, 5.0), Some(-5.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::<Dimension>::from_points(5.0, 5.0, 5.0, 5.0), Some(0.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        fn resolve_or_default_points(
            #[case] input: Rect<Dimension>,
            #[case] context: Option<f32>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        #[rstest]
        #[case(Rect::<Option<Dimension>>::from_percent(5.0, 5.0, 5.0, 5.0), None, Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::from_percent(5.0, 5.0, 5.0, 5.0), Some(5.0), Rect::new(25.0, 25.0, 25.0, 25.0))]
        #[case(Rect::<Option<Dimension>>::from_percent(5.0, 5.0, 5.0, 5.0), Some(-5.0), Rect::new(-25.0, -25.0, -25.0, -25.0))]
        #[case(Rect::<Option<Dimension>>::from_percent(5.0, 5.0, 5.0, 5.0), Some(0.0), Rect::ZERO)]
        fn resolve_or_default_percent(
            #[case] input: Rect<Option<Dimension>>,
            #[case] context: Option<f32>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }
    }

    mod resolve_or_default_rect_option_dimension_to_rect_f32_via_option {
        use crate::geometry::Rect;
        use crate::resolve::ResolveOrDefault;
        use crate::style::Dimension;
        use rstest::rstest;

        #[rstest]
        #[case(Rect::<Option<Dimension>>::NONE, None, Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::NONE, Some(5.0), Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::NONE, Some(-5.0), Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::NONE, Some(0.0), Rect::ZERO)]
        fn resolve_or_default_none(
            #[case] input: Rect<Option<Dimension>>,
            #[case] context: Option<f32>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        #[rstest]
        #[case(Rect::<Option<Dimension>>::AUTO, None, Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::AUTO, Some(5.0), Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::AUTO, Some(-5.0), Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::AUTO, Some(0.0), Rect::ZERO)]
        fn resolve_or_default_auto(
            #[case] input: Rect<Option<Dimension>>,
            #[case] context: Option<f32>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        #[rstest]
        #[case(Rect::<Option<Dimension>>::from_points(5.0, 5.0, 5.0, 5.0), None, Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::<Option<Dimension>>::from_points(5.0, 5.0, 5.0, 5.0), Some(5.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::<Option<Dimension>>::from_points(5.0, 5.0, 5.0, 5.0), Some(-5.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        #[case(Rect::<Option<Dimension>>::from_points(5.0, 5.0, 5.0, 5.0), Some(0.0), Rect::new(5.0, 5.0, 5.0, 5.0))]
        fn resolve_or_default_points(
            #[case] input: Rect<Option<Dimension>>,
            #[case] context: Option<f32>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }

        #[rstest]
        #[case(Rect::<Option<Dimension>>::from_percent(5.0, 5.0, 5.0, 5.0), None, Rect::ZERO)]
        #[case(Rect::<Option<Dimension>>::from_percent(5.0, 5.0, 5.0, 5.0), Some(5.0), Rect::new(25.0, 25.0, 25.0, 25.0))]
        #[case(Rect::<Option<Dimension>>::from_percent(5.0, 5.0, 5.0, 5.0), Some(-5.0), Rect::new(-25.0, -25.0, -25.0, -25.0))]
        #[case(Rect::<Option<Dimension>>::from_percent(5.0, 5.0, 5.0, 5.0), Some(0.0), Rect::ZERO)]
        fn resolve_or_default_percent(
            #[case] input: Rect<Option<Dimension>>,
            #[case] context: Option<f32>,
            #[case] expected: Rect<f32>,
        ) {
            assert_eq!(input.resolve_or_default(context), expected);
        }
    }
}
