//! Contains GridItem used to represent a single grid item during layout
use super::GridTrack;
use crate::axis::AbstractAxis;
use crate::compute::grid::OriginZeroLine;
use crate::compute::{GenericAlgorithm, LayoutAlgorithm};
use crate::geometry::{Line, Rect, Size};
use crate::layout::SizingMode;
use crate::node::Node;
use crate::prelude::LayoutTree;
use crate::resolve::MaybeResolve;
use crate::style::{AvailableSpace, LengthPercentageAuto, MaxTrackSizingFunction, MinTrackSizingFunction, Style};
use crate::style_helpers::*;
use core::ops::Range;

/// Represents a single grid item
#[derive(Debug)]
pub(in super::super) struct GridItem {
    /// The id of the Node that this item represents
    pub node: Node,

    /// The order of the item in the children array
    ///
    /// We sort the list of grid items during track sizing. This field allows us to sort back the original order
    /// for final positioning
    pub source_order: u16,

    /// The item's definite row-start and row-end, as resolved by the placement algorithm
    /// (in origin-zero coordinates)
    pub row: Line<OriginZeroLine>,
    /// The items definite column-start and column-end, as resolved by the placement algorithm
    /// (in origin-zero coordinates)
    pub column: Line<OriginZeroLine>,

    /// The item's margin style
    pub margin: Rect<LengthPercentageAuto>,

    /// The item's definite row-start and row-end (same as `row` field, except in a different coordinate system)
    /// (as indexes into the Vec<GridTrack> stored in a grid's AbstractAxisTracks)
    pub row_indexes: Line<u16>,
    /// The items definite column-start and column-end (same as `column` field, except in a different coordinate system)
    /// (as indexes into the Vec<GridTrack> stored in a grid's AbstractAxisTracks)
    pub column_indexes: Line<u16>,

    /// Whether the item crosses a flexible row
    pub crosses_flexible_row: bool,
    /// Whether the item crosses a flexible column
    pub crosses_flexible_column: bool,

    // Caches for intrinsic size computation. These caches are only valid for a single run of the track-sizing algorithm.
    /// Cache for the known_dimensions input to intrinsic sizing computation
    pub known_dimensions_cache: Option<Size<Option<f32>>>,
    /// Cache for the min-content size
    pub min_content_contribution_cache: Option<Size<f32>>,
    /// Cache for the minimum contribution
    pub minimum_contribution_cache: Option<f32>,
    /// Cache for the max-content size
    pub max_content_contribution_cache: Option<Size<f32>>,
}

impl GridItem {
    /// Create a new item given a concrete placement in both axes
    pub fn new_with_placement_style_and_order(
        node: Node,
        col_span: Line<OriginZeroLine>,
        row_span: Line<OriginZeroLine>,
        style: &Style,
        source_order: u16,
    ) -> Self {
        GridItem {
            node,
            source_order,
            row: row_span,
            column: col_span,
            margin: style.margin,
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

    /// This item's placement in the specified axis in OriginZero coordinates
    pub fn placement(&self, axis: AbstractAxis) -> Line<OriginZeroLine> {
        match axis {
            AbstractAxis::Block => self.row,
            AbstractAxis::Inline => self.column,
        }
    }

    /// This item's placement in the specified axis as GridTrackVec indices
    pub fn placement_indexes(&self, axis: AbstractAxis) -> Line<u16> {
        match axis {
            AbstractAxis::Block => self.row_indexes,
            AbstractAxis::Inline => self.column_indexes,
        }
    }

    /// Returns a range which can be used as an index into the GridTrackVec in the specified axis
    /// which will produce a sub-slice of covering all the tracks and lines that this item spans
    /// excluding the lines that bound it.
    pub fn track_range_excluding_lines(&self, axis: AbstractAxis) -> Range<usize> {
        let indexes = self.placement_indexes(axis);
        (indexes.start as usize + 1)..(indexes.end as usize)
    }

    /// Returns the number of tracks that this item spans in the specified axis
    pub fn span(&self, axis: AbstractAxis) -> u16 {
        match axis {
            AbstractAxis::Block => self.row.span(),
            AbstractAxis::Inline => self.column.span(),
        }
    }

    /// Returns the pre-computed value indicating whether the grid item crosses a flexible track in
    /// the specified axis
    pub fn crosses_flexible_track(&self, axis: AbstractAxis) -> bool {
        match axis {
            AbstractAxis::Inline => self.crosses_flexible_column,
            AbstractAxis::Block => self.crosses_flexible_row,
        }
    }

    /// Compute the known_dimensions to be passed to the child sizing functions
    /// These are estimates based on either the max track sizing function on the provisional base size in the opposite
    /// axis to the one currently being sized.
    /// https://www.w3.org/TR/css-grid-1/#algo-overview
    pub fn known_dimensions_cached(
        &mut self,
        axis: AbstractAxis,
        other_axis_tracks: &[GridTrack],
        other_axis_available_space: AvailableSpace,
        get_track_size_estimate: impl Fn(&GridTrack, AvailableSpace) -> Option<f32>,
    ) -> Size<Option<f32>> {
        self.known_dimensions_cache.unwrap_or_else(|| {
            let item_other_axis_size: Option<f32> = {
                other_axis_tracks[self.track_range_excluding_lines(axis.other())]
                    .iter()
                    .map(|track| {
                        get_track_size_estimate(track, other_axis_available_space)
                            .map(|size| size + track.content_alignment_adjustment)
                    })
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

    /// For an item spanning multiple tracks, the upper limit used to calculate its limited min-/max-content contribution is the
    /// sum of the fixed max track sizing functions of any tracks it spans, and is applied if it only spans such tracks.
    pub fn spanned_fixed_track_limit(
        &mut self,
        axis: AbstractAxis,
        axis_tracks: &[GridTrack],
        axis_available_space: AvailableSpace,
    ) -> Option<f32> {
        let spanned_tracks = &axis_tracks[self.track_range_excluding_lines(axis)];
        let tracks_all_fixed = spanned_tracks
            .iter()
            .all(|track| track.max_track_sizing_function.definite_limit(axis_available_space).is_some());
        if tracks_all_fixed {
            let limit: f32 = spanned_tracks
                .iter()
                .map(|track| track.max_track_sizing_function.definite_limit(axis_available_space).unwrap())
                .sum();
            Some(limit)
        } else {
            None
        }
    }

    /// Retrieve the item's min content contribution from the cache or compute it using the provided parameters
    pub fn min_content_contribution_cached(
        &mut self,
        tree: &mut impl LayoutTree,
        known_dimensions: Size<Option<f32>>,
        inner_node_size: Size<Option<f32>>,
    ) -> Size<f32> {
        self.min_content_contribution_cache.unwrap_or_else(|| {
            let size = GenericAlgorithm::measure_size(
                tree,
                self.node,
                known_dimensions,
                inner_node_size,
                Size::MIN_CONTENT,
                SizingMode::InherentSize,
            );
            self.min_content_contribution_cache = Some(size);
            size
        })
    }

    /// Retrieve the item's max content contribution from the cache or compute it using the provided parameters
    pub fn max_content_contribution_cached(
        &mut self,
        tree: &mut impl LayoutTree,
        known_dimensions: Size<Option<f32>>,
        inner_node_size: Size<Option<f32>>,
    ) -> Size<f32> {
        self.max_content_contribution_cache.unwrap_or_else(|| {
            let size = GenericAlgorithm::measure_size(
                tree,
                self.node,
                known_dimensions,
                inner_node_size,
                Size::MAX_CONTENT,
                SizingMode::InherentSize,
            );
            self.max_content_contribution_cache = Some(size);
            size
        })
    }

    /// The minimum contribution of an item is the smallest outer size it can have.
    /// Specifically:
    ///   - If the item’s computed preferred size behaves as auto or depends on the size of its containing block in the relevant axis:
    ///     Its minimum contribution is the outer size that would result from assuming the item’s used minimum size as its preferred size;
    ///   - Else the item’s minimum contribution is its min-content contribution.
    /// Because the minimum contribution often depends on the size of the item’s content, it is considered a type of intrinsic size contribution.
    pub fn minimum_contribution_cached(
        &mut self,
        tree: &mut impl LayoutTree,
        axis: AbstractAxis,
        axis_tracks: &[GridTrack],
        available_space: Size<AvailableSpace>,
        known_dimensions: Size<Option<f32>>,
        inner_node_size: Size<Option<f32>>,
    ) -> f32 {
        self.minimum_contribution_cache.unwrap_or_else(|| {
            let style = tree.style(self.node);
            style
                .size
                .maybe_resolve(available_space.into_options())
                .maybe_apply_aspect_ratio(style.aspect_ratio)
                .get(axis)
                .or_else(|| {
                    style
                        .min_size
                        .maybe_resolve(available_space.into_options())
                        .maybe_apply_aspect_ratio(style.aspect_ratio)
                        .get(axis)
                })
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
                        self.min_content_contribution_cached(tree, known_dimensions, inner_node_size).get(axis)
                    } else {
                        0.0
                    }
                })
        })
    }

    /// Clears the per-track-sizing-alogrithm-run caches
    pub fn clear_contribution_caches(&mut self) {
        self.known_dimensions_cache = None;
        self.min_content_contribution_cache = None;
        self.max_content_contribution_cache = None;
        self.minimum_contribution_cache = None;
    }
}
