//! Style types for controlling alignment.
//!
//! The public alignment types ([`AlignItems`], [`AlignContent`], and their aliases) are
//! structs with two orthogonal fields: a *position* keyword
//! ([`AlignItemsKeyword`] / [`AlignContentKeyword`]) and an *overflow-position*
//! modifier ([`AlignmentSafety`]). The pre-existing CSS spellings — `Start`, `End`,
//! `FlexStart`, `FlexEnd`, `Center`, `Stretch`, `SpaceBetween`, …, `SafeStart`,
//! `SafeEnd`, `SafeFlexStart`, `SafeFlexEnd`, `SafeCenter` — are exposed as associated
//! constants on the structs, so call sites read identically to the previous enum form.

#[cfg(feature = "parse")]
use crate::util::parse::{CssParseResult, FromCss, Parser, Token};

use crate::style::Direction;

/// The position-keyword half of [`AlignItems`] (and its aliases `AlignSelf`,
/// `JustifyItems`, `JustifySelf`).
///
/// Compute paths match on this enum directly so every match is exhaustive and
/// requires no `Safe*` siblings.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum AlignItemsKeyword {
    /// The default alignment of the layout mode (the initial value of `align-items` and
    /// `justify-items`).
    ///
    /// How `Normal` behaves depends on the layout mode: for flex items it behaves as
    /// `Stretch`; for grid items it behaves as `Stretch`, or as `Start` if the item has a
    /// preferred size or aspect ratio in the relevant axis; for block-level boxes it behaves
    /// as `Stretch` in the inline axis; and for absolutely positioned boxes it behaves either
    /// as `Stretch` or as `Start` depending on the box's insets and whether it is replaced.
    #[default]
    Normal,
    /// Items are packed toward the start of the axis.
    Start,
    /// Items are packed toward the end of the axis.
    End,
    /// Items are packed towards the flex-relative start of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is
    /// equivalent to End. In all other cases it is equivalent to Start.
    FlexStart,
    /// Items are packed towards the flex-relative end of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is
    /// equivalent to Start. In all other cases it is equivalent to End.
    FlexEnd,
    /// Items are packed toward the start of the axis as determined by the item's own
    /// writing mode/direction (rather than the container's).
    ///
    /// Equivalent to Start when the item's `direction` matches the container's, and
    /// to End when they differ (in the inline axis).
    SelfStart,
    /// Items are packed toward the end of the axis as determined by the item's own
    /// writing mode/direction (rather than the container's).
    ///
    /// Equivalent to End when the item's `direction` matches the container's, and
    /// to Start when they differ (in the inline axis).
    SelfEnd,
    /// Items are packed along the center of the cross axis.
    Center,
    /// Items are aligned such as their baselines align.
    Baseline,
    /// Stretch to fill the container.
    Stretch,
}

/// The position-keyword half of [`AlignContent`] (and its alias `JustifyContent`).
///
/// Compute paths match on this enum directly so every match is exhaustive and
/// requires no `Safe*` siblings.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum AlignContentKeyword {
    /// Items are packed toward the start of the axis.
    Start,
    /// Items are packed toward the end of the axis.
    End,
    /// Items are packed towards the flex-relative start of the axis.
    FlexStart,
    /// Items are packed towards the flex-relative end of the axis.
    FlexEnd,
    /// Items are centered around the middle of the axis.
    Center,
    /// Items are stretched to fill the container.
    Stretch,
    /// The first and last items are aligned flush with the edges of the container
    /// (no gap). The gap between items is distributed evenly.
    SpaceBetween,
    /// The gap between the first and last items is exactly THE SAME as the gap
    /// between items. The gaps are distributed evenly.
    SpaceEvenly,
    /// The gap between the first and last items is exactly HALF the gap between
    /// items. The gaps are distributed evenly in proportion to these ratios.
    SpaceAround,
}

impl AlignContentKeyword {
    /// Returns the reversed keyword for RTL (right-to-left) contexts: `Start`↔`End`,
    /// `FlexStart`↔`FlexEnd`. `Stretch` maps to `End` to preserve the layout
    /// algorithms' historical handling. Center and the distribution keywords
    /// (`SpaceBetween`, `SpaceEvenly`, `SpaceAround`) are unaffected because their
    /// visual placement is direction-symmetric.
    pub(crate) fn reversed(self) -> Self {
        match self {
            Self::Start => Self::End,
            Self::End => Self::Start,
            Self::FlexStart => Self::FlexEnd,
            Self::FlexEnd => Self::FlexStart,
            Self::Stretch => Self::End,
            Self::Center | Self::SpaceBetween | Self::SpaceEvenly | Self::SpaceAround => self,
        }
    }
}

/// The overflow-position modifier per [CSS Box Alignment §4.3][css-align-overflow].
///
/// `Safe` falls back to start-edge alignment when the alignment subject would
/// overflow the alignment container, so the start of the content stays visible.
/// `Unsafe` keeps the requested alignment even when that causes overflow at the
/// start edge. `Default` (no overflow-position keyword specified) behaves as
/// `Unsafe`, except for absolutely positioned boxes whose alignment is instead
/// adjusted to minimize overflow per [CSS Box Alignment §4.4.1.2][css-align-abspos].
///
/// CSS only defines `safe` / `unsafe` against the position values `start`, `end`,
/// `flex-start`, `flex-end`, `center`. The struct shape does not enforce that
/// constraint at the type level — the parser rejects invalid combinations, and
/// the compute pass treats `Safe` paired with a non-position keyword (`Stretch`,
/// `Baseline`, `Space*`) the same as `Unsafe`.
///
/// [css-align-overflow]: https://www.w3.org/TR/css-align-3/#overflow-values
/// [css-align-abspos]: https://www.w3.org/TR/css-align-3/#auto-safety-position
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum AlignmentSafety {
    /// No overflow-position keyword specified. Behaves as `Unsafe`, except for absolutely
    /// positioned boxes, whose alignment is adjusted to minimize overflow of their
    /// containing block.
    #[default]
    Default,
    /// Keeps the requested alignment even when the subject overflows the alignment
    /// container at the start edge.
    Unsafe,
    /// Falls back to the start edge when the subject would overflow, to avoid data
    /// loss.
    Safe,
}

/// Used to control how child nodes are aligned.
/// For Flexbox it controls alignment in the cross axis.
/// For Grid it controls alignment in the block axis.
///
/// The default value is [`AlignItems::NORMAL`]. This type is also used for the `*-self`
/// properties (see [`AlignSelf`]), which are an `Option<AlignSelf>` where `None` represents
/// the CSS `auto` keyword (defer to the parent's `align-items` / `justify-items`).
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-items)
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct AlignItems {
    /// Position keyword.
    pub keyword: AlignItemsKeyword,
    /// Overflow-position modifier (`safe` / `unsafe`).
    pub safety: AlignmentSafety,
}

impl AlignItems {
    /// The default alignment of the layout mode. This is the default value of `align-items`
    /// and `justify-items`. See [`AlignItemsKeyword::Normal`].
    pub const NORMAL: Self = Self { keyword: AlignItemsKeyword::Normal, safety: AlignmentSafety::Default };
    /// Items are packed toward the start of the axis.
    pub const START: Self = Self { keyword: AlignItemsKeyword::Start, safety: AlignmentSafety::Default };
    /// Items are packed toward the end of the axis.
    pub const END: Self = Self { keyword: AlignItemsKeyword::End, safety: AlignmentSafety::Default };
    /// Items are packed towards the flex-relative start of the axis.
    pub const FLEX_START: Self = Self { keyword: AlignItemsKeyword::FlexStart, safety: AlignmentSafety::Default };
    /// Items are packed towards the flex-relative end of the axis.
    pub const FLEX_END: Self = Self { keyword: AlignItemsKeyword::FlexEnd, safety: AlignmentSafety::Default };
    /// Items are packed toward the start of the axis as determined by the item's own direction.
    pub const SELF_START: Self = Self { keyword: AlignItemsKeyword::SelfStart, safety: AlignmentSafety::Default };
    /// Items are packed toward the end of the axis as determined by the item's own direction.
    pub const SELF_END: Self = Self { keyword: AlignItemsKeyword::SelfEnd, safety: AlignmentSafety::Default };
    /// Items are packed along the center of the cross axis.
    pub const CENTER: Self = Self { keyword: AlignItemsKeyword::Center, safety: AlignmentSafety::Default };
    /// Items are aligned such as their baselines align.
    pub const BASELINE: Self = Self { keyword: AlignItemsKeyword::Baseline, safety: AlignmentSafety::Default };
    /// Stretch to fill the container.
    pub const STRETCH: Self = Self { keyword: AlignItemsKeyword::Stretch, safety: AlignmentSafety::Default };
    /// Like [`AlignItems::START`], but falls back to [`AlignItems::START`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    pub const SAFE_START: Self = Self { keyword: AlignItemsKeyword::Start, safety: AlignmentSafety::Safe };
    /// Like [`AlignItems::END`], but falls back to [`AlignItems::START`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    pub const SAFE_END: Self = Self { keyword: AlignItemsKeyword::End, safety: AlignmentSafety::Safe };
    /// Like [`AlignItems::FLEX_START`], but falls back to [`AlignItems::START`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    pub const SAFE_FLEX_START: Self = Self { keyword: AlignItemsKeyword::FlexStart, safety: AlignmentSafety::Safe };
    /// Like [`AlignItems::FLEX_END`], but falls back to [`AlignItems::START`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    pub const SAFE_FLEX_END: Self = Self { keyword: AlignItemsKeyword::FlexEnd, safety: AlignmentSafety::Safe };
    /// Like [`AlignItems::CENTER`], but falls back to [`AlignItems::START`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    pub const SAFE_CENTER: Self = Self { keyword: AlignItemsKeyword::Center, safety: AlignmentSafety::Safe };
    /// Like [`AlignItems::SELF_START`], but falls back to [`AlignItems::START`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    pub const SAFE_SELF_START: Self = Self { keyword: AlignItemsKeyword::SelfStart, safety: AlignmentSafety::Safe };
    /// Like [`AlignItems::SELF_END`], but falls back to [`AlignItems::START`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    pub const SAFE_SELF_END: Self = Self { keyword: AlignItemsKeyword::SelfEnd, safety: AlignmentSafety::Safe };

    /// Returns `true` iff this carries the `safe` overflow-position modifier.
    #[inline]
    pub const fn is_safe(self) -> bool {
        matches!(self.safety, AlignmentSafety::Safe)
    }

    /// Returns this alignment with the given overflow-position modifier.
    #[inline]
    pub const fn with_safety(self, safety: AlignmentSafety) -> Self {
        Self { keyword: self.keyword, safety }
    }

    /// Returns the underlying position keyword, discarding the safety modifier.
    #[inline]
    pub const fn keyword(self) -> AlignItemsKeyword {
        self.keyword
    }

    /// Returns true if the keyword is `Stretch` or `Normal` (ignoring any overflow-position
    /// modifier).
    #[inline]
    pub const fn is_stretch_or_normal(self) -> bool {
        matches!(self.keyword, AlignItemsKeyword::Stretch | AlignItemsKeyword::Normal)
    }

    /// Resolve the writing-mode-relative `SelfStart`/`SelfEnd` keywords to `Start`/`End`
    /// based on the item's own `direction` per CSS Box Alignment §5.2
    /// <https://www.w3.org/TR/css-align-3/#self-alignment>. All other keywords
    /// (including `Normal`) are returned unchanged.
    ///
    /// The `Start`/`End` keywords used by the compute paths are relative to the
    /// *container's* writing mode/direction, so in the inline axis `SelfStart` resolves
    /// to `Start` when the item's direction matches the container's and to `End` when it
    /// differs. Taffy only supports the `horizontal-tb` writing mode, so in the block
    /// axis `SelfStart`/`SelfEnd` always resolve to `Start`/`End` respectively.
    #[inline]
    pub fn resolve_self_relative(
        self,
        item_direction: Direction,
        container_direction: Direction,
        axis_is_inline: bool,
    ) -> Self {
        let flip = axis_is_inline && item_direction != container_direction;
        let keyword = match self.keyword {
            AlignItemsKeyword::SelfStart => {
                if flip {
                    AlignItemsKeyword::End
                } else {
                    AlignItemsKeyword::Start
                }
            }
            AlignItemsKeyword::SelfEnd => {
                if flip {
                    AlignItemsKeyword::Start
                } else {
                    AlignItemsKeyword::End
                }
            }
            other => other,
        };
        Self { keyword, safety: self.safety }
    }
}

#[cfg(feature = "parse")]
impl FromCss for AlignItems {
    fn from_css<'i>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        let first = input.expect_ident()?.clone();
        cssparser::match_ignore_ascii_case! { &*first,
            "safe" => {
                let pos = input.expect_ident()?.clone();
                cssparser::match_ignore_ascii_case! { &*pos,
                    "start" => Ok(Self::SAFE_START),
                    "end" => Ok(Self::SAFE_END),
                    "flex-start" => Ok(Self::SAFE_FLEX_START),
                    "flex-end" => Ok(Self::SAFE_FLEX_END),
                    "self-start" => Ok(Self::SAFE_SELF_START),
                    "self-end" => Ok(Self::SAFE_SELF_END),
                    "center" => Ok(Self::SAFE_CENTER),
                    _ => Err(input.new_unexpected_token_error(Token::Ident(pos))),
                }
            },
            "unsafe" => {
                let pos = input.expect_ident()?.clone();
                let keyword = cssparser::match_ignore_ascii_case! { &*pos,
                    "start" => AlignItemsKeyword::Start,
                    "end" => AlignItemsKeyword::End,
                    "flex-start" => AlignItemsKeyword::FlexStart,
                    "flex-end" => AlignItemsKeyword::FlexEnd,
                    "self-start" => AlignItemsKeyword::SelfStart,
                    "self-end" => AlignItemsKeyword::SelfEnd,
                    "center" => AlignItemsKeyword::Center,
                    _ => return Err(input.new_unexpected_token_error(Token::Ident(pos))),
                };
                Ok(Self { keyword, safety: AlignmentSafety::Unsafe })
            },
            "normal" => Ok(Self::NORMAL),
            "start" => Ok(Self::START),
            "end" => Ok(Self::END),
            "flex-start" => Ok(Self::FLEX_START),
            "flex-end" => Ok(Self::FLEX_END),
            "self-start" => Ok(Self::SELF_START),
            "self-end" => Ok(Self::SELF_END),
            "center" => Ok(Self::CENTER),
            "baseline" => Ok(Self::BASELINE),
            "stretch" => Ok(Self::STRETCH),
            _ => Err(input.new_unexpected_token_error(Token::Ident(first))),
        }
    }
}

#[cfg(feature = "parse")]
crate::util::parse::from_str_from_css!(AlignItems);

/// Used to control how child nodes are aligned.
/// Does not apply to Flexbox, and will be ignored if specified on a flex container.
/// For Grid it controls alignment in the inline axis.
///
/// The default value is [`AlignItems::NORMAL`].
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items)
pub type JustifyItems = AlignItems;
/// Controls alignment of an individual node.
///
/// Overrides the parent Node's `AlignItems` property.
/// For Flexbox it controls alignment in the cross axis.
/// For Grid it controls alignment in the block axis.
///
/// The `align-self` style is an `Option<AlignSelf>`. The default value is `None`, which
/// represents the CSS `auto` keyword and defers to the parent Node's `AlignItems` property.
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-self)
pub type AlignSelf = AlignItems;
/// Controls alignment of an individual node.
///
/// Overrides the parent Node's `JustifyItems` property.
/// Does not apply to Flexbox, and will be ignored if specified on a flex child.
/// For Grid it controls alignment in the inline axis.
///
/// The `justify-self` style is an `Option<JustifySelf>`. The default value is `None`, which
/// represents the CSS `auto` keyword and defers to the parent Node's `JustifyItems` property.
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self)
pub type JustifySelf = AlignItems;

/// Sets the distribution of space between and around content items.
/// For Flexbox it controls alignment in the cross axis.
/// For Grid it controls alignment in the block axis.
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-content)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AlignContent {
    /// Position keyword.
    pub keyword: AlignContentKeyword,
    /// Overflow-position modifier (`safe` / `unsafe`).
    pub safety: AlignmentSafety,
}

impl AlignContent {
    /// Items are packed toward the start of the axis.
    pub const START: Self = Self { keyword: AlignContentKeyword::Start, safety: AlignmentSafety::Default };
    /// Items are packed toward the end of the axis.
    pub const END: Self = Self { keyword: AlignContentKeyword::End, safety: AlignmentSafety::Default };
    /// Items are packed towards the flex-relative start of the axis.
    pub const FLEX_START: Self = Self { keyword: AlignContentKeyword::FlexStart, safety: AlignmentSafety::Default };
    /// Items are packed towards the flex-relative end of the axis.
    pub const FLEX_END: Self = Self { keyword: AlignContentKeyword::FlexEnd, safety: AlignmentSafety::Default };
    /// Items are centered around the middle of the axis.
    pub const CENTER: Self = Self { keyword: AlignContentKeyword::Center, safety: AlignmentSafety::Default };
    /// Items are stretched to fill the container.
    pub const STRETCH: Self = Self { keyword: AlignContentKeyword::Stretch, safety: AlignmentSafety::Default };
    /// The first and last items are aligned flush with the edges of the container.
    pub const SPACE_BETWEEN: Self =
        Self { keyword: AlignContentKeyword::SpaceBetween, safety: AlignmentSafety::Default };
    /// The gap between the first and last items equals the gap between items.
    pub const SPACE_EVENLY: Self = Self { keyword: AlignContentKeyword::SpaceEvenly, safety: AlignmentSafety::Default };
    /// The gap between the first and last items is half the gap between items.
    pub const SPACE_AROUND: Self = Self { keyword: AlignContentKeyword::SpaceAround, safety: AlignmentSafety::Default };
    /// Like [`AlignContent::START`], but falls back to [`AlignContent::START`] when the
    /// content overflows the alignment container, to avoid data loss.
    pub const SAFE_START: Self = Self { keyword: AlignContentKeyword::Start, safety: AlignmentSafety::Safe };
    /// Like [`AlignContent::END`], but falls back to [`AlignContent::START`] when the
    /// content overflows the alignment container, to avoid data loss.
    pub const SAFE_END: Self = Self { keyword: AlignContentKeyword::End, safety: AlignmentSafety::Safe };
    /// Like [`AlignContent::FLEX_START`], but falls back to [`AlignContent::START`] when
    /// the content overflows the alignment container, to avoid data loss.
    pub const SAFE_FLEX_START: Self = Self { keyword: AlignContentKeyword::FlexStart, safety: AlignmentSafety::Safe };
    /// Like [`AlignContent::FLEX_END`], but falls back to [`AlignContent::START`] when the
    /// content overflows the alignment container, to avoid data loss.
    pub const SAFE_FLEX_END: Self = Self { keyword: AlignContentKeyword::FlexEnd, safety: AlignmentSafety::Safe };
    /// Like [`AlignContent::CENTER`], but falls back to [`AlignContent::START`] when the
    /// content overflows the alignment container, to avoid data loss.
    pub const SAFE_CENTER: Self = Self { keyword: AlignContentKeyword::Center, safety: AlignmentSafety::Safe };

    /// Returns `true` iff this carries the `safe` overflow-position modifier.
    #[inline]
    pub const fn is_safe(self) -> bool {
        matches!(self.safety, AlignmentSafety::Safe)
    }

    /// Returns this alignment with the given overflow-position modifier.
    #[inline]
    pub const fn with_safety(self, safety: AlignmentSafety) -> Self {
        Self { keyword: self.keyword, safety }
    }

    /// Returns the underlying position keyword, discarding the safety modifier.
    #[inline]
    pub const fn keyword(self) -> AlignContentKeyword {
        self.keyword
    }
}

#[cfg(feature = "parse")]
impl FromCss for AlignContent {
    fn from_css<'i>(input: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        let first = input.expect_ident()?.clone();
        cssparser::match_ignore_ascii_case! { &*first,
            "safe" => {
                let pos = input.expect_ident()?.clone();
                cssparser::match_ignore_ascii_case! { &*pos,
                    "start" => Ok(Self::SAFE_START),
                    "end" => Ok(Self::SAFE_END),
                    "flex-start" => Ok(Self::SAFE_FLEX_START),
                    "flex-end" => Ok(Self::SAFE_FLEX_END),
                    "center" => Ok(Self::SAFE_CENTER),
                    _ => Err(input.new_unexpected_token_error(Token::Ident(pos))),
                }
            },
            "unsafe" => {
                let pos = input.expect_ident()?.clone();
                let keyword = cssparser::match_ignore_ascii_case! { &*pos,
                    "start" => AlignContentKeyword::Start,
                    "end" => AlignContentKeyword::End,
                    "flex-start" => AlignContentKeyword::FlexStart,
                    "flex-end" => AlignContentKeyword::FlexEnd,
                    "center" => AlignContentKeyword::Center,
                    _ => return Err(input.new_unexpected_token_error(Token::Ident(pos))),
                };
                Ok(Self { keyword, safety: AlignmentSafety::Unsafe })
            },
            "start" => Ok(Self::START),
            "end" => Ok(Self::END),
            "flex-start" => Ok(Self::FLEX_START),
            "flex-end" => Ok(Self::FLEX_END),
            "center" => Ok(Self::CENTER),
            "stretch" => Ok(Self::STRETCH),
            "space-between" => Ok(Self::SPACE_BETWEEN),
            "space-evenly" => Ok(Self::SPACE_EVENLY),
            "space-around" => Ok(Self::SPACE_AROUND),
            _ => Err(input.new_unexpected_token_error(Token::Ident(first))),
        }
    }
}

#[cfg(feature = "parse")]
crate::util::parse::from_str_from_css!(AlignContent);

/// Sets the distribution of space between and around content items.
/// For Flexbox it controls alignment in the main axis.
/// For Grid it controls alignment in the inline axis.
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content)
pub type JustifyContent = AlignContent;

// ---------------------------------------------------------------------------
// Serde — custom impls preserve the pre-struct wire format (single tag string
// per public spelling) so consumers reading data serialized before the refactor
// continue to deserialize correctly.
// ---------------------------------------------------------------------------

/// Canonical tag-string set accepted by [`AlignItems`] serde deserialization, used in
/// `unknown_variant` errors. Mirrors the spellings produced by `Serialize`.
#[cfg(feature = "serde")]
const ALIGN_ITEMS_NAMES: &[&str] = &[
    "Normal",
    "Start",
    "End",
    "FlexStart",
    "FlexEnd",
    "SelfStart",
    "SelfEnd",
    "Center",
    "Baseline",
    "Stretch",
    "SafeStart",
    "SafeEnd",
    "SafeFlexStart",
    "SafeFlexEnd",
    "SafeSelfStart",
    "SafeSelfEnd",
    "SafeCenter",
    "UnsafeStart",
    "UnsafeEnd",
    "UnsafeFlexStart",
    "UnsafeFlexEnd",
    "UnsafeSelfStart",
    "UnsafeSelfEnd",
    "UnsafeCenter",
];

#[cfg(feature = "serde")]
impl serde::Serialize for AlignItems {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let name = match (self.keyword, self.safety) {
            (AlignItemsKeyword::Start, AlignmentSafety::Default) => "Start",
            (AlignItemsKeyword::End, AlignmentSafety::Default) => "End",
            (AlignItemsKeyword::FlexStart, AlignmentSafety::Default) => "FlexStart",
            (AlignItemsKeyword::FlexEnd, AlignmentSafety::Default) => "FlexEnd",
            (AlignItemsKeyword::SelfStart, AlignmentSafety::Default) => "SelfStart",
            (AlignItemsKeyword::SelfEnd, AlignmentSafety::Default) => "SelfEnd",
            (AlignItemsKeyword::Center, AlignmentSafety::Default) => "Center",
            (AlignItemsKeyword::Start, AlignmentSafety::Unsafe) => "UnsafeStart",
            (AlignItemsKeyword::End, AlignmentSafety::Unsafe) => "UnsafeEnd",
            (AlignItemsKeyword::FlexStart, AlignmentSafety::Unsafe) => "UnsafeFlexStart",
            (AlignItemsKeyword::FlexEnd, AlignmentSafety::Unsafe) => "UnsafeFlexEnd",
            (AlignItemsKeyword::SelfStart, AlignmentSafety::Unsafe) => "UnsafeSelfStart",
            (AlignItemsKeyword::SelfEnd, AlignmentSafety::Unsafe) => "UnsafeSelfEnd",
            (AlignItemsKeyword::Center, AlignmentSafety::Unsafe) => "UnsafeCenter",
            (AlignItemsKeyword::Normal, _) => "Normal",
            (AlignItemsKeyword::Baseline, _) => "Baseline",
            (AlignItemsKeyword::Stretch, _) => "Stretch",
            (AlignItemsKeyword::Start, AlignmentSafety::Safe) => "SafeStart",
            (AlignItemsKeyword::End, AlignmentSafety::Safe) => "SafeEnd",
            (AlignItemsKeyword::FlexStart, AlignmentSafety::Safe) => "SafeFlexStart",
            (AlignItemsKeyword::FlexEnd, AlignmentSafety::Safe) => "SafeFlexEnd",
            (AlignItemsKeyword::SelfStart, AlignmentSafety::Safe) => "SafeSelfStart",
            (AlignItemsKeyword::SelfEnd, AlignmentSafety::Safe) => "SafeSelfEnd",
            (AlignItemsKeyword::Center, AlignmentSafety::Safe) => "SafeCenter",
        };
        serializer.serialize_str(name)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AlignItems {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct AlignItemsVisitor;
        impl<'de> serde::de::Visitor<'de> for AlignItemsVisitor {
            type Value = AlignItems;
            fn expecting(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_str("an AlignItems variant tag string")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Normal" => AlignItems::NORMAL,
                    "Start" => AlignItems::START,
                    "End" => AlignItems::END,
                    "FlexStart" => AlignItems::FLEX_START,
                    "FlexEnd" => AlignItems::FLEX_END,
                    "SelfStart" => AlignItems::SELF_START,
                    "SelfEnd" => AlignItems::SELF_END,
                    "Center" => AlignItems::CENTER,
                    "Baseline" => AlignItems::BASELINE,
                    "Stretch" => AlignItems::STRETCH,
                    "SafeStart" => AlignItems::SAFE_START,
                    "SafeEnd" => AlignItems::SAFE_END,
                    "SafeFlexStart" => AlignItems::SAFE_FLEX_START,
                    "SafeFlexEnd" => AlignItems::SAFE_FLEX_END,
                    "SafeSelfStart" => AlignItems::SAFE_SELF_START,
                    "SafeSelfEnd" => AlignItems::SAFE_SELF_END,
                    "SafeCenter" => AlignItems::SAFE_CENTER,
                    "UnsafeStart" => AlignItems::START.with_safety(AlignmentSafety::Unsafe),
                    "UnsafeEnd" => AlignItems::END.with_safety(AlignmentSafety::Unsafe),
                    "UnsafeFlexStart" => AlignItems::FLEX_START.with_safety(AlignmentSafety::Unsafe),
                    "UnsafeFlexEnd" => AlignItems::FLEX_END.with_safety(AlignmentSafety::Unsafe),
                    "UnsafeSelfStart" => AlignItems::SELF_START.with_safety(AlignmentSafety::Unsafe),
                    "UnsafeSelfEnd" => AlignItems::SELF_END.with_safety(AlignmentSafety::Unsafe),
                    "UnsafeCenter" => AlignItems::CENTER.with_safety(AlignmentSafety::Unsafe),
                    other => return Err(E::unknown_variant(other, ALIGN_ITEMS_NAMES)),
                })
            }
        }
        deserializer.deserialize_str(AlignItemsVisitor)
    }
}

/// Canonical tag-string set accepted by [`AlignContent`] serde deserialization, used in
/// `unknown_variant` errors. Mirrors the spellings produced by `Serialize`.
#[cfg(feature = "serde")]
const ALIGN_CONTENT_NAMES: &[&str] = &[
    "Start",
    "End",
    "FlexStart",
    "FlexEnd",
    "Center",
    "Stretch",
    "SpaceBetween",
    "SpaceEvenly",
    "SpaceAround",
    "SafeStart",
    "SafeEnd",
    "SafeFlexStart",
    "SafeFlexEnd",
    "SafeCenter",
    "UnsafeStart",
    "UnsafeEnd",
    "UnsafeFlexStart",
    "UnsafeFlexEnd",
    "UnsafeCenter",
];

#[cfg(feature = "serde")]
impl serde::Serialize for AlignContent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let name = match (self.keyword, self.safety) {
            (AlignContentKeyword::Start, AlignmentSafety::Default) => "Start",
            (AlignContentKeyword::End, AlignmentSafety::Default) => "End",
            (AlignContentKeyword::FlexStart, AlignmentSafety::Default) => "FlexStart",
            (AlignContentKeyword::FlexEnd, AlignmentSafety::Default) => "FlexEnd",
            (AlignContentKeyword::Center, AlignmentSafety::Default) => "Center",
            (AlignContentKeyword::Start, AlignmentSafety::Unsafe) => "UnsafeStart",
            (AlignContentKeyword::End, AlignmentSafety::Unsafe) => "UnsafeEnd",
            (AlignContentKeyword::FlexStart, AlignmentSafety::Unsafe) => "UnsafeFlexStart",
            (AlignContentKeyword::FlexEnd, AlignmentSafety::Unsafe) => "UnsafeFlexEnd",
            (AlignContentKeyword::Center, AlignmentSafety::Unsafe) => "UnsafeCenter",
            (AlignContentKeyword::Stretch, _) => "Stretch",
            (AlignContentKeyword::SpaceBetween, _) => "SpaceBetween",
            (AlignContentKeyword::SpaceEvenly, _) => "SpaceEvenly",
            (AlignContentKeyword::SpaceAround, _) => "SpaceAround",
            (AlignContentKeyword::Start, AlignmentSafety::Safe) => "SafeStart",
            (AlignContentKeyword::End, AlignmentSafety::Safe) => "SafeEnd",
            (AlignContentKeyword::FlexStart, AlignmentSafety::Safe) => "SafeFlexStart",
            (AlignContentKeyword::FlexEnd, AlignmentSafety::Safe) => "SafeFlexEnd",
            (AlignContentKeyword::Center, AlignmentSafety::Safe) => "SafeCenter",
        };
        serializer.serialize_str(name)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AlignContent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct AlignContentVisitor;
        impl<'de> serde::de::Visitor<'de> for AlignContentVisitor {
            type Value = AlignContent;
            fn expecting(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_str("an AlignContent variant tag string")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Start" => AlignContent::START,
                    "End" => AlignContent::END,
                    "FlexStart" => AlignContent::FLEX_START,
                    "FlexEnd" => AlignContent::FLEX_END,
                    "Center" => AlignContent::CENTER,
                    "Stretch" => AlignContent::STRETCH,
                    "SpaceBetween" => AlignContent::SPACE_BETWEEN,
                    "SpaceEvenly" => AlignContent::SPACE_EVENLY,
                    "SpaceAround" => AlignContent::SPACE_AROUND,
                    "SafeStart" => AlignContent::SAFE_START,
                    "SafeEnd" => AlignContent::SAFE_END,
                    "SafeFlexStart" => AlignContent::SAFE_FLEX_START,
                    "SafeFlexEnd" => AlignContent::SAFE_FLEX_END,
                    "SafeCenter" => AlignContent::SAFE_CENTER,
                    "UnsafeStart" => AlignContent::START.with_safety(AlignmentSafety::Unsafe),
                    "UnsafeEnd" => AlignContent::END.with_safety(AlignmentSafety::Unsafe),
                    "UnsafeFlexStart" => AlignContent::FLEX_START.with_safety(AlignmentSafety::Unsafe),
                    "UnsafeFlexEnd" => AlignContent::FLEX_END.with_safety(AlignmentSafety::Unsafe),
                    "UnsafeCenter" => AlignContent::CENTER.with_safety(AlignmentSafety::Unsafe),
                    other => return Err(E::unknown_variant(other, ALIGN_CONTENT_NAMES)),
                })
            }
        }
        deserializer.deserialize_str(AlignContentVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    // Size budget — struct = 1B keyword + 1B safety, no niche packing.
    // Pre-refactor each was a single-byte enum; spec §V13 caps regression at +2B.
    #[test]
    fn align_types_within_size_budget() {
        assert!(size_of::<AlignItems>() <= 2, "AlignItems grew to {}", size_of::<AlignItems>());
        assert!(size_of::<AlignContent>() <= 2, "AlignContent grew to {}", size_of::<AlignContent>());
        assert!(size_of::<Option<AlignContent>>() <= 3);
    }

    #[test]
    fn align_items_is_safe() {
        assert!(AlignItems::SAFE_START.is_safe());
        assert!(AlignItems::SAFE_END.is_safe());
        assert!(AlignItems::SAFE_FLEX_START.is_safe());
        assert!(AlignItems::SAFE_FLEX_END.is_safe());
        assert!(AlignItems::SAFE_CENTER.is_safe());
        assert!(!AlignItems::START.is_safe());
        assert!(!AlignItems::END.is_safe());
        assert!(!AlignItems::FLEX_START.is_safe());
        assert!(!AlignItems::FLEX_END.is_safe());
        assert!(!AlignItems::CENTER.is_safe());
        assert!(!AlignItems::BASELINE.is_safe());
        assert!(!AlignItems::STRETCH.is_safe());
        assert!(AlignItems::SAFE_SELF_START.is_safe());
        assert!(AlignItems::SAFE_SELF_END.is_safe());
        assert!(!AlignItems::SELF_START.is_safe());
        assert!(!AlignItems::SELF_END.is_safe());
        assert!(!AlignItems::NORMAL.is_safe());
    }

    #[test]
    fn align_items_keyword_strips_safe() {
        assert_eq!(AlignItems::SAFE_START.keyword(), AlignItemsKeyword::Start);
        assert_eq!(AlignItems::SAFE_END.keyword(), AlignItemsKeyword::End);
        assert_eq!(AlignItems::SAFE_FLEX_START.keyword(), AlignItemsKeyword::FlexStart);
        assert_eq!(AlignItems::SAFE_FLEX_END.keyword(), AlignItemsKeyword::FlexEnd);
        assert_eq!(AlignItems::SAFE_CENTER.keyword(), AlignItemsKeyword::Center);
        assert_eq!(AlignItems::SAFE_SELF_START.keyword(), AlignItemsKeyword::SelfStart);
        assert_eq!(AlignItems::SAFE_SELF_END.keyword(), AlignItemsKeyword::SelfEnd);
    }

    #[test]
    fn align_items_keyword_passthrough() {
        assert_eq!(AlignItems::START.keyword(), AlignItemsKeyword::Start);
        assert_eq!(AlignItems::STRETCH.keyword(), AlignItemsKeyword::Stretch);
        assert_eq!(AlignItems::BASELINE.keyword(), AlignItemsKeyword::Baseline);
        assert_eq!(AlignItems::FLEX_START.keyword(), AlignItemsKeyword::FlexStart);
        assert_eq!(AlignItems::NORMAL.keyword(), AlignItemsKeyword::Normal);
    }

    #[test]
    fn align_items_default_is_normal() {
        assert_eq!(AlignItems::default(), AlignItems::NORMAL);
        assert_eq!(AlignItemsKeyword::default(), AlignItemsKeyword::Normal);
        assert_eq!(AlignmentSafety::default(), AlignmentSafety::Default);
    }

    #[test]
    fn resolve_self_relative_inline_axis() {
        use Direction::{Ltr, Rtl};
        // Same direction: self-start == start, self-end == end
        assert_eq!(AlignItems::SELF_START.resolve_self_relative(Ltr, Ltr, true), AlignItems::START);
        assert_eq!(AlignItems::SELF_END.resolve_self_relative(Ltr, Ltr, true), AlignItems::END);
        assert_eq!(AlignItems::SELF_START.resolve_self_relative(Rtl, Rtl, true), AlignItems::START);
        assert_eq!(AlignItems::SELF_END.resolve_self_relative(Rtl, Rtl, true), AlignItems::END);
        // Opposite direction: self-start == end, self-end == start
        assert_eq!(AlignItems::SELF_START.resolve_self_relative(Ltr, Rtl, true), AlignItems::END);
        assert_eq!(AlignItems::SELF_END.resolve_self_relative(Ltr, Rtl, true), AlignItems::START);
        assert_eq!(AlignItems::SELF_START.resolve_self_relative(Rtl, Ltr, true), AlignItems::END);
        assert_eq!(AlignItems::SELF_END.resolve_self_relative(Rtl, Ltr, true), AlignItems::START);
        // Safety modifier is preserved
        assert_eq!(AlignItems::SAFE_SELF_START.resolve_self_relative(Ltr, Rtl, true), AlignItems::SAFE_END);
        // Other keywords are unchanged
        assert_eq!(AlignItems::START.resolve_self_relative(Ltr, Rtl, true), AlignItems::START);
        assert_eq!(AlignItems::FLEX_END.resolve_self_relative(Ltr, Rtl, true), AlignItems::FLEX_END);
        assert_eq!(AlignItems::NORMAL.resolve_self_relative(Ltr, Rtl, true), AlignItems::NORMAL);
    }

    #[test]
    fn resolve_self_relative_block_axis() {
        use Direction::{Ltr, Rtl};
        // In the block axis (horizontal-tb only) direction never flips self-start/self-end
        assert_eq!(AlignItems::SELF_START.resolve_self_relative(Ltr, Rtl, false), AlignItems::START);
        assert_eq!(AlignItems::SELF_END.resolve_self_relative(Rtl, Ltr, false), AlignItems::END);
    }

    #[test]
    fn align_content_is_safe() {
        assert!(AlignContent::SAFE_START.is_safe());
        assert!(AlignContent::SAFE_CENTER.is_safe());
        assert!(!AlignContent::SPACE_BETWEEN.is_safe());
        assert!(!AlignContent::STRETCH.is_safe());
    }

    #[test]
    fn align_content_keyword_strips_safe() {
        assert_eq!(AlignContent::SAFE_START.keyword(), AlignContentKeyword::Start);
        assert_eq!(AlignContent::SAFE_FLEX_END.keyword(), AlignContentKeyword::FlexEnd);
        assert_eq!(AlignContent::SAFE_CENTER.keyword(), AlignContentKeyword::Center);
        assert_eq!(AlignContent::SPACE_BETWEEN.keyword(), AlignContentKeyword::SpaceBetween);
    }

    #[test]
    fn align_content_keyword_reversed_swaps_start_end() {
        assert_eq!(AlignContentKeyword::Start.reversed(), AlignContentKeyword::End);
        assert_eq!(AlignContentKeyword::End.reversed(), AlignContentKeyword::Start);
        assert_eq!(AlignContentKeyword::FlexStart.reversed(), AlignContentKeyword::FlexEnd);
        assert_eq!(AlignContentKeyword::FlexEnd.reversed(), AlignContentKeyword::FlexStart);
        // Stretch reverses to End — preserves pre-refactor behaviour.
        assert_eq!(AlignContentKeyword::Stretch.reversed(), AlignContentKeyword::End);
        assert_eq!(AlignContentKeyword::Center.reversed(), AlignContentKeyword::Center);
        assert_eq!(AlignContentKeyword::SpaceBetween.reversed(), AlignContentKeyword::SpaceBetween);
        assert_eq!(AlignContentKeyword::SpaceEvenly.reversed(), AlignContentKeyword::SpaceEvenly);
        assert_eq!(AlignContentKeyword::SpaceAround.reversed(), AlignContentKeyword::SpaceAround);
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_items_plain() {
        assert_eq!("start".parse::<AlignItems>().unwrap(), AlignItems::START);
        assert_eq!("end".parse::<AlignItems>().unwrap(), AlignItems::END);
        assert_eq!("flex-start".parse::<AlignItems>().unwrap(), AlignItems::FLEX_START);
        assert_eq!("flex-end".parse::<AlignItems>().unwrap(), AlignItems::FLEX_END);
        assert_eq!("center".parse::<AlignItems>().unwrap(), AlignItems::CENTER);
        assert_eq!("self-start".parse::<AlignItems>().unwrap(), AlignItems::SELF_START);
        assert_eq!("self-end".parse::<AlignItems>().unwrap(), AlignItems::SELF_END);
        assert_eq!("baseline".parse::<AlignItems>().unwrap(), AlignItems::BASELINE);
        assert_eq!("stretch".parse::<AlignItems>().unwrap(), AlignItems::STRETCH);
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_items_normal() {
        assert_eq!("normal".parse::<AlignItems>().unwrap(), AlignItems::NORMAL);
        assert_eq!("NORMAL".parse::<AlignItems>().unwrap(), AlignItems::NORMAL);
        assert_eq!(AlignItems::NORMAL.safety, AlignmentSafety::Default);
        // `auto` is not an `AlignItems` keyword: it is represented by `None` on the `*-self` styles
        assert!("auto".parse::<AlignSelf>().is_err());
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_items_safe() {
        assert_eq!("safe start".parse::<AlignItems>().unwrap(), AlignItems::SAFE_START);
        assert_eq!("safe end".parse::<AlignItems>().unwrap(), AlignItems::SAFE_END);
        assert_eq!("safe flex-start".parse::<AlignItems>().unwrap(), AlignItems::SAFE_FLEX_START);
        assert_eq!("safe flex-end".parse::<AlignItems>().unwrap(), AlignItems::SAFE_FLEX_END);
        assert_eq!("safe self-start".parse::<AlignItems>().unwrap(), AlignItems::SAFE_SELF_START);
        assert_eq!("safe self-end".parse::<AlignItems>().unwrap(), AlignItems::SAFE_SELF_END);
        assert_eq!("safe center".parse::<AlignItems>().unwrap(), AlignItems::SAFE_CENTER);
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_items_safe_case_insensitive() {
        assert_eq!("SAFE Start".parse::<AlignItems>().unwrap(), AlignItems::SAFE_START);
        assert_eq!("Safe FLEX-end".parse::<AlignItems>().unwrap(), AlignItems::SAFE_FLEX_END);
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_items_unsafe() {
        let unsafe_ = |keyword| AlignItems { keyword, safety: AlignmentSafety::Unsafe };
        assert_eq!("unsafe start".parse::<AlignItems>().unwrap(), unsafe_(AlignItemsKeyword::Start));
        assert_eq!("unsafe end".parse::<AlignItems>().unwrap(), unsafe_(AlignItemsKeyword::End));
        assert_eq!("unsafe self-start".parse::<AlignItems>().unwrap(), unsafe_(AlignItemsKeyword::SelfStart));
        assert_eq!("unsafe self-end".parse::<AlignItems>().unwrap(), unsafe_(AlignItemsKeyword::SelfEnd));
        assert_eq!("unsafe center".parse::<AlignItems>().unwrap(), unsafe_(AlignItemsKeyword::Center));
        // The bare keyword carries no overflow-position modifier
        assert_eq!(AlignItems::START.safety, AlignmentSafety::Default);
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_items_rejects_invalid_safe_combos() {
        assert!("safe stretch".parse::<AlignItems>().is_err());
        assert!("safe baseline".parse::<AlignItems>().is_err());
        assert!("safe space-between".parse::<AlignItems>().is_err());
        assert!("safe".parse::<AlignItems>().is_err());
        assert!("safe garbage".parse::<AlignItems>().is_err());
        assert!("unsafe stretch".parse::<AlignItems>().is_err());
        assert!("unsafe baseline".parse::<AlignItems>().is_err());
        assert!("safe normal".parse::<AlignItems>().is_err());
        assert!("safe auto".parse::<AlignItems>().is_err());
        assert!("unsafe normal".parse::<AlignItems>().is_err());
        assert!("unsafe auto".parse::<AlignItems>().is_err());
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_content_plain() {
        assert_eq!("start".parse::<AlignContent>().unwrap(), AlignContent::START);
        assert_eq!("space-between".parse::<AlignContent>().unwrap(), AlignContent::SPACE_BETWEEN);
        assert_eq!("space-evenly".parse::<AlignContent>().unwrap(), AlignContent::SPACE_EVENLY);
        assert_eq!("space-around".parse::<AlignContent>().unwrap(), AlignContent::SPACE_AROUND);
        assert_eq!("stretch".parse::<AlignContent>().unwrap(), AlignContent::STRETCH);
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_content_safe() {
        assert_eq!("safe start".parse::<AlignContent>().unwrap(), AlignContent::SAFE_START);
        assert_eq!("safe end".parse::<AlignContent>().unwrap(), AlignContent::SAFE_END);
        assert_eq!("safe flex-start".parse::<AlignContent>().unwrap(), AlignContent::SAFE_FLEX_START);
        assert_eq!("safe flex-end".parse::<AlignContent>().unwrap(), AlignContent::SAFE_FLEX_END);
        assert_eq!("safe center".parse::<AlignContent>().unwrap(), AlignContent::SAFE_CENTER);
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_content_unsafe() {
        let unsafe_ = |keyword| AlignContent { keyword, safety: AlignmentSafety::Unsafe };
        assert_eq!("unsafe start".parse::<AlignContent>().unwrap(), unsafe_(AlignContentKeyword::Start));
        assert_eq!("unsafe flex-end".parse::<AlignContent>().unwrap(), unsafe_(AlignContentKeyword::FlexEnd));
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_content_rejects_invalid_safe_combos() {
        assert!("safe stretch".parse::<AlignContent>().is_err());
        assert!("safe space-between".parse::<AlignContent>().is_err());
        assert!("safe space-evenly".parse::<AlignContent>().is_err());
        assert!("safe space-around".parse::<AlignContent>().is_err());
        assert!("safe".parse::<AlignContent>().is_err());
        assert!("unsafe stretch".parse::<AlignContent>().is_err());
        assert!("unsafe space-between".parse::<AlignContent>().is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_align_items_round_trip() {
        let cases = [
            (AlignItems::NORMAL, "\"Normal\""),
            (AlignItems::START, "\"Start\""),
            (AlignItems::END, "\"End\""),
            (AlignItems::FLEX_START, "\"FlexStart\""),
            (AlignItems::FLEX_END, "\"FlexEnd\""),
            (AlignItems::CENTER, "\"Center\""),
            (AlignItems::BASELINE, "\"Baseline\""),
            (AlignItems::STRETCH, "\"Stretch\""),
            (AlignItems::SAFE_START, "\"SafeStart\""),
            (AlignItems::SAFE_END, "\"SafeEnd\""),
            (AlignItems::SAFE_FLEX_START, "\"SafeFlexStart\""),
            (AlignItems::SAFE_FLEX_END, "\"SafeFlexEnd\""),
            (AlignItems::SAFE_CENTER, "\"SafeCenter\""),
            (AlignItems::SELF_START, "\"SelfStart\""),
            (AlignItems::SELF_END, "\"SelfEnd\""),
            (AlignItems::SAFE_SELF_START, "\"SafeSelfStart\""),
            (AlignItems::SAFE_SELF_END, "\"SafeSelfEnd\""),
        ];
        for (value, expected) in cases {
            let serialized = serde_json::to_string(&value).unwrap();
            assert_eq!(serialized, expected, "serialize {:?}", value);
            let deserialized: AlignItems = serde_json::from_str(expected).unwrap();
            assert_eq!(deserialized, value, "round-trip {:?}", value);
        }
        assert!(serde_json::from_str::<AlignItems>("\"NotAVariant\"").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_align_content_round_trip() {
        let cases = [
            (AlignContent::START, "\"Start\""),
            (AlignContent::END, "\"End\""),
            (AlignContent::FLEX_START, "\"FlexStart\""),
            (AlignContent::FLEX_END, "\"FlexEnd\""),
            (AlignContent::CENTER, "\"Center\""),
            (AlignContent::STRETCH, "\"Stretch\""),
            (AlignContent::SPACE_BETWEEN, "\"SpaceBetween\""),
            (AlignContent::SPACE_EVENLY, "\"SpaceEvenly\""),
            (AlignContent::SPACE_AROUND, "\"SpaceAround\""),
            (AlignContent::SAFE_START, "\"SafeStart\""),
            (AlignContent::SAFE_END, "\"SafeEnd\""),
            (AlignContent::SAFE_FLEX_START, "\"SafeFlexStart\""),
            (AlignContent::SAFE_FLEX_END, "\"SafeFlexEnd\""),
            (AlignContent::SAFE_CENTER, "\"SafeCenter\""),
        ];
        for (value, expected) in cases {
            let serialized = serde_json::to_string(&value).unwrap();
            assert_eq!(serialized, expected, "serialize {:?}", value);
            let deserialized: AlignContent = serde_json::from_str(expected).unwrap();
            assert_eq!(deserialized, value, "round-trip {:?}", value);
        }
        assert!(serde_json::from_str::<AlignContent>("\"NotAVariant\"").is_err());
    }
}
