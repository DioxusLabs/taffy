//! Contains the [`Number`] type and associated methods

use core::ops::{Add, Div, Mul, Sub};

/// A [`f32`] that may be undefined
///
/// NOTE: this should *really* be an [`Option<f32>`],
/// but we're optimizing for backwards compatibility for now.
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Number {
    /// Pretend this is [`Some(f32)`]
    ///
    /// WARNING: the contained value may be NaN
    Defined(f32),
    /// Pretend this is [`None`]
    Undefined,
}

/// An extension method used for [`Number`]
pub trait OrElse<T> {
    /// Returns the contained value if it is defined, otherwise returns `other`
    fn or_else(self, other: T) -> T;
}

impl Default for Number {
    fn default() -> Self {
        Self::Undefined
    }
}

impl OrElse<f32> for Number {
    fn or_else(self, other: f32) -> f32 {
        match self {
            Number::Defined(val) => val,
            Number::Undefined => other,
        }
    }
}

impl OrElse<Self> for Number {
    fn or_else(self, other: Self) -> Self {
        match self {
            Number::Defined(_) => self,
            Number::Undefined => other,
        }
    }
}

impl Number {
    /// Is the number defined?
    pub fn is_defined(self) -> bool {
        match self {
            Number::Defined(_) => true,
            Number::Undefined => false,
        }
    }

    /// Is the number undefined?
    pub fn is_undefined(self) -> bool {
        match self {
            Number::Defined(_) => false,
            Number::Undefined => true,
        }
    }
}

/// An extension trait for [`Number`]
pub trait MinMax<In, Out> {
    /// Returns the minimum of self and rhs
    ///
    /// If either either value is invalid, returns the other value.
    fn maybe_min(self, rhs: In) -> Out;
    /// Returns the maximum of self and rhs
    ///
    /// If either either value is invalid, returns the other value.
    fn maybe_max(self, rhs: In) -> Out;
}

impl MinMax<Self, Self> for Number {
    fn maybe_min(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::Defined(l), Number::Defined(r)) => Number::Defined(l.min(r)),
            (Number::Defined(_), _) => self,
            _ => Number::Undefined,
        }
    }

    fn maybe_max(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::Defined(l), Number::Defined(r)) => Number::Defined(l.max(r)),
            (Number::Defined(_), _) => self,
            _ => Number::Undefined,
        }
    }
}

impl MinMax<f32, Number> for Number {
    fn maybe_min(self, rhs: f32) -> Self {
        match self {
            Number::Defined(val) => Number::Defined(val.min(rhs)),
            Number::Undefined => Number::Undefined,
        }
    }

    fn maybe_max(self, rhs: f32) -> Self {
        match self {
            Number::Defined(val) => Number::Defined(val.max(rhs)),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl MinMax<Number, f32> for f32 {
    fn maybe_min(self, rhs: Number) -> f32 {
        match rhs {
            Number::Defined(val) => self.min(val),
            Number::Undefined => self,
        }
    }

    fn maybe_max(self, rhs: Number) -> f32 {
        match rhs {
            Number::Defined(val) => self.max(val),
            Number::Undefined => self,
        }
    }
}

impl From<f32> for Number {
    fn from(v: f32) -> Self {
        Self::Defined(v)
    }
}

impl Add<f32> for Number {
    type Output = Number;

    fn add(self, rhs: f32) -> Self {
        match self {
            Number::Defined(val) => Number::Defined(val + rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::Defined(l), Number::Defined(r)) => Number::Defined(l + r),
            (Number::Defined(_), _) => self,
            _ => Number::Undefined,
        }
    }
}

impl Sub<f32> for Number {
    type Output = Number;

    fn sub(self, rhs: f32) -> Self {
        match self {
            Number::Defined(val) => Number::Defined(val - rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl Sub<Number> for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::Defined(l), Number::Defined(r)) => Number::Defined(l - r),
            (Number::Defined(_), _) => self,
            _ => Number::Undefined,
        }
    }
}

impl Mul<f32> for Number {
    type Output = Number;

    fn mul(self, rhs: f32) -> Self {
        match self {
            Number::Defined(val) => Number::Defined(val * rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::Defined(l), Number::Defined(r)) => Number::Defined(l * r),
            (Number::Defined(_), _) => self,
            _ => Number::Undefined,
        }
    }
}

impl Div<f32> for Number {
    type Output = Number;

    fn div(self, rhs: f32) -> Self {
        match self {
            Number::Defined(val) => Number::Defined(val / rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl Div<Number> for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::Defined(l), Number::Defined(r)) => Number::Defined(l / r),
            (Number::Defined(_), _) => self,
            _ => Number::Undefined,
        }
    }
}
