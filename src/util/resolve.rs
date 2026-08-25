//! Helper trait to calculate dimensions during layout resolution

use crate::geometry::{Rect, Size};
use crate::style::{Dimension, LengthPercentage, LengthPercentageAuto};
use crate::style_helpers::TaffyZero;
use crate::util::OptF32;
use crate::CompactLength;

/// Trait to encapsulate behaviour where we need to resolve from a
/// potentially context-dependent size or dimension into
/// a context-independent size or dimension.
///
/// Will return a `None` if it unable to resolve.
pub trait MaybeResolve<In, Out> {
    /// Resolve a dimension that might be dependent on a context, with `None` as fallback value
    fn maybe_resolve(self, context: In, calc: impl Fn(*const (), f32) -> f32) -> Out;
}

/// Trait to encapsulate behaviour where we need to resolve from a
/// potentially context-dependent size or dimension into
/// a context-independent size or dimension.
///
/// Will return a default value if it unable to resolve.
pub trait ResolveOrZero<TContext, TOutput: TaffyZero> {
    /// Resolve a dimension that might be dependent on a context, with a default fallback value
    fn resolve_or_zero(self, context: TContext, calc: impl Fn(*const (), f32) -> f32) -> TOutput;
}

impl MaybeResolve<OptF32, OptF32> for LengthPercentage {
    /// Converts the given [`LengthPercentage`] into an absolute length
    /// Can return `None`
    fn maybe_resolve(self, context: OptF32, calc: impl Fn(*const (), f32) -> f32) -> OptF32 {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => OptF32::some(self.0.value()),
            CompactLength::PERCENT_TAG => context.map(|dim| dim * self.0.value()),
            #[cfg(feature = "calc")]
            _ if self.0.is_calc() => context.map(|dim| calc(self.0.calc_value(), dim)),
            _ => unreachable!(),
        }
    }
}

impl MaybeResolve<OptF32, OptF32> for LengthPercentageAuto {
    /// Converts the given [`LengthPercentageAuto`] into an absolute length
    /// Can return `None`
    fn maybe_resolve(self, context: OptF32, calc: impl Fn(*const (), f32) -> f32) -> OptF32 {
        match self.0.tag() {
            CompactLength::AUTO_TAG => OptF32::NONE,
            CompactLength::LENGTH_TAG => OptF32::some(self.0.value()),
            CompactLength::PERCENT_TAG => context.map(|dim| dim * self.0.value()),
            #[cfg(feature = "calc")]
            _ if self.0.is_calc() => context.map(|dim| calc(self.0.calc_value(), dim)),
            _ => unreachable!(),
        }
    }
}

impl MaybeResolve<OptF32, OptF32> for Dimension {
    /// Converts the given [`Dimension`] into an absolute length
    ///
    /// Can return `None`
    fn maybe_resolve(self, context: OptF32, calc: impl Fn(*const (), f32) -> f32) -> OptF32 {
        match self.0.tag() {
            CompactLength::AUTO_TAG => OptF32::NONE,
            // The content keyword is only valid for flex-basis. In any other context it behaves as auto.
            CompactLength::CONTENT_TAG => OptF32::NONE,
            CompactLength::LENGTH_TAG => OptF32::some(self.0.value()),
            CompactLength::PERCENT_TAG => context.map(|dim| dim * self.0.value()),
            #[cfg(feature = "calc")]
            _ if self.0.is_calc() => context.map(|dim| calc(self.0.calc_value(), dim)),
            // Intrinsic sizing keywords cannot be resolved to a definite size out of context.
            // Layout algorithms that support them must handle them explicitly.
            _ if self.0.is_sizing_keyword() => OptF32::NONE,
            _ => unreachable!(),
        }
    }
}

// Generic implementation of MaybeResolve for f32 context where MaybeResolve is implemented
// for OptF32 context
impl<T: MaybeResolve<OptF32, OptF32>> MaybeResolve<f32, OptF32> for T {
    /// Converts the given MaybeResolve value into an absolute length
    /// Can return `None`
    fn maybe_resolve(self, context: f32, calc: impl Fn(*const (), f32) -> f32) -> OptF32 {
        self.maybe_resolve(OptF32::some(context), calc)
    }
}

// Generic MaybeResolve for Size
impl<In, Out, T: MaybeResolve<In, Out>> MaybeResolve<Size<In>, Size<Out>> for Size<T> {
    /// Converts any `parent`-relative values for size into an absolute size
    fn maybe_resolve(self, context: Size<In>, calc: impl Fn(*const (), f32) -> f32) -> Size<Out> {
        Size {
            width: self.width.maybe_resolve(context.width, &calc),
            height: self.height.maybe_resolve(context.height, &calc),
        }
    }
}

impl ResolveOrZero<OptF32, f32> for LengthPercentage {
    /// Will return a default value of result is evaluated to `None`
    fn resolve_or_zero(self, context: OptF32, calc: impl Fn(*const (), f32) -> f32) -> f32 {
        self.maybe_resolve(context, calc).unwrap_or(0.0)
    }
}

impl ResolveOrZero<OptF32, f32> for LengthPercentageAuto {
    /// Will return a default value of result is evaluated to `None`
    fn resolve_or_zero(self, context: OptF32, calc: impl Fn(*const (), f32) -> f32) -> f32 {
        self.maybe_resolve(context, calc).unwrap_or(0.0)
    }
}

impl ResolveOrZero<OptF32, f32> for Dimension {
    /// Will return a default value of result is evaluated to `None`
    fn resolve_or_zero(self, context: OptF32, calc: impl Fn(*const (), f32) -> f32) -> f32 {
        self.maybe_resolve(context, calc).unwrap_or(0.0)
    }
}

// Generic ResolveOrZero for Size
impl<In, Out: TaffyZero, T: ResolveOrZero<In, Out>> ResolveOrZero<Size<In>, Size<Out>> for Size<T> {
    /// Converts any `parent`-relative values for size into an absolute size
    fn resolve_or_zero(self, context: Size<In>, calc: impl Fn(*const (), f32) -> f32) -> Size<Out> {
        Size {
            width: self.width.resolve_or_zero(context.width, &calc),
            height: self.height.resolve_or_zero(context.height, &calc),
        }
    }
}

// Generic ResolveOrZero for resolving Rect against Size
impl<In: Copy, Out: TaffyZero, T: ResolveOrZero<In, Out>> ResolveOrZero<Size<In>, Rect<Out>> for Rect<T> {
    /// Converts any `parent`-relative values for Rect into an absolute Rect
    fn resolve_or_zero(self, context: Size<In>, calc: impl Fn(*const (), f32) -> f32) -> Rect<Out> {
        Rect {
            left: self.left.resolve_or_zero(context.width, &calc),
            right: self.right.resolve_or_zero(context.width, &calc),
            top: self.top.resolve_or_zero(context.height, &calc),
            bottom: self.bottom.resolve_or_zero(context.height, &calc),
        }
    }
}

// Generic ResolveOrZero for resolving Rect against Option
impl<Out: TaffyZero, T: ResolveOrZero<OptF32, Out>> ResolveOrZero<OptF32, Rect<Out>> for Rect<T> {
    /// Converts any `parent`-relative values for Rect into an absolute Rect
    fn resolve_or_zero(self, context: OptF32, calc: impl Fn(*const (), f32) -> f32) -> Rect<Out> {
        Rect {
            left: self.left.resolve_or_zero(context, &calc),
            right: self.right.resolve_or_zero(context, &calc),
            top: self.top.resolve_or_zero(context, &calc),
            bottom: self.bottom.resolve_or_zero(context, &calc),
        }
    }
}

// Generic ResolveOrZero for resolving Rect against f32
impl<Out: TaffyZero, T: ResolveOrZero<f32, Out>> ResolveOrZero<f32, Rect<Out>> for Rect<T> {
    /// Converts any `parent`-relative values for Rect into an absolute Rect
    fn resolve_or_zero(self, context: f32, calc: impl Fn(*const (), f32) -> f32) -> Rect<Out> {
        Rect {
            left: self.left.resolve_or_zero(context, &calc),
            right: self.right.resolve_or_zero(context, &calc),
            top: self.top.resolve_or_zero(context, &calc),
            bottom: self.bottom.resolve_or_zero(context, &calc),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MaybeResolve, ResolveOrZero};
    use crate::style_helpers::TaffyZero;
    use core::fmt::Debug;

    // MaybeResolve test runner
    fn mr_case<Lhs, Rhs, Out>(input: Lhs, context: Rhs, expected: Out)
    where
        Lhs: MaybeResolve<Rhs, Out>,
        Out: PartialEq + Debug,
    {
        assert_eq!(input.maybe_resolve(context, |_, _| 42.42), expected);
    }

    // ResolveOrZero test runner
    fn roz_case<Lhs, Rhs, Out>(input: Lhs, context: Rhs, expected: Out)
    where
        Lhs: ResolveOrZero<Rhs, Out>,
        Out: PartialEq + Debug + TaffyZero,
    {
        assert_eq!(input.resolve_or_zero(context, |_, _| 42.42), expected);
    }

    mod maybe_resolve_dimension {
        use super::mr_case;
        use crate::style::Dimension;
        use crate::style_helpers::*;
        use crate::util::OptF32;

        /// `Dimension::Auto` should always return `None`
        ///
        /// The parent / context should not affect the outcome.
        #[test]
        fn resolve_auto() {
            mr_case(Dimension::AUTO, OptF32::NONE, OptF32::NONE);
            mr_case(Dimension::AUTO, OptF32::some(5.0), OptF32::NONE);
            mr_case(Dimension::AUTO, OptF32::some(-5.0), OptF32::NONE);
            mr_case(Dimension::AUTO, OptF32::some(0.), OptF32::NONE);
        }

        /// `Dimension::Length` should always return `Some(f32)`
        /// where the f32 value is the inner absolute length.
        ///
        /// The parent / context should not affect the outcome.
        #[test]
        fn resolve_length() {
            mr_case(Dimension::from_length(1.0), OptF32::NONE, OptF32::some(1.0));
            mr_case(Dimension::from_length(1.0), OptF32::some(5.0), OptF32::some(1.0));
            mr_case(Dimension::from_length(1.0), OptF32::some(-5.0), OptF32::some(1.0));
            mr_case(Dimension::from_length(1.0), OptF32::some(0.), OptF32::some(1.0));
        }

        /// `Dimension::Percent` should return `None` if context is  `None`.
        /// Otherwise it should return `Some(f32)`
        /// where the f32 value is the inner value of the percent * context value.
        ///
        /// The parent / context __should__ affect the outcome.
        #[test]
        fn resolve_percent() {
            mr_case(Dimension::from_percent(1.0), OptF32::NONE, OptF32::NONE);
            mr_case(Dimension::from_percent(1.0), OptF32::some(5.0), OptF32::some(5.0));
            mr_case(Dimension::from_percent(1.0), OptF32::some(-5.0), OptF32::some(-5.0));
            mr_case(Dimension::from_percent(1.0), OptF32::some(50.0), OptF32::some(50.0));
        }
    }

    mod maybe_resolve_size_dimension {
        use super::mr_case;
        use crate::geometry::Size;
        use crate::style::Dimension;

        /// Size<Dimension::Auto> should always return Size<None>
        ///
        /// The parent / context should not affect the outcome.
        #[test]
        fn maybe_resolve_auto() {
            mr_case(Size::<Dimension>::auto(), Size::NONE, Size::NONE);
            mr_case(Size::<Dimension>::auto(), Size::new(5.0, 5.0), Size::NONE);
            mr_case(Size::<Dimension>::auto(), Size::new(-5.0, -5.0), Size::NONE);
            mr_case(Size::<Dimension>::auto(), Size::new(0.0, 0.0), Size::NONE);
        }

        /// Size<Dimension::Length> should always return a Size<Some(f32)>
        /// where the f32 values are the absolute length.
        ///
        /// The parent / context should not affect the outcome.
        #[test]
        fn maybe_resolve_length() {
            mr_case(Size::from_lengths(5.0, 5.0), Size::NONE, Size::new(5.0, 5.0));
            mr_case(Size::from_lengths(5.0, 5.0), Size::new(5.0, 5.0), Size::new(5.0, 5.0));
            mr_case(Size::from_lengths(5.0, 5.0), Size::new(-5.0, -5.0), Size::new(5.0, 5.0));
            mr_case(Size::from_lengths(5.0, 5.0), Size::new(0.0, 0.0), Size::new(5.0, 5.0));
        }

        /// `Size<Dimension::Percent>` should return `Size<None>` if context is `Size<None>`.
        /// Otherwise it should return `Size<Some(f32)>`
        /// where the f32 value is the inner value of the percent * context value.
        ///
        /// The context __should__ affect the outcome.
        #[test]
        fn maybe_resolve_percent() {
            mr_case(Size::from_percent(5.0, 5.0), Size::NONE, Size::NONE);
            mr_case(Size::from_percent(5.0, 5.0), Size::new(5.0, 5.0), Size::new(25.0, 25.0));
            mr_case(Size::from_percent(5.0, 5.0), Size::new(-5.0, -5.0), Size::new(-25.0, -25.0));
            mr_case(Size::from_percent(5.0, 5.0), Size::new(0.0, 0.0), Size::new(0.0, 0.0));
        }
    }

    mod resolve_or_zero_dimension_to_option_f32 {
        use super::roz_case;
        use crate::style::Dimension;
        use crate::style_helpers::*;
        use crate::util::OptF32;

        #[test]
        fn resolve_or_zero_auto() {
            roz_case(Dimension::AUTO, OptF32::NONE, 0.0);
            roz_case(Dimension::AUTO, OptF32::some(5.0), 0.0);
            roz_case(Dimension::AUTO, OptF32::some(-5.0), 0.0);
            roz_case(Dimension::AUTO, OptF32::some(0.0), 0.0);
        }
        #[test]
        fn resolve_or_zero_length() {
            roz_case(Dimension::from_length(5.0), OptF32::NONE, 5.0);
            roz_case(Dimension::from_length(5.0), OptF32::some(5.0), 5.0);
            roz_case(Dimension::from_length(5.0), OptF32::some(-5.0), 5.0);
            roz_case(Dimension::from_length(5.0), OptF32::some(0.0), 5.0);
        }
        #[test]
        fn resolve_or_zero_percent() {
            roz_case(Dimension::from_percent(5.0), OptF32::NONE, 0.0);
            roz_case(Dimension::from_percent(5.0), OptF32::some(5.0), 25.0);
            roz_case(Dimension::from_percent(5.0), OptF32::some(-5.0), -25.0);
            roz_case(Dimension::from_percent(5.0), OptF32::some(0.0), 0.0);
        }
    }

    mod resolve_or_zero_rect_dimension_to_rect {
        use super::roz_case;
        use crate::geometry::{Rect, Size};
        use crate::style::Dimension;

        #[test]
        fn resolve_or_zero_auto() {
            roz_case(Rect::<Dimension>::auto(), Size::NONE, Rect::zero());
            roz_case(Rect::<Dimension>::auto(), Size::new(5.0, 5.0), Rect::zero());
            roz_case(Rect::<Dimension>::auto(), Size::new(-5.0, -5.0), Rect::zero());
            roz_case(Rect::<Dimension>::auto(), Size::new(0.0, 0.0), Rect::zero());
        }

        #[test]
        fn resolve_or_zero_length() {
            roz_case(Rect::from_length(5.0, 5.0, 5.0, 5.0), Size::NONE, Rect::new(5.0, 5.0, 5.0, 5.0));
            roz_case(Rect::from_length(5.0, 5.0, 5.0, 5.0), Size::new(5.0, 5.0), Rect::new(5.0, 5.0, 5.0, 5.0));
            roz_case(Rect::from_length(5.0, 5.0, 5.0, 5.0), Size::new(-5.0, -5.0), Rect::new(5.0, 5.0, 5.0, 5.0));
            roz_case(Rect::from_length(5.0, 5.0, 5.0, 5.0), Size::new(0.0, 0.0), Rect::new(5.0, 5.0, 5.0, 5.0));
        }

        #[test]
        fn resolve_or_zero_percent() {
            roz_case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), Size::NONE, Rect::zero());
            roz_case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), Size::new(5.0, 5.0), Rect::new(25.0, 25.0, 25.0, 25.0));
            roz_case(
                Rect::from_percent(5.0, 5.0, 5.0, 5.0),
                Size::new(-5.0, -5.0),
                Rect::new(-25.0, -25.0, -25.0, -25.0),
            );
            roz_case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), Size::new(0.0, 0.0), Rect::zero());
        }
    }

    mod resolve_or_zero_rect_dimension_to_rect_f32_via_option {
        use super::roz_case;
        use crate::geometry::Rect;
        use crate::style::Dimension;
        use crate::util::OptF32;

        #[test]
        fn resolve_or_zero_auto() {
            roz_case(Rect::<Dimension>::auto(), OptF32::NONE, Rect::zero());
            roz_case(Rect::<Dimension>::auto(), OptF32::some(5.0), Rect::zero());
            roz_case(Rect::<Dimension>::auto(), OptF32::some(-5.0), Rect::zero());
            roz_case(Rect::<Dimension>::auto(), OptF32::some(0.0), Rect::zero());
        }

        #[test]
        fn resolve_or_zero_length() {
            roz_case(Rect::from_length(5.0, 5.0, 5.0, 5.0), OptF32::NONE, Rect::new(5.0, 5.0, 5.0, 5.0));
            roz_case(Rect::from_length(5.0, 5.0, 5.0, 5.0), OptF32::some(5.0), Rect::new(5.0, 5.0, 5.0, 5.0));
            roz_case(Rect::from_length(5.0, 5.0, 5.0, 5.0), OptF32::some(-5.0), Rect::new(5.0, 5.0, 5.0, 5.0));
            roz_case(Rect::from_length(5.0, 5.0, 5.0, 5.0), OptF32::some(0.0), Rect::new(5.0, 5.0, 5.0, 5.0));
        }

        #[test]
        fn resolve_or_zero_percent() {
            roz_case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), OptF32::NONE, Rect::zero());
            roz_case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), OptF32::some(5.0), Rect::new(25.0, 25.0, 25.0, 25.0));
            roz_case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), OptF32::some(-5.0), Rect::new(-25.0, -25.0, -25.0, -25.0));
            roz_case(Rect::from_percent(5.0, 5.0, 5.0, 5.0), OptF32::some(0.0), Rect::zero());
        }
    }
}
