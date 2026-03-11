use std::borrow::Cow;

pub(crate) use cssparser::{ParseError, Parser, ParserInput, Token};

pub(crate) type ParseResult<'i, T> = Result<T, ParseError<'i, Cow<'i, str>>>;

#[doc(hidden)]
pub trait FromCss: Sized {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self>;
}

pub(crate) fn parse_css_str_entirely<'i, T: FromCss>(input: &'i str) -> Result<T, ()> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);
    parser.parse_entirely(|parser| T::from_css(parser)).map_err(|_| ())
    // Ok(parser.parse_entirely(|parser| T::from_css(parser)).unwrap())
}

macro_rules! from_str_from_css {
    ($ty:ident) => {
        #[cfg(feature = "from_str")]
        impl core::str::FromStr for $ty {
            type Err = ();
            fn from_str(input: &str) -> Result<Self, Self::Err> {
                parse_css_str_entirely(input)
            }
        }
    };
}
pub(crate) use from_str_from_css;
