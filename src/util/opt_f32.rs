//! A compact 4-byte replacement for `Option<f32>`
use crate::style::compact_length::compat::{f32_from_bits, f32_to_bits};
use core::fmt;

/// A 4-byte equivalent of `Option<f32>` that uses a single sentinel NaN bit pattern
/// to represent `None`. All other values (including other NaNs) are "some" values.
///
/// Taffy assumes that input lengths are non-NaN, so the sentinel pattern colliding
/// with a genuinely computed NaN is not a concern in practice.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct OptF32(f32);

impl OptF32 {
    /// The bit pattern used to represent `None`: a quiet NaN with a distinctive payload
    const NONE_BITS: u32 = 0x7FC0_F32A;

    /// The `None` value
    pub const NONE: Self = Self(f32_from_bits(Self::NONE_BITS));

    /// Creates a "some" value
    #[inline(always)]
    pub const fn some(value: f32) -> Self {
        Self(value)
    }

    /// Returns `true` if the value is `None`
    #[inline(always)]
    pub const fn is_none(self) -> bool {
        self.to_bits() == Self::NONE_BITS
    }

    /// Returns `true` if the value is not `None`
    #[inline(always)]
    pub const fn is_some(self) -> bool {
        !self.is_none()
    }

    /// Converts to an `Option<f32>`
    #[inline(always)]
    pub const fn into_option(self) -> Option<f32> {
        if self.is_none() {
            None
        } else {
            Some(self.0)
        }
    }

    /// Returns the contained value without checking whether it is `None`.
    /// If the value is `None` then the sentinel NaN is returned.
    #[inline(always)]
    pub const fn unchecked_value(self) -> f32 {
        self.0
    }

    /// Returns the raw bit representation
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        f32_to_bits(self.0)
    }

    /// Returns the contained value, panicking if it is `None`
    #[inline(always)]
    #[track_caller]
    pub fn unwrap(self) -> f32 {
        if self.is_none() {
            panic!("called `OptF32::unwrap()` on a `None` value");
        }
        self.0
    }

    /// Returns the contained value or the provided default if `None`
    #[inline(always)]
    pub fn unwrap_or(self, default: f32) -> f32 {
        if self.is_none() {
            default
        } else {
            self.0
        }
    }

    /// Returns the contained value or computes it from the closure if `None`
    #[inline(always)]
    pub fn unwrap_or_else(self, default: impl FnOnce() -> f32) -> f32 {
        if self.is_none() {
            default()
        } else {
            self.0
        }
    }

    /// Returns self if it is "some", otherwise returns `other`
    #[inline(always)]
    pub fn or(self, other: Self) -> Self {
        if self.is_none() {
            other
        } else {
            self
        }
    }

    /// Returns self if it is "some", otherwise computes the fallback from the closure
    #[inline(always)]
    pub fn or_else(self, other: impl FnOnce() -> Self) -> Self {
        if self.is_none() {
            other()
        } else {
            self
        }
    }

    /// Applies a function to the contained value (if any)
    #[inline(always)]
    pub fn map(self, f: impl FnOnce(f32) -> f32) -> Self {
        if self.is_none() {
            self
        } else {
            Self(f(self.0))
        }
    }

    /// Sums an iterator of `OptF32`s, returning `None` if any of the values are `None`
    /// (matching the behaviour of `Option<f32>`)
    fn sum_impl(iter: impl Iterator<Item = Self>) -> Self {
        let mut total = 0.0;
        for item in iter {
            if item.is_none() {
                return Self::NONE;
            }
            total += item.0;
        }
        Self(total)
    }

    /// Returns `default` if the value is `None`, or applies `f` to the contained value
    #[inline(always)]
    pub fn map_or<U>(self, default: U, f: impl FnOnce(f32) -> U) -> U {
        if self.is_none() {
            default
        } else {
            f(self.0)
        }
    }

    /// Returns `None` if the value is `None`, otherwise calls `f` with the value and returns the result
    #[inline(always)]
    pub fn and_then(self, f: impl FnOnce(f32) -> Self) -> Self {
        if self.is_none() {
            self
        } else {
            f(self.0)
        }
    }

    /// Returns `None` if the value is `None` or the predicate returns false
    #[inline(always)]
    pub fn filter(self, predicate: impl FnOnce(f32) -> bool) -> Self {
        if self.is_some() && predicate(self.0) {
            self
        } else {
            Self::NONE
        }
    }
}

impl core::iter::Sum for OptF32 {
    /// Sums the values, returning `None` if any of the values are `None`
    /// (matching the behaviour of `Option<f32>`)
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::sum_impl(iter)
    }
}

impl From<Option<f32>> for OptF32 {
    #[inline(always)]
    fn from(value: Option<f32>) -> Self {
        match value {
            Some(value) => Self::some(value),
            None => Self::NONE,
        }
    }
}

impl From<OptF32> for Option<f32> {
    #[inline(always)]
    fn from(value: OptF32) -> Self {
        value.into_option()
    }
}

impl From<f32> for OptF32 {
    #[inline(always)]
    fn from(value: f32) -> Self {
        Self::some(value)
    }
}

impl PartialEq for OptF32 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        // Values compare like f32, except that `NONE == NONE` (which the float
        // comparison alone would report as false as the sentinel is a NaN)
        (self.0 == other.0) | (self.to_bits() == other.to_bits())
    }
}

impl fmt::Debug for OptF32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format like Option<f32> for familiarity
        if self.is_none() {
            f.write_str("None")
        } else {
            f.debug_tuple("Some").field(&self.0).finish()
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for OptF32 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.into_option().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OptF32 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Option::<f32>::deserialize(deserializer).map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    use super::OptF32;

    #[test]
    fn size_is_4_bytes() {
        assert_eq!(core::mem::size_of::<OptF32>(), 4);
    }

    #[test]
    fn none_roundtrip() {
        assert_eq!(OptF32::NONE.into_option(), None);
        assert!(OptF32::NONE.is_none());
        assert!(!OptF32::NONE.is_some());
        assert_eq!(OptF32::from(None), OptF32::NONE);
    }

    #[test]
    fn some_roundtrip() {
        assert_eq!(OptF32::some(5.0).into_option(), Some(5.0));
        assert_eq!(OptF32::from(Some(-0.0)).into_option(), Some(-0.0));
        assert!(OptF32::some(0.0).is_some());
    }

    #[test]
    fn nan_is_some() {
        // A regular NaN is not the sentinel and so is a "some" value
        assert!(OptF32::some(f32::NAN).is_some());
    }

    #[test]
    fn equality() {
        assert_eq!(OptF32::NONE, OptF32::NONE);
        assert_eq!(OptF32::some(3.0), OptF32::some(3.0));
        assert_ne!(OptF32::some(3.0), OptF32::some(4.0));
        assert_ne!(OptF32::some(3.0), OptF32::NONE);
        assert_eq!(OptF32::some(0.0), OptF32::some(-0.0));
    }
}
