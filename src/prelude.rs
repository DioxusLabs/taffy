//! Commonly used types

#[cfg(feature = "flexbox")]
use crate::{
    compute::LayoutAlgorithm,
    layout::{SizeAndBaselines, SizingMode},
};

/// Apply the flexbox algorithm and recursively layout the specified node
#[cfg(feature = "flexbox")]
#[inline(always)]
pub fn layout_flexbox(
    tree: &mut impl LayoutTree,
    node: Node,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    sizing_mode: SizingMode,
) -> SizeAndBaselines {
    crate::compute::flexbox::FlexboxAlgorithm::perform_layout(
        tree,
        node,
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
        AlignContent, AlignItems, AlignSelf, AvailableSpace, Dimension, Display, JustifyContent, JustifyItems,
        JustifySelf, LengthPercentage, LengthPercentageAuto, Position, Style,
    },
    style_helpers::{
        auto, fit_content, max_content, min_content, percent, points, zero, FromFlex, FromPercent, FromPoints,
        TaffyAuto, TaffyFitContent, TaffyMaxContent, TaffyMinContent, TaffyZero,
    },
    tree::LayoutTree,
};

#[cfg(feature = "flexbox")]
pub use crate::style::{FlexDirection, FlexWrap};

#[cfg(feature = "grid")]
pub use crate::style::{
    GridAutoFlow, GridPlacement, GridTrackRepetition, MaxTrackSizingFunction, MinTrackSizingFunction,
    NonRepeatedTrackSizingFunction, TrackSizingFunction,
};
#[cfg(feature = "grid")]
pub use crate::style_helpers::{
    evenly_sized_tracks, flex, fr, line, minmax, repeat, span, TaffyGridLine, TaffyGridSpan,
};
