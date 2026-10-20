//! Style type for representing available space as a sizing constraint
use crate::{
    prelude::{FromLength, TaffyMaxContent, TaffyMinContent, TaffyZero},
    util::au::px_to_au,
    Size,
};
use app_units::{Au, MAX_AU};

#[cfg(feature = "parse")]
use crate::util::parse::{from_str_from_css, CssParseResult, FromCss, Parser, Token};

/// The amount of space available to a node in a given axis
/// <https://www.w3.org/TR/css-sizing-3/#available>
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AvailableSpace {
    /// The amount of space available is the specified amount
    Definite(Au),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}
impl TaffyZero for AvailableSpace {
    const ZERO: Self = Self::Definite(Au(0));
}
impl TaffyMaxContent for AvailableSpace {
    const MAX_CONTENT: Self = Self::MaxContent;
}
impl TaffyMinContent for AvailableSpace {
    const MIN_CONTENT: Self = Self::MinContent;
}
impl FromLength for AvailableSpace {
    fn from_length<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::Definite(px_to_au(value.into() as f32))
    }
}

#[cfg(feature = "parse")]
impl FromCss for AvailableSpace {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        match parser.next()?.clone() {
            Token::Number { value, .. } if value >= 0.0 => Ok(Self::Definite(px_to_au(value))),
            Token::Dimension { value, .. } if value >= 0.0 => Ok(Self::Definite(px_to_au(value))),
            Token::Ident(ident) if ident == "max-content" => Ok(Self::MaxContent),
            Token::Ident(ident) if ident == "min-content" => Ok(Self::MinContent),
            token => Err(parser.new_unexpected_token_error(token))?,
        }
    }
}
#[cfg(feature = "parse")]
from_str_from_css!(AvailableSpace);

impl AvailableSpace {
    /// Returns true for definite values, else false
    pub const fn is_definite(self) -> bool {
        matches!(self, AvailableSpace::Definite(_))
    }

    /// Convert to Option
    /// Definite values become Some(value). Constraints become None.
    pub const fn into_option(self) -> Option<Au> {
        match self {
            AvailableSpace::Definite(value) => Some(value),
            _ => None,
        }
    }

    /// Return the definite value or a default value
    pub fn unwrap_or(self, default: Au) -> Au {
        self.into_option().unwrap_or(default)
    }

    /// Return the definite value. Panic is the value is not definite.
    #[track_caller]
    pub fn unwrap(self) -> Au {
        self.into_option().unwrap()
    }

    /// Return self if definite or a default value
    pub fn or(self, default: AvailableSpace) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(_) => self,
            _ => default,
        }
    }

    /// Return self if definite or a the result of the default value callback
    pub fn or_else(self, default_cb: impl FnOnce() -> AvailableSpace) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(_) => self,
            _ => default_cb(),
        }
    }

    /// Return the definite value or the result of the default value callback
    pub fn unwrap_or_else(self, default_cb: impl FnOnce() -> Au) -> Au {
        self.into_option().unwrap_or_else(default_cb)
    }

    /// If passed value is Some then return AvailableSpace::Definite containing that value, else return self
    pub fn maybe_set(self, value: Option<Au>) -> AvailableSpace {
        match value {
            Some(value) => AvailableSpace::Definite(value),
            None => self,
        }
    }

    /// If passed value is Some then return AvailableSpace::Definite containing that value, else return self
    pub fn map_definite_value(self, map_function: impl FnOnce(Au) -> Au) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(value) => AvailableSpace::Definite(map_function(value)),
            _ => self,
        }
    }

    /// Compute free_space given the passed used_space
    pub fn compute_free_space(&self, used_space: Au) -> Au {
        match self {
            AvailableSpace::MaxContent => MAX_AU,
            AvailableSpace::MinContent => Au(0),
            AvailableSpace::Definite(available_space) => *available_space - used_space,
        }
    }

    /// Compare equality with another AvailableSpace. Au values are exact, so definite values are
    /// equal only if they are identical
    pub fn is_roughly_equal(self, other: AvailableSpace) -> bool {
        use AvailableSpace::*;
        match (self, other) {
            (Definite(a), Definite(b)) => a == b,
            (MinContent, MinContent) => true,
            (MaxContent, MaxContent) => true,
            _ => false,
        }
    }
}

impl From<Au> for AvailableSpace {
    fn from(value: Au) -> Self {
        Self::Definite(value)
    }
}

impl From<Option<Au>> for AvailableSpace {
    fn from(option: Option<Au>) -> Self {
        match option {
            Some(value) => Self::Definite(value),
            None => Self::MaxContent,
        }
    }
}

impl Size<AvailableSpace> {
    /// Convert `Size<AvailableSpace>` into `Size<Option<Au>>`
    pub fn into_options(self) -> Size<Option<Au>> {
        Size { width: self.width.into_option(), height: self.height.into_option() }
    }

    /// If passed value is Some then return AvailableSpace::Definite containing that value, else return self
    pub fn maybe_set(self, value: Size<Option<Au>>) -> Size<AvailableSpace> {
        Size { width: self.width.maybe_set(value.width), height: self.height.maybe_set(value.height) }
    }
}
