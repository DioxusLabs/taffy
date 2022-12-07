use super::{GridAxis, GridTrack};
use crate::compute::compute_node_layout;
use crate::geometry::{Line, Size};
use crate::layout::{AvailableSpace, RunMode, SizingMode};
use crate::node::Node;
use crate::prelude::LayoutTree;
use crate::resolve::MaybeResolve;
use crate::style::{MaxTrackSizingFunction, MinTrackSizingFunction};
use core::cmp::max;
use core::ops::Range;

#[derive(Debug)]
pub(in super::super) struct GridItem {
    /// The id of the Node that this item represents
    pub node: Node,

    /// The item's definite row-start and row-end, as resolved by the placement algorithm
    /// (in origin-zero coordinates)
    pub row: Line<i16>,
    /// The items definite column-start and column-end, as resolved by the placement algorithm
    /// (in origin-zero coordinates)
    pub column: Line<i16>,

    /// The item's definite row-start and row-end (same as `row` field, except in a different coordinate system)
    /// (as indexes into the Vec<GridTrack> stored in a grid's GridAxisTracks)
    pub row_indexes: Line<u16>,
    /// The items definite column-start and column-end (same as `column` field, except in a different coordinate system)
    /// (as indexes into the Vec<GridTrack> stored in a grid's GridAxisTracks)
    pub column_indexes: Line<u16>,

    /// Whether the item crosses a flexible row
    pub crosses_flexible_row: bool,
    /// Whether the item crosses a flexible column
    pub crosses_flexible_column: bool,

    /// The order of the item in the children array (this is significant for auto-placement!)
    // pub source_order: u16,

    // Caches for intrinsic size computation
    pub known_dimensions_cache: Option<Size<Option<f32>>>,
    pub min_content_contribution_cache: Option<Size<f32>>,
    pub minimum_contribution_cache: Option<f32>,
    pub max_content_contribution_cache: Option<Size<f32>>,
}

impl GridItem {
    pub fn new_with_placement(node: Node, col_span: Line<i16>, row_span: Line<i16>) -> Self {
        GridItem {
            node,
            row: row_span,
            column: col_span,
            row_indexes: Line { start: 0, end: 0 }, // Properly initialised later
            column_indexes: Line { start: 0, end: 0 }, // Properly initialised later
            crosses_flexible_row: false,            // Properly initialised later
            crosses_flexible_column: false,         // Properly initialised later
            known_dimensions_cache: None,
            min_content_contribution_cache: None,
            max_content_contribution_cache: None,
            minimum_contribution_cache: None,
        }
    }

    pub fn placement(&self, axis: GridAxis) -> Line<i16> {
        match axis {
            GridAxis::Block => self.row,
            GridAxis::Inline => self.column,
        }
    }

    pub fn placement_indexes(&self, axis: GridAxis) -> Line<u16> {
        match axis {
            GridAxis::Block => self.row_indexes,
            GridAxis::Inline => self.column_indexes,
        }
    }

    pub fn track_range_excluding_lines(&self, axis: GridAxis) -> Range<usize> {
        let indexes = self.placement_indexes(axis);
        (indexes.start as usize + 1)..(indexes.end as usize)
    }

    pub fn span(&self, axis: GridAxis) -> u16 {
        match axis {
            GridAxis::Block => max(self.row.end - self.row.start, 0) as u16,
            GridAxis::Inline => max(self.column.end - self.column.start, 0) as u16,
        }
    }

    pub fn crosses_flexible_track(&self, axis: GridAxis) -> bool {
        match axis {
            GridAxis::Inline => self.crosses_flexible_column,
            GridAxis::Block => self.crosses_flexible_row,
        }
    }

    pub fn known_dimensions_cached(
        &mut self,
        axis: GridAxis,
        other_axis_tracks: &[GridTrack],
        other_axis_available_space: AvailableSpace,
        get_track_size_estimate: impl Fn(&GridTrack, AvailableSpace) -> Option<f32>,
    ) -> Size<Option<f32>> {
        self.known_dimensions_cache.unwrap_or_else(|| {
            let item_other_axis_size: Option<f32> = {
                (&other_axis_tracks)[self.track_range_excluding_lines(axis.other())]
                    .iter()
                    .map(|track| get_track_size_estimate(track, other_axis_available_space))
                    .sum::<Option<f32>>()
            };
            let known_dimensions = {
                let mut size = Size::NONE;
                size.set(axis.other(), item_other_axis_size);
                size
            };

            self.known_dimensions_cache = Some(known_dimensions);

            known_dimensions
        })
    }

    // For an item spanning multiple tracks, the upper limit used to calculate its limited min-/max-content contribution is the
    // sum of the fixed max track sizing functions of any tracks it spans, and is applied if it only spans such tracks.
    pub fn spanned_fixed_track_limit(
        &mut self,
        axis: GridAxis,
        axis_tracks: &[GridTrack],
        axis_available_space: AvailableSpace,
    ) -> Option<f32> {
        let spanned_tracks = &axis_tracks[self.track_range_excluding_lines(axis)];
        let tracks_all_fixed = spanned_tracks
            .iter()
            .all(|track| track.max_track_sizing_function.definite_value(axis_available_space).is_some());
        if tracks_all_fixed {
            let limit: f32 = spanned_tracks
                .iter()
                .map(|track| track.max_track_sizing_function.definite_value(axis_available_space).unwrap())
                .sum();
            Some(limit)
        } else {
            None
        }
    }

    pub fn min_content_contribution_cached(
        &mut self,
        tree: &mut impl LayoutTree,
        known_dimensions: Size<Option<f32>>,
    ) -> Size<f32> {
        self.min_content_contribution_cache.unwrap_or_else(|| {
            let size = compute_node_layout(
                tree,
                self.node,
                known_dimensions,
                Size::MIN_CONTENT,
                RunMode::ComputeSize,
                SizingMode::InherentSize,
            );
            self.min_content_contribution_cache = Some(size);
            size
        })
    }

    pub fn max_content_contribution_cached(
        &mut self,
        tree: &mut impl LayoutTree,
        known_dimensions: Size<Option<f32>>,
    ) -> Size<f32> {
        self.max_content_contribution_cache.unwrap_or_else(|| {
            let size = compute_node_layout(
                tree,
                self.node,
                known_dimensions,
                Size::MAX_CONTENT,
                RunMode::ComputeSize,
                SizingMode::InherentSize,
            );
            self.max_content_contribution_cache = Some(size);
            size
        })
    }

    // The minimum contribution of an item is the smallest outer size it can have.
    // Specifically:
    //   - If the item’s computed preferred size behaves as auto or depends on the size of its containing block in the relevant axis:
    //     Its minimum contribution is the outer size that would result from assuming the item’s used minimum size as its preferred size;
    //   - Else the item’s minimum contribution is its min-content contribution.
    // Because the minimum contribution often depends on the size of the item’s content, it is considered a type of intrinsic size contribution.
    pub fn minimum_contribution_cached(
        &mut self,
        tree: &mut impl LayoutTree,
        axis: GridAxis,
        axis_tracks: &[GridTrack],
        available_space: Size<AvailableSpace>,
        known_dimensions: Size<Option<f32>>,
    ) -> f32 {
        self.minimum_contribution_cache.unwrap_or_else(|| {
            let style = tree.style(self.node);
            let axis_available_space = available_space.get(axis).into_option();
            style
                .size
                .get(axis)
                .maybe_resolve(axis_available_space)
                .or_else(|| style.min_size.get(axis).maybe_resolve(axis_available_space))
                .unwrap_or_else(|| {
                    // Automatic minimum size. See https://www.w3.org/TR/css-grid-1/#min-size-auto

                    // To provide a more reasonable default minimum size for grid items, the used value of its automatic minimum size
                    // in a given axis is the content-based minimum size if all of the following are true:
                    let item_axis_tracks = &axis_tracks[self.track_range_excluding_lines(axis)];

                    // it is not a scroll container
                    // TODO: support overflow propety

                    // it spans at least one track in that axis whose min track sizing function is auto
                    let spans_auto_min_track = axis_tracks
                        .iter()
                        // TODO: should this be 'behaves as auto' rather than just literal auto?
                        .any(|track| track.min_track_sizing_function == MinTrackSizingFunction::Auto);

                    // if it spans more than one track in that axis, none of those tracks are flexible
                    let only_span_one_track = item_axis_tracks.len() == 1;
                    let spans_a_flexible_track = axis_tracks
                        .iter()
                        .any(|track| matches!(track.max_track_sizing_function, MaxTrackSizingFunction::Flex(_)));

                    let use_content_based_minimum =
                        spans_auto_min_track && (only_span_one_track || !spans_a_flexible_track);

                    // Otherwise, the automatic minimum size is zero, as usual.
                    if use_content_based_minimum {
                        self.min_content_contribution_cached(tree, known_dimensions).get(axis)
                    } else {
                        0.0
                    }
                })
        })
    }

    pub fn clear_contribution_caches(&mut self) {
        self.known_dimensions_cache = None;
        self.min_content_contribution_cache = None;
        self.max_content_contribution_cache = None;
        self.minimum_contribution_cache = None;
    }
}
