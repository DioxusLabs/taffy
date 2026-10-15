//! Port of `yoga/numeric/Comparison.h` and `yoga/numeric/FloatOptional.h`.
//!
//! Yoga represents an undefined value as NaN and compares with a `1e-4` tolerance. The port keeps both,
//! and performs the same float operations in the same order as the C++, so results match bit for bit.

// Used by the algorithm files ported in later PRs.
#![allow(dead_code)]

use crate::util::sys::abs;
use core::cmp::Ordering;
use core::ops::Add;

/// Yoga's "undefined" value is NaN.
pub(crate) fn is_undefined(value: f32) -> bool {
    value.is_nan()
}

/// Not NaN.
pub(crate) fn is_defined(value: f32) -> bool {
    !value.is_nan()
}

/// `std::max(a, b)`: returns `a` unless `a < b`, which matters for signed zeros.
fn cpp_max(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}

/// `std::min(a, b)`: returns `a` unless `b < a`.
fn cpp_min(a: f32, b: f32) -> f32 {
    if b < a {
        b
    } else {
        a
    }
}

/// The larger of two values, or whichever one is defined.
pub(crate) fn max_or_defined(a: f32, b: f32) -> f32 {
    if is_defined(a) && is_defined(b) {
        return cpp_max(a, b);
    }
    if is_undefined(a) {
        b
    } else {
        a
    }
}

/// The smaller of two values, or whichever one is defined.
pub(crate) fn min_or_defined(a: f32, b: f32) -> f32 {
    if is_defined(a) && is_defined(b) {
        return cpp_min(a, b);
    }
    if is_undefined(a) {
        b
    } else {
        a
    }
}

/// Equal within `0.0001`; two undefined values are equal.
pub(crate) fn inexact_equals(a: f32, b: f32) -> bool {
    if is_defined(a) && is_defined(b) {
        return abs(a - b) < 0.0001;
    }
    is_undefined(a) && is_undefined(b)
}

/// `inexact_equals` for the `f64` intermediates of the pixel grid rounding.
pub(crate) fn inexact_equals_f64(a: f64, b: f64) -> bool {
    if !a.is_nan() && !b.is_nan() {
        let diff = a - b;
        let diff = if diff < 0.0 { -diff } else { diff };
        return diff < 0.0001;
    }
    a.is_nan() && b.is_nan()
}

/// An `f32` that may be undefined (NaN). Comparisons and arithmetic follow Yoga's `FloatOptional`:
/// two undefined values compare equal, arithmetic propagates NaN.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FloatOptional(f32);

impl FloatOptional {
    /// The undefined value.
    pub(crate) const UNDEFINED: FloatOptional = FloatOptional(f32::NAN);

    /// Wraps `value`; NaN is undefined.
    pub(crate) const fn new(value: f32) -> FloatOptional {
        FloatOptional(value)
    }

    /// The wrapped value, or NaN if undefined.
    pub(crate) fn unwrap(self) -> f32 {
        self.0
    }

    /// The wrapped value, or `default` if undefined.
    pub(crate) fn unwrap_or_default(self, default: f32) -> f32 {
        if self.is_undefined() {
            default
        } else {
            self.0
        }
    }

    /// Whether the value is undefined (NaN).
    pub(crate) fn is_undefined(self) -> bool {
        is_undefined(self.0)
    }

    /// Whether the value is defined.
    pub(crate) fn is_defined(self) -> bool {
        is_defined(self.0)
    }
}

impl Default for FloatOptional {
    fn default() -> Self {
        FloatOptional::UNDEFINED
    }
}

impl From<Option<f32>> for FloatOptional {
    fn from(value: Option<f32>) -> Self {
        FloatOptional(value.unwrap_or(f32::NAN))
    }
}

impl From<FloatOptional> for Option<f32> {
    fn from(value: FloatOptional) -> Self {
        if value.is_defined() {
            Some(value.0)
        } else {
            None
        }
    }
}

impl PartialEq for FloatOptional {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 || (self.is_undefined() && other.is_undefined())
    }
}

impl PartialEq<f32> for FloatOptional {
    fn eq(&self, other: &f32) -> bool {
        *self == FloatOptional(*other)
    }
}

impl PartialOrd for FloatOptional {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_undefined() && other.is_undefined() {
            Some(Ordering::Equal)
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}

impl Add for FloatOptional {
    type Output = FloatOptional;

    fn add(self, rhs: FloatOptional) -> FloatOptional {
        FloatOptional(self.0 + rhs.0)
    }
}

/// [`max_or_defined`] for `FloatOptional`.
pub(crate) fn max_or_defined_optional(a: FloatOptional, b: FloatOptional) -> FloatOptional {
    FloatOptional(max_or_defined(a.unwrap(), b.unwrap()))
}

/// [`inexact_equals`] for `FloatOptional`.
pub(crate) fn inexact_equals_optional(a: FloatOptional, b: FloatOptional) -> bool {
    inexact_equals(a.unwrap(), b.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_min_or_defined() {
        assert_eq!(max_or_defined(1.0, 2.0), 2.0);
        assert_eq!(max_or_defined(f32::NAN, 2.0), 2.0);
        assert_eq!(max_or_defined(1.0, f32::NAN), 1.0);
        assert!(max_or_defined(f32::NAN, f32::NAN).is_nan());
        assert_eq!(min_or_defined(1.0, 2.0), 1.0);
        assert_eq!(min_or_defined(f32::NAN, 2.0), 2.0);
        assert_eq!(min_or_defined(1.0, f32::NAN), 1.0);
        // std::max(+0, -0) is +0 and std::min(+0, -0) is +0 (the first argument)
        assert!(max_or_defined(0.0, -0.0).is_sign_positive());
        assert!(min_or_defined(0.0, -0.0).is_sign_positive());
        assert!(max_or_defined(-0.0, 0.0).is_sign_negative());
    }

    #[test]
    fn inexact() {
        assert!(inexact_equals(1.0, 1.00005));
        assert!(!inexact_equals(1.0, 1.0002));
        assert!(inexact_equals(f32::NAN, f32::NAN));
        assert!(!inexact_equals(f32::NAN, 1.0));
        assert!(inexact_equals_f64(1.0, 1.00005));
        assert!(!inexact_equals_f64(1.0, 1.0002));
    }

    #[test]
    fn float_optional() {
        let undefined = FloatOptional::UNDEFINED;
        let one = FloatOptional::new(1.0);
        let two = FloatOptional::new(2.0);
        assert_eq!(undefined, FloatOptional::default());
        assert_ne!(undefined, one);
        assert_eq!(one, 1.0);
        assert!(one < two);
        assert!(two >= one);
        assert!(undefined >= undefined);
        assert!(!(undefined > undefined));
        assert!(!(undefined < one));
        assert!(!(undefined >= one));
        assert!((one + two) == 3.0);
        assert!((one + undefined).is_undefined());
        assert_eq!(undefined.unwrap_or_default(5.0), 5.0);
        assert_eq!(max_or_defined_optional(undefined, two), two);
        assert_eq!(Option::<f32>::from(undefined), None);
        assert_eq!(FloatOptional::from(Some(2.0)), two);
    }
}
