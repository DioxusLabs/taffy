//! Commonly used types

pub use crate::{
    geometry::{Line, Rect, Size},
    style::{
        AlignContent, AlignItems, AlignSelf, AvailableSpace, BoxSizing, CompactLength, Dimension, Display,
        JustifyContent, JustifyItems, JustifySelf, LengthPercentage, LengthPercentageAuto, Position, Style,
    },
    style_helpers::{
        auto, fit_content, length, max_content, min_content, percent, zero, FromFr, FromLength, FromPercent, TaffyAuto,
        TaffyFitContent, TaffyMaxContent, TaffyMinContent, TaffyZero,
    },
    tree::{Layout, LayoutPartialTree, NodeId, PrintTree, RoundTree, TraversePartialTree, TraverseTree},
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

#[cfg(feature = "taffy_tree")]
pub use crate::TaffyTree;

#[cfg(feature = "builder")]
pub use crate::style::{NodeIdRef, StyleBuilder};
