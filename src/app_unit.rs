/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! An `Au` is an "App Unit" and represents 1/60th of a CSS pixel. It was
//! originally proposed in 2002 as a standard unit of measure in Gecko.
//! See <https://bugzilla.mozilla.org/show_bug.cgi?id=177805> for more info.
//!
//! Vendored from the `app_units` crate (v0.7.8, MPL-2.0, Copyright The Servo Project Developers).
//! Only the API used by Taffy is kept. Unlike upstream, arithmetic operators wrap instead of saturating.

use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

/// The number of app units in a pixel.
pub const AU_PER_PX: i32 = 60;
/// The minimum number of app units, same as in Gecko.
pub const MIN_AU: Au = Au(-((1 << 30) - 1));
/// The maximum number of app units, same as in Gecko.
///
/// (1 << 30) - 1 lets us add/subtract two Au and check for overflow after the operation.
pub const MAX_AU: Au = Au((1 << 30) - 1);

/// An App Unit, the fundamental unit of length in Servo. Usually
/// 1/60th of a pixel (see `AU_PER_PX`)
///
/// Please ensure that the values are between `MIN_AU` and `MAX_AU`.
/// It is safe to construct invalid `Au` values, but it may lead to
/// overflows.
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct Au(pub i32);

impl fmt::Debug for Au {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}px", (self.0 as f64) / (AU_PER_PX as f64))
    }
}

impl Add for Au {
    type Output = Au;

    #[inline]
    fn add(self, other: Au) -> Au {
        Au(self.0.wrapping_add(other.0))
    }
}

impl Sub for Au {
    type Output = Au;

    #[inline]
    fn sub(self, other: Au) -> Au {
        Au(self.0.wrapping_sub(other.0))
    }
}

impl Mul<Au> for i32 {
    type Output = Au;

    #[inline]
    fn mul(self, other: Au) -> Au {
        Au(self.wrapping_mul(other.0))
    }
}

impl Mul<i32> for Au {
    type Output = Au;

    #[inline]
    fn mul(self, other: i32) -> Au {
        Au(self.0.wrapping_mul(other))
    }
}

impl Div for Au {
    type Output = i32;

    #[inline]
    fn div(self, other: Au) -> i32 {
        self.0 / other.0
    }
}

impl Div<i32> for Au {
    type Output = Au;

    #[inline]
    fn div(self, other: i32) -> Au {
        Au(self.0 / other)
    }
}

impl Rem for Au {
    type Output = Au;

    #[inline]
    fn rem(self, other: Au) -> Au {
        Au(self.0 % other.0)
    }
}

impl Rem<i32> for Au {
    type Output = Au;

    #[inline]
    fn rem(self, other: i32) -> Au {
        Au(self.0 % other)
    }
}

impl Neg for Au {
    type Output = Au;

    #[inline]
    fn neg(self) -> Au {
        Au(-self.0)
    }
}

impl AddAssign for Au {
    #[inline]
    fn add_assign(&mut self, other: Au) {
        *self = *self + other;
    }
}

impl SubAssign for Au {
    #[inline]
    fn sub_assign(&mut self, other: Au) {
        *self = *self - other;
    }
}

impl MulAssign<i32> for Au {
    #[inline]
    fn mul_assign(&mut self, other: i32) {
        *self = *self * other;
    }
}

impl DivAssign<i32> for Au {
    #[inline]
    fn div_assign(&mut self, other: i32) {
        *self = *self / other;
    }
}

impl<'a> Sum<&'a Self> for Au {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Au(0), |a, b| a + *b)
    }
}

impl Sum<Self> for Au {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Au(0), |a, b| a + b)
    }
}

impl Au {
    /// Constructs an `Au` from an app unit count, saturating at `MIN_AU` and `MAX_AU`.
    #[inline]
    pub fn new(value: i32) -> Au {
        Ord::clamp(Au(value), MIN_AU, MAX_AU)
    }

    /// Converts a whole number of pixels to app units.
    #[inline]
    pub fn from_px(px: i32) -> Au {
        Au(px) * AU_PER_PX
    }

    /// Converts to fractional pixels.
    #[inline]
    pub fn to_f32_px(self) -> f32 {
        (self.0 as f32) / (AU_PER_PX as f32)
    }

    /// Absolute value.
    #[inline]
    pub fn abs(self) -> Self {
        Au(self.0.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::{Au, AU_PER_PX, MAX_AU, MIN_AU};

    #[test]
    fn convert() {
        assert_eq!(AU_PER_PX, 60);
        assert_eq!(Au::from_px(5), Au(300));
        assert_eq!(Au(300).to_f32_px(), 5.);
        assert_eq!(Au(312).to_f32_px(), 5.2);
    }

    #[test]
    fn operations() {
        assert_eq!(Au(7) + Au(5), Au(12));
        assert_eq!(Au(7) - Au(5), Au(2));
        assert_eq!(Au(7) * 5, Au(35));
        assert_eq!(5 * Au(7), Au(35));
        assert_eq!(Au(35) / 5, Au(7));
        assert_eq!(Au(35) / Au(5), 7);
        assert_eq!(Au(35) % 6, Au(5));
        assert_eq!(-Au(7), Au(-7));
        let mut au = Au(7);
        au += Au(5);
        au -= Au(2);
        au *= 3;
        au /= 2;
        assert_eq!(au, Au(15));
    }

    #[test]
    fn wraps_instead_of_saturating() {
        assert_eq!(Au(i32::MAX) + Au(1), Au(i32::MIN));
        assert_eq!(Au(i32::MIN) - Au(1), Au(i32::MAX));
        assert_eq!(MAX_AU * 2, Au(MAX_AU.0.wrapping_mul(2)));
        assert_eq!(MIN_AU * -1, Au(MIN_AU.0.wrapping_mul(-1)));
    }

    #[test]
    fn new_saturates() {
        assert_eq!(Au::new(i32::MAX), MAX_AU);
        assert_eq!(Au::new(i32::MIN), MIN_AU);
        assert_eq!(Au::new(7), Au(7));
    }

    #[test]
    fn sum() {
        assert_eq!([Au(1), Au(2), Au(3)].iter().sum::<Au>(), Au(6));
        assert_eq!([Au(1), Au(2)].into_iter().sum::<Au>(), Au(3));
        assert_eq!(std::iter::empty::<Au>().sum::<Au>(), Au(0));
    }
}
