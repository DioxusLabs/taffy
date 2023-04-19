use core::fmt::Write;

use crate::geometry::Point;
use crate::geometry::Rect;
use crate::style::css_syntax::match_ignore_ascii_case;
use crate::style::css_syntax::CssValue;
use crate::style::css_syntax::ParseError;
use crate::style::css_syntax::ParseErrorKind;
use crate::style::Dimension;
use crate::style::Display;
use crate::style::LengthPercentage;
use crate::style::LengthPercentageAuto;
use crate::style::Overflow;
use crate::style::Position;
use crate::Taffy;
use cssparser::Parser;
use cssparser::Token;

impl CssValue for f32 {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>, _taffy: &Taffy) -> Result<Self, ParseError<'i>> {
        Ok(input.expect_number()?)
    }

    fn serialize(&self, dest: &mut String, _taffy: &Taffy) {
        // `unwrap` should never panic since `impl fmt::Write for String` never returns `Err`
        write!(dest, "{}", self).unwrap()
    }
}

pub(crate) enum MaybeAuto<T> {
    Auto,
    NotAuto(T),
}

impl<T: CssValue> CssValue for MaybeAuto<T> {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>, taffy: &Taffy) -> Result<Self, ParseError<'i>> {
        let state = input.state();
        if let Ok(()) = input.expect_ident_matching("auto") {
            return Ok(Self::Auto);
        }
        input.reset(&state);
        Ok(Self::NotAuto(T::parse(input, taffy)?))
    }

    fn serialize(&self, dest: &mut String, taffy: &Taffy) {
        match self {
            Self::Auto => dest.push_str("auto"),
            Self::NotAuto(x) => x.serialize(dest, taffy),
        }
    }
}

pub(crate) struct NonNegative<T>(pub(crate) T);

// Constrains type inference
pub(crate) fn non_negative<T>(value: NonNegative<T>) -> T {
    value.0
}

trait IsNonNegative {
    fn is_non_negative(&self) -> bool;
}

impl IsNonNegative for f32 {
    fn is_non_negative(&self) -> bool {
        *self >= 0.
    }
}

impl IsNonNegative for LengthPercentage {
    fn is_non_negative(&self) -> bool {
        match *self {
            Self::Length(x) => x >= 0.,
            Self::Percent(x) => x >= 0.,
        }
    }
}

impl IsNonNegative for LengthPercentageAuto {
    fn is_non_negative(&self) -> bool {
        match *self {
            Self::Length(x) => x >= 0.,
            Self::Percent(x) => x >= 0.,
            Self::Auto => true,
        }
    }
}

impl IsNonNegative for Dimension {
    fn is_non_negative(&self) -> bool {
        match *self {
            Self::Length(x) => x >= 0.,
            Self::Percent(x) => x >= 0.,
            Self::Auto => true,
        }
    }
}

impl<T: IsNonNegative> IsNonNegative for Rect<T> {
    fn is_non_negative(&self) -> bool {
        let Self { top, right, bottom, left } = self;
        top.is_non_negative() && right.is_non_negative() && bottom.is_non_negative() && left.is_non_negative()
    }
}

impl<T: CssValue + IsNonNegative> CssValue for NonNegative<T> {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>, taffy: &Taffy) -> Result<Self, ParseError<'i>> {
        let value = T::parse(input, taffy)?;
        if value.is_non_negative() {
            Ok(NonNegative(value))
        } else {
            Err(input.new_custom_error(ParseErrorKind::NegativeValue))
        }
    }

    fn serialize(&self, dest: &mut String, taffy: &Taffy) {
        self.0.serialize(dest, taffy)
    }
}

pub(crate) struct Ratio {
    numerator: f32,
    denominator: f32,
}

impl CssValue for Ratio {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>, taffy: &Taffy) -> Result<Self, ParseError<'i>> {
        let numerator = non_negative(CssValue::parse(input, taffy)?);
        let denominator = if input.is_exhausted() { 1.0 } else { non_negative(CssValue::parse(input, taffy)?) };
        Ok(Self { numerator, denominator })
    }

    fn serialize(&self, dest: &mut String, taffy: &Taffy) {
        self.numerator.serialize(dest, taffy);
        if self.denominator != 1.0 {
            dest.push_str(" / ");
            self.denominator.serialize(dest, taffy)
        }
    }
}

impl MaybeAuto<Ratio> {
    pub(crate) fn to_opt_f32(self) -> Option<f32> {
        match self {
            MaybeAuto::Auto => None,
            MaybeAuto::NotAuto(ratio) => Some(ratio.numerator / ratio.denominator),
        }
    }
}

/// Assumes `<x> <y>?` syntax where a single value sets both components
impl<T: CssValue + PartialEq + Clone> CssValue for Point<T> {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>, taffy: &Taffy) -> Result<Self, ParseError<'i>> {
        let x = T::parse(input, taffy)?;
        let y = if input.is_exhausted() { x.clone() } else { T::parse(input, taffy)? };
        Ok(Point { x, y })
    }

    fn serialize(&self, dest: &mut String, taffy: &Taffy) {
        self.x.serialize(dest, taffy);
        if self.x != self.y {
            dest.push(' ');
            self.y.serialize(dest, taffy);
        }
    }
}

/// Assumes `<top> (<right> (<bottom> <left>?)?)?` syntax like `margin` etc.
///
/// <https://w3c.github.io/csswg-drafts/css-box-4/#propdef-margin>
impl<T: CssValue + PartialEq + Clone> CssValue for Rect<T> {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>, taffy: &Taffy) -> Result<Self, ParseError<'i>> {
        let top = T::parse(input, taffy)?;
        let right = if input.is_exhausted() { top.clone() } else { T::parse(input, taffy)? };
        let bottom = if input.is_exhausted() { top.clone() } else { T::parse(input, taffy)? };
        let left = if input.is_exhausted() { right.clone() } else { T::parse(input, taffy)? };
        Ok(Rect { top, right, bottom, left })
    }

    fn serialize(&self, dest: &mut String, taffy: &Taffy) {
        let right_is_needed = self.right != self.top;
        let bottom_is_needed = self.bottom != self.top;
        let left_is_needed = self.left != self.right;
        self.top.serialize(dest, taffy);
        if right_is_needed || bottom_is_needed || left_is_needed {
            dest.push(' ');
            self.right.serialize(dest, taffy);
            if bottom_is_needed || left_is_needed {
                dest.push(' ');
                self.bottom.serialize(dest, taffy);
                if left_is_needed {
                    dest.push(' ');
                    self.left.serialize(dest, taffy);
                }
            }
        }
    }
}

impl CssValue for LengthPercentage {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>, taffy: &Taffy) -> Result<Self, ParseError<'i>> {
        let token = input.next()?.clone();
        match &token {
            Token::Dimension { value, unit, .. } => {
                // We do not support relative length units are not supported
                // Absolute units all have a fix ratio to each other:
                // https://drafts.csswg.org/css-values/#absolute-lengths
                let units_per_inch = match_ignore_ascii_case! { &*unit,
                    "px" => 96.,
                    "pt" => 72., // point
                    "pc" => 6., // pica
                    "in" => 1.,
                    "cm" => 2.54,
                    "mm" => 25.4,
                    "q" =>  25.4 * 4., // quarter millimeter
                    _ => return Err(input.new_unexpected_token_error(token))
                };
                let css_inches = value / units_per_inch;
                let css_pxs = css_inches * 96.;
                let taffy_units = css_pxs * taffy.config.pixel_ratio;
                Ok(Self::Length(taffy_units))
            }
            Token::Percentage { unit_value, .. } => Ok(LengthPercentage::Percent(*unit_value)),
            _ => Err(input.new_unexpected_token_error(token)),
        }
    }

    fn serialize(&self, dest: &mut String, taffy: &Taffy) {
        match *self {
            LengthPercentage::Length(taffy_units) => {
                let css_pxs: f32 = taffy_units / taffy.config.pixel_ratio;
                css_pxs.serialize(dest, taffy);
                dest.push_str("px")
            }
            LengthPercentage::Percent(percent) => {
                percent.serialize(dest, taffy);
                dest.push('%')
            }
        }
    }
}

impl CssValue for LengthPercentageAuto {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>, taffy: &Taffy) -> Result<Self, ParseError<'i>> {
        Ok(match CssValue::parse(input, taffy)? {
            MaybeAuto::Auto => Self::Auto,
            MaybeAuto::NotAuto(LengthPercentage::Length(x)) => Self::Length(x),
            MaybeAuto::NotAuto(LengthPercentage::Percent(x)) => Self::Percent(x),
        })
    }

    fn serialize(&self, dest: &mut String, taffy: &Taffy) {
        match *self {
            Self::Length(x) => MaybeAuto::NotAuto(LengthPercentage::Length(x)),
            Self::Percent(x) => MaybeAuto::NotAuto(LengthPercentage::Percent(x)),
            Self::Auto => MaybeAuto::Auto,
        }
        .serialize(dest, taffy)
    }
}

impl CssValue for Dimension {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>, taffy: &Taffy) -> Result<Self, ParseError<'i>> {
        Ok(match CssValue::parse(input, taffy)? {
            MaybeAuto::Auto => Self::Auto,
            MaybeAuto::NotAuto(LengthPercentage::Length(x)) => Self::Length(x),
            MaybeAuto::NotAuto(LengthPercentage::Percent(x)) => Self::Percent(x),
        })
    }

    fn serialize(&self, dest: &mut String, taffy: &Taffy) {
        match *self {
            Self::Length(x) => MaybeAuto::NotAuto(LengthPercentage::Length(x)),
            Self::Percent(x) => MaybeAuto::NotAuto(LengthPercentage::Percent(x)),
            Self::Auto => MaybeAuto::Auto,
        }
        .serialize(dest, taffy)
    }
}

macro_rules! keywords_only {
    ( $(
        $( #[$meta: meta] )*
        $keyword: literal => $enum_variant: ident,
    )+ ) => {
        fn parse<'i, 't>(input: &mut Parser<'i, 't>, _taffy: &Taffy) -> Result<Self, ParseError<'i>> {
            let ident = input.expect_ident()?;
            match_ignore_ascii_case! { &*ident,
                $(
                    $( #[$meta] )*
                    $keyword => return Ok(Self::$enum_variant),
                )+
                _ => {
                    let error = ParseErrorKind::InvalidOrUnknownKeyword(ident.clone());
                    Err(input.new_custom_error(error))
                }
            }
        }

        fn serialize(&self, dest: &mut String, _taffy: &Taffy) {
            dest.push_str(match self {
                $(
                    $( #[$meta] )*
                    Self::$enum_variant => $keyword,
                )+
            })
        }
    };
}

impl CssValue for Display {
    keywords_only! {
        #[cfg(feature = "flexbox")]
        "flex" => Flex,
        #[cfg(feature = "grid")]
        "grid" => Grid,
        "none" => None,
    }
}

impl CssValue for Overflow {
    keywords_only! {
        "visible" => Visible,
        "hidden" => Hidden,
    }
}

impl CssValue for Position {
    keywords_only! {
        "relative" => Relative,
        "absolute" => Absolute,
    }
}
