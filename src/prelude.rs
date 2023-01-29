//! Commonly used types

use crate::compute::LayoutAlgorithm;
use crate::layout::{SizeAndBaselines, SizingMode};

/// Apply the flexbox algorithm and recursively layout the specified node
#[inline(always)]
pub fn layout_flexbox(
    tree: &mut impl LayoutTree,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    sizing_mode: SizingMode,
) -> SizeAndBaselines {
    crate::compute::flexbox::FlexboxAlgorithm::perform_layout(
        tree,
        known_dimensions,
        parent_size,
        available_space,
        sizing_mode,
    )
}

pub use crate::{
    geometry::{Line, Rect, Size},
    layout::Layout,
    node::{Node, Taffy},
    style::{
        AlignContent, AlignItems, AlignSelf, AvailableSpace, Dimension, Display, FlexDirection, FlexWrap,
        JustifyContent, JustifyItems, JustifySelf, LengthPercentage, LengthPercentageAuto, Position, Style,
    },
    style_helpers::{
        auto, evenly_sized_tracks, fit_content, flex, fr, max_content, min_content, minmax, percent, points, zero,
        FromFlex, FromPercent, FromPoints, TaffyAuto, TaffyFitContent, TaffyMaxContent, TaffyMinContent, TaffyZero,
    },
    tree::LayoutTree,
};

#[cfg(feature = "grid")]
pub use crate::style::{
    GridAutoFlow, GridPlacement, GridTrackRepetition, MaxTrackSizingFunction, MinTrackSizingFunction,
    NonRepeatedTrackSizingFunction, TrackSizingFunction,
};
#[cfg(feature = "grid")]
pub use crate::style_helpers::{line, repeat, span, TaffyGridLine, TaffyGridSpan};
