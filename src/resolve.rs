use crate::prelude::{Dimension, Rect, Size};

pub(crate) trait ResolveOrDefault<TContext, TOutput> {
    /// Resolves a value from something that can be either absolute or relative
    /// into one that is absolute.
    ///
    /// `context` is size of the parent
    fn resolve_or_default(self, context: TContext) -> TOutput;
}

pub(crate) trait MaybeResolve<T> {
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

#[cfg(test)]
mod tests {
    mod maybe_resolve_dimension {

        use crate::resolve::MaybeResolve;
        use crate::style::Dimension;
        use rstest::rstest;

        /// `Dimension::Undefined` should always return `None`
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(Dimension::Undefined, None, None)]
        #[case(Dimension::Undefined, Some(5.0), None)]
        #[case(Dimension::Undefined, Some(-5.0), None)]
        #[case(Dimension::Undefined, Some(0.), None)]
        fn resolve_undefined(#[case] input: Dimension, #[case] context: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(input.maybe_resolve(context), expected);
        }

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

        /// Size<Dimension::Undefined> should always return Size<None>
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(Size::from_undefined(), Size::from_none(), Size::from_none())]
        #[case(Size::from_undefined(), Size::from_some(5.0, 5.0), Size::from_none())]
        #[case(Size::from_undefined(), Size::from_some(-5.0, -5.0), Size::from_none())]
        #[case(Size::from_undefined(), Size::from_some(0.0, 0.0), Size::from_none())]
        fn maybe_resolve_undefined(
            #[case] input: Size<Dimension>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Size<Option<f32>>,
        ) {
            assert_eq!(input.maybe_resolve(context), expected);
        }

        /// Size<Dimension::Auto> should always return Size<None>
        ///
        /// The parent / context should not affect the outcome.
        #[rstest]
        #[case(Size::from_auto(), Size::from_none(), Size::from_none())]
        #[case(Size::from_auto(), Size::from_some(5.0, 5.0), Size::from_none())]
        #[case(Size::from_auto(), Size::from_some(-5.0, -5.0), Size::from_none())]
        #[case(Size::from_auto(), Size::from_some(0.0, 0.0), Size::from_none())]
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
        #[case(Size::from_points(5.0, 5.0), Size::from_none(), Size::from_some(5.0, 5.0))]
        #[case(Size::from_points(5.0, 5.0), Size::from_some(5.0, 5.0), Size::from_some(5.0, 5.0))]
        #[case(Size::from_points(5.0, 5.0), Size::from_some(-5.0, -5.0), Size::from_some(5.0, 5.0))]
        #[case(Size::from_points(5.0, 5.0), Size::from_some(0.0, 0.0), Size::from_some(5.0, 5.0))]
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
        #[case(Size::from_percent(5.0, 5.0), Size::from_none(), Size::from_none())]
        #[case(Size::from_percent(5.0, 5.0), Size::from_some(5.0, 5.0), Size::from_some(25.0, 25.0))]
        #[case(Size::from_percent(5.0, 5.0), Size::from_some(-5.0, -5.0), Size::from_some(-25.0, -25.0))]
        #[case(Size::from_percent(5.0, 5.0), Size::from_some(0.0, 0.0), Size::from_some(0.0, 0.0))]
        fn maybe_resolve_percent(
            #[case] input: Size<Dimension>,
            #[case] context: Size<Option<f32>>,
            #[case] expected: Size<Option<f32>>,
        ) {
            assert_eq!(input.maybe_resolve(context), expected);
        }
    }

    mod resolve_or_default_dimension_to_option_f32 {
        use crate::resolve::ResolveOrDefault;
        use crate::style::Dimension;
        use rstest::rstest;

        #[rstest]
        #[case(Dimension::Undefined, None, 0.0)]
        #[case(Dimension::Undefined, Some(5.0), 0.0)]
        #[case(Dimension::Undefined, Some(-5.0), 0.0)]
        #[case(Dimension::Undefined, Some(0.0), 0.0)]
        fn resolve_or_default_undefined(#[case] input: Dimension, #[case] context: Option<f32>, #[case] expected: f32) {
            assert_eq!(input.resolve_or_default(context), expected);
        }
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
}
