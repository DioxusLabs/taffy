use core::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub enum Number {
    Defined(f32),
    Undefined,
}

pub trait OrElse<T> {
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
    pub fn is_defined(self) -> bool {
        match self {
            Number::Defined(_) => true,
            Number::Undefined => false,
        }
    }

    pub fn is_undefined(self) -> bool {
        match self {
            Number::Defined(_) => false,
            Number::Undefined => true,
        }
    }
}

pub trait MinMax<In, Out> {
    fn maybe_min(self, rhs: In) -> Out;
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

impl ops::Add<f32> for Number {
    type Output = Number;

    fn add(self, rhs: f32) -> Self {
        match self {
            Number::Defined(val) => Number::Defined(val + rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::Defined(l), Number::Defined(r)) => Number::Defined(l + r),
            (Number::Defined(_), _) => self,
            _ => Number::Undefined,
        }
    }
}

impl ops::Sub<f32> for Number {
    type Output = Number;

    fn sub(self, rhs: f32) -> Self {
        match self {
            Number::Defined(val) => Number::Defined(val - rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Sub<Number> for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::Defined(l), Number::Defined(r)) => Number::Defined(l - r),
            (Number::Defined(_), _) => self,
            _ => Number::Undefined,
        }
    }
}

impl ops::Mul<f32> for Number {
    type Output = Number;

    fn mul(self, rhs: f32) -> Self {
        match self {
            Number::Defined(val) => Number::Defined(val * rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::Defined(l), Number::Defined(r)) => Number::Defined(l * r),
            (Number::Defined(_), _) => self,
            _ => Number::Undefined,
        }
    }
}

impl ops::Div<f32> for Number {
    type Output = Number;

    fn div(self, rhs: f32) -> Self {
        match self {
            Number::Defined(val) => Number::Defined(val / rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Div<Number> for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::Defined(l), Number::Defined(r)) => Number::Defined(l / r),
            (Number::Defined(_), _) => self,
            _ => Number::Undefined,
        }
    }
}
