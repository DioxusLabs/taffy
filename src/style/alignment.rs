//! Style types for controlling alignment

#[cfg(feature = "parse")]
use crate::util::parse::{CssParseResult, FromCss, Parser, Token};

/// Used to control how child nodes are aligned.
/// For Flexbox it controls alignment in the cross axis
/// For Grid it controls alignment in the block axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-items)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AlignItems {
    /// Items are packed toward the start of the axis
    Start,
    /// Items are packed toward the end of the axis
    End,
    /// Items are packed towards the flex-relative start of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to End. In all other cases it is equivalent to Start.
    FlexStart,
    /// Items are packed towards the flex-relative end of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to Start. In all other cases it is equivalent to End.
    FlexEnd,
    /// Items are packed along the center of the cross axis
    Center,
    /// Items are aligned such as their baselines align
    Baseline,
    /// Stretch to fill the container
    Stretch,
    /// Like [`AlignItems::Start`], but falls back to [`AlignItems::Start`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    SafeStart,
    /// Like [`AlignItems::End`], but falls back to [`AlignItems::Start`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    SafeEnd,
    /// Like [`AlignItems::FlexStart`], but falls back to [`AlignItems::Start`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    SafeFlexStart,
    /// Like [`AlignItems::FlexEnd`], but falls back to [`AlignItems::Start`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    SafeFlexEnd,
    /// Like [`AlignItems::Center`], but falls back to [`AlignItems::Start`] when the
    /// alignment subject overflows the alignment container, to avoid data loss.
    SafeCenter,
}

impl AlignItems {
    /// Returns true if this is a safe overflow-aware variant.
    #[inline]
    pub const fn is_safe(self) -> bool {
        matches!(
            self,
            Self::SafeStart | Self::SafeEnd | Self::SafeFlexStart | Self::SafeFlexEnd | Self::SafeCenter
        )
    }

    /// Strips the safe modifier and returns the underlying position keyword.
    /// Non-safe variants are returned unchanged.
    #[inline]
    pub const fn position(self) -> Self {
        match self {
            Self::SafeStart => Self::Start,
            Self::SafeEnd => Self::End,
            Self::SafeFlexStart => Self::FlexStart,
            Self::SafeFlexEnd => Self::FlexEnd,
            Self::SafeCenter => Self::Center,
            other => other,
        }
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
/// Does not apply to Flexbox, and will be ignored if specified on a flex container
/// For Grid it controls alignment in the inline axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items)
pub type JustifyItems = AlignItems;
/// Controls alignment of an individual node
///
/// Overrides the parent Node's `AlignItems` property.
/// For Flexbox it controls alignment in the cross axis
/// For Grid it controls alignment in the block axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-self)
pub type AlignSelf = AlignItems;
/// Controls alignment of an individual node
///
/// Overrides the parent Node's `JustifyItems` property.
/// Does not apply to Flexbox, and will be ignored if specified on a flex child
/// For Grid it controls alignment in the inline axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self)
pub type JustifySelf = AlignItems;

/// Sets the distribution of space between and around content items
/// For Flexbox it controls alignment in the cross axis
/// For Grid it controls alignment in the block axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-content)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AlignContent {
    /// Items are packed toward the start of the axis
    Start,
    /// Items are packed toward the end of the axis
    End,
    /// Items are packed towards the flex-relative start of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to End. In all other cases it is equivalent to Start.
    FlexStart,
    /// Items are packed towards the flex-relative end of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to Start. In all other cases it is equivalent to End.
    FlexEnd,
    /// Items are centered around the middle of the axis
    Center,
    /// Items are stretched to fill the container
    Stretch,
    /// The first and last items are aligned flush with the edges of the container (no gap)
    /// The gap between items is distributed evenly.
    SpaceBetween,
    /// The gap between the first and last items is exactly THE SAME as the gap between items.
    /// The gaps are distributed evenly
    SpaceEvenly,
    /// The gap between the first and last items is exactly HALF the gap between items.
    /// The gaps are distributed evenly in proportion to these ratios.
    SpaceAround,
    /// Like [`AlignContent::Start`], but falls back to [`AlignContent::Start`] when the
    /// content overflows the alignment container, to avoid data loss.
    SafeStart,
    /// Like [`AlignContent::End`], but falls back to [`AlignContent::Start`] when the
    /// content overflows the alignment container, to avoid data loss.
    SafeEnd,
    /// Like [`AlignContent::FlexStart`], but falls back to [`AlignContent::Start`] when the
    /// content overflows the alignment container, to avoid data loss.
    SafeFlexStart,
    /// Like [`AlignContent::FlexEnd`], but falls back to [`AlignContent::Start`] when the
    /// content overflows the alignment container, to avoid data loss.
    SafeFlexEnd,
    /// Like [`AlignContent::Center`], but falls back to [`AlignContent::Start`] when the
    /// content overflows the alignment container, to avoid data loss.
    SafeCenter,
}

impl AlignContent {
    /// Returns true if this is a safe overflow-aware variant.
    #[inline]
    pub const fn is_safe(self) -> bool {
        matches!(
            self,
            Self::SafeStart | Self::SafeEnd | Self::SafeFlexStart | Self::SafeFlexEnd | Self::SafeCenter
        )
    }

    /// Strips the safe modifier and returns the underlying position keyword.
    /// Non-safe variants are returned unchanged.
    #[inline]
    pub const fn position(self) -> Self {
        match self {
            Self::SafeStart => Self::Start,
            Self::SafeEnd => Self::End,
            Self::SafeFlexStart => Self::FlexStart,
            Self::SafeFlexEnd => Self::FlexEnd,
            Self::SafeCenter => Self::Center,
            other => other,
        }
    }

    /// Returns the reversed alignment for RTL (right-to-left) contexts.
    pub(crate) fn reversed(self) -> Self {
        match self {
            Self::Start => Self::End,
            Self::End => Self::Start,
            Self::FlexStart => Self::FlexEnd,
            Self::FlexEnd => Self::FlexStart,
            Self::Stretch => Self::End,
            Self::SafeStart => Self::SafeEnd,
            Self::SafeEnd => Self::SafeStart,
            Self::SafeFlexStart => Self::SafeFlexEnd,
            Self::SafeFlexEnd => Self::SafeFlexStart,
            style => style,
        }
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

/// Sets the distribution of space between and around content items
/// For Flexbox it controls alignment in the main axis
/// For Grid it controls alignment in the inline axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content)
pub type JustifyContent = AlignContent;

#[cfg(test)]
mod tests {
    use super::*;

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
    fn align_items_position_strips_safe() {
        assert_eq!(AlignItems::SafeStart.position(), AlignItems::Start);
        assert_eq!(AlignItems::SafeEnd.position(), AlignItems::End);
        assert_eq!(AlignItems::SafeFlexStart.position(), AlignItems::FlexStart);
        assert_eq!(AlignItems::SafeFlexEnd.position(), AlignItems::FlexEnd);
        assert_eq!(AlignItems::SafeCenter.position(), AlignItems::Center);
    }

    #[test]
    fn align_items_position_passthrough() {
        assert_eq!(AlignItems::Start.position(), AlignItems::Start);
        assert_eq!(AlignItems::Stretch.position(), AlignItems::Stretch);
        assert_eq!(AlignItems::Baseline.position(), AlignItems::Baseline);
        assert_eq!(AlignItems::FlexStart.position(), AlignItems::FlexStart);
    }

    #[test]
    fn align_content_is_safe() {
        assert!(AlignContent::SafeStart.is_safe());
        assert!(AlignContent::SafeCenter.is_safe());
        assert!(!AlignContent::SpaceBetween.is_safe());
        assert!(!AlignContent::Stretch.is_safe());
    }

    #[test]
    fn align_content_position_strips_safe() {
        assert_eq!(AlignContent::SafeStart.position(), AlignContent::Start);
        assert_eq!(AlignContent::SafeFlexEnd.position(), AlignContent::FlexEnd);
        assert_eq!(AlignContent::SafeCenter.position(), AlignContent::Center);
        assert_eq!(AlignContent::SpaceBetween.position(), AlignContent::SpaceBetween);
    }

    #[test]
    fn align_content_reversed_pairs_safe_variants() {
        assert_eq!(AlignContent::SafeStart.reversed(), AlignContent::SafeEnd);
        assert_eq!(AlignContent::SafeEnd.reversed(), AlignContent::SafeStart);
        assert_eq!(AlignContent::SafeFlexStart.reversed(), AlignContent::SafeFlexEnd);
        assert_eq!(AlignContent::SafeFlexEnd.reversed(), AlignContent::SafeFlexStart);
        assert_eq!(AlignContent::SafeCenter.reversed(), AlignContent::SafeCenter);
    }

    #[test]
    fn align_content_reversed_existing_unchanged() {
        assert_eq!(AlignContent::Start.reversed(), AlignContent::End);
        assert_eq!(AlignContent::End.reversed(), AlignContent::Start);
        assert_eq!(AlignContent::FlexStart.reversed(), AlignContent::FlexEnd);
        assert_eq!(AlignContent::FlexEnd.reversed(), AlignContent::FlexStart);
        assert_eq!(AlignContent::Stretch.reversed(), AlignContent::End);
        assert_eq!(AlignContent::SpaceBetween.reversed(), AlignContent::SpaceBetween);
        assert_eq!(AlignContent::SpaceAround.reversed(), AlignContent::SpaceAround);
        assert_eq!(AlignContent::SpaceEvenly.reversed(), AlignContent::SpaceEvenly);
        assert_eq!(AlignContent::Center.reversed(), AlignContent::Center);
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
}
