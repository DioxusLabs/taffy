//! Helpers for parsing style types from strings

use cssparser::BasicParseError;
pub(crate) use cssparser::{Parser, Token};
use std::borrow::Cow;

/// Error type for parsing a type from `cssparser::Parser`
pub(crate) type CssParseError<'i> = cssparser::ParseError<Cow<'i, str>>;

/// Result type for parsing a type from `cssparser::Parser`
pub(crate) type CssParseResult<'i, T> = Result<T, CssParseError<'i>>;

/// Error type for parsing a type from string
#[derive(Clone, Debug)]
pub struct ParseError(String);

impl core::fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(fmt, "{}", &self.0)
    }
}
impl<'i> From<CssParseError<'i>> for ParseError {
    fn from(value: CssParseError<'i>) -> Self {
        Self(value.to_string())
    }
}
impl From<BasicParseError> for ParseError {
    fn from(value: BasicParseError) -> Self {
        Self(value.kind.to_string())
    }
}
#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

/// Result type for parsing a type from string
pub type ParseResult<T> = Result<T, ParseError>;

/// Trait for parsing a type from a `cssparser::Parser` input
pub(crate) trait FromCss: Sized {
    /// Parse type from a `cssparser::Parser` input
    fn from_css<'i>(parser: &mut Parser<'i>) -> CssParseResult<'i, Self>;
}

/// Parse a string into a type that implements `FromCss`, ensuring that the
/// entire input string is consumed.
pub(crate) fn parse_css_str_entirely<T: FromCss>(input: &str) -> Result<T, ParseError> {
    let mut parser = Parser::new(input);
    parser.parse_entirely(|parser| T::from_css(parser)).map_err(|err| ParseError(err.to_string()))
}

/// Automatically implement `FromStr` for a type that already implemented `FromCss`
macro_rules! from_str_from_css {
    ($ty:ident) => {
        #[cfg(feature = "parse")]
        impl core::str::FromStr for $ty {
            type Err = $crate::ParseError;
            fn from_str(input: &str) -> Result<Self, Self::Err> {
                $crate::util::parse::parse_css_str_entirely(input)
            }
        }
    };
}
pub(crate) use from_str_from_css;

/// Implements the `FromCss` and `FromStr` traits for a simple enum that consists of just keywords
macro_rules! impl_parse_for_keyword_enum {
    (
        // The type name (followed by a comma)
        $ty:ident,

        // Repeat 1-or-more times
        $(
            // 0-or-more metadata attributes (e.g. #[cfg] attributes for conditional compilation)
            $( #[$meta: meta] )*

            // keyword => enum_variant (e.g. "center" => AlignItems::Center)
            $keyword:literal => $enum_variant:ident,
        )+
    ) => {

        // Impl FromCss for the type
        impl $crate::util::parse::FromCss for $ty {
            fn from_css<'i>(input: &mut cssparser::Parser<'i>) -> $crate::util::parse::CssParseResult<'i, Self> {

                // Parse an ident (or return an error)
                let ident = input.expect_ident()?;

                // Match on the ident (case insensitive)
                cssparser::match_ignore_ascii_case! { &*ident,

                    // Define each item in the macro definition as a match case
                    $(
                        // Add the metadata attributes to the match case
                        $( #[$meta] )*
                        // If the keyword matches, return the corresponding variant of the enum
                        $keyword => return Ok(Self::$enum_variant),
                    )+

                    // If none of the cases match the input, return an "unexpected token" error
                    _ => Err($crate::util::parse::CssParseError::unexpected_token()),
                }
            }
        }

        // Impl FromStr for the type
        impl core::str::FromStr for $ty {
            type Err = $crate::ParseError;
            fn from_str(input: &str) -> Result<Self, Self::Err> {
                $crate::util::parse::parse_css_str_entirely(input)
            }
        }
    };
}
pub(crate) use impl_parse_for_keyword_enum;
