use std::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Number {
    Defined(f32),
    Undefined,
}

impl Default for Number {
    fn default() -> Number {
        Number::Undefined
    }
}

impl Number {
    pub fn unwrap_or(self, default: f32) -> f32 {
        match self {
            Number::Defined(val) => val,
            Number::Undefined => default,
        }
    }

    pub fn is_defined(self) -> bool {
        match self {
            Number::Defined(_) => true,
            Number::Undefined => false,
        }
    }

    pub fn from_f32(from: f32) -> Number {
        if from.is_nan() {
            Number::Undefined
        } else {
            Number::Defined(from)
        }
    }
}

pub trait MinMax<T> {
    fn maybe_min(self, rhs: T) -> Number;
    fn maybe_max(self, rhs: T) -> Number;
}

impl MinMax<Number> for Number {
    fn maybe_min(self, rhs: Number) -> Number {
        match self {
            Number::Defined(val) => match rhs {
                Number::Defined(other) => Number::Defined(val.min(other)),
                Number::Undefined => self,
            },
            Number::Undefined => Number::Undefined,
        }
    }

    fn maybe_max(self, rhs: Number) -> Number {
        match self {
            Number::Defined(val) => match rhs {
                Number::Defined(other) => Number::Defined(val.max(other)),
                Number::Undefined => self,
            },
            Number::Undefined => Number::Undefined,
        }
    }
}

impl MinMax<f32> for Number {
    fn maybe_min(self, rhs: f32) -> Number {
        match self {
            Number::Defined(val) => Number::Defined(val.min(rhs)),
            Number::Undefined => Number::Undefined,
        }
    }

    fn maybe_max(self, rhs: f32) -> Number {
        match self {
            Number::Defined(val) => Number::Defined(val.max(rhs)),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl MinMax<Number> for f32 {
    fn maybe_min(self, rhs: Number) -> Number {
        match rhs {
            Number::Defined(val) => Number::Defined(self.min(val)),
            Number::Undefined => Number::Defined(self),
        }
    }

    fn maybe_max(self, rhs: Number) -> Number {
        match rhs {
            Number::Defined(val) => Number::Defined(self.max(val)),
            Number::Undefined => Number::Defined(self),
        }
    }
}

impl ops::Add<f32> for Number {
    type Output = Number;

    fn add(self, rhs: f32) -> Number {
        match self {
            Number::Defined(val) => Number::Defined(val + rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Number {
        match self {
            Number::Defined(val) => match rhs {
                Number::Defined(other) => Number::Defined(val + other),
                Number::Undefined => self,
            },
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Sub<f32> for Number {
    type Output = Number;

    fn sub(self, rhs: f32) -> Number {
        match self {
            Number::Defined(val) => Number::Defined(val - rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Sub<Number> for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Number {
        match self {
            Number::Defined(val) => match rhs {
                Number::Defined(other) => Number::Defined(val - other),
                Number::Undefined => self,
            },
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Mul<f32> for Number {
    type Output = Number;

    fn mul(self, rhs: f32) -> Number {
        match self {
            Number::Defined(val) => Number::Defined(val * rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Number {
        match self {
            Number::Defined(val) => match rhs {
                Number::Defined(other) => Number::Defined(val * other),
                Number::Undefined => self,
            },
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Div<f32> for Number {
    type Output = Number;

    fn div(self, rhs: f32) -> Number {
        match self {
            Number::Defined(val) => Number::Defined(val / rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Div<Number> for Number {
    type Output = Number;

    fn div(self, rhs: Number) -> Number {
        match self {
            Number::Defined(val) => match rhs {
                Number::Defined(other) => Number::Defined(val / other),
                Number::Undefined => self,
            },
            Number::Undefined => Number::Undefined,
        }
    }
}
