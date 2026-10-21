//! Contains numerical helper traits and functions
#![allow(clippy::manual_clamp)]

use crate::app_unit::Au;
use crate::geometry::Size;
use crate::style::AvailableSpace;

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

impl MaybeMath<Option<Au>, Option<Au>> for Option<Au> {
    #[inline]
    fn maybe_min(self, rhs: Option<Au>) -> Option<Au> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l.min(r)),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }

    #[inline]
    fn maybe_max(self, rhs: Option<Au>) -> Option<Au> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l.max(r)),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }

    #[inline]
    fn maybe_clamp(self, min: Option<Au>, max: Option<Au>) -> Option<Au> {
        match (self, min, max) {
            (Some(base), Some(min), Some(max)) => Some(base.min(max).max(min)),
            (Some(base), None, Some(max)) => Some(base.min(max)),
            (Some(base), Some(min), None) => Some(base.max(min)),
            (Some(_), None, None) => self,
            (None, _, _) => None,
        }
    }

    #[inline]
    fn maybe_add(self, rhs: Option<Au>) -> Option<Au> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l + r),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }

    #[inline]
    fn maybe_sub(self, rhs: Option<Au>) -> Option<Au> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l - r),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }
}

impl MaybeMath<Au, Option<Au>> for Option<Au> {
    #[inline]
    fn maybe_min(self, rhs: Au) -> Option<Au> {
        self.map(|val| val.min(rhs))
    }

    #[inline]
    fn maybe_max(self, rhs: Au) -> Option<Au> {
        self.map(|val| val.max(rhs))
    }

    #[inline]
    fn maybe_clamp(self, min: Au, max: Au) -> Option<Au> {
        self.map(|val| val.min(max).max(min))
    }

    #[inline]
    fn maybe_add(self, rhs: Au) -> Option<Au> {
        self.map(|val| val + rhs)
    }

    #[inline]
    fn maybe_sub(self, rhs: Au) -> Option<Au> {
        self.map(|val| val - rhs)
    }
}

impl MaybeMath<Option<Au>, Au> for Au {
    #[inline]
    fn maybe_min(self, rhs: Option<Au>) -> Au {
        match rhs {
            Some(val) => self.min(val),
            None => self,
        }
    }

    #[inline]
    fn maybe_max(self, rhs: Option<Au>) -> Au {
        match rhs {
            Some(val) => self.max(val),
            None => self,
        }
    }

    #[inline]
    fn maybe_clamp(self, min: Option<Au>, max: Option<Au>) -> Au {
        match (min, max) {
            (Some(min), Some(max)) => self.min(max).max(min),
            (None, Some(max)) => self.min(max),
            (Some(min), None) => self.max(min),
            (None, None) => self,
        }
    }

    #[inline]
    fn maybe_add(self, rhs: Option<Au>) -> Au {
        match rhs {
            Some(val) => self + val,
            None => self,
        }
    }

    #[inline]
    fn maybe_sub(self, rhs: Option<Au>) -> Au {
        match rhs {
            Some(val) => self - val,
            None => self,
        }
    }
}

impl MaybeMath<Au, AvailableSpace> for AvailableSpace {
    #[inline]
    fn maybe_min(self, rhs: Au) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val.min(rhs)),
            AvailableSpace::MinContent => AvailableSpace::Definite(rhs),
            AvailableSpace::MaxContent => AvailableSpace::Definite(rhs),
        }
    }
    #[inline]
    fn maybe_max(self, rhs: Au) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val.max(rhs)),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }

    #[inline]
    fn maybe_clamp(self, min: Au, max: Au) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val.min(max).max(min)),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }

    #[inline]
    fn maybe_add(self, rhs: Au) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val + rhs),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
    #[inline]
    fn maybe_sub(self, rhs: Au) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val - rhs),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
}

impl MaybeMath<Option<Au>, AvailableSpace> for AvailableSpace {
    #[inline]
    fn maybe_min(self, rhs: Option<Au>) -> AvailableSpace {
        match (self, rhs) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val.min(rhs)),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, Some(rhs)) => AvailableSpace::Definite(rhs),
            (AvailableSpace::MinContent, None) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, Some(rhs)) => AvailableSpace::Definite(rhs),
            (AvailableSpace::MaxContent, None) => AvailableSpace::MaxContent,
        }
    }
    #[inline]
    fn maybe_max(self, rhs: Option<Au>) -> AvailableSpace {
        match (self, rhs) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val.max(rhs)),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _) => AvailableSpace::MaxContent,
        }
    }

    #[inline]
    fn maybe_clamp(self, min: Option<Au>, max: Option<Au>) -> AvailableSpace {
        match (self, min, max) {
            (AvailableSpace::Definite(val), Some(min), Some(max)) => AvailableSpace::Definite(val.min(max).max(min)),
            (AvailableSpace::Definite(val), None, Some(max)) => AvailableSpace::Definite(val.min(max)),
            (AvailableSpace::Definite(val), Some(min), None) => AvailableSpace::Definite(val.max(min)),
            (AvailableSpace::Definite(val), None, None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _, _) => AvailableSpace::MaxContent,
        }
    }

    #[inline]
    fn maybe_add(self, rhs: Option<Au>) -> AvailableSpace {
        match (self, rhs) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val + rhs),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _) => AvailableSpace::MaxContent,
        }
    }
    #[inline]
    fn maybe_sub(self, rhs: Option<Au>) -> AvailableSpace {
        match (self, rhs) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val - rhs),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _) => AvailableSpace::MaxContent,
        }
    }
}

impl<In, Out, T: MaybeMath<In, Out>> MaybeMath<Size<In>, Size<Out>> for Size<T> {
    #[inline]
    fn maybe_min(self, rhs: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_min(rhs.width), height: self.height.maybe_min(rhs.height) }
    }

    #[inline]
    fn maybe_max(self, rhs: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_max(rhs.width), height: self.height.maybe_max(rhs.height) }
    }

    #[inline]
    fn maybe_clamp(self, min: Size<In>, max: Size<In>) -> Size<Out> {
        Size {
            width: self.width.maybe_clamp(min.width, max.width),
            height: self.height.maybe_clamp(min.height, max.height),
        }
    }

    #[inline]
    fn maybe_add(self, rhs: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_add(rhs.width), height: self.height.maybe_add(rhs.height) }
    }

    #[inline]
    fn maybe_sub(self, rhs: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_sub(rhs.width), height: self.height.maybe_sub(rhs.height) }
    }
}

#[cfg(test)]
mod tests {
    mod lhs_option_au_rhs_option_au {
        use crate::app_unit::Au;
        use crate::util::MaybeMath;

        #[test]
        fn test_maybe_min() {
            assert_eq!(Some(Au(180)).maybe_min(Some(Au(300))), Some(Au(180)));
            assert_eq!(Some(Au(300)).maybe_min(Some(Au(180))), Some(Au(180)));
            assert_eq!(Some(Au(180)).maybe_min(None), Some(Au(180)));
            assert_eq!(None.maybe_min(Some(Au(180))), None);
            assert_eq!(None.maybe_min(None), None);
        }

        #[test]
        fn test_maybe_max() {
            assert_eq!(Some(Au(180)).maybe_max(Some(Au(300))), Some(Au(300)));
            assert_eq!(Some(Au(300)).maybe_max(Some(Au(180))), Some(Au(300)));
            assert_eq!(Some(Au(180)).maybe_max(None), Some(Au(180)));
            assert_eq!(None.maybe_max(Some(Au(180))), None);
            assert_eq!(None.maybe_max(None), None);
        }

        #[test]
        fn test_maybe_add() {
            assert_eq!(Some(Au(180)).maybe_add(Some(Au(300))), Some(Au(480)));
            assert_eq!(Some(Au(300)).maybe_add(Some(Au(180))), Some(Au(480)));
            assert_eq!(Some(Au(180)).maybe_add(None), Some(Au(180)));
            assert_eq!(None.maybe_add(Some(Au(180))), None);
            assert_eq!(None.maybe_add(None), None);
        }

        #[test]
        fn test_maybe_sub() {
            assert_eq!(Some(Au(180)).maybe_sub(Some(Au(300))), Some(Au(-120)));
            assert_eq!(Some(Au(300)).maybe_sub(Some(Au(180))), Some(Au(120)));
            assert_eq!(Some(Au(180)).maybe_sub(None), Some(Au(180)));
            assert_eq!(None.maybe_sub(Some(Au(180))), None);
            assert_eq!(None.maybe_sub(None), None);
        }
    }

    mod lhs_option_au_rhs_au {
        use crate::app_unit::Au;
        use crate::util::MaybeMath;

        #[test]
        fn test_maybe_min() {
            assert_eq!(Some(Au(180)).maybe_min(Au(300)), Some(Au(180)));
            assert_eq!(Some(Au(300)).maybe_min(Au(180)), Some(Au(180)));
            assert_eq!(None.maybe_min(Au(180)), None);
        }

        #[test]
        fn test_maybe_max() {
            assert_eq!(Some(Au(180)).maybe_max(Au(300)), Some(Au(300)));
            assert_eq!(Some(Au(300)).maybe_max(Au(180)), Some(Au(300)));
            assert_eq!(None.maybe_max(Au(180)), None);
        }

        #[test]
        fn test_maybe_add() {
            assert_eq!(Some(Au(180)).maybe_add(Au(300)), Some(Au(480)));
            assert_eq!(Some(Au(300)).maybe_add(Au(180)), Some(Au(480)));
            assert_eq!(None.maybe_add(Au(180)), None);
        }

        #[test]
        fn test_maybe_sub() {
            assert_eq!(Some(Au(180)).maybe_sub(Au(300)), Some(Au(-120)));
            assert_eq!(Some(Au(300)).maybe_sub(Au(180)), Some(Au(120)));
            assert_eq!(None.maybe_sub(Au(180)), None);
        }
    }

    mod lhs_au_rhs_option_au {
        use crate::app_unit::Au;
        use crate::util::MaybeMath;

        #[test]
        fn test_maybe_min() {
            assert_eq!(Au(180).maybe_min(Some(Au(300))), Au(180));
            assert_eq!(Au(300).maybe_min(Some(Au(180))), Au(180));
            assert_eq!(Au(180).maybe_min(None), Au(180));
        }

        #[test]
        fn test_maybe_max() {
            assert_eq!(Au(180).maybe_max(Some(Au(300))), Au(300));
            assert_eq!(Au(300).maybe_max(Some(Au(180))), Au(300));
            assert_eq!(Au(180).maybe_max(None), Au(180));
        }

        #[test]
        fn test_maybe_add() {
            assert_eq!(Au(180).maybe_add(Some(Au(300))), Au(480));
            assert_eq!(Au(300).maybe_add(Some(Au(180))), Au(480));
            assert_eq!(Au(180).maybe_add(None), Au(180));
        }

        #[test]
        fn test_maybe_sub() {
            assert_eq!(Au(180).maybe_sub(Some(Au(300))), Au(-120));
            assert_eq!(Au(300).maybe_sub(Some(Au(180))), Au(120));
            assert_eq!(Au(180).maybe_sub(None), Au(180));
        }
    }
}
