//! Helpers for parsing style types from strings

pub(crate) use cssparser::{ParseError, Parser, ParserInput, Token};
use std::borrow::Cow;

/// Result type for parsing a type from a `cssparser::Parser` input
pub(crate) type ParseResult<'i, T> = Result<T, ParseError<'i, Cow<'i, str>>>;

/// Trait for parsing a type from a `cssparser::Parser` input
pub(crate) trait FromCss: Sized {
    /// Parse type from a `cssparser::Parser` input
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self>;
}

/// Parse a string into a type that implements `FromCss`, ensuring that the
/// entire input string is consumed.
pub(crate) fn parse_css_str_entirely<T: FromCss>(input: &str) -> Result<T, ()> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);
    parser.parse_entirely(|parser| T::from_css(parser)).map_err(|_| ())
    // Ok(parser.parse_entirely(|parser| T::from_css(parser)).unwrap())
}

/// Automatically implement `FromStr` for a type that already implemented `FromCss`
macro_rules! from_str_from_css {
    ($ty:ident) => {
        #[cfg(feature = "parse")]
        impl core::str::FromStr for $ty {
            type Err = ();
            fn from_str(input: &str) -> Result<Self, Self::Err> {
                parse_css_str_entirely(input)
            }
        }
    };
}
pub(crate) use from_str_from_css;
