//! Generic CSS alignment code that is shared between both the Flexbox and CSS Grid algorithms.
use crate::style::AlignContent;

/// Implement fallback alignment.
///
/// In addition to the spec at https://www.w3.org/TR/css-align-3/ this implementation follows
/// the resolution of https://github.com/w3c/csswg-drafts/issues/10154
pub(crate) fn apply_alignment_fallback(
    free_space: f32,
    num_items: usize,
    mut alignment_mode: AlignContent,
    mut is_safe: bool,
) -> AlignContent {
    // Fallback occurs in two cases:

    // 1. If there is only a single item being aligned and alignment is a distributed alignment keyword
    //    https://www.w3.org/TR/css-align-3/#distribution-values
    if num_items <= 1 || free_space <= 0.0 {
        (alignment_mode, is_safe) = match alignment_mode {
            AlignContent::Stretch => (AlignContent::FlexStart, true),
            AlignContent::SpaceBetween => (AlignContent::FlexStart, true),
            AlignContent::SpaceAround => (AlignContent::Center, true),
            AlignContent::SpaceEvenly => (AlignContent::Center, true),
            _ => (alignment_mode, is_safe),
        }
    };

    // 2. If free space is negative the "safe" alignment variants all fallback to Start alignment
    if free_space <= 0.0 && is_safe {
        alignment_mode = AlignContent::Start;
    }

    alignment_mode
}

/// Generic alignment function that is used:
///   - For both align-content and justify-content alignment
///   - For both the Flexbox and CSS Grid algorithms
///
/// CSS Grid does not apply gaps as part of alignment, so the gap parameter should
/// always be set to zero for CSS Grid.
pub(crate) fn compute_alignment_offset(
    free_space: f32,
    num_items: usize,
    gap: f32,
    alignment_mode: AlignContent,
    layout_is_flex_reversed: bool,
    is_first: bool,
) -> f32 {
    if is_first {
        match alignment_mode {
            AlignContent::Start => 0.0,
            AlignContent::FlexStart => {
                if layout_is_flex_reversed {
                    free_space
                } else {
                    0.0
                }
            }
            AlignContent::End => free_space,
            AlignContent::FlexEnd => {
                if layout_is_flex_reversed {
                    0.0
                } else {
                    free_space
                }
            }
            AlignContent::Center => free_space / 2.0,
            AlignContent::Stretch => 0.0,
            AlignContent::SpaceBetween => 0.0,
            AlignContent::SpaceAround => {
                if free_space >= 0.0 {
                    (free_space / num_items as f32) / 2.0
                } else {
                    free_space / 2.0
                }
            }
            AlignContent::SpaceEvenly => {
                if free_space >= 0.0 {
                    free_space / (num_items + 1) as f32
                } else {
                    free_space / 2.0
                }
            }
        }
    } else {
        let free_space = free_space.max(0.0);
        gap + match alignment_mode {
            AlignContent::Start => 0.0,
            AlignContent::FlexStart => 0.0,
            AlignContent::End => 0.0,
            AlignContent::FlexEnd => 0.0,
            AlignContent::Center => 0.0,
            AlignContent::Stretch => 0.0,
            AlignContent::SpaceBetween => free_space / (num_items - 1) as f32,
            AlignContent::SpaceAround => free_space / num_items as f32,
            AlignContent::SpaceEvenly => free_space / (num_items + 1) as f32,
        }
    }
}
