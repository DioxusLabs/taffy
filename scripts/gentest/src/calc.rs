use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use winnow::ascii::{digit1, float, multispace0, multispace1};
use winnow::combinator::separated;
use winnow::combinator::{alt, delimited, opt, repeat};
use winnow::error::ParserError;
use winnow::stream::Stream;
use winnow::token::literal;
use winnow::{PResult, Parser};

fn parse_fn<'a, I, O, E, P>(input: &mut I, ident: &'a str, inner: P) -> PResult<O, E>
where
    P: Parser<I, O, E>,
    I: Stream + winnow::stream::Compare<&'a str> + winnow::stream::StreamIsPartial,
    E: ParserError<I>,
{
    (literal(ident), delimited("(", inner, ")")).parse_next(input).map(|(_, inner)| inner)
}

fn parse_min(input: &mut &str) -> PResult<TokenStream> {
    let out: Vec<TokenStream> = parse_fn(input, "min", separated(0.., parse_expr, ','))?;
    Ok(quote! {
        taffy::style::CalcNode::Min(Vec::from[#(Box::new(#out)),*])
    })
}
fn parse_max(input: &mut &str) -> PResult<TokenStream> {
    let out: Vec<TokenStream> = parse_fn(input, "max", separated(0.., parse_expr, ','))?;
    Ok(quote! {
        taffy::style::CalcNode::Max(Vec::from[#(Box::new(#out)),*])
    })
}
fn parse_clamp(input: &mut &str) -> PResult<TokenStream> {
    let out: Vec<TokenStream> = parse_fn(input, "clamp", separated(3, parse_expr, (',', multispace0)))?;
    let min = &out[0];
    let val = &out[1];
    let max = &out[2];
    Ok(quote! {
        taffy::style::CalcNode::Clamp { min: Box::new(#min), value: Box::new(#val), max: Box::new(#max) }
    })
}
fn parse_leaf(input: &mut &str) -> PResult<TokenStream> {
    fn parse_float(input: &mut &str) -> PResult<f32> {
        alt((float, digit1.map(|digits| f32::from(i16::from_str(digits).unwrap())))).parse_next(input)
    }

    (parse_float, opt(alt((literal("%").map(|_| true), literal("px").map(|_| false)))))
        .map(|(inner, is_percentage): (f32, Option<bool>)| {
            if is_percentage.is_some_and(|t| t) {
                let inner = inner / 100.0;
                quote! { taffy::style::CalcNode::Leaf(LengthPercentage::Percent(#inner)) }
            } else {
                quote! { taffy::style::CalcNode::Leaf(LengthPercentage::Length(#inner)) }
            }
        })
        .parse_next(input)
}
fn parse_negate(input: &mut &str) -> PResult<TokenStream> {
    let out = (literal('-'), parse_expr).parse_next(input)?.1;
    Ok(quote! {
        taffy::style::CalcNode::Negate(Box::new(#out))
    })
}
fn parse_parens(i: &mut &str) -> PResult<TokenStream> {
    delimited('(', parse_expr, ')').parse_next(i)
}

fn parse_atom(input: &mut &str) -> PResult<TokenStream> {
    alt((parse_parens, parse_clamp, parse_min, parse_max, parse_leaf)).parse_next(input)
}
fn parse_expr(input: &mut &str) -> PResult<TokenStream> {
    let lhs = parse_term.parse_next(input)?;

    repeat(0.., (alt((delimited(multispace1, '+', multispace1), delimited(multispace1, '-', multispace1))), parse_term))
        .fold(
            move || lhs.clone(),
            |acc, (op, tk): (char, TokenStream)| match op {
                '+' => quote! {
                    taffy::style::CalcNode::Sum(Box::new(#acc), Box::new(#tk))
                },
                '-' => quote! {
                    taffy::style::CalcNode::Difference(Box::new(#acc), Box::new(#tk))
                },
                _ => unreachable!(),
            },
        )
        .parse_next(input)
}
fn parse_term(input: &mut &str) -> PResult<TokenStream> {
    let lhs = parse_atom.parse_next(input)?;

    repeat(0.., (alt((delimited(multispace0, '*', multispace0), delimited(multispace0, '/', multispace0))), parse_atom))
        .fold(
            move || lhs.clone(),
            |acc, (op, tk): (char, TokenStream)| match op {
                '*' => quote! {
                    taffy::style::CalcNode::Product(Box::new(#acc), Box::new(#tk))
                },
                '/' => quote! {
                    taffy::style::CalcNode::Quotient(Box::new(#acc), Box::new(#tk))
                },
                _ => unreachable!(),
            },
        )
        .parse_next(input)
}

pub fn parse_calc_expression(input: &str) -> TokenStream {
    let mut inner = input.trim_start_matches("calc(").trim_end_matches(')');
    let outer_most = parse_expr(&mut inner).unwrap();
    let out = quote! {
        taffy::style::Calc::from(#outer_most)
    };
    out
}
