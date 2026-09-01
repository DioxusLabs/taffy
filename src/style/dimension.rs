//! Style types for representing lengths / sizes
use super::CompactLength;
use crate::geometry::Rect;
use crate::style_helpers::{FromLength, FromPercent, TaffyAuto, TaffyZero};
#[cfg(feature = "parse")]
use crate::util::parse::{from_str_from_css, CssParseResult, FromCss, Parser, Token};
use crate::util::OptF32;

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`](crate::geometry::Size).
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LengthPercentage(pub(crate) CompactLength);
impl TaffyZero for LengthPercentage {
    const ZERO: Self = Self(CompactLength::ZERO);
}
impl FromLength for LengthPercentage {
    fn from_length<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::length(value.into() as f32)
    }
}
impl FromPercent for LengthPercentage {
    fn from_percent<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::percent(value.into() as f32)
    }
}

#[cfg(feature = "parse")]
impl FromCss for LengthPercentage {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        match parser.next()?.clone() {
            Token::Percentage { unit_value, .. } => Ok(Self::percent(unit_value)),
            Token::Dimension { unit, value, .. } if unit == "px" => Ok(Self::length(value)),
            token => Err(parser.new_unexpected_token_error(token))?,
        }
    }
}
#[cfg(feature = "parse")]
from_str_from_css!(LengthPercentage);

impl LengthPercentage {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    #[inline(always)]
    pub const fn length(val: f32) -> Self {
        Self(CompactLength::length(val))
    }

    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    #[inline(always)]
    pub const fn percent(val: f32) -> Self {
        Self(CompactLength::percent(val))
    }

    /// A `calc()` value. The value passed here is treated as an opaque handle to
    /// the actual calc representation and may be a pointer, index, etc.
    ///
    /// The low 3 bits are used as a tag value and will be returned as 0.
    #[inline(always)]
    #[cfg(feature = "calc")]
    pub fn calc(ptr: *const ()) -> Self {
        Self(CompactLength::calc(ptr))
    }

    /// Create a LengthPercentage from a raw `CompactLength`.
    /// # Safety
    /// CompactLength must represent a valid variant for LengthPercentage
    #[allow(unsafe_code)]
    pub const unsafe fn from_raw(val: CompactLength) -> Self {
        Self(val)
    }

    /// Get the underlying `CompactLength` representation of the value
    pub const fn into_raw(self) -> CompactLength {
        self.0
    }

    /// Expand the compact representation into an [`ExpandedLengthPercentage`] enum.
    ///
    /// This is useful when integrating with other libraries (e.g. for style inspection or
    /// serialization) as it allows the value to be pattern-matched without having to work
    /// with the raw [`CompactLength`] tagged-pointer representation directly.
    pub fn expand(self) -> ExpandedLengthPercentage {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => ExpandedLengthPercentage::Length(self.0.value()),
            CompactLength::PERCENT_TAG => ExpandedLengthPercentage::Percent(self.0.value()),
            #[cfg(feature = "calc")]
            _ if self.0.is_calc() => ExpandedLengthPercentage::Calc(self.0.calc_value()),
            _ => unreachable!("LengthPercentage contains a value with an invalid tag"),
        }
    }
}

/// The expanded, non-compact representation of a [`LengthPercentage`].
///
/// Obtained via [`LengthPercentage::expand`]. Can be converted back into a [`LengthPercentage`]
/// using the [`From`] implementation.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ExpandedLengthPercentage {
    /// An absolute length (see [`LengthPercentage::length`])
    Length(f32),
    /// A percentage length (see [`LengthPercentage::percent`])
    Percent(f32),
    /// A `calc()` value (see [`LengthPercentage::calc`]). The pointer is an opaque handle to the
    /// calc representation, exactly as passed to the constructor.
    #[cfg(feature = "calc")]
    Calc(*const ()),
}

impl From<LengthPercentage> for ExpandedLengthPercentage {
    fn from(value: LengthPercentage) -> Self {
        value.expand()
    }
}

impl From<ExpandedLengthPercentage> for LengthPercentage {
    fn from(value: ExpandedLengthPercentage) -> Self {
        match value {
            ExpandedLengthPercentage::Length(val) => Self::length(val),
            ExpandedLengthPercentage::Percent(val) => Self::percent(val),
            #[cfg(feature = "calc")]
            ExpandedLengthPercentage::Calc(ptr) => Self::calc(ptr),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LengthPercentage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = CompactLength::deserialize(deserializer)?;
        // Note: validation intentionally excludes the CALC_TAG as deserializing calc() values is not supported
        if matches!(inner.tag(), CompactLength::LENGTH_TAG | CompactLength::PERCENT_TAG) {
            Ok(Self(inner))
        } else {
            Err(serde::de::Error::custom("Invalid tag"))
        }
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`](crate::geometry::Size).
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LengthPercentageAuto(pub(crate) CompactLength);
impl TaffyZero for LengthPercentageAuto {
    const ZERO: Self = Self(CompactLength::ZERO);
}
impl TaffyAuto for LengthPercentageAuto {
    const AUTO: Self = Self(CompactLength::AUTO);
}
impl FromLength for LengthPercentageAuto {
    fn from_length<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::length(value.into() as f32)
    }
}
impl FromPercent for LengthPercentageAuto {
    fn from_percent<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::percent(value.into() as f32)
    }
}
impl From<LengthPercentage> for LengthPercentageAuto {
    fn from(input: LengthPercentage) -> Self {
        Self(input.0)
    }
}

#[cfg(feature = "parse")]
impl FromCss for LengthPercentageAuto {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        match parser.next()?.clone() {
            Token::Percentage { unit_value, .. } => Ok(Self::percent(unit_value)),
            Token::Dimension { unit, value, .. } if unit == "px" => Ok(Self::length(value)),
            Token::Ident(ident) if ident == "auto" => Ok(Self::auto()),
            token => Err(parser.new_unexpected_token_error(token))?,
        }
    }
}
#[cfg(feature = "parse")]
from_str_from_css!(LengthPercentageAuto);

impl LengthPercentageAuto {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    #[inline(always)]
    pub const fn length(val: f32) -> Self {
        Self(CompactLength::length(val))
    }

    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    #[inline(always)]
    pub const fn percent(val: f32) -> Self {
        Self(CompactLength::percent(val))
    }

    /// The dimension should be automatically computed according to algorithm-specific rules
    /// regarding the default size of boxes.
    #[inline(always)]
    pub const fn auto() -> Self {
        Self(CompactLength::auto())
    }

    /// A `calc()` value. The value passed here is treated as an opaque handle to
    /// the actual calc representation and may be a pointer, index, etc.
    ///
    /// The low 3 bits are used as a tag value and will be returned as 0.
    #[inline]
    #[cfg(feature = "calc")]
    pub fn calc(ptr: *const ()) -> Self {
        Self(CompactLength::calc(ptr))
    }

    /// Create a LengthPercentageAuto from a raw `CompactLength`.
    /// # Safety
    /// CompactLength must represent a valid variant for LengthPercentageAuto
    #[allow(unsafe_code)]
    pub const unsafe fn from_raw(val: CompactLength) -> Self {
        Self(val)
    }

    /// Get the underlying `CompactLength` representation of the value
    pub const fn into_raw(self) -> CompactLength {
        self.0
    }

    /// Returns:
    ///   - Some(length) for Length variants
    ///   - Some(resolved) using the provided context for Percent variants
    ///   - None for Auto variants
    #[inline(always)]
    pub fn resolve_to_option(self, context: f32, calc_resolver: impl Fn(*const (), f32) -> f32) -> OptF32 {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => OptF32::some(self.0.value()),
            CompactLength::PERCENT_TAG => OptF32::some(context * self.0.value()),
            CompactLength::AUTO_TAG => OptF32::NONE,
            #[cfg(feature = "calc")]
            _ if self.0.is_calc() => OptF32::some(calc_resolver(self.0.calc_value(), context)),
            _ => unreachable!("LengthPercentageAuto values cannot be constructed with other tags"),
        }
    }

    /// Returns true if value is LengthPercentageAuto::Auto
    #[inline(always)]
    pub fn is_auto(self) -> bool {
        self.0.is_auto()
    }

    /// Expand the compact representation into an [`ExpandedLengthPercentageAuto`] enum.
    ///
    /// This is useful when integrating with other libraries (e.g. for style inspection or
    /// serialization) as it allows the value to be pattern-matched without having to work
    /// with the raw [`CompactLength`] tagged-pointer representation directly.
    pub fn expand(self) -> ExpandedLengthPercentageAuto {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => ExpandedLengthPercentageAuto::Length(self.0.value()),
            CompactLength::PERCENT_TAG => ExpandedLengthPercentageAuto::Percent(self.0.value()),
            CompactLength::AUTO_TAG => ExpandedLengthPercentageAuto::Auto,
            #[cfg(feature = "calc")]
            _ if self.0.is_calc() => ExpandedLengthPercentageAuto::Calc(self.0.calc_value()),
            _ => unreachable!("LengthPercentageAuto contains a value with an invalid tag"),
        }
    }
}

/// The expanded, non-compact representation of a [`LengthPercentageAuto`].
///
/// Obtained via [`LengthPercentageAuto::expand`]. Can be converted back into a
/// [`LengthPercentageAuto`] using the [`From`] implementation.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ExpandedLengthPercentageAuto {
    /// An absolute length (see [`LengthPercentageAuto::length`])
    Length(f32),
    /// A percentage length (see [`LengthPercentageAuto::percent`])
    Percent(f32),
    /// The automatic keyword (see [`LengthPercentageAuto::auto`])
    Auto,
    /// A `calc()` value (see [`LengthPercentageAuto::calc`]). The pointer is an opaque handle to
    /// the calc representation, exactly as passed to the constructor.
    #[cfg(feature = "calc")]
    Calc(*const ()),
}

impl From<LengthPercentageAuto> for ExpandedLengthPercentageAuto {
    fn from(value: LengthPercentageAuto) -> Self {
        value.expand()
    }
}

impl From<ExpandedLengthPercentageAuto> for LengthPercentageAuto {
    fn from(value: ExpandedLengthPercentageAuto) -> Self {
        match value {
            ExpandedLengthPercentageAuto::Length(val) => Self::length(val),
            ExpandedLengthPercentageAuto::Percent(val) => Self::percent(val),
            ExpandedLengthPercentageAuto::Auto => Self::auto(),
            #[cfg(feature = "calc")]
            ExpandedLengthPercentageAuto::Calc(ptr) => Self::calc(ptr),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LengthPercentageAuto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = CompactLength::deserialize(deserializer)?;
        // Note: validation intentionally excludes the CALC_TAG as deserializing calc() values is not supported
        if matches!(inner.tag(), CompactLength::LENGTH_TAG | CompactLength::PERCENT_TAG | CompactLength::AUTO_TAG) {
            Ok(Self(inner))
        } else {
            Err(serde::de::Error::custom("Invalid tag"))
        }
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`](crate::geometry::Size).
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Dimension(pub(crate) CompactLength);
impl TaffyZero for Dimension {
    const ZERO: Self = Self(CompactLength::ZERO);
}
impl TaffyAuto for Dimension {
    const AUTO: Self = Self(CompactLength::AUTO);
}
impl FromLength for Dimension {
    fn from_length<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::length(value.into() as f32)
    }
}
impl FromPercent for Dimension {
    fn from_percent<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::percent(value.into() as f32)
    }
}
impl From<LengthPercentage> for Dimension {
    fn from(input: LengthPercentage) -> Self {
        Self(input.0)
    }
}
impl From<LengthPercentageAuto> for Dimension {
    fn from(input: LengthPercentageAuto) -> Self {
        Self(input.0)
    }
}

#[cfg(feature = "parse")]
impl FromCss for Dimension {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        let token = parser.next()?.clone();
        match token {
            Token::Percentage { unit_value, .. } => Ok(Self::percent(unit_value)),
            Token::Dimension { unit, value, .. } if unit == "px" => Ok(Self::length(value)),
            Token::Ident(ref ident) => match ident.as_ref() {
                "auto" => Ok(Self::auto()),
                "min-content" => Ok(Self::min_content()),
                "max-content" => Ok(Self::max_content()),
                "fit-content" => Ok(Self::fit_content()),
                "stretch" => Ok(Self::stretch()),
                "content" => Ok(Self::content()),
                _ => Err(parser.new_unexpected_token_error(token))?,
            },
            Token::Function(ref name) if name.as_ref() == "fit-content" => parser.parse_nested_block(|parser| {
                let token = parser.next()?.clone();
                match token {
                    Token::Percentage { unit_value, .. } => Ok(Self::fit_content_percent(unit_value)),
                    Token::Dimension { unit, value, .. } if unit == "px" => Ok(Self::fit_content_px(value)),
                    token => Err(parser.new_unexpected_token_error(token))?,
                }
            }),
            token => Err(parser.new_unexpected_token_error(token))?,
        }
    }
}
#[cfg(feature = "parse")]
from_str_from_css!(Dimension);

impl Dimension {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    #[inline(always)]
    pub const fn length(val: f32) -> Self {
        Self(CompactLength::length(val))
    }

    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    #[inline(always)]
    pub const fn percent(val: f32) -> Self {
        Self(CompactLength::percent(val))
    }

    /// The dimension should be automatically computed according to algorithm-specific rules
    /// regarding the default size of boxes.
    #[inline(always)]
    pub const fn auto() -> Self {
        Self(CompactLength::auto())
    }

    /// The size should be the "min-content" size.
    /// This is the smallest size that can fit the item's contents with ALL soft line-wrapping opportunities taken
    #[inline(always)]
    pub const fn min_content() -> Self {
        Self(CompactLength::min_content())
    }

    /// The size should be the "max-content" size.
    /// This is the smallest size that can fit the item's contents with NO soft line-wrapping opportunities taken
    #[inline(always)]
    pub const fn max_content() -> Self {
        Self(CompactLength::max_content())
    }

    /// The size should be computed according to the "fit content" formula:
    ///    `max(min_content, min(max_content, stretch))`
    /// where `stretch` is the size the box would take if it filled the available space
    #[inline(always)]
    pub const fn fit_content() -> Self {
        Self(CompactLength::fit_content_keyword())
    }

    /// The size should be computed according to the "fit content" formula:
    ///    `max(min_content, min(max_content, limit))`
    /// where `limit` is a LENGTH value
    #[inline(always)]
    pub const fn fit_content_px(limit: f32) -> Self {
        Self(CompactLength::fit_content_px(limit))
    }

    /// The size should be the "stretch-fit" size: the size the box would take
    /// if it filled the available space
    /// (<https://www.w3.org/TR/css-sizing-4/#stretch-fit-sizing>)
    #[inline(always)]
    pub const fn stretch() -> Self {
        Self(CompactLength::stretch())
    }

    /// The size should be an automatic size based on the box's content
    /// (<https://www.w3.org/TR/css-flexbox-1/#valdef-flex-basis-content>)
    ///
    /// This keyword is only valid for `flex-basis`. In any other context it behaves as [`auto`](Self::auto).
    #[inline(always)]
    pub const fn content() -> Self {
        Self(CompactLength::content())
    }

    /// The size should be computed according to the "fit content" formula:
    ///    `max(min_content, min(max_content, limit))`
    /// where `limit` is a PERCENTAGE value
    #[inline(always)]
    pub const fn fit_content_percent(limit: f32) -> Self {
        Self(CompactLength::fit_content_percent(limit))
    }

    /// A `calc()` value. The value passed here is treated as an opaque handle to
    /// the actual calc representation and may be a pointer, index, etc.
    ///
    /// The low 3 bits are used as a tag value and will be returned as 0.
    #[inline]
    #[cfg(feature = "calc")]
    pub fn calc(ptr: *const ()) -> Self {
        Self(CompactLength::calc(ptr))
    }

    /// Create a LengthPercentageAuto from a raw `CompactLength`.
    /// # Safety
    /// CompactLength must represent a valid variant for LengthPercentageAuto
    #[allow(unsafe_code)]
    pub const unsafe fn from_raw(val: CompactLength) -> Self {
        Self(val)
    }

    /// Get the underlying `CompactLength` representation of the value
    pub const fn into_raw(self) -> CompactLength {
        self.0
    }

    /// Get Length value if value is Length variant
    #[cfg(feature = "grid")]
    pub fn into_option(self) -> OptF32 {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => OptF32::some(self.0.value()),
            _ => OptF32::NONE,
        }
    }
    /// Returns true if value is Auto
    #[inline(always)]
    pub fn is_auto(self) -> bool {
        self.0.is_auto()
    }

    /// Returns true if value is min-content, max-content, fit-content, fit-content(...), or stretch
    #[inline(always)]
    pub fn is_sizing_keyword(self) -> bool {
        self.0.is_sizing_keyword()
    }

    /// Returns true if value is the stretch keyword
    #[inline(always)]
    pub fn is_stretch(self) -> bool {
        self.0.tag() == CompactLength::STRETCH_TAG
    }

    /// Returns true if value is the content keyword
    #[inline(always)]
    pub fn is_content(self) -> bool {
        self.0.is_content()
    }

    /// Get the raw `CompactLength` tag
    pub fn tag(self) -> usize {
        self.0.tag()
    }

    /// Get the raw `CompactLength` value for non-calc variants that have a numeric parameter
    pub fn value(self) -> f32 {
        self.0.value()
    }

    /// Expand the compact representation into an [`ExpandedDimension`] enum.
    ///
    /// This is useful when integrating with other libraries (e.g. for style inspection or
    /// serialization) as it allows the value to be pattern-matched without having to work
    /// with the raw [`CompactLength`] tagged-pointer representation directly.
    pub fn expand(self) -> ExpandedDimension {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => ExpandedDimension::Length(self.0.value()),
            CompactLength::PERCENT_TAG => ExpandedDimension::Percent(self.0.value()),
            CompactLength::AUTO_TAG => ExpandedDimension::Auto,
            CompactLength::MIN_CONTENT_TAG => ExpandedDimension::MinContent,
            CompactLength::MAX_CONTENT_TAG => ExpandedDimension::MaxContent,
            CompactLength::FIT_CONTENT_PX_TAG => ExpandedDimension::FitContentPx(self.0.value()),
            CompactLength::FIT_CONTENT_PERCENT_TAG => ExpandedDimension::FitContentPercent(self.0.value()),
            CompactLength::FIT_CONTENT_KEYWORD_TAG => ExpandedDimension::FitContent,
            CompactLength::STRETCH_TAG => ExpandedDimension::Stretch,
            CompactLength::CONTENT_TAG => ExpandedDimension::Content,
            #[cfg(feature = "calc")]
            _ if self.0.is_calc() => ExpandedDimension::Calc(self.0.calc_value()),
            _ => unreachable!("Dimension contains a value with an invalid tag"),
        }
    }
}

/// The expanded, non-compact representation of a [`Dimension`].
///
/// Obtained via [`Dimension::expand`]. Can be converted back into a [`Dimension`] using the
/// [`From`] implementation.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ExpandedDimension {
    /// An absolute length (see [`Dimension::length`])
    Length(f32),
    /// A percentage length (see [`Dimension::percent`])
    Percent(f32),
    /// The automatic keyword (see [`Dimension::auto`])
    Auto,
    /// The `min-content` keyword (see [`Dimension::min_content`])
    MinContent,
    /// The `max-content` keyword (see [`Dimension::max_content`])
    MaxContent,
    /// A `fit-content(...)` value with a length limit (see [`Dimension::fit_content_px`])
    FitContentPx(f32),
    /// A `fit-content(...)` value with a percentage limit (see [`Dimension::fit_content_percent`])
    FitContentPercent(f32),
    /// The `fit-content` keyword with no limit (see [`Dimension::fit_content`])
    FitContent,
    /// The `stretch` keyword (see [`Dimension::stretch`])
    Stretch,
    /// The `content` keyword (see [`Dimension::content`])
    Content,
    /// A `calc()` value (see [`Dimension::calc`]). The pointer is an opaque handle to the calc
    /// representation, exactly as passed to the constructor.
    #[cfg(feature = "calc")]
    Calc(*const ()),
}

impl From<Dimension> for ExpandedDimension {
    fn from(value: Dimension) -> Self {
        value.expand()
    }
}

impl From<ExpandedDimension> for Dimension {
    fn from(value: ExpandedDimension) -> Self {
        match value {
            ExpandedDimension::Length(val) => Self::length(val),
            ExpandedDimension::Percent(val) => Self::percent(val),
            ExpandedDimension::Auto => Self::auto(),
            ExpandedDimension::MinContent => Self::min_content(),
            ExpandedDimension::MaxContent => Self::max_content(),
            ExpandedDimension::FitContentPx(val) => Self::fit_content_px(val),
            ExpandedDimension::FitContentPercent(val) => Self::fit_content_percent(val),
            ExpandedDimension::FitContent => Self::fit_content(),
            ExpandedDimension::Stretch => Self::stretch(),
            ExpandedDimension::Content => Self::content(),
            #[cfg(feature = "calc")]
            ExpandedDimension::Calc(ptr) => Self::calc(ptr),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Dimension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = CompactLength::deserialize(deserializer)?;
        // Note: validation intentionally excludes the CALC_TAG as deserializing calc() values is not supported
        if matches!(
            inner.tag(),
            CompactLength::LENGTH_TAG
                | CompactLength::PERCENT_TAG
                | CompactLength::AUTO_TAG
                | CompactLength::MIN_CONTENT_TAG
                | CompactLength::MAX_CONTENT_TAG
                | CompactLength::FIT_CONTENT_KEYWORD_TAG
                | CompactLength::FIT_CONTENT_PX_TAG
                | CompactLength::FIT_CONTENT_PERCENT_TAG
                | CompactLength::STRETCH_TAG
                | CompactLength::CONTENT_TAG
        ) {
            Ok(Self(inner))
        } else {
            Err(serde::de::Error::custom("Invalid tag"))
        }
    }
}

impl Rect<Dimension> {
    /// Create a new Rect with length values
    #[must_use]
    pub const fn from_length(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension(CompactLength::length(start)),
            right: Dimension(CompactLength::length(end)),
            top: Dimension(CompactLength::length(top)),
            bottom: Dimension(CompactLength::length(bottom)),
        }
    }

    /// Create a new Rect with percentage values
    #[must_use]
    pub const fn from_percent(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension(CompactLength::percent(start)),
            right: Dimension(CompactLength::percent(end)),
            top: Dimension(CompactLength::percent(top)),
            bottom: Dimension(CompactLength::percent(bottom)),
        }
    }
}

#[cfg(test)]
mod expand_tests {
    use super::*;

    /// A helper that produces a valid `calc()` handle for tests: a non-null pointer whose low
    /// three bits are zero (as required by `CompactLength::calc`).
    #[cfg(feature = "calc")]
    fn calc_handle() -> *const () {
        #[allow(dead_code)]
        #[repr(align(8))]
        struct Aligned(u64);
        static HANDLE: Aligned = Aligned(0);
        &HANDLE as *const Aligned as *const ()
    }

    #[test]
    fn length_percentage_round_trips() {
        let cases = [LengthPercentage::length(12.0), LengthPercentage::percent(0.5), LengthPercentage::ZERO];
        for value in cases {
            assert_eq!(LengthPercentage::from(value.expand()), value);
            assert_eq!(ExpandedLengthPercentage::from(value), value.expand());
        }
        assert_eq!(LengthPercentage::length(3.0).expand(), ExpandedLengthPercentage::Length(3.0));
        assert_eq!(LengthPercentage::percent(0.25).expand(), ExpandedLengthPercentage::Percent(0.25));
    }

    #[test]
    fn length_percentage_auto_round_trips() {
        let cases =
            [LengthPercentageAuto::length(12.0), LengthPercentageAuto::percent(0.5), LengthPercentageAuto::auto()];
        for value in cases {
            assert_eq!(LengthPercentageAuto::from(value.expand()), value);
        }
        assert_eq!(LengthPercentageAuto::auto().expand(), ExpandedLengthPercentageAuto::Auto);
    }

    #[test]
    fn dimension_round_trips_all_keywords() {
        let cases = [
            Dimension::length(12.0),
            Dimension::percent(0.5),
            Dimension::auto(),
            Dimension::min_content(),
            Dimension::max_content(),
            Dimension::fit_content_px(30.0),
            Dimension::fit_content_percent(0.75),
            Dimension::fit_content(),
            Dimension::stretch(),
            Dimension::content(),
        ];
        for value in cases {
            assert_eq!(Dimension::from(value.expand()), value);
        }
        assert_eq!(Dimension::fit_content_px(30.0).expand(), ExpandedDimension::FitContentPx(30.0));
        assert_eq!(Dimension::content().expand(), ExpandedDimension::Content);
    }

    #[cfg(feature = "calc")]
    #[test]
    fn calc_round_trips() {
        let handle = calc_handle();
        assert_eq!(LengthPercentage::calc(handle).expand(), ExpandedLengthPercentage::Calc(handle));
        assert_eq!(LengthPercentage::from(ExpandedLengthPercentage::Calc(handle)), LengthPercentage::calc(handle));
        assert_eq!(Dimension::calc(handle).expand(), ExpandedDimension::Calc(handle));
        assert_eq!(LengthPercentageAuto::calc(handle).expand(), ExpandedLengthPercentageAuto::Calc(handle));
    }
}
