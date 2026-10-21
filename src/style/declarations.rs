//! Parsing of CSS declaration lists (the contents of an HTML `style` attribute) into a [`Style`]
use super::{
    AlignContent, AlignItems, BoxSizing, CheapCloneStr, Contain, Dimension, Direction, Display, LengthPercentage,
    LengthPercentageAuto, Overflow, Position, Style,
};
use crate::geometry::{Point, Rect, Size};
use crate::util::parse::{
    parse_css_str_entirely, CssParseError, CssParseResult, FromCss, ParseError, Parser, ParserInput, Token,
};
use cssparser::{AtRuleParser, CowRcStr, DeclarationParser, ParserState, QualifiedRuleParser, RuleBodyItemParser};
use std::borrow::Cow;

#[cfg(feature = "block_layout")]
use super::TextAlign;
#[cfg(feature = "float_layout")]
use super::{Clear, Float};
#[cfg(feature = "flexbox")]
use super::{FlexDirection, FlexWrap};
#[cfg(feature = "grid")]
use super::{GridAutoFlow, GridAutoTracks, GridPlacement, GridTemplateComponent, GridTemplateTracks, Line};
#[cfg(feature = "grid")]
use crate::sys::Vec;

/// Keywords accepted as `<line-style>` in the `border` shorthands. They are ignored because
/// border-style is not modelled by Taffy, so `none` does not zero a width given alongside it.
const LINE_STYLE_KEYWORDS: [&str; 10] =
    ["none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset"];

/// Why a declaration could not be applied
enum DeclarationError<'i> {
    /// The property name is not one Taffy can express
    UnknownProperty,
    /// The property name is known but the value is invalid
    InvalidValue(CssParseError<'i>),
}

impl<'i> From<CssParseError<'i>> for DeclarationError<'i> {
    fn from(err: CssParseError<'i>) -> Self {
        Self::InvalidValue(err)
    }
}

impl<'i> From<cssparser::BasicParseError<'i>> for DeclarationError<'i> {
    fn from(err: cssparser::BasicParseError<'i>) -> Self {
        Self::InvalidValue(err.into())
    }
}

/// Applies declarations in source order to a style being built up from `Style::DEFAULT`
struct Declarations<S: CheapCloneStr> {
    /// The style being built up
    style: Style<S>,
}

impl<'i, S: CheapCloneStr> DeclarationParser<'i> for Declarations<S> {
    type Declaration = ();
    type Error = Cow<'i, str>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        _declaration_start: &ParserState,
    ) -> Result<(), CssParseError<'i>> {
        let style = &mut self.style;
        input.parse_until_before(cssparser::Delimiter::Bang, |input| match apply(style, &name, input) {
            Ok(()) => Ok(()),
            Err(DeclarationError::UnknownProperty) => Err(input.new_custom_error(format!("unknown property `{name}`"))),
            Err(DeclarationError::InvalidValue(err)) => {
                Err(input.new_custom_error(format!("invalid value for `{name}`: {err}")))
            }
        })?;
        let _ = input.try_parse(cssparser::parse_important);
        input.expect_exhausted()?;
        Ok(())
    }
}

impl<'i, S: CheapCloneStr> AtRuleParser<'i> for Declarations<S> {
    type Prelude = ();
    type AtRule = ();
    type Error = Cow<'i, str>;
}

impl<'i, S: CheapCloneStr> QualifiedRuleParser<'i> for Declarations<S> {
    type Prelude = ();
    type QualifiedRule = ();
    type Error = Cow<'i, str>;
}

impl<'i, S: CheapCloneStr> RuleBodyItemParser<'i, (), Cow<'i, str>> for Declarations<S> {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        false
    }
}

impl<S: CheapCloneStr> FromCss for Style<S> {
    fn from_css<'i>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        let mut declarations = Declarations { style: Self::DEFAULT };
        for declaration in cssparser::RuleBodyParser::new(input, &mut declarations) {
            declaration.map_err(|(err, _)| err)?;
        }
        Ok(declarations.style)
    }
}

impl<S: CheapCloneStr> core::str::FromStr for Style<S> {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_css_str_entirely(input)
    }
}

/// Applies a single declaration to `style`. Only assigns the style when the value is valid.
fn apply<'i, S: CheapCloneStr>(
    style: &mut Style<S>,
    name: &str,
    input: &mut Parser<'i, '_>,
) -> Result<(), DeclarationError<'i>> {
    cssparser::match_ignore_ascii_case! { name,
        "display" => style.display = Display::from_css(input)?,
        "box-sizing" => style.box_sizing = BoxSizing::from_css(input)?,
        "direction" => style.direction = Direction::from_css(input)?,
        "position" => style.position = Position::from_css(input)?,
        "contain" => style.contain = Contain::from_css(input)?,
        "overflow" => {
            let (x, y) = parse_pair(input, Overflow::from_css)?;
            style.overflow = Point { x, y };
        },
        "overflow-x" => style.overflow.x = Overflow::from_css(input)?,
        "overflow-y" => style.overflow.y = Overflow::from_css(input)?,
        "inset" => style.inset = parse_quad(input, LengthPercentageAuto::from_css)?,
        "top" => style.inset.top = LengthPercentageAuto::from_css(input)?,
        "right" => style.inset.right = LengthPercentageAuto::from_css(input)?,
        "bottom" => style.inset.bottom = LengthPercentageAuto::from_css(input)?,
        "left" => style.inset.left = LengthPercentageAuto::from_css(input)?,
        "width" => style.size.width = Dimension::from_css(input)?,
        "height" => style.size.height = Dimension::from_css(input)?,
        "min-width" => style.min_size.width = LengthPercentageAuto::from_css(input)?,
        "min-height" => style.min_size.height = LengthPercentageAuto::from_css(input)?,
        "max-width" => style.max_size.width = LengthPercentageAuto::from_css(input)?,
        "max-height" => style.max_size.height = LengthPercentageAuto::from_css(input)?,
        "aspect-ratio" => style.aspect_ratio = parse_aspect_ratio(input)?,
        "margin" => style.margin = parse_quad(input, LengthPercentageAuto::from_css)?,
        "margin-top" => style.margin.top = LengthPercentageAuto::from_css(input)?,
        "margin-right" => style.margin.right = LengthPercentageAuto::from_css(input)?,
        "margin-bottom" => style.margin.bottom = LengthPercentageAuto::from_css(input)?,
        "margin-left" => style.margin.left = LengthPercentageAuto::from_css(input)?,
        "padding" => style.padding = parse_quad(input, LengthPercentage::from_css)?,
        "padding-top" => style.padding.top = LengthPercentage::from_css(input)?,
        "padding-right" => style.padding.right = LengthPercentage::from_css(input)?,
        "padding-bottom" => style.padding.bottom = LengthPercentage::from_css(input)?,
        "padding-left" => style.padding.left = LengthPercentage::from_css(input)?,
        "border-width" => style.border = parse_quad(input, parse_line_width)?,
        "border-top-width" => style.border.top = parse_line_width(input)?,
        "border-right-width" => style.border.right = parse_line_width(input)?,
        "border-bottom-width" => style.border.bottom = parse_line_width(input)?,
        "border-left-width" => style.border.left = parse_line_width(input)?,
        "border" => {
            let width = parse_border(input)?;
            style.border = Rect { top: width, right: width, bottom: width, left: width };
        },
        "border-top" => style.border.top = parse_border(input)?,
        "border-right" => style.border.right = parse_border(input)?,
        "border-bottom" => style.border.bottom = parse_border(input)?,
        "border-left" => style.border.left = parse_border(input)?,
        "align-self" => style.align_self = parse_self_alignment(input)?,
        "justify-self" => style.justify_self = parse_self_alignment(input)?,
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        "align-items" => style.align_items = AlignItems::from_css(input)?,
        #[cfg(any(feature = "grid", feature = "block_layout"))]
        "justify-items" => style.justify_items = AlignItems::from_css(input)?,
        #[cfg(any(feature = "flexbox", feature = "grid", feature = "block_layout"))]
        "align-content" => style.align_content = AlignContent::from_css(input)?,
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        "justify-content" => style.justify_content = AlignContent::from_css(input)?,
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        "gap" => {
            let (row, column) = parse_pair(input, LengthPercentage::from_css)?;
            style.gap = Size { width: column, height: row };
        },
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        "row-gap" => style.gap.height = LengthPercentage::from_css(input)?,
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        "column-gap" => style.gap.width = LengthPercentage::from_css(input)?,
        #[cfg(feature = "block_layout")]
        "text-align" => style.text_align = TextAlign::from_css(input)?,
        #[cfg(feature = "float_layout")]
        "float" => style.float = Float::from_css(input)?,
        #[cfg(feature = "float_layout")]
        "clear" => style.clear = Clear::from_css(input)?,
        #[cfg(feature = "flexbox")]
        "flex-direction" => style.flex_direction = FlexDirection::from_css(input)?,
        #[cfg(feature = "flexbox")]
        "flex-wrap" => style.flex_wrap = FlexWrap::from_css(input)?,
        #[cfg(feature = "flexbox")]
        "flex-flow" => {
            let (direction, wrap) = parse_flex_flow(input)?;
            style.flex_direction = direction.unwrap_or(FlexDirection::Row);
            style.flex_wrap = wrap.unwrap_or(FlexWrap::NoWrap);
        },
        #[cfg(feature = "flexbox_balance")]
        "flex-line-count" => style.flex_line_count = parse_flex_line_count(input)?,
        #[cfg(feature = "flexbox")]
        "flex-basis" => style.flex_basis = Dimension::from_css(input)?,
        #[cfg(feature = "flexbox")]
        "flex-grow" => style.flex_grow = parse_non_negative(input)?,
        #[cfg(feature = "flexbox")]
        "flex-shrink" => style.flex_shrink = parse_non_negative(input)?,
        #[cfg(feature = "flexbox")]
        "flex" => {
            let (grow, shrink, basis) = parse_flex(input)?;
            style.flex_grow = grow;
            style.flex_shrink = shrink;
            style.flex_basis = basis;
        },
        #[cfg(feature = "grid")]
        "grid-template-rows" => {
            let tracks = GridTemplateTracks::from_css(input)?;
            style.grid_template_rows = tracks.tracks;
            style.grid_template_row_names = tracks.line_names;
        },
        #[cfg(feature = "grid")]
        "grid-template-columns" => {
            let tracks = GridTemplateTracks::from_css(input)?;
            style.grid_template_columns = tracks.tracks;
            style.grid_template_column_names = tracks.line_names;
        },
        #[cfg(feature = "grid")]
        "grid-template" | "grid" => {
            let (rows, columns) = parse_rows_and_columns(input)?;
            style.grid_template_rows = rows.tracks;
            style.grid_template_row_names = rows.line_names;
            style.grid_template_columns = columns.tracks;
            style.grid_template_column_names = columns.line_names;
            if name.eq_ignore_ascii_case("grid") {
                style.grid_auto_rows = Vec::new();
                style.grid_auto_columns = Vec::new();
                style.grid_auto_flow = GridAutoFlow::default();
            }
        },
        #[cfg(feature = "grid")]
        "grid-auto-rows" => style.grid_auto_rows = GridAutoTracks::from_css(input)?.0,
        #[cfg(feature = "grid")]
        "grid-auto-columns" => style.grid_auto_columns = GridAutoTracks::from_css(input)?.0,
        #[cfg(feature = "grid")]
        "grid-auto-flow" => style.grid_auto_flow = GridAutoFlow::from_css(input)?,
        #[cfg(feature = "grid")]
        "grid-row-start" => style.grid_row.start = GridPlacement::from_css(input)?,
        #[cfg(feature = "grid")]
        "grid-row-end" => style.grid_row.end = GridPlacement::from_css(input)?,
        #[cfg(feature = "grid")]
        "grid-column-start" => style.grid_column.start = GridPlacement::from_css(input)?,
        #[cfg(feature = "grid")]
        "grid-column-end" => style.grid_column.end = GridPlacement::from_css(input)?,
        #[cfg(feature = "grid")]
        "grid-row" => style.grid_row = parse_grid_line(input)?,
        #[cfg(feature = "grid")]
        "grid-column" => style.grid_column = parse_grid_line(input)?,
        #[cfg(feature = "grid")]
        "grid-area" => {
            let placements = parse_placements(input, 4)?;
            let row_start = placements[0].clone();
            let column_start = placements.get(1).cloned().unwrap_or_else(|| implicit_end(&row_start));
            let row_end = placements.get(2).cloned().unwrap_or_else(|| implicit_end(&row_start));
            let column_end = placements.get(3).cloned().unwrap_or_else(|| implicit_end(&column_start));
            style.grid_row = Line { start: row_start, end: row_end };
            style.grid_column = Line { start: column_start, end: column_end };
        },
        _ => return Err(DeclarationError::UnknownProperty),
    }
    input.expect_exhausted()?;
    Ok(())
}

/// Parses one or two values. A missing second value repeats the first.
fn parse_pair<'i, T: Copy>(
    input: &mut Parser<'i, '_>,
    parse: impl Fn(&mut Parser<'i, '_>) -> CssParseResult<'i, T>,
) -> CssParseResult<'i, (T, T)> {
    let first = parse(input)?;
    let second = if input.is_exhausted() { first } else { parse(input)? };
    Ok((first, second))
}

/// Parses one to four values into a box in CSS order (top, right, bottom, left)
fn parse_quad<'i, T: Copy>(
    input: &mut Parser<'i, '_>,
    parse: impl Fn(&mut Parser<'i, '_>) -> CssParseResult<'i, T>,
) -> CssParseResult<'i, Rect<T>> {
    let top = parse(input)?;
    let right = if input.is_exhausted() { top } else { parse(input)? };
    let bottom = if input.is_exhausted() { top } else { parse(input)? };
    let left = if input.is_exhausted() { right } else { parse(input)? };
    Ok(Rect { top, right, bottom, left })
}

/// Parses a `<line-width>`: a non-negative length or percentage, or `thin` (1px), `medium` (3px) or `thick` (5px)
fn parse_line_width<'i>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, LengthPercentage> {
    let width = match input.next()?.clone() {
        Token::Percentage { unit_value, .. } if unit_value >= 0.0 => return Ok(LengthPercentage::percent(unit_value)),
        Token::Dimension { unit, value, .. } if unit.eq_ignore_ascii_case("px") && value >= 0.0 => value,
        Token::Ident(ident) if ident.eq_ignore_ascii_case("thin") => 1.0,
        Token::Ident(ident) if ident.eq_ignore_ascii_case("medium") => 3.0,
        Token::Ident(ident) if ident.eq_ignore_ascii_case("thick") => 5.0,
        token => return Err(input.new_unexpected_token_error(token)),
    };
    Ok(LengthPercentage::length(width))
}

/// Parses the value of a `border` or `border-<side>` shorthand, keeping only the `<line-width>`.
/// `<line-style>` and `<color>` are accepted and ignored because border-style and border-color are
/// not modelled. A missing line-width resets the width to 0, so `border: none` clears the border.
fn parse_border<'i>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, LengthPercentage> {
    let mut width = None;
    let mut has_style = false;
    let mut has_color = false;
    while !input.is_exhausted() {
        if width.is_none() {
            if let Ok(value) = input.try_parse(parse_line_width) {
                width = Some(value);
                continue;
            }
        }
        let token = input.next()?.clone();
        match &token {
            Token::Ident(ident) if LINE_STYLE_KEYWORDS.iter().any(|keyword| ident.eq_ignore_ascii_case(keyword)) => {
                if has_style {
                    return Err(input.new_unexpected_token_error(token));
                }
                has_style = true;
            }
            Token::Ident(ident)
                if !has_color
                    && (cssparser::color::parse_named_color(ident).is_ok()
                        || ident.eq_ignore_ascii_case("transparent")
                        || ident.eq_ignore_ascii_case("currentcolor")) =>
            {
                has_color = true;
            }
            Token::Hash(_) | Token::IDHash(_) if !has_color => has_color = true,
            Token::Function(_) if !has_color => {
                input.parse_nested_block(|input| {
                    while input.next().is_ok() {}
                    Ok::<_, CssParseError<'_>>(())
                })?;
                has_color = true;
            }
            _ => return Err(input.new_unexpected_token_error(token)),
        }
    }
    Ok(width.unwrap_or(LengthPercentage::length(0.0)))
}

/// Parses `auto`, or a ratio of `<number> [ / <number> ]`
fn parse_aspect_ratio<'i>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, Option<f32>> {
    if input.try_parse(|input| input.expect_ident_matching("auto")).is_ok() {
        return Ok(None);
    }
    let width = parse_positive(input)?;
    let height = if input.try_parse(|input| input.expect_delim('/')).is_ok() { parse_positive(input)? } else { 1.0 };
    Ok(Some(width / height))
}

/// Parses `auto` into `None`, or any other value as `Some`
fn parse_self_alignment<'i>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, Option<AlignItems>> {
    if input.try_parse(|input| input.expect_ident_matching("auto")).is_ok() {
        Ok(None)
    } else {
        AlignItems::from_css(input).map(Some)
    }
}

/// Parses a number that is zero or greater
#[cfg(feature = "flexbox")]
fn parse_non_negative<'i>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, f32> {
    let value = input.expect_number()?;
    if value < 0.0 {
        return Err(input.new_custom_error("expected a non-negative number"));
    }
    Ok(value)
}

/// Parses a number greater than zero
fn parse_positive<'i>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, f32> {
    let value = input.expect_number()?;
    if value <= 0.0 {
        return Err(input.new_custom_error("expected a positive number"));
    }
    Ok(value)
}

/// Parses a positive integer
#[cfg(feature = "flexbox_balance")]
fn parse_flex_line_count<'i>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, u16> {
    let value = input.expect_integer()?;
    if value < 1 {
        return Err(input.new_custom_error("expected a positive integer"));
    }
    Ok(value.min(u16::MAX as i32) as u16)
}

/// Parses the `flex-direction` and `flex-wrap` components of `flex-flow`, in either order
#[cfg(feature = "flexbox")]
fn parse_flex_flow<'i>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, (Option<FlexDirection>, Option<FlexWrap>)> {
    let mut direction = None;
    let mut wrap = None;
    while !input.is_exhausted() {
        if direction.is_none() {
            if let Ok(value) = input.try_parse(|input| parse_single_ident(input, FlexDirection::from_css)) {
                direction = Some(value);
                continue;
            }
        }
        if wrap.is_none() {
            if let Ok(value) = input.try_parse(|input| parse_single_ident(input, FlexWrap::from_css)) {
                wrap = Some(value);
                continue;
            }
        }
        return Err(input.new_custom_error("invalid flex-flow"));
    }
    if direction.is_none() && wrap.is_none() {
        return Err(input.new_custom_error("expected flex-direction or flex-wrap"));
    }
    Ok((direction, wrap))
}

/// Parses the `flex` shorthand into `(flex-grow, flex-shrink, flex-basis)`
#[cfg(feature = "flexbox")]
fn parse_flex<'i>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, (f32, f32, Dimension)> {
    if input.try_parse(|input| input.expect_ident_matching("none")).is_ok() {
        return Ok((0.0, 0.0, Dimension::auto()));
    }
    if input.try_parse(|input| input.expect_ident_matching("initial")).is_ok() {
        return Ok((0.0, 1.0, Dimension::auto()));
    }

    // The flex factors must be adjacent and may be followed or preceded by the flex-basis.
    // A unitless zero after two factors is the flex-basis (css-flexbox §7.1.1).
    let mut factors = [None, None];
    let mut factor_count = 0;
    let mut basis = None;
    let mut factors_closed = false;
    while !input.is_exhausted() {
        if let Ok(value) = input.try_parse(|input| input.expect_number()) {
            if value < 0.0 || factors_closed {
                return Err(input.new_custom_error("unexpected flex factor"));
            }
            if factor_count < 2 {
                factors[factor_count] = Some(value);
                factor_count += 1;
            } else if value == 0.0 && basis.is_none() {
                basis = Some(Dimension::length(0.0));
                factors_closed = true;
            } else {
                return Err(input.new_custom_error("unexpected flex factor"));
            }
        } else {
            if basis.is_some() {
                return Err(input.new_custom_error("unexpected flex-basis"));
            }
            if factor_count > 0 {
                factors_closed = true;
            }
            basis = Some(Dimension::from_css(input)?);
        }
    }

    if factor_count == 0 && basis.is_none() {
        return Err(input.new_custom_error("expected a flex value"));
    }
    let grow = factors[0].unwrap_or(1.0);
    let shrink = factors[1].unwrap_or(1.0);
    let basis = basis.unwrap_or(Dimension::percent(0.0));
    Ok((grow, shrink, basis))
}

/// Rows and columns of a `grid-template` declaration
#[cfg(feature = "grid")]
type GridTemplateRowsAndColumns<S> =
    (GridTemplateTracks<S, GridTemplateComponent<S>>, GridTemplateTracks<S, GridTemplateComponent<S>>);

/// Parses `<rows> / <columns>`
#[cfg(feature = "grid")]
fn parse_rows_and_columns<'i, S: CheapCloneStr>(
    input: &mut Parser<'i, '_>,
) -> CssParseResult<'i, GridTemplateRowsAndColumns<S>> {
    let (rows, has_slash) = parse_before_slash(input, GridTemplateTracks::from_css)?;
    if !has_slash {
        return Err(input.new_custom_error("expected `/` between grid rows and grid columns"));
    }
    let columns = GridTemplateTracks::from_css(input)?;
    Ok((rows, columns))
}

/// Parses `<start> [ / <end> ]` for `grid-row` or `grid-column`
#[cfg(feature = "grid")]
fn parse_grid_line<'i, S: CheapCloneStr>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, Line<GridPlacement<S>>> {
    let placements = parse_placements(input, 2)?;
    let start = placements[0].clone();
    let end = placements.get(1).cloned().unwrap_or_else(|| implicit_end(&start));
    Ok(Line { start, end })
}

/// Parses between one and `max` grid placements separated by `/`
#[cfg(feature = "grid")]
fn parse_placements<'i, S: CheapCloneStr>(
    input: &mut Parser<'i, '_>,
    max: usize,
) -> CssParseResult<'i, Vec<GridPlacement<S>>> {
    let mut placements = Vec::new();
    loop {
        let (placement, has_slash) = parse_before_slash(input, GridPlacement::from_css)?;
        placements.push(placement);
        if !has_slash {
            return Ok(placements);
        }
        if placements.len() == max {
            return Err(input.new_custom_error("too many grid line values"));
        }
    }
}

/// The value of a grid line that is omitted: a named line repeats the start, anything else is `auto`
#[cfg(feature = "grid")]
fn implicit_end<S: CheapCloneStr>(start: &GridPlacement<S>) -> GridPlacement<S> {
    match start {
        GridPlacement::NamedLine(_, 0) => start.clone(),
        _ => GridPlacement::Auto,
    }
}

/// Parses the next token, which must be an identifier, with `parse`. Keeps `parse` from consuming
/// the rest of the input, which some keyword parsers do.
#[cfg(feature = "flexbox")]
fn parse_single_ident<'i, T>(
    input: &mut Parser<'i, '_>,
    parse: impl FnOnce(&mut Parser<'i, '_>) -> CssParseResult<'i, T>,
) -> CssParseResult<'i, T> {
    let start = input.position();
    input.expect_ident()?;
    let mut sub_input = ParserInput::new(input.slice_from(start));
    let mut sub_parser = Parser::new(&mut sub_input);
    let value = parse(&mut sub_parser)?;
    sub_parser.expect_exhausted()?;
    Ok(value)
}

/// Parses the tokens up to the next top-level `/` (which is consumed) with `parse`.
/// Returns the parsed value and whether a `/` was found.
#[cfg(feature = "grid")]
fn parse_before_slash<'i, T>(
    input: &mut Parser<'i, '_>,
    parse: impl FnOnce(&mut Parser<'i, '_>) -> CssParseResult<'i, T>,
) -> CssParseResult<'i, (T, bool)> {
    let start = input.position();
    let (text, has_slash) = loop {
        let before = input.position();
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(_) => break (input.slice_from(start), false),
        };
        match token {
            Token::Delim('/') => break (input.slice(start..before), true),
            Token::Function(_) | Token::ParenthesisBlock | Token::SquareBracketBlock | Token::CurlyBracketBlock => {
                input.parse_nested_block(|input| {
                    while input.next().is_ok() {}
                    Ok::<_, CssParseError<'_>>(())
                })?;
            }
            _ => {}
        }
    };
    let mut sub_input = ParserInput::new(text);
    let mut sub_parser = Parser::new(&mut sub_input);
    let value = parse(&mut sub_parser)?;
    sub_parser.expect_exhausted()?;
    Ok((value, has_slash))
}
