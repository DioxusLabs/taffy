//! Contains numerical helper traits and functions
#![allow(clippy::manual_clamp)]

use crate::geometry::Size;
use crate::style::AvailableSpace;
use crate::util::OptFloat;

/// A trait to conveniently calculate minimums and maximums when some data may not be defined
///
/// If the left-hand value is [`None`], these operations return [`None`].
/// If the right-hand value is [`None`], it is treated as zero.
pub trait MaybeMath<In, Out> {
    /// Returns the minimum of `self` and `rhs`
    fn maybe_min(self, rhs: In) -> Out;

    /// Returns the maximum of `self` and `rhs`
    fn maybe_max(self, rhs: In) -> Out;

    /// Returns `self` clamped between `min` and `max`
    fn maybe_clamp(self, min: In, max: In) -> Out;

    /// Adds `self` and `rhs`.
    fn maybe_add(self, rhs: In) -> Out;

    /// Subtracts rhs from `self`, treating [`None`] values as default
    fn maybe_sub(self, rhs: In) -> Out;
}

impl MaybeMath<OptFloat, OptFloat> for OptFloat {
    #[inline(always)]
    fn maybe_min(self, rhs: OptFloat) -> OptFloat {
        if self.is_none() || rhs.is_none() {
            self
        } else {
            OptFloat::some(self.unchecked_value().min(rhs.unchecked_value()))
        }
    }

    #[inline(always)]
    fn maybe_max(self, rhs: OptFloat) -> OptFloat {
        if self.is_none() || rhs.is_none() {
            self
        } else {
            OptFloat::some(self.unchecked_value().max(rhs.unchecked_value()))
        }
    }

    #[inline(always)]
    fn maybe_clamp(self, min: OptFloat, max: OptFloat) -> OptFloat {
        self.maybe_min(max).maybe_max(min)
    }

    #[inline(always)]
    fn maybe_add(self, rhs: OptFloat) -> OptFloat {
        if self.is_none() || rhs.is_none() {
            self
        } else {
            OptFloat::some(self.unchecked_value() + rhs.unchecked_value())
        }
    }

    #[inline(always)]
    fn maybe_sub(self, rhs: OptFloat) -> OptFloat {
        if self.is_none() || rhs.is_none() {
            self
        } else {
            OptFloat::some(self.unchecked_value() - rhs.unchecked_value())
        }
    }
}

impl MaybeMath<f32, OptFloat> for OptFloat {
    #[inline(always)]
    fn maybe_min(self, rhs: f32) -> OptFloat {
        self.map(|val| val.min(rhs))
    }

    #[inline(always)]
    fn maybe_max(self, rhs: f32) -> OptFloat {
        self.map(|val| val.max(rhs))
    }

    #[inline(always)]
    fn maybe_clamp(self, min: f32, max: f32) -> OptFloat {
        self.map(|val| val.min(max).max(min))
    }

    #[inline(always)]
    fn maybe_add(self, rhs: f32) -> OptFloat {
        self.map(|val| val + rhs)
    }

    #[inline(always)]
    fn maybe_sub(self, rhs: f32) -> OptFloat {
        self.map(|val| val - rhs)
    }
}

impl MaybeMath<OptFloat, f32> for f32 {
    #[inline(always)]
    fn maybe_min(self, rhs: OptFloat) -> f32 {
        if rhs.is_none() {
            self
        } else {
            self.min(rhs.unchecked_value())
        }
    }

    #[inline(always)]
    fn maybe_max(self, rhs: OptFloat) -> f32 {
        if rhs.is_none() {
            self
        } else {
            self.max(rhs.unchecked_value())
        }
    }

    #[inline(always)]
    fn maybe_clamp(self, min: OptFloat, max: OptFloat) -> f32 {
        self.maybe_min(max).maybe_max(min)
    }

    #[inline(always)]
    fn maybe_add(self, rhs: OptFloat) -> f32 {
        if rhs.is_none() {
            self
        } else {
            self + rhs.unchecked_value()
        }
    }

    #[inline(always)]
    fn maybe_sub(self, rhs: OptFloat) -> f32 {
        if rhs.is_none() {
            self
        } else {
            self - rhs.unchecked_value()
        }
    }
}

impl MaybeMath<f32, AvailableSpace> for AvailableSpace {
    fn maybe_min(self, rhs: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val.min(rhs)),
            AvailableSpace::MinContent => AvailableSpace::Definite(rhs),
            AvailableSpace::MaxContent => AvailableSpace::Definite(rhs),
        }
    }
    fn maybe_max(self, rhs: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val.max(rhs)),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }

    fn maybe_clamp(self, min: f32, max: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val.min(max).max(min)),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }

    fn maybe_add(self, rhs: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val + rhs),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
    fn maybe_sub(self, rhs: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val - rhs),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
}

impl MaybeMath<OptFloat, AvailableSpace> for AvailableSpace {
    fn maybe_min(self, rhs: OptFloat) -> AvailableSpace {
        match (self, rhs.into_option()) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val.min(rhs)),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, Some(rhs)) => AvailableSpace::Definite(rhs),
            (AvailableSpace::MinContent, None) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, Some(rhs)) => AvailableSpace::Definite(rhs),
            (AvailableSpace::MaxContent, None) => AvailableSpace::MaxContent,
        }
    }
    fn maybe_max(self, rhs: OptFloat) -> AvailableSpace {
        match (self, rhs.into_option()) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val.max(rhs)),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _) => AvailableSpace::MaxContent,
        }
    }

    fn maybe_clamp(self, min: OptFloat, max: OptFloat) -> AvailableSpace {
        match (self, min.into_option(), max.into_option()) {
            (AvailableSpace::Definite(val), Some(min), Some(max)) => AvailableSpace::Definite(val.min(max).max(min)),
            (AvailableSpace::Definite(val), None, Some(max)) => AvailableSpace::Definite(val.min(max)),
            (AvailableSpace::Definite(val), Some(min), None) => AvailableSpace::Definite(val.max(min)),
            (AvailableSpace::Definite(val), None, None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _, _) => AvailableSpace::MaxContent,
        }
    }

    fn maybe_add(self, rhs: OptFloat) -> AvailableSpace {
        match (self, rhs.into_option()) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val + rhs),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _) => AvailableSpace::MaxContent,
        }
    }
    fn maybe_sub(self, rhs: OptFloat) -> AvailableSpace {
        match (self, rhs.into_option()) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val - rhs),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _) => AvailableSpace::MaxContent,
        }
    }
}

impl<In, Out, T: MaybeMath<In, Out>> MaybeMath<Size<In>, Size<Out>> for Size<T> {
    fn maybe_min(self, rhs: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_min(rhs.width), height: self.height.maybe_min(rhs.height) }
    }

    fn maybe_max(self, rhs: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_max(rhs.width), height: self.height.maybe_max(rhs.height) }
    }

    fn maybe_clamp(self, min: Size<In>, max: Size<In>) -> Size<Out> {
        Size {
            width: self.width.maybe_clamp(min.width, max.width),
            height: self.height.maybe_clamp(min.height, max.height),
        }
    }

    fn maybe_add(self, rhs: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_add(rhs.width), height: self.height.maybe_add(rhs.height) }
    }

    fn maybe_sub(self, rhs: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_sub(rhs.width), height: self.height.maybe_sub(rhs.height) }
    }
}

#[cfg(test)]
mod tests {
    mod lhs_option_f32_rhs_option_f32 {
        use crate::util::{MaybeMath, OptFloat};

        #[test]
        fn test_maybe_min() {
            assert_eq!(OptFloat::some(3.0).maybe_min(OptFloat::some(5.0)), OptFloat::some(3.0));
            assert_eq!(OptFloat::some(5.0).maybe_min(OptFloat::some(3.0)), OptFloat::some(3.0));
            assert_eq!(OptFloat::some(3.0).maybe_min(OptFloat::NONE), OptFloat::some(3.0));
            assert_eq!(OptFloat::NONE.maybe_min(OptFloat::some(3.0)), OptFloat::NONE);
            assert_eq!(OptFloat::NONE.maybe_min(OptFloat::NONE), OptFloat::NONE);
        }

        #[test]
        fn test_maybe_max() {
            assert_eq!(OptFloat::some(3.0).maybe_max(OptFloat::some(5.0)), OptFloat::some(5.0));
            assert_eq!(OptFloat::some(5.0).maybe_max(OptFloat::some(3.0)), OptFloat::some(5.0));
            assert_eq!(OptFloat::some(3.0).maybe_max(OptFloat::NONE), OptFloat::some(3.0));
            assert_eq!(OptFloat::NONE.maybe_max(OptFloat::some(3.0)), OptFloat::NONE);
            assert_eq!(OptFloat::NONE.maybe_max(OptFloat::NONE), OptFloat::NONE);
        }

        #[test]
        fn test_maybe_add() {
            assert_eq!(OptFloat::some(3.0).maybe_add(OptFloat::some(5.0)), OptFloat::some(8.0));
            assert_eq!(OptFloat::some(5.0).maybe_add(OptFloat::some(3.0)), OptFloat::some(8.0));
            assert_eq!(OptFloat::some(3.0).maybe_add(OptFloat::NONE), OptFloat::some(3.0));
            assert_eq!(OptFloat::NONE.maybe_add(OptFloat::some(3.0)), OptFloat::NONE);
            assert_eq!(OptFloat::NONE.maybe_add(OptFloat::NONE), OptFloat::NONE);
        }

        #[test]
        fn test_maybe_sub() {
            assert_eq!(OptFloat::some(3.0).maybe_sub(OptFloat::some(5.0)), OptFloat::some(-2.0));
            assert_eq!(OptFloat::some(5.0).maybe_sub(OptFloat::some(3.0)), OptFloat::some(2.0));
            assert_eq!(OptFloat::some(3.0).maybe_sub(OptFloat::NONE), OptFloat::some(3.0));
            assert_eq!(OptFloat::NONE.maybe_sub(OptFloat::some(3.0)), OptFloat::NONE);
            assert_eq!(OptFloat::NONE.maybe_sub(OptFloat::NONE), OptFloat::NONE);
        }
    }

    mod lhs_option_f32_rhs_f32 {
        use crate::util::{MaybeMath, OptFloat};

        #[test]
        fn test_maybe_min() {
            assert_eq!(OptFloat::some(3.0).maybe_min(5.0), OptFloat::some(3.0));
            assert_eq!(OptFloat::some(5.0).maybe_min(3.0), OptFloat::some(3.0));
            assert_eq!(OptFloat::NONE.maybe_min(3.0), OptFloat::NONE);
        }

        #[test]
        fn test_maybe_max() {
            assert_eq!(OptFloat::some(3.0).maybe_max(5.0), OptFloat::some(5.0));
            assert_eq!(OptFloat::some(5.0).maybe_max(3.0), OptFloat::some(5.0));
            assert_eq!(OptFloat::NONE.maybe_max(3.0), OptFloat::NONE);
        }

        #[test]
        fn test_maybe_add() {
            assert_eq!(OptFloat::some(3.0).maybe_add(5.0), OptFloat::some(8.0));
            assert_eq!(OptFloat::some(5.0).maybe_add(3.0), OptFloat::some(8.0));
            assert_eq!(OptFloat::NONE.maybe_add(3.0), OptFloat::NONE);
        }

        #[test]
        fn test_maybe_sub() {
            assert_eq!(OptFloat::some(3.0).maybe_sub(5.0), OptFloat::some(-2.0));
            assert_eq!(OptFloat::some(5.0).maybe_sub(3.0), OptFloat::some(2.0));
            assert_eq!(OptFloat::NONE.maybe_sub(3.0), OptFloat::NONE);
        }
    }

    mod lhs_f32_rhs_option_f32 {
        use crate::util::{MaybeMath, OptFloat};

        #[test]
        fn test_maybe_min() {
            assert_eq!(3.0.maybe_min(OptFloat::some(5.0)), 3.0);
            assert_eq!(5.0.maybe_min(OptFloat::some(3.0)), 3.0);
            assert_eq!(3.0.maybe_min(OptFloat::NONE), 3.0);
        }

        #[test]
        fn test_maybe_max() {
            assert_eq!(3.0.maybe_max(OptFloat::some(5.0)), 5.0);
            assert_eq!(5.0.maybe_max(OptFloat::some(3.0)), 5.0);
            assert_eq!(3.0.maybe_max(OptFloat::NONE), 3.0);
        }

        #[test]
        fn test_maybe_add() {
            assert_eq!(3.0.maybe_add(OptFloat::some(5.0)), 8.0);
            assert_eq!(5.0.maybe_add(OptFloat::some(3.0)), 8.0);
            assert_eq!(3.0.maybe_add(OptFloat::NONE), 3.0);
        }

        #[test]
        fn test_maybe_sub() {
            assert_eq!(3.0.maybe_sub(OptFloat::some(5.0)), -2.0);
            assert_eq!(5.0.maybe_sub(OptFloat::some(3.0)), 2.0);
            assert_eq!(3.0.maybe_sub(OptFloat::NONE), 3.0);
        }
    }
}
