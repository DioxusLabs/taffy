//! A tagged-pointer abstraction that allows size styles in Taffy to be represented
//! in just 64 bits. Wrapped by types in the `super::dimension` and `super::grid` modules.
use super::LengthPercentage;
use crate::style_helpers::{
    FromFr, FromLength, FromPercent, TaffyAuto, TaffyFitContent, TaffyMaxContent, TaffyMinContent, TaffyZero,
};

/// Note: these two functions are copied directly from the std (core) library. But by duplicating them
/// here we can reduce MSRV from 1.84 all the way down to 1.65 while retaining const constructors and
/// strict pointer provenance
mod compat {
    #![allow(unsafe_code)]
    #![allow(unknown_lints)]
    #![allow(unnecessary_transmutes)]

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

    /// Tag a pointer preserving provenance (requires Rust 1.84)
    #[inline(always)]
    #[cfg(all(target_pointer_width = "64", feature = "strict_provenance"))]
    pub fn tag_ptr(ptr: *const (), tag: usize) -> *const () {
        ptr.map_addr(|a| a | tag)
    }

    /// Tag a pointer exposing provenance (works back to Rust 1.0)
    #[inline(always)]
    #[cfg(all(target_pointer_width = "64", not(feature = "strict_provenance")))]
    pub fn tag_ptr(ptr: *const (), tag: usize) -> *const () {
        (ptr as usize | tag) as *const ()
    }

    /// Clear the tag bits of a pointer preserving provenance (requires Rust 1.84)
    #[inline(always)]
    #[cfg(all(target_pointer_width = "64", feature = "strict_provenance"))]
    pub fn untag_ptr(ptr: *const (), mask: usize) -> *const () {
        ptr.map_addr(|a| a & !mask)
    }

    /// Clear the tag bits of a pointer exposing provenance (works back to Rust 1.0)
    #[inline(always)]
    #[cfg(all(target_pointer_width = "64", not(feature = "strict_provenance")))]
    pub fn untag_ptr(ptr: *const (), mask: usize) -> *const () {
        (ptr as usize & !mask) as *const ()
    }

    /// Create a pointer without provenance from an address (requires Rust 1.84)
    #[inline(always)]
    #[cfg(all(target_pointer_width = "64", feature = "strict_provenance"))]
    pub fn ptr_from_addr(addr: u64) -> *const () {
        core::ptr::without_provenance(addr as usize)
    }

    /// Create a pointer from an address (works back to Rust 1.0)
    #[inline(always)]
    #[cfg(all(target_pointer_width = "64", not(feature = "strict_provenance")))]
    pub fn ptr_from_addr(addr: u64) -> *const () {
        addr as usize as *const ()
    }

    /// Get the address of a pointer without exposing its provenance (requires Rust 1.84)
    #[inline(always)]
    #[cfg(all(target_pointer_width = "64", feature = "strict_provenance"))]
    pub fn ptr_addr(ptr: *const ()) -> u64 {
        ptr.addr() as u64
    }

    /// Get the address of a pointer (works back to Rust 1.0)
    #[inline(always)]
    #[cfg(all(target_pointer_width = "64", not(feature = "strict_provenance")))]
    pub fn ptr_addr(ptr: *const ()) -> u64 {
        ptr as usize as u64
    }
}

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
std::compile_error!("Taffy only supports targets with a pointer width of 32 or 64 bits");

/// An opaque handle to a `calc()` expression, stored in a [`CompactLength`] and passed back to
/// [`LayoutPartialTree::resolve_calc_value`](crate::LayoutPartialTree::resolve_calc_value) when the
/// expression needs to be resolved.
///
/// A handle is either a pointer ([`CalcHandle::from_ptr`]) or an integer ([`CalcHandle::from_bits`])
/// of up to 64 bits on every target. To be stored in a [`CompactLength`] it must be non-zero and its
/// low 3 bits must be zero (they are used for the tag), which any 8-byte aligned pointer satisfies.
///
/// On 64-bit targets the handle is stored as a pointer, so a handle created with `from_ptr` keeps its
/// provenance (and with the `strict_provenance` feature, Taffy only uses strict provenance APIs).
/// On 32-bit targets the handle is stored as a `u64`, so `from_ptr` exposes the pointer's provenance
/// and [`CalcHandle::ptr`] recovers it.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CalcHandle(#[cfg(target_pointer_width = "64")] *const (), #[cfg(target_pointer_width = "32")] u64);

#[cfg(target_pointer_width = "64")]
impl CalcHandle {
    /// Create a handle from a pointer
    #[inline(always)]
    pub fn from_ptr(ptr: *const ()) -> Self {
        Self(ptr)
    }

    /// Create a handle from an integer
    #[inline(always)]
    pub fn from_bits(bits: u64) -> Self {
        Self(compat::ptr_from_addr(bits))
    }

    /// Get the handle as a pointer. If the handle was created with [`CalcHandle::from_ptr`], this is
    /// the pointer it was created from (with its provenance).
    #[inline(always)]
    pub fn ptr(self) -> *const () {
        self.0
    }

    /// Get the handle as an integer. If the handle was created with [`CalcHandle::from_bits`], this
    /// is the integer it was created from.
    #[inline(always)]
    pub fn to_bits(self) -> u64 {
        compat::ptr_addr(self.0)
    }
}

#[cfg(target_pointer_width = "32")]
impl CalcHandle {
    /// Create a handle from a pointer. On 32-bit targets this exposes the pointer's provenance.
    #[inline(always)]
    pub fn from_ptr(ptr: *const ()) -> Self {
        Self(ptr as usize as u64)
    }

    /// Create a handle from an integer
    #[inline(always)]
    pub fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Get the handle as a pointer. If the handle was created with [`CalcHandle::from_ptr`], this is
    /// the pointer it was created from (with its exposed provenance). Handles created from integers
    /// are truncated to 32 bits.
    #[inline(always)]
    pub fn ptr(self) -> *const () {
        self.0 as usize as *const ()
    }

    /// Get the handle as an integer. If the handle was created with [`CalcHandle::from_bits`], this
    /// is the integer it was created from.
    #[inline(always)]
    pub fn to_bits(self) -> u64 {
        self.0
    }
}

/// CompactLengthInner implementation for 64 bit platforms
#[cfg(target_pointer_width = "64")]
mod inner {
    use super::compat::{f32_from_bits, f32_to_bits, tag_ptr, untag_ptr};
    use super::CalcHandle;

    /// The low byte (8 bits)
    const TAG_MASK: usize = 0b11111111;
    /// The low 3 bits
    const CALC_TAG_MASK: usize = 0b111;

    /// On 64 bit platforms the tag, value and pointer are packed into a single 64 bit pointer
    ///
    /// The tagged pointer always has a tag and may contain an f32 value or a pointer
    /// (or neither) depending on the variant indicated by the tag.
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub(super) struct CompactLengthInner {
        /// The tagged pointer
        tagged_ptr: *const (),
    }
    impl CompactLengthInner {
        /// Construct a `CompactLengthInner` from a tag and calc handle
        #[inline(always)]
        pub(super) fn from_calc(handle: CalcHandle, tag: usize) -> Self {
            let tagged_ptr = tag_ptr(handle.0, tag);
            Self { tagged_ptr }
        }

        /// Construct a `CompactLengthInner` from a tag and numeric value
        #[inline(always)]
        pub(super) const fn from_val(val: f32, tag: usize) -> Self {
            let tagged_ptr = (((f32_to_bits(val) as usize) << 32) | tag) as *const ();
            Self { tagged_ptr }
        }

        /// Construct a `CompactLengthInner` from only a tag
        #[inline(always)]
        pub(super) const fn from_tag(tag: usize) -> Self {
            let tagged_ptr = tag as *const ();
            Self { tagged_ptr }
        }

        /// Get the calc tag (low 3 bits)
        #[inline(always)]
        #[cfg(feature = "calc")]
        pub(super) fn calc_tag(self) -> usize {
            (self.tagged_ptr as usize) & CALC_TAG_MASK
        }

        /// Get the general tag (low 8 bits)
        #[inline(always)]
        pub(super) fn tag(self) -> usize {
            (self.tagged_ptr as usize) & TAG_MASK
        }

        /// Get the calc handle (with the tag bits cleared)
        #[inline(always)]
        pub(super) fn calc_handle(self) -> CalcHandle {
            CalcHandle(untag_ptr(self.tagged_ptr, CALC_TAG_MASK))
        }

        /// Get the numeric value
        #[inline(always)]
        pub(super) fn value(self) -> f32 {
            f32_from_bits((self.tagged_ptr as usize >> 32) as u32)
        }

        /// Get the serialized value.
        #[inline(always)]
        #[cfg(feature = "serde")]
        pub(super) fn serialized(self) -> u64 {
            (self.tagged_ptr as usize as u64).rotate_left(32)
        }

        /// Derialized from a value.
        #[inline(always)]
        #[cfg(feature = "serde")]
        pub(super) fn from_serialized(value: u64) -> Self {
            Self { tagged_ptr: value.rotate_right(32) as usize as *const () }
        }
    }
}

/// CompactLengthInner implementation for 32 bit platforms
#[cfg(target_pointer_width = "32")]
mod inner {
    use super::compat::{f32_from_bits, f32_to_bits};
    use super::CalcHandle;

    /// The low byte (8 bits)
    const TAG_MASK: u64 = 0b11111111;
    /// The low 3 bits
    const CALC_TAG_MASK: u64 = 0b111;

    /// On 32 bit platforms the tag, value and calc handle are packed into a single `u64`, using the
    /// same layout as on 64 bit platforms (so calc handles have the same 61 bits available).
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub(super) struct CompactLengthInner {
        /// The tagged value
        bits: u64,
    }

    impl CompactLengthInner {
        /// Construct a `CompactLengthInner` from a tag and calc handle
        #[inline(always)]
        pub(super) fn from_calc(handle: CalcHandle, tag: usize) -> Self {
            Self { bits: handle.0 | tag as u64 }
        }

        /// Construct a `CompactLengthInner` from a tag and numeric value
        #[inline(always)]
        pub(super) const fn from_val(val: f32, tag: usize) -> Self {
            Self { bits: ((f32_to_bits(val) as u64) << 32) | tag as u64 }
        }

        /// Construct a `CompactLengthInner` from only a tag
        #[inline(always)]
        pub(super) const fn from_tag(tag: usize) -> Self {
            Self { bits: tag as u64 }
        }

        /// Get the calc tag (low 3 bits)
        #[inline(always)]
        #[cfg(feature = "calc")]
        pub(super) fn calc_tag(self) -> usize {
            (self.bits & CALC_TAG_MASK) as usize
        }

        /// Get the general tag (low 8 bits)
        #[inline(always)]
        pub(super) fn tag(self) -> usize {
            (self.bits & TAG_MASK) as usize
        }

        /// Get the calc handle (with the tag bits cleared)
        #[inline(always)]
        pub(super) fn calc_handle(self) -> CalcHandle {
            CalcHandle(self.bits & !CALC_TAG_MASK)
        }

        /// Get the numeric value
        #[inline(always)]
        pub(super) fn value(self) -> f32 {
            f32_from_bits((self.bits >> 32) as u32)
        }

        /// Get the serialized value.
        #[inline(always)]
        #[cfg(feature = "serde")]
        pub(super) fn serialized(self) -> u64 {
            self.bits.rotate_left(32)
        }

        /// Derialized from a value.
        #[inline(always)]
        #[cfg(feature = "serde")]
        pub(super) fn from_serialized(value: u64) -> Self {
            Self { bits: value.rotate_right(32) }
        }
    }
}

use inner::CompactLengthInner;

/// A representation of a length as a compact 64-bit tagged pointer
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct CompactLength(CompactLengthInner);

/// Tag layout
///
/// Tags are 8 bits. The low 3 bits are shared with pointer variants (such as `calc()`), which require
/// an 8-byte aligned pointer and so always have those bits clear. They are assigned as follows:
///
///   - Bit 0 (`NON_POINTER_BIT`): set for every variant that is *not* a pointer. Pointer variants
///     (`calc()`) have this bit clear, so `is_plain_calc()` is a single bit test.
///   - Bit 1 (`KEYWORD_BIT`): set for every variant that is *not* a plain length, percentage or calc()
///     value (i.e. everything that cannot be resolved to a length given only a percentage basis).
///   - Bit 2 (`PERCENTAGE_BIT`): set for every variant whose resolved value depends on a percentage basis,
///     including all `calc()` variants. `uses_percentage()` is a single bit test.
///
/// Pointer variants are distinguished by bit 1: `calc()` is `0b100` and `fit-content(calc())` is `0b110`.
/// Any pointer variant's `tag()` includes address bits 3..7, so pointer variants must be tested with low-bit
/// masks (`is_plain_calc()`, `is_fit_content_calc()`, `has_calc()`) rather than as `match` arms.
///
/// Within the keyword class, bit 3 (`SIZING_KEYWORD_BIT`) is set for the CSS sizing keywords
/// (`min-content`, `max-content`, `fit-content`, `fit-content()`, `stretch`) and clear for `auto`,
/// `fr` and `content`. The remaining high bits distinguish variants within a class.
impl CompactLength {
    /// Bit set for all non-pointer (i.e. not `calc()`) tags
    pub const NON_POINTER_BIT: usize = 0b0001;
    /// Bit set for all tags that are not a plain length, percentage or calc() value
    pub const KEYWORD_BIT: usize = 0b0010;
    /// Bit set for all tags whose value depends on a percentage basis (including `calc()`)
    pub const PERCENTAGE_BIT: usize = 0b0100;
    /// Bit set (in addition to `KEYWORD_BIT`) for all CSS sizing keyword tags
    pub const SIZING_KEYWORD_BIT: usize = 0b1000;

    /// The tag indicating a calc() value
    #[cfg(feature = "calc")]
    pub const CALC_TAG: usize = 0b0000_0100;
    /// The tag indicating a fit-content() value with a calc() limit
    #[cfg(feature = "calc")]
    pub const FIT_CONTENT_CALC_TAG: usize = 0b0000_0110;
    /// The tag indicating a length value
    pub const LENGTH_TAG: usize = 0b0000_0001;
    /// The tag indicating a percentage value
    pub const PERCENT_TAG: usize = 0b0000_0101;
    /// The tag indicating an auto value
    pub const AUTO_TAG: usize = 0b0000_0011;
    /// The tag indicating an fr value
    pub const FR_TAG: usize = 0b0001_0011;
    /// The tag indicating a content keyword value
    pub const CONTENT_TAG: usize = 0b0010_0011;
    /// The tag indicating a min-content value
    pub const MIN_CONTENT_TAG: usize = 0b0000_1011;
    /// The tag indicating a max-content value
    pub const MAX_CONTENT_TAG: usize = 0b0001_1011;
    /// The tag indicating a fit-content value with px limit
    pub const FIT_CONTENT_PX_TAG: usize = 0b0010_1011;
    /// The tag indicating a plain fit-content keyword value (no limit)
    pub const FIT_CONTENT_KEYWORD_TAG: usize = 0b0011_1011;
    /// The tag indicating a stretch keyword value
    pub const STRETCH_TAG: usize = 0b0100_1011;
    /// The tag indicating a fit-content value with percent limit
    pub const FIT_CONTENT_PERCENT_TAG: usize = 0b0000_1111;
}

impl CompactLength {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    #[inline(always)]
    pub const fn length(val: f32) -> Self {
        Self(CompactLengthInner::from_val(val, Self::LENGTH_TAG))
    }

    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    #[inline(always)]
    pub const fn percent(val: f32) -> Self {
        Self(CompactLengthInner::from_val(val, Self::PERCENT_TAG))
    }

    /// A `calc()` value. The value passed here is treated as an opaque handle to
    /// the actual calc representation and may be a pointer, index, etc.
    ///
    /// The low 3 bits are used as a tag value and will be returned as 0.
    #[inline]
    #[cfg(feature = "calc")]
    pub fn calc(handle: CalcHandle) -> Self {
        assert_ne!(handle.to_bits(), 0);
        assert_eq!(handle.to_bits() & 0b111, 0);
        Self(CompactLengthInner::from_calc(handle, Self::CALC_TAG))
    }

    /// A `fit-content(calc())` value. The value passed here is treated as an opaque handle to
    /// the actual calc representation of the limit and may be a pointer, index, etc.
    ///
    /// The low 3 bits are used as a tag value and will be returned as 0.
    #[inline]
    #[cfg(feature = "calc")]
    pub fn fit_content_calc(handle: CalcHandle) -> Self {
        assert_ne!(handle.to_bits(), 0);
        assert_eq!(handle.to_bits() & 0b111, 0);
        Self(CompactLengthInner::from_calc(handle, Self::FIT_CONTENT_CALC_TAG))
    }

    /// The dimension should be automatically computed according to algorithm-specific rules
    /// regarding the default size of boxes.
    #[inline(always)]
    pub const fn auto() -> Self {
        Self(CompactLengthInner::from_tag(Self::AUTO_TAG))
    }

    /// The dimension as a fraction of the total available grid space (`fr` units in CSS)
    /// Specified value is the numerator of the fraction. Denominator is the sum of all fraction specified in that grid dimension
    /// Spec: <https://www.w3.org/TR/css3-grid-layout/#fr-unit>
    #[inline(always)]
    pub const fn fr(val: f32) -> Self {
        Self(CompactLengthInner::from_val(val, Self::FR_TAG))
    }

    /// The size should be the "min-content" size.
    /// This is the smallest size that can fit the item's contents with ALL soft line-wrapping opportunities taken
    #[inline(always)]
    pub const fn min_content() -> Self {
        Self(CompactLengthInner::from_tag(Self::MIN_CONTENT_TAG))
    }

    /// The size should be the "max-content" size.
    /// This is the smallest size that can fit the item's contents with NO soft line-wrapping opportunities taken
    #[inline(always)]
    pub const fn max_content() -> Self {
        Self(CompactLengthInner::from_tag(Self::MAX_CONTENT_TAG))
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
        Self(CompactLengthInner::from_val(limit, Self::FIT_CONTENT_PX_TAG))
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
        Self(CompactLengthInner::from_val(limit, Self::FIT_CONTENT_PERCENT_TAG))
    }

    /// The size should be computed according to the "fit content" formula:
    ///    `max(min_content, min(max_content, stretch))`
    /// where:
    ///    - `min_content` is the [min-content](Self::min_content) size
    ///    - `max_content` is the [max-content](Self::max_content) size
    ///    - `stretch` is the "stretch-fit" size (the size the box would take if it filled the available space)
    #[inline(always)]
    pub const fn fit_content_keyword() -> Self {
        Self(CompactLengthInner::from_tag(Self::FIT_CONTENT_KEYWORD_TAG))
    }

    /// The size should be the "stretch-fit" size: the size the box would take
    /// if it filled the available space
    /// (<https://www.w3.org/TR/css-sizing-4/#stretch-fit-sizing>)
    #[inline(always)]
    pub const fn stretch() -> Self {
        Self(CompactLengthInner::from_tag(Self::STRETCH_TAG))
    }

    /// The size should be an automatic size based on the box's content
    /// (<https://www.w3.org/TR/css-flexbox-1/#valdef-flex-basis-content>)
    ///
    /// This keyword is only valid for `flex-basis`. In any other context it behaves as [`auto`](Self::auto).
    #[inline(always)]
    pub const fn content() -> Self {
        Self(CompactLengthInner::from_tag(Self::CONTENT_TAG))
    }

    /// Get the primary tag
    #[inline(always)]
    pub fn tag(self) -> usize {
        self.0.tag()
    }

    /// Get the numeric value associated with the `CompactLength`
    /// (e.g. the pixel value for a LENGTH variant)
    #[inline(always)]
    pub fn value(self) -> f32 {
        self.0.value()
    }

    /// Get the calc handle of the `CompactLength`
    #[inline(always)]
    #[cfg(feature = "calc")]
    pub fn calc_value(self) -> CalcHandle {
        self.0.calc_handle()
    }

    /// Returns true if the value is a plain calc() value (not a fit-content(calc()) value)
    #[inline(always)]
    #[cfg(feature = "calc")]
    pub fn is_plain_calc(self) -> bool {
        self.0.calc_tag() & (Self::NON_POINTER_BIT | Self::KEYWORD_BIT) == 0
    }

    /// Returns true if the value is a fit-content() value with a calc() limit
    #[inline(always)]
    pub fn is_fit_content_calc(self) -> bool {
        self.tag() & (Self::NON_POINTER_BIT | Self::KEYWORD_BIT) == Self::KEYWORD_BIT
    }

    /// Returns true if the value holds a calc() pointer: either a plain calc() value or a
    /// fit-content() value with a calc() limit. `calc_value()` is valid for exactly these values.
    #[inline(always)]
    pub fn has_calc(self) -> bool {
        self.tag() & Self::NON_POINTER_BIT == 0
    }

    /// Returns true if the value is 0 px
    #[inline(always)]
    pub fn is_zero(self) -> bool {
        self.0 == Self::ZERO.0
    }

    /// Returns true if the value is a length or percentage value
    #[inline(always)]
    pub fn is_length_or_percentage(self) -> bool {
        self.tag() & (Self::NON_POINTER_BIT | Self::KEYWORD_BIT) == Self::NON_POINTER_BIT
    }

    /// Returns true if the value is a length, a percentage or a plain `calc()` value (not a
    /// `fit-content(calc())` value): anything that resolves to a length given a percentage basis
    #[inline(always)]
    pub fn is_length_percentage_or_calc(self) -> bool {
        self.tag() & Self::KEYWORD_BIT == 0
    }

    /// Returns true if the value is auto
    #[inline(always)]
    pub fn is_auto(self) -> bool {
        self.tag() == Self::AUTO_TAG
    }

    /// Returns true if the value is the content keyword
    #[inline(always)]
    pub fn is_content(self) -> bool {
        self.tag() == Self::CONTENT_TAG
    }

    /// Returns true if the value is min-content
    #[inline(always)]
    pub fn is_min_content(self) -> bool {
        matches!(self.tag(), Self::MIN_CONTENT_TAG)
    }

    /// Returns true if the value is max-content
    #[inline(always)]
    pub fn is_max_content(self) -> bool {
        matches!(self.tag(), Self::MAX_CONTENT_TAG)
    }

    /// Returns true if the value is a fit-content(...) value
    #[inline(always)]
    pub fn is_fit_content(self) -> bool {
        matches!(self.tag(), Self::FIT_CONTENT_PX_TAG | Self::FIT_CONTENT_PERCENT_TAG) || self.is_fit_content_calc()
    }

    /// Returns true if the value is min-content, max-content, fit-content, fit-content(...), or stretch
    #[inline(always)]
    pub fn is_sizing_keyword(self) -> bool {
        const MASK: usize =
            CompactLength::NON_POINTER_BIT | CompactLength::KEYWORD_BIT | CompactLength::SIZING_KEYWORD_BIT;
        self.tag() & MASK == MASK || self.is_fit_content_calc()
    }

    /// Returns true if the value is max-content or a fit-content(...) value
    #[inline(always)]
    pub fn is_max_or_fit_content(self) -> bool {
        matches!(self.tag(), Self::MAX_CONTENT_TAG | Self::FIT_CONTENT_PX_TAG | Self::FIT_CONTENT_PERCENT_TAG)
            || self.is_fit_content_calc()
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
        ) || self.is_fit_content_calc()
    }

    /// Returns true if the min track sizing function is `MinContent` or `MaxContent`, else false.
    #[inline(always)]
    pub fn is_min_or_max_content(&self) -> bool {
        matches!(self.tag(), Self::MIN_CONTENT_TAG | Self::MAX_CONTENT_TAG)
    }

    /// Returns true if the value is auto, min-content, max-content, or fit-content(...)
    #[inline(always)]
    pub fn is_intrinsic(self) -> bool {
        matches!(
            self.tag(),
            Self::AUTO_TAG
                | Self::MIN_CONTENT_TAG
                | Self::MAX_CONTENT_TAG
                | Self::FIT_CONTENT_PX_TAG
                | Self::FIT_CONTENT_PERCENT_TAG
        ) || self.is_fit_content_calc()
    }

    /// Returns true if the value is and fr value
    #[inline(always)]
    pub fn is_fr(self) -> bool {
        self.tag() == Self::FR_TAG
    }

    /// Whether the track sizing functions depends on the size of the parent node
    #[inline(always)]
    pub fn uses_percentage(self) -> bool {
        self.tag() & Self::PERCENTAGE_BIT != 0
    }

    /// Resolve percentage values against the passed parent_size, returning Some(value)
    /// Non-percentage values always return None.
    #[inline(always)]
    pub fn resolved_percentage_size(
        self,
        parent_size: f32,
        calc_resolver: impl Fn(CalcHandle, f32) -> f32,
    ) -> Option<f32> {
        match self.tag() {
            CompactLength::PERCENT_TAG => Some(self.value() * parent_size),
            #[cfg(feature = "calc")]
            _ if self.is_plain_calc() => Some(calc_resolver(self.0.calc_handle(), parent_size)),
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
    fn from_length<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::length(value.into() as f32)
    }
}
impl FromPercent for CompactLength {
    fn from_percent<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::percent(value.into() as f32)
    }
}
impl FromFr for CompactLength {
    fn from_fr<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::fr(value.into() as f32)
    }
}
impl TaffyFitContent for CompactLength {
    fn fit_content(lp: LengthPercentage) -> Self {
        let value = lp.0.value();
        match lp.0.tag() {
            Self::LENGTH_TAG => Self::fit_content_px(value),
            Self::PERCENT_TAG => Self::fit_content_percent(value),
            #[cfg(feature = "calc")]
            _ if lp.0.is_plain_calc() => Self::fit_content_calc(lp.0.calc_value()),
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for CompactLength {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[cfg(feature = "calc")]
        {
            if self.has_calc() {
                Err(serde::ser::Error::custom("Cannot serialize Calc value"))
            } else {
                serializer.serialize_u64(self.0.serialized())
            }
        }

        #[cfg(not(feature = "calc"))]
        serializer.serialize_u64(self.0.serialized())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CompactLength {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bits: u64 = u64::deserialize(deserializer)?;
        let value = Self(CompactLengthInner::from_serialized(bits));
        // Note: validation intentionally excludes the CALC_TAG as deserializing calc() values is not supported.
        // All of the accepted tags have `NON_POINTER_BIT` set, so a calc() tag can never pass validation.
        if matches!(
            value.tag(),
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
                | CompactLength::FR_TAG
        ) {
            Ok(value)
        } else {
            Err(serde::de::Error::custom("Cannot deserialize Calc value"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CalcHandle, CompactLength, CompactLengthInner};

    impl CompactLength {
        fn from_raw_tag(tag: usize) -> Self {
            Self(CompactLengthInner::from_tag(tag))
        }
    }

    const NON_PERCENTAGE_TAGS: &[usize] = &[
        CompactLength::LENGTH_TAG,
        CompactLength::AUTO_TAG,
        CompactLength::FR_TAG,
        CompactLength::MIN_CONTENT_TAG,
        CompactLength::MAX_CONTENT_TAG,
        CompactLength::FIT_CONTENT_PX_TAG,
        CompactLength::FIT_CONTENT_KEYWORD_TAG,
        CompactLength::STRETCH_TAG,
        CompactLength::CONTENT_TAG,
    ];
    const PERCENTAGE_TAGS: &[usize] = &[CompactLength::PERCENT_TAG, CompactLength::FIT_CONTENT_PERCENT_TAG];

    #[test]
    fn tags_are_unique_and_fit_in_a_byte() {
        let mut all: Vec<usize> = NON_PERCENTAGE_TAGS.iter().chain(PERCENTAGE_TAGS).copied().collect();
        #[cfg(feature = "calc")]
        all.push(CompactLength::CALC_TAG);
        #[cfg(feature = "calc")]
        all.push(CompactLength::FIT_CONTENT_CALC_TAG);
        assert!(all.iter().all(|tag| *tag <= 0xFF));
        all.sort_unstable();
        all.dedup();
        assert_eq!(all.len(), NON_PERCENTAGE_TAGS.len() + PERCENTAGE_TAGS.len() + 2 * cfg!(feature = "calc") as usize);
    }

    #[test]
    fn non_pointer_tags_have_non_pointer_bit_set() {
        for tag in NON_PERCENTAGE_TAGS.iter().chain(PERCENTAGE_TAGS) {
            assert_ne!(tag & CompactLength::NON_POINTER_BIT, 0, "tag {tag:#010b}");
        }
        #[cfg(feature = "calc")]
        assert_eq!(CompactLength::CALC_TAG & CompactLength::NON_POINTER_BIT, 0);
    }

    #[test]
    fn keyword_bits_match_predicates() {
        let all = NON_PERCENTAGE_TAGS.iter().chain(PERCENTAGE_TAGS).map(|tag| CompactLength::from_raw_tag(*tag));
        for value in all {
            let tag = value.tag();
            let is_plain = matches!(tag, CompactLength::LENGTH_TAG | CompactLength::PERCENT_TAG);
            assert_eq!(tag & CompactLength::KEYWORD_BIT == 0, is_plain, "tag {tag:#010b}");
            assert_eq!(value.is_length_or_percentage(), is_plain, "tag {tag:#010b}");
            let is_sizing_keyword = matches!(
                tag,
                CompactLength::MIN_CONTENT_TAG
                    | CompactLength::MAX_CONTENT_TAG
                    | CompactLength::FIT_CONTENT_KEYWORD_TAG
                    | CompactLength::FIT_CONTENT_PX_TAG
                    | CompactLength::FIT_CONTENT_PERCENT_TAG
                    | CompactLength::STRETCH_TAG
            );
            assert_eq!(value.is_sizing_keyword(), is_sizing_keyword, "tag {tag:#010b}");
        }
        #[cfg(feature = "calc")]
        {
            assert_eq!(CompactLength::CALC_TAG & CompactLength::KEYWORD_BIT, 0);
            assert_ne!(CompactLength::FIT_CONTENT_CALC_TAG & CompactLength::KEYWORD_BIT, 0);
            assert_eq!(CompactLength::FIT_CONTENT_CALC_TAG & CompactLength::NON_POINTER_BIT, 0);
            assert_ne!(CompactLength::FIT_CONTENT_CALC_TAG & CompactLength::PERCENTAGE_BIT, 0);
        }
    }

    #[test]
    fn percentage_bit_matches_uses_percentage() {
        for tag in NON_PERCENTAGE_TAGS {
            assert_eq!(tag & CompactLength::PERCENTAGE_BIT, 0, "tag {tag:#010b}");
        }
        for tag in PERCENTAGE_TAGS {
            assert_ne!(tag & CompactLength::PERCENTAGE_BIT, 0, "tag {tag:#010b}");
        }
        #[cfg(feature = "calc")]
        assert_ne!(CompactLength::CALC_TAG & CompactLength::PERCENTAGE_BIT, 0);

        assert!(!CompactLength::length(10.0).uses_percentage());
        assert!(!CompactLength::auto().uses_percentage());
        assert!(!CompactLength::fr(1.0).uses_percentage());
        assert!(!CompactLength::min_content().uses_percentage());
        assert!(!CompactLength::max_content().uses_percentage());
        assert!(!CompactLength::fit_content_px(10.0).uses_percentage());
        assert!(!CompactLength::fit_content_keyword().uses_percentage());
        assert!(!CompactLength::stretch().uses_percentage());
        assert!(!CompactLength::content().uses_percentage());
        assert!(CompactLength::percent(0.5).uses_percentage());
        assert!(CompactLength::fit_content_percent(0.5).uses_percentage());
    }

    #[test]
    #[cfg(feature = "calc")]
    fn fit_content_calc_round_trips_pointer() {
        let boxed = Box::new(0u64);
        let ptr = CalcHandle::from_ptr(&*boxed as *const u64 as *const ());
        let value = CompactLength::fit_content_calc(ptr);
        assert!(value.is_fit_content_calc());
        assert!(value.has_calc());
        assert!(!value.is_plain_calc());
        assert!(value.is_fit_content());
        assert!(value.is_sizing_keyword());
        assert!(value.is_intrinsic());
        assert!(value.is_max_or_fit_content());
        assert!(value.is_max_content_alike());
        assert!(value.uses_percentage());
        assert!(!value.is_length_or_percentage());
        assert_eq!(value.calc_value(), ptr);

        let calc = CompactLength::calc(ptr);
        assert!(calc.is_plain_calc());
        assert!(calc.has_calc());
        assert!(!calc.is_fit_content_calc());
        assert!(!calc.is_fit_content());
        assert!(!calc.is_sizing_keyword());
    }

    #[test]
    fn calc_round_trips_pointer() {
        let boxed = Box::new(42u64);
        let ptr = CalcHandle::from_ptr(&*boxed as *const u64 as *const ());
        let value = CompactLength::calc(ptr);
        assert!(value.is_plain_calc());
        assert!(value.uses_percentage());
        assert!(!value.is_length_or_percentage());
        assert!(!value.is_auto());
        assert!(!value.is_intrinsic());
        assert_eq!(value.calc_value(), ptr);
    }

    #[test]
    #[cfg(feature = "calc")]
    fn calc_round_trips_64_bit_integer_handle() {
        let bits = (0xDEAD_BEEF_u64 << 32) | (0x1234_5678 << 3);
        let handle = CalcHandle::from_bits(bits);
        assert_eq!(handle.to_bits(), bits);

        let value = CompactLength::calc(handle);
        assert!(value.is_plain_calc());
        assert_eq!(value.calc_value(), handle);
        assert_eq!(value.calc_value().to_bits(), bits);

        let value = CompactLength::fit_content_calc(handle);
        assert!(value.is_fit_content_calc());
        assert_eq!(value.calc_value().to_bits(), bits);
    }

    #[test]
    #[cfg(feature = "calc")]
    #[allow(unsafe_code)]
    fn calc_pointer_handle_can_be_dereferenced() {
        let boxed = Box::new(42u64);
        let value = CompactLength::calc(CalcHandle::from_ptr(&*boxed as *const u64 as *const ()));
        // SAFETY: the handle was created from a pointer to `boxed`, which is still alive
        let read = unsafe { *(value.calc_value().ptr() as *const u64) };
        assert_eq!(read, 42);
    }
}
