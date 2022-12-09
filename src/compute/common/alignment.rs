//! Generic CSS alignment code that is shared between both the Flexbox and CSS Grid algorithms.
use crate::style::AlignContent;

/// Generic alignment function that is used:
///   - For both align-content and justify-content alignment
///   - For both the Flexbox and CSS Grid algorithms
/// CSS Grid does not apply gaps as part of alignment, so the gap parameter should
/// always be set to zero for CSS Grid.
pub(crate) fn compute_alignment_offset(
    free_space: f32,
    num_items: usize,
    gap: f32,
    alignment_mode: AlignContent,
    layout_is_reversed: bool,
    is_first: bool,
) -> f32 {
    match alignment_mode {
        AlignContent::Start => {
            if is_first {
                if layout_is_reversed {
                    free_space
                } else {
                    0.0
                }
            } else {
                gap
            }
        }
        AlignContent::End => {
            if is_first {
                if !layout_is_reversed {
                    free_space
                } else {
                    0.0
                }
            } else {
                gap
            }
        }
        AlignContent::Center => {
            if is_first {
                free_space / 2.0
            } else {
                gap
            }
        }
        AlignContent::Stretch => {
            if is_first {
                0.0
            } else {
                gap
            }
        }
        AlignContent::SpaceBetween => {
            if is_first {
                0.0
            } else {
                gap + (free_space / (num_items - 1) as f32)
            }
        }
        AlignContent::SpaceAround => {
            if is_first {
                (free_space / num_items as f32) / 2.0
            } else {
                gap + (free_space / num_items as f32)
            }
        }
        AlignContent::SpaceEvenly => {
            if is_first {
                free_space / (num_items + 1) as f32
            } else {
                gap + (free_space / (num_items + 1) as f32)
            }
        }
    }
}
