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

/// The position-keyword half of [`AlignItems`] (and its aliases `AlignSelf`,
/// `JustifyItems`, `JustifySelf`).
///
/// Compute paths match on this enum directly so every match is exhaustive and
/// requires no `Safe*` siblings.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum AlignItemsKeyword {
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

/// The overflow-position modifier per [CSS Box Alignment §4.3_f32][css-align-overflow].
///
/// `Safe` falls back to start-edge alignment when the alignment subject would
/// overflow the alignment container, so the start of the content stays visible.
/// `Unsafe` (the default) keeps the requested alignment even when that causes
/// overflow at the start edge.
///
/// CSS only defines `safe` / `unsafe` against the position values `start`, `end`,
/// `flex-start`, `flex-end`, `center`. The struct shape does not enforce that
/// constraint at the type level — the parser rejects invalid combinations, and
/// the compute pass treats `Safe` paired with a non-position keyword (`Stretch`,
/// `Baseline`, `Space*`) the same as `Unsafe`.
///
/// [css-align-overflow]: https://www.w3.org/TR/css-align-3/#overflow-values
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum AlignmentSafety {
    /// Default — keeps the requested alignment even when the subject overflows the
    /// alignment container at the start edge.
    Unsafe,
    /// Falls back to the start edge when the subject would overflow, to avoid data
    /// loss.
    Safe,
}

/// Used to control how child nodes are aligned.
/// For Flexbox it controls alignment in the cross axis.
/// For Grid it controls alignment in the block axis.
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-items)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AlignItems {
    /// Position keyword.
    pub keyword: AlignItemsKeyword,
    /// Overflow-position modifier (`safe` / `unsafe`).
    pub safety: AlignmentSafety,
}

#[allow(non_upper_case_globals)]
impl AlignItems {
    /// Items are packed toward the start of the axis.
    pub const Start: Self = Self { keyword: AlignItemsKeyword::Start, safety: AlignmentSafety::Unsafe };
    /// Items are packed toward the end of the axis.
    pub const End: Self = Self { keyword: AlignItemsKeyword::End, safety: AlignmentSafety::Unsafe };
    /// Items are packed towards the flex-relative start of the axis.
    pub const FlexStart: Self = Self { keyword: AlignItemsKeyword::FlexStart, safety: AlignmentSafety::Unsafe };
    /// Items are packed towards the flex-relative end of the axis.
    pub const FlexEnd: Self = Self { keyword: AlignItemsKeyword::FlexEnd, safety: AlignmentSafety::Unsafe };
    /// Items are packed along the center of the cross axis.
    pub const Center: Self = Self { keyword: AlignItemsKeyword::Center, safety: AlignmentSafety::Unsafe };
    /// Items are aligned such as their baselines align.
    pub const Baseline: Self = Self { keyword: AlignItemsKeyword::Baseline, safety: AlignmentSafety::Unsafe };
    /// Stretch to fill the container.
    pub const Stretch: Self = Self { keyword: AlignItemsKeyword::Stretch, safety: AlignmentSafety::Unsafe };
    /// Like [`AlignItems::Start`], but falls back to [`AlignItems::Start`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    pub const SafeStart: Self = Self { keyword: AlignItemsKeyword::Start, safety: AlignmentSafety::Safe };
    /// Like [`AlignItems::End`], but falls back to [`AlignItems::Start`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    pub const SafeEnd: Self = Self { keyword: AlignItemsKeyword::End, safety: AlignmentSafety::Safe };
    /// Like [`AlignItems::FlexStart`], but falls back to [`AlignItems::Start`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    pub const SafeFlexStart: Self = Self { keyword: AlignItemsKeyword::FlexStart, safety: AlignmentSafety::Safe };
    /// Like [`AlignItems::FlexEnd`], but falls back to [`AlignItems::Start`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    pub const SafeFlexEnd: Self = Self { keyword: AlignItemsKeyword::FlexEnd, safety: AlignmentSafety::Safe };
    /// Like [`AlignItems::Center`], but falls back to [`AlignItems::Start`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    pub const SafeCenter: Self = Self { keyword: AlignItemsKeyword::Center, safety: AlignmentSafety::Safe };

    /// Returns `true` iff this carries the `safe` overflow-position modifier.
    #[inline]
    pub const fn is_safe(self) -> bool {
        matches!(self.safety, AlignmentSafety::Safe)
    }

    /// Returns the underlying position keyword, discarding the safety modifier.
    #[inline]
    pub const fn keyword(self) -> AlignItemsKeyword {
        self.keyword
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
                    "start" => Ok(Self::SafeStart),
                    "end" => Ok(Self::SafeEnd),
                    "flex-start" => Ok(Self::SafeFlexStart),
                    "flex-end" => Ok(Self::SafeFlexEnd),
                    "center" => Ok(Self::SafeCenter),
                    _ => Err(input.new_unexpected_token_error(Token::Ident(pos))),
                }
            },
            "unsafe" => {
                let pos = input.expect_ident()?.clone();
                cssparser::match_ignore_ascii_case! { &*pos,
                    "start" => Ok(Self::Start),
                    "end" => Ok(Self::End),
                    "flex-start" => Ok(Self::FlexStart),
                    "flex-end" => Ok(Self::FlexEnd),
                    "center" => Ok(Self::Center),
                    _ => Err(input.new_unexpected_token_error(Token::Ident(pos))),
                }
            },
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "flex-start" => Ok(Self::FlexStart),
            "flex-end" => Ok(Self::FlexEnd),
            "center" => Ok(Self::Center),
            "baseline" => Ok(Self::Baseline),
            "stretch" => Ok(Self::Stretch),
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
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items)
pub type JustifyItems = AlignItems;
/// Controls alignment of an individual node.
///
/// Overrides the parent Node's `AlignItems` property.
/// For Flexbox it controls alignment in the cross axis.
/// For Grid it controls alignment in the block axis.
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-self)
pub type AlignSelf = AlignItems;
/// Controls alignment of an individual node.
///
/// Overrides the parent Node's `JustifyItems` property.
/// Does not apply to Flexbox, and will be ignored if specified on a flex child.
/// For Grid it controls alignment in the inline axis.
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

#[allow(non_upper_case_globals)]
impl AlignContent {
    /// Items are packed toward the start of the axis.
    pub const Start: Self = Self { keyword: AlignContentKeyword::Start, safety: AlignmentSafety::Unsafe };
    /// Items are packed toward the end of the axis.
    pub const End: Self = Self { keyword: AlignContentKeyword::End, safety: AlignmentSafety::Unsafe };
    /// Items are packed towards the flex-relative start of the axis.
    pub const FlexStart: Self = Self { keyword: AlignContentKeyword::FlexStart, safety: AlignmentSafety::Unsafe };
    /// Items are packed towards the flex-relative end of the axis.
    pub const FlexEnd: Self = Self { keyword: AlignContentKeyword::FlexEnd, safety: AlignmentSafety::Unsafe };
    /// Items are centered around the middle of the axis.
    pub const Center: Self = Self { keyword: AlignContentKeyword::Center, safety: AlignmentSafety::Unsafe };
    /// Items are stretched to fill the container.
    pub const Stretch: Self = Self { keyword: AlignContentKeyword::Stretch, safety: AlignmentSafety::Unsafe };
    /// The first and last items are aligned flush with the edges of the container.
    pub const SpaceBetween: Self = Self { keyword: AlignContentKeyword::SpaceBetween, safety: AlignmentSafety::Unsafe };
    /// The gap between the first and last items equals the gap between items.
    pub const SpaceEvenly: Self = Self { keyword: AlignContentKeyword::SpaceEvenly, safety: AlignmentSafety::Unsafe };
    /// The gap between the first and last items is half the gap between items.
    pub const SpaceAround: Self = Self { keyword: AlignContentKeyword::SpaceAround, safety: AlignmentSafety::Unsafe };
    /// Like [`AlignContent::Start`], but falls back to [`AlignContent::Start`] when the
    /// content overflows the alignment container, to avoid data loss.
    pub const SafeStart: Self = Self { keyword: AlignContentKeyword::Start, safety: AlignmentSafety::Safe };
    /// Like [`AlignContent::End`], but falls back to [`AlignContent::Start`] when the
    /// content overflows the alignment container, to avoid data loss.
    pub const SafeEnd: Self = Self { keyword: AlignContentKeyword::End, safety: AlignmentSafety::Safe };
    /// Like [`AlignContent::FlexStart`], but falls back to [`AlignContent::Start`] when
    /// the content overflows the alignment container, to avoid data loss.
    pub const SafeFlexStart: Self = Self { keyword: AlignContentKeyword::FlexStart, safety: AlignmentSafety::Safe };
    /// Like [`AlignContent::FlexEnd`], but falls back to [`AlignContent::Start`] when the
    /// content overflows the alignment container, to avoid data loss.
    pub const SafeFlexEnd: Self = Self { keyword: AlignContentKeyword::FlexEnd, safety: AlignmentSafety::Safe };
    /// Like [`AlignContent::Center`], but falls back to [`AlignContent::Start`] when the
    /// content overflows the alignment container, to avoid data loss.
    pub const SafeCenter: Self = Self { keyword: AlignContentKeyword::Center, safety: AlignmentSafety::Safe };

    /// Returns `true` iff this carries the `safe` overflow-position modifier.
    #[inline]
    pub const fn is_safe(self) -> bool {
        matches!(self.safety, AlignmentSafety::Safe)
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
                    "start" => Ok(Self::SafeStart),
                    "end" => Ok(Self::SafeEnd),
                    "flex-start" => Ok(Self::SafeFlexStart),
                    "flex-end" => Ok(Self::SafeFlexEnd),
                    "center" => Ok(Self::SafeCenter),
                    _ => Err(input.new_unexpected_token_error(Token::Ident(pos))),
                }
            },
            "unsafe" => {
                let pos = input.expect_ident()?.clone();
                cssparser::match_ignore_ascii_case! { &*pos,
                    "start" => Ok(Self::Start),
                    "end" => Ok(Self::End),
                    "flex-start" => Ok(Self::FlexStart),
                    "flex-end" => Ok(Self::FlexEnd),
                    "center" => Ok(Self::Center),
                    _ => Err(input.new_unexpected_token_error(Token::Ident(pos))),
                }
            },
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "flex-start" => Ok(Self::FlexStart),
            "flex-end" => Ok(Self::FlexEnd),
            "center" => Ok(Self::Center),
            "stretch" => Ok(Self::Stretch),
            "space-between" => Ok(Self::SpaceBetween),
            "space-evenly" => Ok(Self::SpaceEvenly),
            "space-around" => Ok(Self::SpaceAround),
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
    "Start",
    "End",
    "FlexStart",
    "FlexEnd",
    "Center",
    "Baseline",
    "Stretch",
    "SafeStart",
    "SafeEnd",
    "SafeFlexStart",
    "SafeFlexEnd",
    "SafeCenter",
];

#[cfg(feature = "serde")]
impl serde::Serialize for AlignItems {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let name = match (self.keyword, self.safety) {
            (AlignItemsKeyword::Start, AlignmentSafety::Unsafe) => "Start",
            (AlignItemsKeyword::End, AlignmentSafety::Unsafe) => "End",
            (AlignItemsKeyword::FlexStart, AlignmentSafety::Unsafe) => "FlexStart",
            (AlignItemsKeyword::FlexEnd, AlignmentSafety::Unsafe) => "FlexEnd",
            (AlignItemsKeyword::Center, AlignmentSafety::Unsafe) => "Center",
            (AlignItemsKeyword::Baseline, _) => "Baseline",
            (AlignItemsKeyword::Stretch, _) => "Stretch",
            (AlignItemsKeyword::Start, AlignmentSafety::Safe) => "SafeStart",
            (AlignItemsKeyword::End, AlignmentSafety::Safe) => "SafeEnd",
            (AlignItemsKeyword::FlexStart, AlignmentSafety::Safe) => "SafeFlexStart",
            (AlignItemsKeyword::FlexEnd, AlignmentSafety::Safe) => "SafeFlexEnd",
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
                    "Start" => AlignItems::Start,
                    "End" => AlignItems::End,
                    "FlexStart" => AlignItems::FlexStart,
                    "FlexEnd" => AlignItems::FlexEnd,
                    "Center" => AlignItems::Center,
                    "Baseline" => AlignItems::Baseline,
                    "Stretch" => AlignItems::Stretch,
                    "SafeStart" => AlignItems::SafeStart,
                    "SafeEnd" => AlignItems::SafeEnd,
                    "SafeFlexStart" => AlignItems::SafeFlexStart,
                    "SafeFlexEnd" => AlignItems::SafeFlexEnd,
                    "SafeCenter" => AlignItems::SafeCenter,
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
];

#[cfg(feature = "serde")]
impl serde::Serialize for AlignContent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let name = match (self.keyword, self.safety) {
            (AlignContentKeyword::Start, AlignmentSafety::Unsafe) => "Start",
            (AlignContentKeyword::End, AlignmentSafety::Unsafe) => "End",
            (AlignContentKeyword::FlexStart, AlignmentSafety::Unsafe) => "FlexStart",
            (AlignContentKeyword::FlexEnd, AlignmentSafety::Unsafe) => "FlexEnd",
            (AlignContentKeyword::Center, AlignmentSafety::Unsafe) => "Center",
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
                    "Start" => AlignContent::Start,
                    "End" => AlignContent::End,
                    "FlexStart" => AlignContent::FlexStart,
                    "FlexEnd" => AlignContent::FlexEnd,
                    "Center" => AlignContent::Center,
                    "Stretch" => AlignContent::Stretch,
                    "SpaceBetween" => AlignContent::SpaceBetween,
                    "SpaceEvenly" => AlignContent::SpaceEvenly,
                    "SpaceAround" => AlignContent::SpaceAround,
                    "SafeStart" => AlignContent::SafeStart,
                    "SafeEnd" => AlignContent::SafeEnd,
                    "SafeFlexStart" => AlignContent::SafeFlexStart,
                    "SafeFlexEnd" => AlignContent::SafeFlexEnd,
                    "SafeCenter" => AlignContent::SafeCenter,
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
        assert!(size_of::<Option<AlignItems>>() <= 3);
        assert!(size_of::<Option<AlignContent>>() <= 3);
    }

    #[test]
    fn align_items_is_safe() {
        assert!(AlignItems::SafeStart.is_safe());
        assert!(AlignItems::SafeEnd.is_safe());
        assert!(AlignItems::SafeFlexStart.is_safe());
        assert!(AlignItems::SafeFlexEnd.is_safe());
        assert!(AlignItems::SafeCenter.is_safe());
        assert!(!AlignItems::Start.is_safe());
        assert!(!AlignItems::End.is_safe());
        assert!(!AlignItems::FlexStart.is_safe());
        assert!(!AlignItems::FlexEnd.is_safe());
        assert!(!AlignItems::Center.is_safe());
        assert!(!AlignItems::Baseline.is_safe());
        assert!(!AlignItems::Stretch.is_safe());
    }

    #[test]
    fn align_items_keyword_strips_safe() {
        assert_eq!(AlignItems::SafeStart.keyword(), AlignItemsKeyword::Start);
        assert_eq!(AlignItems::SafeEnd.keyword(), AlignItemsKeyword::End);
        assert_eq!(AlignItems::SafeFlexStart.keyword(), AlignItemsKeyword::FlexStart);
        assert_eq!(AlignItems::SafeFlexEnd.keyword(), AlignItemsKeyword::FlexEnd);
        assert_eq!(AlignItems::SafeCenter.keyword(), AlignItemsKeyword::Center);
    }

    #[test]
    fn align_items_keyword_passthrough() {
        assert_eq!(AlignItems::Start.keyword(), AlignItemsKeyword::Start);
        assert_eq!(AlignItems::Stretch.keyword(), AlignItemsKeyword::Stretch);
        assert_eq!(AlignItems::Baseline.keyword(), AlignItemsKeyword::Baseline);
        assert_eq!(AlignItems::FlexStart.keyword(), AlignItemsKeyword::FlexStart);
    }

    #[test]
    fn align_content_is_safe() {
        assert!(AlignContent::SafeStart.is_safe());
        assert!(AlignContent::SafeCenter.is_safe());
        assert!(!AlignContent::SpaceBetween.is_safe());
        assert!(!AlignContent::Stretch.is_safe());
    }

    #[test]
    fn align_content_keyword_strips_safe() {
        assert_eq!(AlignContent::SafeStart.keyword(), AlignContentKeyword::Start);
        assert_eq!(AlignContent::SafeFlexEnd.keyword(), AlignContentKeyword::FlexEnd);
        assert_eq!(AlignContent::SafeCenter.keyword(), AlignContentKeyword::Center);
        assert_eq!(AlignContent::SpaceBetween.keyword(), AlignContentKeyword::SpaceBetween);
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
        assert_eq!("start".parse::<AlignItems>().unwrap(), AlignItems::Start);
        assert_eq!("end".parse::<AlignItems>().unwrap(), AlignItems::End);
        assert_eq!("flex-start".parse::<AlignItems>().unwrap(), AlignItems::FlexStart);
        assert_eq!("flex-end".parse::<AlignItems>().unwrap(), AlignItems::FlexEnd);
        assert_eq!("center".parse::<AlignItems>().unwrap(), AlignItems::Center);
        assert_eq!("baseline".parse::<AlignItems>().unwrap(), AlignItems::Baseline);
        assert_eq!("stretch".parse::<AlignItems>().unwrap(), AlignItems::Stretch);
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_items_safe() {
        assert_eq!("safe start".parse::<AlignItems>().unwrap(), AlignItems::SafeStart);
        assert_eq!("safe end".parse::<AlignItems>().unwrap(), AlignItems::SafeEnd);
        assert_eq!("safe flex-start".parse::<AlignItems>().unwrap(), AlignItems::SafeFlexStart);
        assert_eq!("safe flex-end".parse::<AlignItems>().unwrap(), AlignItems::SafeFlexEnd);
        assert_eq!("safe center".parse::<AlignItems>().unwrap(), AlignItems::SafeCenter);
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_items_safe_case_insensitive() {
        assert_eq!("SAFE Start".parse::<AlignItems>().unwrap(), AlignItems::SafeStart);
        assert_eq!("Safe FLEX-end".parse::<AlignItems>().unwrap(), AlignItems::SafeFlexEnd);
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_items_unsafe_drops_modifier() {
        assert_eq!("unsafe start".parse::<AlignItems>().unwrap(), AlignItems::Start);
        assert_eq!("unsafe end".parse::<AlignItems>().unwrap(), AlignItems::End);
        assert_eq!("unsafe center".parse::<AlignItems>().unwrap(), AlignItems::Center);
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
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_content_plain() {
        assert_eq!("start".parse::<AlignContent>().unwrap(), AlignContent::Start);
        assert_eq!("space-between".parse::<AlignContent>().unwrap(), AlignContent::SpaceBetween);
        assert_eq!("space-evenly".parse::<AlignContent>().unwrap(), AlignContent::SpaceEvenly);
        assert_eq!("space-around".parse::<AlignContent>().unwrap(), AlignContent::SpaceAround);
        assert_eq!("stretch".parse::<AlignContent>().unwrap(), AlignContent::Stretch);
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_content_safe() {
        assert_eq!("safe start".parse::<AlignContent>().unwrap(), AlignContent::SafeStart);
        assert_eq!("safe end".parse::<AlignContent>().unwrap(), AlignContent::SafeEnd);
        assert_eq!("safe flex-start".parse::<AlignContent>().unwrap(), AlignContent::SafeFlexStart);
        assert_eq!("safe flex-end".parse::<AlignContent>().unwrap(), AlignContent::SafeFlexEnd);
        assert_eq!("safe center".parse::<AlignContent>().unwrap(), AlignContent::SafeCenter);
    }

    #[cfg(feature = "parse")]
    #[test]
    fn parse_align_content_unsafe_drops_modifier() {
        assert_eq!("unsafe start".parse::<AlignContent>().unwrap(), AlignContent::Start);
        assert_eq!("unsafe flex-end".parse::<AlignContent>().unwrap(), AlignContent::FlexEnd);
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
            (AlignItems::Start, "\"Start\""),
            (AlignItems::End, "\"End\""),
            (AlignItems::FlexStart, "\"FlexStart\""),
            (AlignItems::FlexEnd, "\"FlexEnd\""),
            (AlignItems::Center, "\"Center\""),
            (AlignItems::Baseline, "\"Baseline\""),
            (AlignItems::Stretch, "\"Stretch\""),
            (AlignItems::SafeStart, "\"SafeStart\""),
            (AlignItems::SafeEnd, "\"SafeEnd\""),
            (AlignItems::SafeFlexStart, "\"SafeFlexStart\""),
            (AlignItems::SafeFlexEnd, "\"SafeFlexEnd\""),
            (AlignItems::SafeCenter, "\"SafeCenter\""),
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
            (AlignContent::Start, "\"Start\""),
            (AlignContent::End, "\"End\""),
            (AlignContent::FlexStart, "\"FlexStart\""),
            (AlignContent::FlexEnd, "\"FlexEnd\""),
            (AlignContent::Center, "\"Center\""),
            (AlignContent::Stretch, "\"Stretch\""),
            (AlignContent::SpaceBetween, "\"SpaceBetween\""),
            (AlignContent::SpaceEvenly, "\"SpaceEvenly\""),
            (AlignContent::SpaceAround, "\"SpaceAround\""),
            (AlignContent::SafeStart, "\"SafeStart\""),
            (AlignContent::SafeEnd, "\"SafeEnd\""),
            (AlignContent::SafeFlexStart, "\"SafeFlexStart\""),
            (AlignContent::SafeFlexEnd, "\"SafeFlexEnd\""),
            (AlignContent::SafeCenter, "\"SafeCenter\""),
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
