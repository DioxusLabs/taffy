//! Commonly used types

pub use crate::{
    geometry::{Line, Rect, Size},
    style::{
        AlignContent, AlignItems, AlignSelf, AvailableSpace, Dimension, Display, JustifyContent, JustifyItems,
        JustifySelf, LengthPercentage, LengthPercentageAuto, Position, Style,
    },
    style_helpers::{
        auto, fit_content, max_content, min_content, percent, points, zero, FromFlex, FromPercent, FromPoints,
        TaffyAuto, TaffyFitContent, TaffyMaxContent, TaffyMinContent, TaffyZero,
    },
    tree::{Layout, LayoutTree, NodeId, Taffy},
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
