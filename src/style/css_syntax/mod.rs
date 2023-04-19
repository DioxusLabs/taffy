use core::fmt;

use crate::style::css_syntax::values::non_negative;
use crate::style::css_syntax::values::MaybeAuto;
use crate::style::Style;
use crate::Taffy;
use cssparser::match_ignore_ascii_case;
use cssparser::BasicParseErrorKind;
use cssparser::CowRcStr;
use cssparser::DeclarationListParser;
use cssparser::Parser;
use cssparser::ParserInput;
use cssparser::SourceLocation;
use cssparser::Token;

mod values;

#[derive(Debug, Clone)]
pub struct CssParseError<'input> {
    location: SourceLocation,
    declaration_source: &'input str,
    kind: ParseErrorKind<'input>,
}

#[derive(Debug, Clone)]
enum ParseErrorKind<'i> {
    InvalidOrUnknownProperty,
    InvalidOrUnknownKeyword(CowRcStr<'i>),
    UnexpectedToken(Token<'i>),
    UnexpectedEndOfInput,
    NegativeValue,
}

type ParseError<'i> = cssparser::ParseError<'i, ParseErrorKind<'i>>;

impl fmt::Display for CssParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let SourceLocation { line, column } = self.location;
        write!(f, "{} at {line}:{column}: `{}`", self.kind, self.declaration_source)
    }
}

impl fmt::Display for ParseErrorKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOrUnknownProperty => write!(f, "invalid or unknown property"),
            Self::InvalidOrUnknownKeyword(keyword) => write!(f, "invalid or unknown keyword: `{keyword}`"),
            Self::UnexpectedEndOfInput => write!(f, "unexpected end of input"),
            Self::UnexpectedToken(token) => write!(f, "unexpected token {token:?}"),
            Self::NegativeValue => write!(f, "value must be positive or zero"),
        }
    }
}

impl<'i> From<BasicParseErrorKind<'i>> for ParseErrorKind<'i> {
    fn from(kind: BasicParseErrorKind<'i>) -> Self {
        match kind {
            BasicParseErrorKind::UnexpectedToken(token) => Self::UnexpectedToken(token),
            BasicParseErrorKind::EndOfInput => Self::UnexpectedEndOfInput,
            BasicParseErrorKind::AtRuleInvalid(_)
            | BasicParseErrorKind::AtRuleBodyInvalid
            | BasicParseErrorKind::QualifiedRuleInvalid => unreachable!(),
        }
    }
}

impl Taffy {
    /// Parse [`Style`] from a list declarations in CSS syntax.
    ///
    /// Parsing is infallible.
    /// Errors such as invalid syntax or unknown/unsupported property or value
    /// are logged in the returned `Vec` and cause the current declaration to be ignored.
    /// Unspecified properties get their initial values from [`Style::DEFAULT`].
    ///
    /// Requires the `css-syntax` Cargo feature to be enabled.
    pub fn parse_css_style<'input>(&self, css: &'input str) -> (Style, Vec<CssParseError<'input>>) {
        let mut errors = Vec::new();
        let mut input = ParserInput::new(css);
        let mut parser = Parser::new(&mut input);
        let declaration_parser = DeclarationParser { style: Style::DEFAULT, taffy: self };
        let mut iter = DeclarationListParser::new(&mut parser, declaration_parser);
        for result in &mut iter {
            match result {
                Ok(()) => {}
                Err((error, declaration_source)) => errors.push(CssParseError {
                    location: error.location,
                    declaration_source,
                    kind: match error.kind {
                        cssparser::ParseErrorKind::Basic(kind) => kind.into(),
                        cssparser::ParseErrorKind::Custom(kind) => kind,
                    },
                }),
            }
        }
        (iter.parser.style, errors)
    }

    /// Serialize the given style to CSS syntax
    pub fn style_to_css(&self, style: &Style) -> String {
        // Cause a compiler error or warning if we never set a struct field during parsing
        let Style {
            display,
            overflow,
            position,
            inset,
            size,
            min_size,
            max_size,
            aspect_ratio,
            margin,
            padding,
            border,
            align_items,
            align_self,
            justify_items,
            justify_self,
            align_content,
            justify_content,
            gap,
            flex_direction,
            flex_wrap,
            flex_basis,
            flex_grow,
            flex_shrink,
            grid_template_rows,
            grid_template_columns,
            grid_auto_rows,
            grid_auto_columns,
            grid_auto_flow,
            grid_row,
            grid_column,
        } = style;

        let mut css = String::new();

        // This is a macro because closures can’t take a generic parameter
        // and `fn` items can’t capture local variables.
        macro_rules! decl {
            ($name: expr, $value: expr) => {
                serialize_one_declaration(self, &mut css, $name, $value)
            };
        }

        // TODO: deal with shorthand v.s. longhand properties per
        // https://drafts.csswg.org/cssom/#serialize-a-css-declaration-block
        if *display != Style::DEFAULT.display {
            decl!("display", display);
        }
        todo!()
    }
}

fn serialize_one_declaration(taffy: &Taffy, dest: &mut String, name: &str, value: &impl CssValue) {
    if !dest.is_empty() {
        dest.push_str("; ")
    }
    // `unwrap` should never panic since `impl fmt::Write for String` never returns `Err`
    cssparser::serialize_identifier(name, dest).unwrap();
    dest.push_str(": ");
    value.serialize(dest, taffy);
}

struct DeclarationParser<'taffy> {
    style: Style,
    taffy: &'taffy Taffy,
}

impl<'i> cssparser::DeclarationParser<'i> for DeclarationParser<'_> {
    // Instead of a data structure to return a single parsed declaration
    // we mutate `self.style` in place.
    type Declaration = ();

    type Error = ParseErrorKind<'i>;

    fn parse_value<'t>(
        &mut self,
        property_name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<(), ParseError<'i>> {
        // Cause a compiler error or warning if we never set a struct field during parsing
        let Style {
            display,
            overflow,
            position,
            inset,
            size,
            min_size,
            max_size,
            aspect_ratio,
            margin,
            padding,
            border,
            align_items,
            align_self,
            justify_items,
            justify_self,
            align_content,
            justify_content,
            gap,
            flex_direction,
            flex_wrap,
            flex_basis,
            flex_grow,
            flex_shrink,
            grid_template_rows,
            grid_template_columns,
            grid_auto_rows,
            grid_auto_columns,
            grid_auto_flow,
            grid_row,
            grid_column,
        } = &mut self.style;

        // This is a macro because closures can’t have a generic return type
        // and `fn` items can’t capture local variables.
        macro_rules! parse {
            () => {
                input.parse_entirely(|input| CssValue::parse(input, &self.taffy))?
            };
        }

        match_ignore_ascii_case! { &*property_name,
            // https://drafts.csswg.org/css2/#display-prop
            // https://drafts.csswg.org/css-flexbox/#flex-containers
            // https://drafts.csswg.org/css-grid/#grid-containers
            "display" => *display = parse!(),
            // https://w3c.github.io/csswg-drafts/css-overflow/#propdef-overflow
            "overflow" => *overflow = parse!(),
            "overflow-x" => overflow.x = parse!(),
            "overflow-y" => overflow.y = parse!(),
            // https://w3c.github.io/csswg-drafts/css-position-3/#position-property
            "position" => *position = parse!(),
            // https://w3c.github.io/csswg-drafts/css-position-3/#inset-shorthands
            "inset" => *inset = parse!(),
            "top" => inset.top = parse!(),
            "right" => inset.right = parse!(),
            "bottom" => inset.bottom = parse!(),
            "left" => inset.left = parse!(),
            // https://drafts.csswg.org/css-sizing/#preferred-size-properties
            "width" => size.width = non_negative(parse!()),
            "height" => size.height = non_negative(parse!()),
            // https://drafts.csswg.org/css-sizing/#min-size-properties
            "min-width" => min_size.width = non_negative(parse!()),
            "min-height" => min_size.height = non_negative(parse!()),
            // https://drafts.csswg.org/css-sizing/#max-size-properties
            "max-width" => max_size.width = non_negative(parse!()),
            "max-height" => max_size.height = non_negative(parse!()),
            // https://w3c.github.io/csswg-drafts/css-sizing-4/#aspect-ratio
            "aspect-ratio" => *aspect_ratio = MaybeAuto::to_opt_f32(parse!()),
            // https://drafts.csswg.org/css2/#margin-properties
            "margin" => *margin = parse!(),
            "margin-top" => margin.top = parse!(),
            "margin-right" => margin.right = parse!(),
            "margin-bottom" => margin.bottom = parse!(),
            "margin-left" => margin.left = parse!(),
            // https://drafts.csswg.org/css2/#padding-properties
            "padding" => *padding = non_negative(parse!()),
            "padding-top" => padding.top = non_negative(parse!()),
            "padding-right" => padding.right = non_negative(parse!()),
            "padding-bottom" => padding.bottom = non_negative(parse!()),
            "padding-left" => padding.left = non_negative(parse!()),

            _ => {
                return Err(input.new_custom_error(ParseErrorKind::InvalidOrUnknownProperty))
            }
        }
        Ok(())
    }
}

// Default methods always return Err for no supported at-rule
impl<'i> cssparser::AtRuleParser<'i> for DeclarationParser<'_> {
    type Prelude = ();
    type AtRule = ();
    type Error = ParseErrorKind<'i>;
}

// Default methods always return Err for no supported nested qualified rule
impl<'i> cssparser::QualifiedRuleParser<'i> for DeclarationParser<'_> {
    type Prelude = ();
    type QualifiedRule = ();
    type Error = ParseErrorKind<'i>;
}

trait CssValue: Sized {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>, taffy: &Taffy) -> Result<Self, ParseError<'i>>;

    fn serialize(&self, dest: &mut String, taffy: &Taffy);
}
