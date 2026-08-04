//! Return types for C FFI

#[macro_export]
macro_rules! ok {
    ($value:expr) => {
        return TaffyFFIResult::from_value($value);
    };
}

#[macro_export]
macro_rules! bail {
    ($return_code:ident) => {
        return TaffyFFIResult::from_return_code(TaffyReturnCode::$return_code);
    };
}

#[macro_export]
macro_rules! bail_if_null {
    ($raw_ptr:expr, $err_code:ident) => {
        if $raw_ptr.is_null() {
            $crate::bail!($err_code);
        }
    };
}

#[macro_export]
macro_rules! debug_assert_non_null {
    ($raw_ptr:expr) => {
        #[cfg(debug)]
        if $raw_ptr.is_null() {
            eprintln!("Supplied pointer was null");
            ::std::process::exit(1);
        }
    };
}

#[macro_export]
macro_rules! try_or {
    ($error_code:ident, $block:expr) => {
        match { $block } {
            Ok(val) => val,
            Err(_) => {
                bail!($error_code);
            }
        }
    };
}

pub(crate) trait TaffyFFIResult {
    type Value;
    fn from_value(value: Self::Value) -> Self;
    fn from_return_code(return_code: TaffyReturnCode) -> Self;
}

pub(crate) trait TaffyFFIDefault {
    fn default() -> Self;
}
impl TaffyFFIDefault for f32 {
    fn default() -> Self {
        0.0
    }
}
impl TaffyFFIDefault for i32 {
    fn default() -> Self {
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum TaffyReturnCode {
    /// Operation suceeded
    Ok,
    /// The style pointer passed was null
    NullStylePointer,
    /// The tree pointer passed was null
    NullTreePointer,
    /// The node referenced by the node id passed does not exist
    InvalidNodeId,
    /// An enum value was specified that was outside the range of valid value for this enum
    InvalidEnumValue,
    /// A Points unit was specified but is not valid in this context
    InvalidNone,
    /// A Points unit was specified but is not valid in this context
    InvalidPoints,
    /// A Percent unit was specified but is not valid in this context
    InvalidPercent,
    /// A MinContent unit was specified but is not valid in this context
    InvalidMinContent,
    /// A MaxContent unit was specified but is not valid in this context
    InvalidMaxContent,
    /// A FitContentPx unit was specified but is not valid in this context
    InvalidFitContentPx,
    /// A FitContentPercent unit was specified but is not valid in this context
    InvalidFitContentPercent,
    /// An Auto unit was specified but is not valid in this context
    InvalidAuto,
    /// An Fr unit was specified but is not valid in this context
    InvalidFr,
    /// A NaN value was specified but is not valid in this context
    UnexpectedNaN,
    /// A infinite value was specified but is not valid in this context
    UnexpectedInfinity,
    /// A negative value was specified but is not valid in this context
    UnexpectedNegative,
}

impl TaffyFFIResult for TaffyReturnCode {
    type Value = TaffyReturnCode;
    fn from_value(value: Self::Value) -> Self {
        value
    }
    fn from_return_code(return_code: TaffyReturnCode) -> Self {
        return_code
    }
}

#[repr(C)]
pub struct TaffyResult<T> {
    pub return_code: TaffyReturnCode,
    pub value: T,
}

impl<T: TaffyFFIDefault> TaffyFFIResult for TaffyResult<T> {
    type Value = T;
    fn from_value(value: Self::Value) -> Self {
        Self { return_code: TaffyReturnCode::Ok, value }
    }
    fn from_return_code(return_code: TaffyReturnCode) -> Self {
        Self { return_code, value: T::default() }
    }
}
