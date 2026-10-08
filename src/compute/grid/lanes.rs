//! Grid Lanes layout: <https://drafts.csswg.org/css-grid-3/>
//!
//! Tracks exist in one axis only (the "grid axis"); items are packed along the other axis (the
//! "stacking axis") by placing each one into the shortest run of tracks it can span.
//! Shares the explicit-grid, track-sizing and item-positioning machinery of [`super::compute_grid_layout`].
//!
//! Current limitations: no dense packing (§4.3), no stacking-axis alignment other than start (§6.3/§6.4), no baselines
//! (§6.5) and no detailed layout info.
use super::alignment::{align_and_position_item, align_tracks};
use super::explicit_grid::{compute_explicit_grid_size_in_axis, initialize_grid_tracks, AutoRepeatStrategy};
use super::placement::ItemPlacement;
use super::track_sizing::{
    determine_if_item_crosses_flexible_or_intrinsic_tracks, resolve_item_track_indexes, track_sizing_algorithm,
};
use super::types::{GridItem, GridTrack, NamedLineResolver, OriginZeroLine, TrackCounts};
use super::{compute_container_constants, resolve_static_position_grid_area, GridContainerConstants, MAX_GRID_TRACKS};
use crate::geometry::{AbsoluteAxis, AbstractAxis, InBothAbsAxis, Line, Point, Rect, Size};
use crate::style::{AlignSelf, OriginZeroGridPlacement};
use crate::tree::{
    AxisStaticPosition, Baselines, Layout, LayoutInput, LayoutOutput, LayoutPartialTreeExt, NodeId, OofCandidate,
    OofCandidates, OofPositioningArea, RunMode, SizingMode,
};
use crate::util::sys::{f32_max, f32_min, GridTrackVec, Vec};
use crate::util::{MaybeMath, MaybeResolve, ResolveOrZero};
use crate::{
    style_helpers::*, BoxGenerationMode, BoxSizing, CoreStyle, GridContainerStyle, GridItemStyle, LayoutGridContainer,
    LengthPercentage, MaxTrackSizingFunction, MinTrackSizingFunction, Overflow, RequestedAxis,
};

/// Where an item ended up in the grid axis and the stacking axis
#[derive(Clone, Copy)]
struct LanePlacement {
    /// Index of the item's first grid-axis track
    start_track: usize,
    /// Number of grid-axis tracks spanned
    span: usize,
    /// Stacking-axis offset of the item's margin box from the container's content box start
    position: f32,
    /// Stacking-axis size of the item's margin box (floored at zero)
    outer_size: f32,
}

/// The abstract axis corresponding to an absolute axis (horizontal writing mode)
#[inline(always)]
fn abstract_axis(axis: AbsoluteAxis) -> AbstractAxis {
    match axis {
        AbsoluteAxis::Horizontal => AbstractAxis::Inline,
        AbsoluteAxis::Vertical => AbstractAxis::Block,
    }
}

/// Sum of the two edges of a rect in the given axis
#[inline(always)]
fn axis_sum(rect: Rect<f32>, axis: AbsoluteAxis) -> f32 {
    match axis {
        AbsoluteAxis::Horizontal => rect.horizontal_axis_sum(),
        AbsoluteAxis::Vertical => rect.vertical_axis_sum(),
    }
}

/// Largest running position among the tracks `start..start + span`
#[inline(always)]
fn max_running_position(running_positions: &[f32], start: usize, span: usize) -> f32 {
    running_positions[start..start + span].iter().copied().fold(0.0, f32_max)
}

/// The largest grid-axis size contributions (margins included) of the auto-placed items sharing a span.
/// One virtual item per possible start line stands in for the group during track sizing (spec §3.4.2).
struct ItemGroup {
    /// Node of one of the group's items. Never laid out: the virtual items' contribution caches are pre-filled.
    node: NodeId,
    /// Grid-axis span shared by the items
    span: u16,
    /// Largest min-content contribution
    min_content: f32,
    /// Largest max-content contribution
    max_content: f32,
    /// Largest minimum contribution among items whose minimum does not depend on the spanned tracks
    /// (definite preferred or minimum size, or scroll containers)
    explicit_minimum: f32,
    /// Largest content-based automatic minimum among items whose minimum depends on the spanned tracks
    content_minimum: Option<ContentMinimum>,
}

/// A content-based automatic minimum before the fixed-track limit clamp, with the margin and the
/// padding + border (the clamp's floor) that go with it
#[derive(Copy, Clone)]
struct ContentMinimum {
    /// Content-based minimum contribution
    contribution: f32,
    /// Grid-axis margin sum
    margin: f32,
    /// Grid-axis padding + border sum
    padding_border: f32,
}

/// Group the auto-placed items by span and measure their grid-axis contributions (spec §3.4.2 steps 1 and 2).
/// Items are measured with an indefinite grid area, as they would be when contributing to intrinsic tracks.
fn collect_item_groups(
    tree: &mut impl LayoutGridContainer,
    items: &mut [GridItem],
    g_axis: AbstractAxis,
    inner_node_size: Size<Option<f32>>,
) -> Vec<ItemGroup> {
    let mut groups: Vec<ItemGroup> = Vec::new();
    for item in items.iter_mut() {
        let span = item.span(g_axis);
        let margin = item.margins_axis_sums_with_baseline_shims(None, tree).get(g_axis);
        // Contributions are measured under min-/max-content in the grid axis, as in the shared track sizing, and at
        // the stacking-axis content-box size when it is definite, as the definitely placed items are
        let available_space = inner_node_size.with(g_axis, None);
        let grid_area_size = Size::NONE.with(g_axis.other(), inner_node_size.get(g_axis.other()));
        let min_content = item.min_content_contribution_cached(g_axis, tree, grid_area_size, available_space);
        let max_content = item.max_content_contribution_cached(g_axis, tree, grid_area_size, available_space);
        let padding_border = item.padding_border_size(tree, Size::NONE);
        let explicit_minimum = item.explicit_minimum_contribution(tree, g_axis, Size::NONE, padding_border);
        let content_minimum = if explicit_minimum.is_some() {
            None
        } else {
            // Compressible replaced elements cap their content-based minimum at their definite preferred and
            // maximum sizes, with indefinite percentages resolved against zero
            let mut content_minimum = min_content;
            if item.is_compressible_replaced {
                let size = item.size.get(g_axis).maybe_resolve(Some(0.0), |val, basis| tree.calc(val, basis));
                let max_size = item.max_size.get(g_axis).maybe_resolve(Some(0.0), |val, basis| tree.calc(val, basis));
                content_minimum = content_minimum.maybe_min(size).maybe_min(max_size);
            }
            Some(ContentMinimum { contribution: content_minimum, margin, padding_border: padding_border.get(g_axis) })
        };

        let group = match groups.iter_mut().position(|group| group.span == span) {
            Some(index) => &mut groups[index],
            None => {
                groups.push(ItemGroup {
                    node: item.node,
                    span,
                    min_content: 0.0,
                    max_content: 0.0,
                    explicit_minimum: 0.0,
                    content_minimum: None,
                });
                groups.last_mut().unwrap()
            }
        };
        group.min_content = f32_max(group.min_content, min_content + margin);
        group.max_content = f32_max(group.max_content, max_content + margin);
        if let Some(explicit_minimum) = explicit_minimum {
            group.explicit_minimum = f32_max(group.explicit_minimum, explicit_minimum + margin);
        }
        if let Some(content_minimum) = content_minimum {
            group.content_minimum = Some(match group.content_minimum {
                Some(largest) => ContentMinimum {
                    contribution: f32_max(largest.contribution, content_minimum.contribution),
                    margin: f32_max(largest.margin, content_minimum.margin),
                    padding_border: f32_max(largest.padding_border, content_minimum.padding_border),
                },
                None => content_minimum,
            });
        }
    }
    groups
}

impl ItemGroup {
    /// Synthesize the group's virtual item for the grid-axis tracks `g_lines` (spec §3.4.2 step 3). The item has
    /// no styles of its own: the contribution caches are pre-filled, so track sizing never reaches the tree.
    #[allow(clippy::too_many_arguments)]
    fn virtual_item(
        &self,
        tree: &mut impl LayoutGridContainer,
        grid_axis: AbsoluteAxis,
        g_axis: AbstractAxis,
        g_tracks: &[GridTrack],
        g_lines: Line<OriginZeroLine>,
        g_indexes: Line<u16>,
        s_lines: Line<OriginZeroLine>,
        inner_node_size: Size<Option<f32>>,
    ) -> GridItem {
        let s_indexes = Line { start: 0, end: 2 };
        let (row, column, row_indexes, column_indexes) = match grid_axis {
            AbsoluteAxis::Horizontal => (s_lines, g_lines, s_indexes, g_indexes),
            AbsoluteAxis::Vertical => (g_lines, s_lines, g_indexes, s_indexes),
        };
        let mut item = GridItem {
            node: self.node,
            source_order: u16::MAX,
            row,
            column,
            is_compressible_replaced: false,
            overflow: Point { x: Overflow::Visible, y: Overflow::Visible },
            box_sizing: BoxSizing::BorderBox,
            size: Size::auto(),
            min_size: Size::auto(),
            max_size: Size::auto(),
            aspect_ratio: None,
            padding: Rect::zero(),
            border: Rect::zero(),
            margin: Rect::zero(),
            align_self: AlignSelf::START,
            justify_self: AlignSelf::START,
            baseline: None,
            baseline_shim: 0.0,
            row_indexes,
            column_indexes,
            crosses_flexible_row: false,
            crosses_flexible_column: false,
            crosses_intrinsic_row: false,
            crosses_intrinsic_column: false,
            grid_area_size_cache: None,
            known_dimensions_cache: None,
            min_content_contribution_cache: Size::NONE.with(g_axis, Some(self.min_content)),
            minimum_contribution_cache: Size::NONE,
            max_content_contribution_cache: Size::NONE.with(g_axis, Some(self.max_content)),
            y_position: 0.0,
            height: 0.0,
            oof_candidates: OofCandidates::NONE,
        };

        // The automatic minimum size depends on the spanned tracks (css-grid-1 §6.6), so it is resolved per copy:
        // the content-based minimum applies if the item spans an auto-min track and, when spanning several
        // tracks, no flexible track. It is clamped by the sum of fixed max track sizing functions, floored at the
        // item's padding + border.
        let spanned_tracks = &g_tracks[item.track_range_excluding_lines(g_axis)];
        let spans_auto_min_track = spanned_tracks
            .iter()
            .any(|track| track.min_track_sizing_function.behaves_as_auto(inner_node_size.get(g_axis)));
        let use_content_based_minimum = spans_auto_min_track
            && (spanned_tracks.len() == 1
                || !spanned_tracks.iter().any(|track| track.max_track_sizing_function.is_fr()));
        let minimum = match self.content_minimum {
            Some(ContentMinimum { contribution, margin, padding_border }) if use_content_based_minimum => {
                let limit =
                    item.spanned_fixed_track_limit(g_axis, g_tracks, inner_node_size.get(g_axis), &|val, basis| {
                        tree.resolve_calc_value(val, basis)
                    });
                f32_max(
                    self.explicit_minimum,
                    contribution.maybe_min(limit.map(|limit| f32_max(limit, padding_border))) + margin,
                )
            }
            _ => self.explicit_minimum,
        };
        item.minimum_contribution_cache.set(g_axis, Some(minimum));
        item
    }
}

/// Grid Lanes layout algorithm
pub fn compute_grid_lanes_layout<Tree: LayoutGridContainer>(
    tree: &mut Tree,
    node: NodeId,
    inputs: LayoutInput,
) -> LayoutOutput {
    let LayoutInput { known_dimensions, run_mode, .. } = inputs;
    let style = tree.get_grid_container_style(node);
    let constants = compute_container_constants(tree, &style, inputs);
    let GridContainerConstants {
        direction,
        contain,
        containing_block_claims,
        padding,
        border,
        padding_border_size,
        min_size,
        max_size,
        preferred_size,
        scrollbar_gutter,
        #[cfg(feature = "content_size")]
        is_scroll_container,
        content_box_inset,
        align_content,
        justify_content,
        align_items,
        justify_items,
        available_grid_space,
        outer_node_size,
        inner_min_size,
        inner_max_size,
        mut inner_node_size,
    } = constants;

    let grid_axis = style.grid_lanes_direction().grid_axis();
    let stacking_axis = style.grid_lanes_direction().stacking_axis();
    let g_axis = abstract_axis(grid_axis);
    let s_axis = abstract_axis(stacking_axis);
    let fit_tolerance = style.fit_tolerance();
    let stacking_gap_style: LengthPercentage = match stacking_axis {
        AbsoluteAxis::Horizontal => style.gap().width,
        AbsoluteAxis::Vertical => style.gap().height,
    };
    let (g_align_content, s_align_content) = match grid_axis {
        AbsoluteAxis::Horizontal => (justify_content, align_content),
        AbsoluteAxis::Vertical => (align_content, justify_content),
    };

    if run_mode == RunMode::ComputeSize {
        if let Size { width: Some(width), height: Some(height) } = outer_node_size {
            return LayoutOutput::from_outer_size(Size { width, height });
        }
        match inputs.axis {
            RequestedAxis::Horizontal => {
                if let Some(width) = outer_node_size.width {
                    return LayoutOutput::from_outer_size(Size { width, height: 0.0 });
                }
            }
            RequestedAxis::Vertical => {
                if let Some(height) = outer_node_size.height {
                    return LayoutOutput::from_outer_size(Size { width: 0.0, height });
                }
            }
            RequestedAxis::Both => {}
        }
    }

    let auto_fit_container_size = outer_node_size
        .or(max_size)
        .or(min_size)
        .maybe_clamp(min_size, max_size)
        .maybe_max(padding_border_size)
        .maybe_sub(content_box_inset.sum_axes());

    // If the grid container has a definite size or max size in the relevant axis:
    //   - then the number of repetitions is the largest possible positive integer that does not cause the grid to overflow the content
    //     box of its grid container.
    // Otherwise, if the grid container has a definite min size in the relevant axis:
    //   - then the number of repetitions is the smallest possible positive integer that fulfills that minimum requirement
    // Otherwise, the specified track list repeats only once.
    let auto_repeat_fit_strategy = outer_node_size.or(max_size).map(|val| match val {
        Some(_) => AutoRepeatStrategy::MaxRepetitionsThatDoNotOverflow,
        None => AutoRepeatStrategy::MinRepetitionsThatDoOverflow,
    });

    // Compute the number of rows and columns in the explicit grid *template*
    // (explicit tracks from grid_areas are computed separately below)

    // 2. Resolve the explicit grid in the grid axis. The stacking axis has a single implicit "track".
    let (g_auto_repetition_count, g_template_track_count) = compute_explicit_grid_size_in_axis(
        &style,
        auto_fit_container_size.get(g_axis),
        auto_repeat_fit_strategy.get(g_axis),
        |val, basis| tree.calc(val, basis),
        grid_axis,
    );
    let (col_auto_repetition_count, row_auto_repetition_count) = match grid_axis {
        AbsoluteAxis::Horizontal => (g_auto_repetition_count, 0),
        AbsoluteAxis::Vertical => (0, g_auto_repetition_count),
    };
    let mut name_resolver = NamedLineResolver::new(&style, col_auto_repetition_count, row_auto_repetition_count);
    let explicit_g_count = match grid_axis {
        AbsoluteAxis::Horizontal => g_template_track_count.max(name_resolver.area_column_count()),
        AbsoluteAxis::Vertical => g_template_track_count.max(name_resolver.area_row_count()),
    }
    .min(MAX_GRID_TRACKS);
    match grid_axis {
        AbsoluteAxis::Horizontal => name_resolver.set_explicit_column_count(explicit_g_count),
        AbsoluteAxis::Vertical => name_resolver.set_explicit_row_count(explicit_g_count),
    }

    // 3. Create the grid items and resolve their grid-axis placement. Items with a definite grid-axis
    // position come first so that they alone can be passed to the track sizing algorithm.
    let child_count = tree.child_count(node);
    let mut items: Vec<GridItem> = Vec::with_capacity(child_count);
    let mut placements: Vec<ItemPlacement> = Vec::with_capacity(child_count);
    let mut definite_count = 0usize;
    for (index, child_node) in tree.child_ids(node).enumerate() {
        let child_style = tree.get_grid_child_style(child_node);
        if child_style.box_generation_mode() == BoxGenerationMode::None || child_style.position().is_out_of_flow() {
            continue;
        }
        let g_placement = match grid_axis {
            AbsoluteAxis::Horizontal => name_resolver.resolve_column_names(&child_style.grid_column()),
            AbsoluteAxis::Vertical => name_resolver.resolve_row_names(&child_style.grid_row()),
        }
        .into_origin_zero(explicit_g_count);
        let s_placement = Line { start: OriginZeroGridPlacement::Auto, end: OriginZeroGridPlacement::Auto };
        let placement = match grid_axis {
            AbsoluteAxis::Horizontal => InBothAbsAxis { horizontal: g_placement, vertical: s_placement },
            AbsoluteAxis::Vertical => InBothAbsAxis { horizontal: s_placement, vertical: g_placement },
        };
        let mut item =
            GridItem::new_with_style_and_order(child_node, child_style, align_items, justify_items, index as u16);
        // Items are not stretched in the stacking axis when measured: they take their max-content size
        let s_self_alignment = match stacking_axis {
            AbsoluteAxis::Horizontal => &mut item.justify_self,
            AbsoluteAxis::Vertical => &mut item.align_self,
        };
        if s_self_alignment.is_stretch_or_normal() {
            *s_self_alignment = AlignSelf::START;
        }
        if g_placement.is_definite() {
            items.insert(definite_count, item);
            placements.insert(definite_count, placement);
            definite_count += 1;
        } else {
            items.push(item);
            placements.push(placement);
        }
    }

    // 4. Grid-axis track counts: definitely placed items may create implicit tracks, auto-placed items never do.
    // A grid axis with no explicit tracks still gets one implicit track for auto-placed items.
    let mut min_line: i16 = 0;
    let mut max_line: i16 = (explicit_g_count as i16).max(1);
    for placement in placements[..definite_count].iter() {
        let lines = placement.get(grid_axis).resolve_definite_grid_lines();
        min_line = min_line.min(lines.start.0);
        max_line = max_line.max(lines.end.0);
    }
    let g_counts =
        TrackCounts::from_raw((-min_line) as u16, explicit_g_count, (max_line - explicit_g_count as i16) as u16);
    let s_counts = TrackCounts::from_raw(0, 1, 0);
    let track_count = (g_counts.negative_implicit + g_counts.explicit + g_counts.positive_implicit) as usize;
    let (col_counts, row_counts) = match grid_axis {
        AbsoluteAxis::Horizontal => (g_counts, s_counts),
        AbsoluteAxis::Vertical => (s_counts, g_counts),
    };

    // 5. Initialize tracks. The stacking axis gets one auto track so that items see an indefinite
    // stacking-axis grid area during track sizing.
    let mut g_tracks = GridTrackVec::new();
    initialize_grid_tracks(&mut g_tracks, g_counts, &style, grid_axis, g_auto_repetition_count, |_| true);
    let mut s_tracks: GridTrackVec<GridTrack> = GridTrackVec::new();
    s_tracks.push(GridTrack::gutter(LengthPercentage::ZERO));
    s_tracks.push(GridTrack::new(MinTrackSizingFunction::AUTO, MaxTrackSizingFunction::AUTO));
    s_tracks.push(GridTrack::gutter(LengthPercentage::ZERO));

    drop(style);

    // Definite items get their resolved grid-axis lines; auto items get a provisional placement at the first
    // line (overwritten once they are placed below) so that track indexes can be resolved for every item.
    for (item, placement) in items.iter_mut().zip(placements.iter()) {
        let g_placement = placement.get(grid_axis);
        let g_lines = if g_placement.is_definite() {
            g_placement.resolve_definite_grid_lines()
        } else {
            let span = (g_placement.indefinite_span() as usize).clamp(1, track_count.max(1)) as u16;
            Line { start: g_counts.track_to_prev_oz_line(0), end: g_counts.track_to_prev_oz_line(span) }
        };
        let s_lines = Line { start: s_counts.track_to_prev_oz_line(0), end: s_counts.track_to_prev_oz_line(1) };
        match grid_axis {
            AbsoluteAxis::Horizontal => {
                item.column = g_lines;
                item.row = s_lines;
            }
            AbsoluteAxis::Vertical => {
                item.row = g_lines;
                item.column = s_lines;
            }
        }
    }
    drop(placements);

    // Auto-placed items are assumed to be placed at every possible start line for track sizing (spec §3.4).
    // Rather than copying every item, one virtual item per span group stands in for the group at each start
    // line (§3.4.2). Virtual items only matter if some grid-axis track is intrinsically sized, which includes
    // percentage tracks while the container's grid-axis size is indefinite (track sizing treats them as auto).
    let s_lines = Line { start: s_counts.track_to_prev_oz_line(0), end: s_counts.track_to_prev_oz_line(1) };
    let mut virtual_count = 0usize;
    let g_size_indefinite = inner_node_size.get(g_axis).is_none();
    if g_tracks
        .iter()
        .any(|track| track.has_intrinsic_sizing_function() || (g_size_indefinite && track.uses_percentage()))
    {
        let groups = collect_item_groups(tree, &mut items[definite_count..], g_axis, inner_node_size);
        for group in groups.iter() {
            let span = (group.span as usize).clamp(1, track_count.max(1));
            virtual_count += track_count.saturating_sub(span) + 1;
        }
        let mut virtual_items: Vec<GridItem> = Vec::with_capacity(virtual_count);
        for group in groups.iter() {
            let span = (group.span as usize).clamp(1, track_count.max(1));
            for start in 0..=track_count.saturating_sub(span) {
                let g_lines = Line {
                    start: g_counts.track_to_prev_oz_line(start as u16),
                    end: g_counts.track_to_prev_oz_line((start + span) as u16),
                };
                let g_indexes = Line { start: (2 * start) as u16, end: (2 * (start + span)) as u16 };
                virtual_items.push(group.virtual_item(
                    tree,
                    grid_axis,
                    g_axis,
                    &g_tracks,
                    g_lines,
                    g_indexes,
                    s_lines,
                    inner_node_size,
                ));
            }
        }
        items.splice(definite_count..definite_count, virtual_items);
    }
    let sizing_count = definite_count + virtual_count;

    resolve_item_track_indexes(&mut items, col_counts, row_counts);
    match grid_axis {
        AbsoluteAxis::Horizontal => {
            determine_if_item_crosses_flexible_or_intrinsic_tracks(&mut items, &g_tracks, &s_tracks)
        }
        AbsoluteAxis::Vertical => {
            determine_if_item_crosses_flexible_or_intrinsic_tracks(&mut items, &s_tracks, &g_tracks)
        }
    }

    // 6. Size the grid-axis tracks from the definitely placed items and the virtual items
    // Items are measured at the stacking-axis content-box size when it is definite, as grid items are measured at
    // their column sizes once those are known
    let s_inner_size = inner_node_size.get(s_axis);
    let s_track_size_estimate: fn(&GridTrack, Option<f32>, &Tree) -> Option<f32> = match s_inner_size {
        Some(size) => {
            s_tracks[1].base_size = size;
            |track, _, _| Some(track.base_size)
        }
        None => |track, parent_size, tree| {
            track.max_track_sizing_function.definite_value(parent_size, |val, basis| tree.calc(val, basis))
        },
    };
    track_sizing_algorithm(
        tree,
        g_axis,
        inner_min_size.get(g_axis),
        inner_max_size.get(g_axis),
        g_align_content,
        s_align_content,
        available_grid_space,
        inner_node_size,
        &mut g_tracks,
        &mut s_tracks,
        &mut items[..sizing_count],
        s_track_size_estimate,
        false,
    );
    items.drain(definite_count..sizing_count);
    let g_track_sum = g_tracks.iter().map(|track| track.base_size).sum::<f32>();

    // The container's grid-axis size is now final
    let resolved_style_size = known_dimensions.or(preferred_size);
    let g_border_box = resolved_style_size
        .get(g_axis)
        .unwrap_or(g_track_sum + axis_sum(content_box_inset, grid_axis))
        .maybe_clamp(min_size.get(g_axis), max_size.get(g_axis))
        .max(padding_border_size.get(g_axis));
    let g_content_box = f32_max(0.0, g_border_box - axis_sum(content_box_inset, grid_axis));
    inner_node_size.set(g_axis, Some(g_content_box));

    // Percentage track sizes and gaps resolve against the now-definite content box, so track sizing is re-run once
    // when the container's grid-axis size was indefinite, as the grid algorithm does
    if outer_node_size.get(g_axis).is_none() && g_tracks.iter().any(|track| track.uses_percentage()) {
        for item in items[..definite_count].iter_mut() {
            item.grid_area_size_cache = None;
            item.min_content_contribution_cache.set(g_axis, None);
            item.max_content_contribution_cache.set(g_axis, None);
            item.minimum_contribution_cache.set(g_axis, None);
        }
        track_sizing_algorithm(
            tree,
            g_axis,
            inner_min_size.get(g_axis),
            inner_max_size.get(g_axis),
            g_align_content,
            s_align_content,
            available_grid_space,
            inner_node_size,
            &mut g_tracks,
            &mut s_tracks,
            &mut items[..sizing_count],
            s_track_size_estimate,
            false,
        );
    }

    let only_grid_axis_requested = matches!(
        (inputs.axis, grid_axis),
        (RequestedAxis::Horizontal, AbsoluteAxis::Horizontal) | (RequestedAxis::Vertical, AbsoluteAxis::Vertical)
    );
    if run_mode == RunMode::ComputeSize && only_grid_axis_requested {
        let mut size = Size::ZERO;
        size.set(g_axis, g_border_box);
        return LayoutOutput::from_outer_size(size);
    }

    // 7. Place items along the stacking axis (spec §4.4), measuring each item's stacking-axis size as it is placed
    let tie_threshold = fit_tolerance.resolve_or_zero(Some(g_content_box), |val, basis| tree.calc(val, basis));
    let stacking_gap =
        stacking_gap_style.resolve_or_zero(inner_node_size.get(s_axis), |val, basis| tree.calc(val, basis));
    let mut running_positions: Vec<f32> = Vec::with_capacity(track_count);
    running_positions.resize(track_count, 0.0);
    let mut cursor: usize = 0;
    let mut stacking_range_end: f32 = 0.0;
    let mut lane_placements: Vec<LanePlacement> = Vec::with_capacity(items.len());
    lane_placements.resize(items.len(), LanePlacement { start_track: 0, span: 0, position: 0.0, outer_size: 0.0 });
    // Items are placed in document order (the definite-first order of `items` only serves track sizing)
    let mut document_order: Vec<usize> = (0..items.len()).collect();
    document_order.sort_unstable_by_key(|&index| items[index].source_order);
    for &index in document_order.iter() {
        let item = &mut items[index];
        let indexes = match grid_axis {
            AbsoluteAxis::Horizontal => item.column_indexes,
            AbsoluteAxis::Vertical => item.row_indexes,
        };
        let span = ((indexes.end - indexes.start) / 2) as usize;
        let start_track = if index < definite_count {
            (indexes.start / 2) as usize
        } else {
            // Candidate start lines are those within the tie threshold of the lowest running position;
            // the first candidate at or after the cursor wins, otherwise the first candidate.
            let last_start = track_count - span;
            let lowest = (0..=last_start)
                .map(|start| max_running_position(&running_positions, start, span))
                .fold(f32::INFINITY, f32_min);
            let mut candidates = (0..=last_start)
                .filter(|&start| max_running_position(&running_positions, start, span) <= lowest + tie_threshold);
            let first_candidate = candidates.next().unwrap_or(0);
            let start = if first_candidate >= cursor {
                first_candidate
            } else {
                candidates.find(|&start| start >= cursor).unwrap_or(first_candidate)
            };
            cursor = start + span;
            let g_lines = Line {
                start: g_counts.track_to_prev_oz_line(start as u16),
                end: g_counts.track_to_prev_oz_line((start + span) as u16),
            };
            let g_indexes = Line { start: (2 * start) as u16, end: (2 * (start + span)) as u16 };
            match grid_axis {
                AbsoluteAxis::Horizontal => {
                    item.column = g_lines;
                    item.column_indexes = g_indexes;
                }
                AbsoluteAxis::Vertical => {
                    item.row = g_lines;
                    item.row_indexes = g_indexes;
                }
            }
            start
        };
        let position = max_running_position(&running_positions, start_track, span);

        // The item's containing block is its grid area in the grid axis and the container's content box in the stacking axis
        let g_area_size: f32 =
            g_tracks[2 * start_track + 1..2 * (start_track + span)].iter().map(|track| track.base_size).sum();
        let mut grid_area_size = inner_node_size;
        grid_area_size.set(g_axis, Some(g_area_size));
        let margins = item.margins_axis_sums_with_baseline_shims(grid_area_size.width, tree);
        let outer_size = f32_max(
            0.0,
            item.max_content_contribution(s_axis, tree, grid_area_size, grid_area_size) + margins.get(s_axis),
        );

        let new_position = position + outer_size + stacking_gap;
        running_positions[start_track..start_track + span].iter_mut().for_each(|pos| *pos = new_position);
        stacking_range_end = f32_max(stacking_range_end, position + outer_size);
        lane_placements[index] = LanePlacement { start_track, span, position, outer_size };
    }
    drop(document_order);

    // 8. The container's stacking-axis size (spec §5: the stacking range when indefinite)
    let s_border_box = resolved_style_size
        .get(s_axis)
        .unwrap_or(stacking_range_end + axis_sum(content_box_inset, stacking_axis))
        .maybe_clamp(min_size.get(s_axis), max_size.get(s_axis))
        .max(padding_border_size.get(s_axis));
    let mut container_border_box = Size::ZERO;
    container_border_box.set(g_axis, g_border_box);
    container_border_box.set(s_axis, s_border_box);
    if run_mode == RunMode::ComputeSize {
        return LayoutOutput::from_outer_size(container_border_box);
    }
    let s_content_box = f32_max(0.0, s_border_box - axis_sum(content_box_inset, stacking_axis));
    inner_node_size.set(s_axis, Some(s_content_box));

    // 9. Align the grid-axis tracks and position the items
    let inline_size_without_scrollbar = f32_max(container_border_box.width - padding_border_size.width, 0.0);
    let inline_scrollbar_gutter_for_alignment = f32_min(scrollbar_gutter.x, inline_size_without_scrollbar);
    match grid_axis {
        AbsoluteAxis::Horizontal => align_tracks(
            g_content_box,
            Line {
                start: padding.left + if direction.is_rtl() { inline_scrollbar_gutter_for_alignment } else { 0.0 },
                end: padding.right + if direction.is_rtl() { 0.0 } else { inline_scrollbar_gutter_for_alignment },
            },
            Line { start: border.left, end: border.right },
            &mut g_tracks,
            g_align_content,
            direction.is_rtl(),
        ),
        AbsoluteAxis::Vertical => align_tracks(
            g_content_box,
            Line { start: padding.top, end: padding.bottom },
            Line { start: border.top, end: border.bottom },
            &mut g_tracks,
            g_align_content,
            false,
        ),
    }
    // Items stack from the content box's start edge; a horizontal stacking axis follows the container's direction
    let stacking_reversed = stacking_axis == AbsoluteAxis::Horizontal && direction.is_rtl();
    let stacking_origin = match stacking_axis {
        AbsoluteAxis::Vertical => border.top + padding.top,
        AbsoluteAxis::Horizontal if stacking_reversed => {
            container_border_box.width - border.right - padding.right - inline_scrollbar_gutter_for_alignment
        }
        AbsoluteAxis::Horizontal => border.left + padding.left,
    };

    #[cfg_attr(not(feature = "content_size"), allow(unused_mut))]
    let mut item_overflow_rect = Rect::ZERO;
    let mut oof_candidates = OofCandidates::new();
    let container_alignment_styles = InBothAbsAxis { horizontal: justify_items, vertical: align_items };
    for (index, (item, lane)) in items.iter_mut().zip(lane_placements.iter()).enumerate() {
        let start_index = 2 * lane.start_track;
        let end_index = 2 * (lane.start_track + lane.span);
        let g_area = if grid_axis == AbsoluteAxis::Horizontal && direction.is_rtl() {
            Line { start: g_tracks[end_index - 1].offset, end: g_tracks[start_index].offset }
        } else {
            Line { start: g_tracks[start_index + 1].offset, end: g_tracks[end_index].offset }
        };
        let s_area = if stacking_reversed {
            Line { start: stacking_origin - lane.position - lane.outer_size, end: stacking_origin - lane.position }
        } else {
            Line { start: stacking_origin + lane.position, end: stacking_origin + lane.position + lane.outer_size }
        };
        // The item's containing block is its grid area in the grid axis and the container's content box in the stacking axis
        let mut containing_block_size = Size::ZERO;
        containing_block_size.set(g_axis, g_area.end - g_area.start);
        containing_block_size.set(s_axis, s_content_box);
        let grid_area = match grid_axis {
            AbsoluteAxis::Horizontal => {
                Rect { left: g_area.start, right: g_area.end, top: s_area.start, bottom: s_area.end }
            }
            AbsoluteAxis::Vertical => {
                Rect { left: s_area.start, right: s_area.end, top: g_area.start, bottom: g_area.end }
            }
        };
        #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
        let (overflow_contribution, y_position, height) = align_and_position_item(
            tree,
            item.node,
            index as u32,
            grid_area,
            containing_block_size,
            container_alignment_styles,
            item.baseline_shim,
            direction,
            container_border_box.width,
            border,
            #[cfg(feature = "content_size")]
            is_scroll_container,
            &mut item.oof_candidates,
        );
        item.y_position = y_position;
        item.height = height;

        #[cfg(feature = "content_size")]
        {
            item_overflow_rect = item_overflow_rect.union(overflow_contribution);
        }
    }
    drop(lane_placements);

    // 10. Hidden and absolutely positioned children (same as grid layout)
    let (rows, columns) = match grid_axis {
        AbsoluteAxis::Horizontal => (&s_tracks, &g_tracks),
        AbsoluteAxis::Vertical => (&g_tracks, &s_tracks),
    };
    let mut order = items.len() as u32;
    let mut in_flow_sources: Vec<(u16, usize)> =
        items.iter().enumerate().map(|(i, item)| (item.source_order, i)).collect();
    in_flow_sources.sort_unstable();
    let mut in_flow_sources = in_flow_sources.into_iter().peekable();
    (0..child_count).for_each(|index| {
        if let Some((_, item_index)) = in_flow_sources.next_if(|(source_order, _)| *source_order as usize == index) {
            oof_candidates.append(&mut items[item_index].oof_candidates);
            return;
        }

        let child = tree.get_child_id(node, index);
        let child_style = tree.get_grid_child_style(child);

        if child_style.box_generation_mode() == BoxGenerationMode::None {
            drop(child_style);
            tree.set_unrounded_layout(child, &Layout::with_order(order));
            tree.perform_child_layout(
                child,
                Size::NONE,
                Size::NONE,
                Size::MAX_CONTENT,
                SizingMode::InherentSize,
                Line::FALSE,
            );
            order += 1;
            return;
        }

        if child_style.position().is_out_of_flow() {
            let position = child_style.position();
            let item_direction = child_style.direction();
            let justify_self = child_style.justify_self().unwrap_or(justify_items).resolve_self_relative(
                item_direction,
                direction,
                true,
            );
            let align_self =
                child_style.align_self().unwrap_or(align_items).resolve_self_relative(item_direction, direction, false);

            let area = if containing_block_claims.for_position(position) {
                resolve_static_position_grid_area(
                    &child_style,
                    &name_resolver,
                    row_counts,
                    col_counts,
                    rows,
                    columns,
                    direction,
                    border,
                    scrollbar_gutter,
                    container_border_box,
                )
            } else {
                Rect {
                    left: border.left + padding.left + if direction.is_rtl() { scrollbar_gutter.x } else { 0.0 },
                    right: container_border_box.width
                        - border.right
                        - padding.right
                        - if direction.is_rtl() { 0.0 } else { scrollbar_gutter.x },
                    top: border.top + padding.top,
                    bottom: container_border_box.height - border.bottom - padding.bottom - scrollbar_gutter.y,
                }
            };
            drop(child_style);

            oof_candidates.push(OofCandidate {
                node: child,
                order,
                position,
                static_position: Point {
                    x: AxisStaticPosition::from_alignment(
                        justify_self,
                        Line { start: area.left, end: area.right },
                        direction.is_rtl(),
                    ),
                    y: AxisStaticPosition::from_alignment(
                        align_self,
                        Line { start: area.top, end: area.bottom },
                        false,
                    ),
                },
            });

            order += 1;
        }
    });

    let absolute_position_inset = border
        + Rect {
            left: if direction.is_rtl() { scrollbar_gutter.x } else { 0.0 },
            right: if direction.is_rtl() { 0.0 } else { scrollbar_gutter.x },
            top: 0.0,
            bottom: scrollbar_gutter.y,
        };
    let absolute_position_area = container_border_box - absolute_position_inset.sum_axes();
    let absolute_position_offset = Point { x: absolute_position_inset.left, y: absolute_position_inset.top };
    let oof_positioning_area =
        Some(OofPositioningArea { size: absolute_position_area, offset: absolute_position_offset });

    #[cfg(feature = "content_size")]
    let scrollable_overflow_rect = {
        let mut overflow_rect = item_overflow_rect;
        if is_scroll_container {
            overflow_rect.right += if direction.is_rtl() { padding.left } else { padding.right };
            overflow_rect.bottom += padding.bottom;
        }
        overflow_rect
    };
    #[cfg(not(feature = "content_size"))]
    let scrollable_overflow_rect = item_overflow_rect;

    // The container's baseline is the first item's (in document order) baseline, or its bottom edge
    let container_baseline = if contain.suppresses_baseline() {
        None
    } else {
        items
            .iter()
            .min_by_key(|item| item.source_order)
            .map(|item| item.y_position + item.baseline.unwrap_or(item.height))
    };

    let mut output = LayoutOutput::from_sizes_and_baselines(
        container_border_box,
        scrollable_overflow_rect,
        Baselines::from_first(container_baseline),
    );
    output.oof_candidates = oof_candidates;
    output.oof_positioning_area = oof_positioning_area;
    output
}
