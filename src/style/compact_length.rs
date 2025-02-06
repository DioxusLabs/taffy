//! A tagged-pointer abstraction that allows size styles in Taffy to be represented
//! in just 64 bits. Wrapped by types in the `super::dimension` and `super::grid` modules.
use super::LengthPercentage;
use crate::style_helpers::{
    FromFr, FromLength, FromPercent, TaffyAuto, TaffyFitContent, TaffyMaxContent, TaffyMinContent, TaffyZero,
};
use compat::{f32_from_bits, f32_to_bits};

/// A representation of a length as a compact 64-bit tagged pointer
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompactLength(u64);

impl CompactLength {
    // Masks

    /// The low byte (8 bits)
    pub const TAG_MASK: u64 = 0b11111111;
    /// The low 3 bits
    pub const CALC_TAG_MASK: u64 = 0b111;
    /// The high 63 bits
    pub const CALC_PTR_MASK: u64 = u64::MAX ^ 0b111;

    // Primary tags

    /// The tag indicating a calc() value
    pub const CALC_TAG: u64 = 0b000;
    /// The tag indicating a length value
    pub const LENGTH_TAG: u64 = 0b0000_0001;
    /// The tag indicating a percentage value
    pub const PERCENT_TAG: u64 = 0b0000_0010;
    /// The tag indicating an auto value
    pub const AUTO_TAG: u64 = 0b0000_0011;
    /// The tag indicating an fr value
    pub const FR_TAG: u64 = 0b0000_0100;
    /// The tag indicating a min-content value
    pub const MIN_CONTENT_TAG: u64 = 0b00000111;
    /// The tag indicating a max-content value
    pub const MAX_CONTENT_TAG: u64 = 0b00001111;
    /// The tag indicating a fit-content value with px limit
    pub const FIT_CONTENT_PX_TAG: u64 = 0b00010111;
    /// The tag indicating a fit-content value with percent limit
    pub const FIT_CONTENT_PERCENT_TAG: u64 = 0b00011111;
}

impl CompactLength {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    #[inline(always)]
    pub const fn length(val: f32) -> Self {
        Self(((f32_to_bits(val) as u64) << 32) | Self::LENGTH_TAG)
    }

    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    #[inline(always)]
    pub const fn percent(val: f32) -> Self {
        Self(((f32_to_bits(val) as u64) << 32) | Self::PERCENT_TAG)
    }

    /// A `calc()` value. The value passed here is treated as an opaque handle to
    /// the actual calc representation and may be a pointer, index, etc.
    ///
    /// The low 3 bits are used as a tag value and will be returned as 0.
    #[inline]
    pub fn calc(ptr: *const ()) -> Self {
        assert_ne!(ptr as u64, 0);
        assert_eq!(ptr as u64 & 0b111, 0);
        Self(ptr as u64 | Self::CALC_TAG)
    }

    /// The dimension should be automatically computed according to algorithm-specific rules
    /// regarding the default size of boxes.
    #[inline(always)]
    pub const fn auto() -> Self {
        Self(Self::AUTO_TAG)
    }

    /// The dimension as a fraction of the total available grid space (`fr` units in CSS)
    /// Specified value is the numerator of the fraction. Denominator is the sum of all fraction specified in that grid dimension
    /// Spec: <https://www.w3.org/TR/css3-grid-layout/#fr-unit>
    #[inline(always)]
    pub const fn fr(val: f32) -> Self {
        Self(((f32_to_bits(val) as u64) << 32) | Self::FR_TAG)
    }

    /// The size should be the "min-content" size.
    /// This is the smallest size that can fit the item's contents with ALL soft line-wrapping opportunities taken
    #[inline(always)]
    pub const fn min_content() -> Self {
        Self(Self::MIN_CONTENT_TAG)
    }

    /// The size should be the "max-content" size.
    /// This is the smallest size that can fit the item's contents with NO soft line-wrapping opportunities taken
    #[inline(always)]
    pub const fn max_content() -> Self {
        Self(Self::MAX_CONTENT_TAG)
    }

    /// The size should be computed according to the "fit content" formula:
    ///    `max(min_content, min(max_content, limit))`
    /// where:
    ///    - `min_content` is the [min-content](Self::min_content) size
    ///    - `max_content` is the [max-content](Self::max_content) size
    ///    - `limit` is a LENGTH value passed to this function
    ///
    /// The effect of this is that the item takes the size of `limit` clamped
    /// by the min-content and max-content sizes.
    #[inline(always)]
    pub const fn fit_content_px(limit: f32) -> Self {
        Self(((f32_to_bits(limit) as u64) << 32) | Self::FIT_CONTENT_PX_TAG)
    }

    /// The size should be computed according to the "fit content" formula:
    ///    `max(min_content, min(max_content, limit))`
    /// where:
    ///    - `min_content` is the [min-content](Self::min_content) size
    ///    - `max_content` is the [max-content](Self::max_content) size
    ///    - `limit` is a PERCENTAGE value passed to this function
    ///
    /// The effect of this is that the item takes the size of `limit` clamped
    /// by the min-content and max-content sizes.
    #[inline(always)]
    pub const fn fit_content_percent(limit: f32) -> Self {
        Self(((f32_to_bits(limit) as u64) << 32) | Self::FIT_CONTENT_PERCENT_TAG)
    }

    /// Get the primary tag
    #[inline(always)]
    pub const fn tag(self) -> u64 {
        self.0 & Self::TAG_MASK
    }

    /// Get the numeric value associated with the `CompactLength`
    /// (e.g. the pixel value for a LENGTH variant)
    #[inline(always)]
    pub const fn value(self) -> f32 {
        f32_from_bits((self.0 >> 32) as u32)
    }

    /// Get the numeric value associated with the `CompactLength`
    /// (e.g. the pixel value for a LENGTH variant)
    #[inline(always)]
    pub const fn calc_value(self) -> u64 {
        self.0
    }

    /// Returns true if the value is 0 px
    #[inline(always)]
    pub const fn is_calc(self) -> bool {
        self.0 & Self::CALC_TAG_MASK == 0
    }

    /// Returns true if the value is 0 px
    #[inline(always)]
    pub const fn is_zero(self) -> bool {
        self.0 == Self::ZERO.0
    }

    /// Returns true if the value is a length or percentage value
    #[inline(always)]
    pub const fn is_length_or_percentage(self) -> bool {
        matches!(self.tag(), Self::LENGTH_TAG | Self::PERCENT_TAG)
    }

    /// Returns true if the value is auto
    #[inline(always)]
    pub const fn is_auto(self) -> bool {
        self.tag() == Self::AUTO_TAG
    }

    /// Returns true if the value is min-content
    #[inline(always)]
    pub const fn is_min_content(self) -> bool {
        matches!(self.tag(), Self::MIN_CONTENT_TAG)
    }

    /// Returns true if the value is max-content
    #[inline(always)]
    pub const fn is_max_content(self) -> bool {
        matches!(self.tag(), Self::MAX_CONTENT_TAG)
    }

    /// Returns true if the value is a fit-content(...) value
    #[inline(always)]
    pub const fn is_fit_content(self) -> bool {
        matches!(self.tag(), Self::FIT_CONTENT_PX_TAG | Self::FIT_CONTENT_PERCENT_TAG)
    }

    /// Returns true if the value is max-content or a fit-content(...) value
    #[inline(always)]
    pub const fn is_max_or_fit_content(self) -> bool {
        matches!(self.tag(), Self::MAX_CONTENT_TAG | Self::FIT_CONTENT_PX_TAG | Self::FIT_CONTENT_PERCENT_TAG)
    }

    /// Returns true if the max track sizing function is `MaxContent`, `FitContent` or `Auto` else false.
    /// "In all cases, treat auto and fit-content() as max-content, except where specified otherwise for fit-content()."
    /// See: <https://www.w3.org/TR/css-grid-1/#algo-terms>
    #[inline(always)]
    pub fn is_max_content_alike(&self) -> bool {
        matches!(
            self.tag(),
            CompactLength::AUTO_TAG
                | CompactLength::MAX_CONTENT_TAG
                | CompactLength::FIT_CONTENT_PX_TAG
                | CompactLength::FIT_CONTENT_PERCENT_TAG
        )
    }

    /// Returns true if the min track sizing function is `MinContent` or `MaxContent`, else false.
    #[inline(always)]
    pub fn is_min_or_max_content(&self) -> bool {
        matches!(self.tag(), Self::MIN_CONTENT_TAG | Self::MAX_CONTENT_TAG)
    }

    /// Returns true if the value is auto, min-content, max-content, or fit-content(...)
    #[inline(always)]
    pub const fn is_intrinsic(self) -> bool {
        matches!(
            self.tag(),
            Self::AUTO_TAG
                | Self::MIN_CONTENT_TAG
                | Self::MAX_CONTENT_TAG
                | Self::FIT_CONTENT_PX_TAG
                | Self::FIT_CONTENT_PERCENT_TAG
        )
    }

    /// Returns true if the value is and fr value
    #[inline(always)]
    pub const fn is_fr(self) -> bool {
        self.tag() == Self::FR_TAG
    }

    /// Whether the track sizing functions depends on the size of the parent node
    #[inline(always)]
    pub const fn uses_percentage(self) -> bool {
        // TODO: handle calc() values
        matches!(self.tag(), CompactLength::PERCENT_TAG | CompactLength::FIT_CONTENT_PERCENT_TAG) || self.is_calc()
    }

    /// Resolve percentage values against the passed parent_size, returning Some(value)
    /// Non-percentage values always return None.
    #[inline(always)]
    pub fn resolved_percentage_size(self, parent_size: f32, calc_resolver: impl Fn(u64, f32) -> f32) -> Option<f32> {
        match self.tag() {
            CompactLength::PERCENT_TAG => Some(self.value() * parent_size),
            _ if self.is_calc() => Some(calc_resolver(self.0, parent_size)),
            _ => None,
        }
    }
}

impl TaffyZero for CompactLength {
    const ZERO: Self = Self::length(0.0);
}
impl TaffyAuto for CompactLength {
    const AUTO: Self = Self::auto();
}
impl TaffyMinContent for CompactLength {
    const MIN_CONTENT: Self = Self::min_content();
}
impl TaffyMaxContent for CompactLength {
    const MAX_CONTENT: Self = Self::max_content();
}
impl FromLength for CompactLength {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::length(value.into())
    }
}
impl FromPercent for CompactLength {
    fn from_percent<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::percent(value.into())
    }
}
impl FromFr for CompactLength {
    fn from_fr<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::fr(value.into())
    }
}
impl TaffyFitContent for CompactLength {
    fn fit_content(lp: LengthPercentage) -> Self {
        let value = lp.0.value();
        match lp.0.tag() {
            Self::LENGTH_TAG => Self::fit_content_px(value),
            Self::PERCENT_TAG => Self::fit_content_percent(value),
            _ => unreachable!(),
        }
    }
}

/// Note: these two functions are copied directly from the std (core) library. But by duplicating them
/// here we can reduce MSRV from 1.83 all the way down to 1.65 while retaining const constructors.
mod compat {
    #![allow(unsafe_code)]
    #![allow(clippy::transmute_float_to_int)]

    /// Raw transmutation from `f32` to `u32`.
    pub const fn f32_to_bits(val: f32) -> u32 {
        // SAFETY: `u32` is a plain old datatype so we can always transmute to it.
        unsafe { core::mem::transmute(val) }
    }
    /// Raw transmutation from `u32` to `f32`.
    pub const fn f32_from_bits(v: u32) -> f32 {
        // SAFETY: `u32` is a plain old datatype so we can always transmute from it.
        unsafe { core::mem::transmute(v) }
    }
}
