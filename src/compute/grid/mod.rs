//! This module is a partial implementation of the CSS Grid Level 1 specification
//! <https://www.w3.org/TR/css-grid-1>
use crate::geometry::{AbsoluteAxis, AbstractAxis, InBothAbsAxis};
use crate::geometry::{Line, Point, Rect, Size};
use crate::style::{AlignItems, AvailableSpace, Overflow, Position};
use crate::tree::{Baselines, Layout, LayoutInput, LayoutOutput, LayoutPartialTreeExt, NodeId, RunMode, SizingMode};
use crate::util::debug::debug_log;
use crate::util::sys::{f32_max, f32_min, GridTrackVec, Vec};
use crate::util::MaybeMath;
use crate::util::{MaybeResolve, ResolveOrZero};
use crate::{
    style_helpers::*, AlignContent, BoxGenerationMode, BoxSizing, CoreStyle, Direction, GridContainerStyle,
    GridItemStyle, JustifyContent, LayoutGridContainer, RequestedAxis,
};
use alignment::{align_and_position_item, align_tracks};
use explicit_grid::{compute_explicit_grid_size_in_axis, initialize_grid_tracks, AutoRepeatStrategy};
use implicit_grid::compute_grid_size_estimate;
use placement::place_grid_items;
use track_sizing::{
    determine_if_item_crosses_flexible_or_intrinsic_tracks, resolve_item_track_indexes, track_sizing_algorithm,
};
use types::{CellOccupancyMatrix, GridTrack, NamedLineResolver};

#[cfg(feature = "detailed_layout_info")]
use crate::sys::{format, DefaultCheapStr, String};
#[cfg(feature = "detailed_layout_info")]
use crate::{CheapCloneStr, GenericGridTemplateComponent, GenericRepetition as _, RepetitionCount};
#[cfg(feature = "detailed_layout_info")]
use types::{GridItem, GridTrackKind, TrackCounts};

pub(crate) use types::{GridCoordinate, GridLine, OriginZeroLine, MAX_GRID_TRACKS, MAX_OZ_LINE, MIN_OZ_LINE};

mod alignment;
mod explicit_grid;
mod implicit_grid;
mod placement;
mod track_sizing;
mod types;
mod util;

/// Grid layout algorithm
/// This consists of a few phases:
///   - Resolving the explicit grid
///   - Placing items (which also resolves the implicit grid)
///   - Track (row/column) sizing
///   - Alignment & Final item placement
pub fn compute_grid_layout<Tree: LayoutGridContainer>(
    tree: &mut Tree,
    node: NodeId,
    inputs: LayoutInput,
) -> LayoutOutput {
    let LayoutInput { known_dimensions, parent_size, available_space, run_mode, .. } = inputs;

    let style = tree.get_grid_container_style(node);
    let direction = style.direction();
    let contain = style.contain();

    // 1. Compute "available grid space"
    // https://www.w3.org/TR/css-grid-1/#available-grid-space
    let aspect_ratio = style.aspect_ratio();
    let padding = style.padding().resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
    let border = style.border().resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
    let padding_border = padding + border;
    let padding_border_size = padding_border.sum_axes();
    let box_sizing_adjustment =
        if style.box_sizing() == BoxSizing::ContentBox { padding_border_size } else { Size::ZERO };

    let min_size = style
        .min_size()
        .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let max_size = style
        .max_size()
        .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let preferred_size = if inputs.sizing_mode == SizingMode::InherentSize {
        style
            .size()
            .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(style.aspect_ratio())
            .maybe_add(box_sizing_adjustment)
    } else {
        Size::NONE
    };

    // Scrollbar gutters are reserved when the `overflow` property is set to `Overflow::Scroll`.
    // However, the axis are switched (transposed) because a node that scrolls vertically needs
    // *horizontal* space to be reserved for a scrollbar
    let scrollbar_gutter = style.overflow().transpose().map(|overflow| match overflow {
        Overflow::Scroll => style.scrollbar_width(),
        _ => 0.0,
    });
    #[cfg(feature = "content_size")]
    let is_scroll_container = {
        let overflow = style.overflow();
        overflow.x.is_scroll_container() || overflow.y.is_scroll_container()
    };
    let mut content_box_inset = padding_border;
    content_box_inset.bottom += scrollbar_gutter.y;

    match direction {
        Direction::Ltr => content_box_inset.right += scrollbar_gutter.x,
        Direction::Rtl => content_box_inset.left += scrollbar_gutter.x,
    };

    let align_content = style.align_content().unwrap_or(AlignContent::STRETCH);
    let justify_content = style.justify_content().unwrap_or(JustifyContent::STRETCH);
    let align_items = style.align_items();
    let justify_items = style.justify_items();

    // Note: we avoid accessing the grid rows/columns methods more than once as this can
    // cause an expensive-ish computation
    let grid_template_columns = style.grid_template_columns();
    let grid_template_rows = style.grid_template_rows();
    let grid_auto_columns = style.grid_auto_columns();
    let grid_auto_rows = style.grid_auto_rows();

    let constrained_available_space = known_dimensions
        .or(preferred_size)
        .map(|size| size.map(AvailableSpace::Definite))
        .unwrap_or(available_space)
        .maybe_clamp(min_size, max_size)
        .maybe_max(padding_border_size);

    let available_grid_space = Size {
        width: constrained_available_space
            .width
            .map_definite_value(|space| space - content_box_inset.horizontal_axis_sum()),
        height: constrained_available_space
            .height
            .map_definite_value(|space| space - content_box_inset.vertical_axis_sum()),
    };

    let outer_node_size =
        known_dimensions.or(preferred_size).maybe_clamp(min_size, max_size).maybe_max(padding_border_size);

    // The track sizing algorithm operates on the grid container's content box, so the min/max sizes
    // (which are border-box sizes) need converting to content-box sizes before being passed to it
    let inner_min_size = min_size.maybe_sub(content_box_inset.sum_axes());
    let inner_max_size = max_size.maybe_sub(content_box_inset.sum_axes());
    let mut inner_node_size = Size {
        width: outer_node_size.width.map(|space| space - content_box_inset.horizontal_axis_sum()),
        height: outer_node_size.height.map(|space| space - content_box_inset.vertical_axis_sum()),
    };

    debug_log!("parent_size", dbg:parent_size);
    debug_log!("outer_node_size", dbg:outer_node_size);
    debug_log!("inner_node_size", dbg:inner_node_size);

    // Short-circuit layout if the container's size is fully determined by the container's size and the run mode
    // is ComputeSize (and thus the container's size is all that we're interested in)
    if run_mode == RunMode::ComputeSize {
        if let Size { width: Some(width), height: Some(height) } = outer_node_size {
            return LayoutOutput::from_outer_size(Size { width, height });
        }

        // We can also short-circuit if the width is known and only the width has been requested.
        if inputs.axis == RequestedAxis::Horizontal {
            if let Some(width) = outer_node_size.width {
                return LayoutOutput::from_outer_size(Size { width, height: 0.0 });
            }
        }
    }

    // Absolutely positioned children do not take part in grid placement and do not create
    // implicit tracks, so they are excluded from the grid size estimate.
    let get_child_styles_iter = |node| {
        tree.child_ids(node).map(|child_node: NodeId| tree.get_grid_child_style(child_node)).filter(|style| {
            style.box_generation_mode() != BoxGenerationMode::None && style.position() != Position::Absolute
        })
    };
    let child_styles_iter = get_child_styles_iter(node);

    // 2. Resolve the explicit grid

    // This is very similar to the inner_node_size except if the inner_node_size is not definite but the node
    // has a min- or max- size style then that will be used in it's place.
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
    let (col_auto_repetition_count, grid_template_col_count) = compute_explicit_grid_size_in_axis(
        &style,
        auto_fit_container_size.width,
        auto_repeat_fit_strategy.width,
        |val, basis| tree.calc(val, basis),
        AbsoluteAxis::Horizontal,
    );
    let (row_auto_repetition_count, grid_template_row_count) = compute_explicit_grid_size_in_axis(
        &style,
        auto_fit_container_size.height,
        auto_repeat_fit_strategy.height,
        |val, basis| tree.calc(val, basis),
        AbsoluteAxis::Vertical,
    );

    // type CustomIdent<'a> = <<Tree as LayoutPartialTree>::CoreContainerStyle<'_> as CoreStyle>::CustomIdent;
    let mut name_resolver = NamedLineResolver::new(&style, col_auto_repetition_count, row_auto_repetition_count);

    // Clamp the explicit grid to MAX_GRID_TRACKS tracks in each axis
    // https://www.w3.org/TR/css-grid-1/#overlarge-grids
    let explicit_col_count = grid_template_col_count.max(name_resolver.area_column_count()).min(MAX_GRID_TRACKS);
    let explicit_row_count = grid_template_row_count.max(name_resolver.area_row_count()).min(MAX_GRID_TRACKS);

    name_resolver.set_explicit_column_count(explicit_col_count);
    name_resolver.set_explicit_row_count(explicit_row_count);

    // Expand the per-line names of the explicit grid (must be done while the style is still borrowed)
    #[cfg(feature = "detailed_layout_info")]
    let detailed_column_line_names =
        expand_explicit_line_names(&style, AbsoluteAxis::Horizontal, col_auto_repetition_count, explicit_col_count);
    #[cfg(feature = "detailed_layout_info")]
    let detailed_row_line_names =
        expand_explicit_line_names(&style, AbsoluteAxis::Vertical, row_auto_repetition_count, explicit_row_count);

    // 3. Implicit Grid: Estimate Track Counts
    // Estimate the number of rows and columns in the implicit grid (= the entire grid)
    // This is necessary as part of placement. Doing it early here is a perf optimisation to reduce allocations.
    let (est_col_counts, est_row_counts) =
        compute_grid_size_estimate(explicit_col_count, explicit_row_count, child_styles_iter);

    // 4. Grid Item Placement
    // Match items (children) to a definite grid position (row start/end and column start/end position)
    let mut items = Vec::with_capacity(tree.child_count(node));
    let mut cell_occupancy_matrix = CellOccupancyMatrix::with_track_counts(est_col_counts, est_row_counts);
    let in_flow_children_iter = || {
        tree.child_ids(node)
            .enumerate()
            .map(|(index, child_node)| (index, child_node, tree.get_grid_child_style(child_node)))
            .filter(|(_, _, style)| {
                style.box_generation_mode() != BoxGenerationMode::None && style.position() != Position::Absolute
            })
    };
    place_grid_items(
        &mut cell_occupancy_matrix,
        &mut items,
        in_flow_children_iter,
        style.grid_auto_flow(),
        align_items.unwrap_or(AlignItems::STRETCH),
        justify_items.unwrap_or(AlignItems::STRETCH),
        &name_resolver,
    );

    // Extract track counts from previous step (auto-placement can expand the number of tracks)
    let final_col_counts = *cell_occupancy_matrix.track_counts(AbsoluteAxis::Horizontal);
    let final_row_counts = *cell_occupancy_matrix.track_counts(AbsoluteAxis::Vertical);

    // 5. Initialize Tracks
    // Initialize (explicit and implicit) grid tracks (and gutters)
    // This resolves the min and max track sizing functions for all tracks and gutters
    let mut columns = GridTrackVec::new();
    let mut rows = GridTrackVec::new();
    initialize_grid_tracks(
        &mut columns,
        final_col_counts,
        &style,
        AbsoluteAxis::Horizontal,
        col_auto_repetition_count,
        |column_index| cell_occupancy_matrix.column_is_occupied(column_index),
    );
    initialize_grid_tracks(
        &mut rows,
        final_row_counts,
        &style,
        AbsoluteAxis::Vertical,
        row_auto_repetition_count,
        |row_index| cell_occupancy_matrix.row_is_occupied(row_index),
    );

    drop(grid_template_rows);
    drop(grid_template_columns);
    drop(grid_auto_rows);
    drop(grid_auto_columns);
    drop(style);

    // 6. Track Sizing

    // Convert grid placements in origin-zero coordinates to indexes into the GridTrack (rows and columns) vectors
    // This computation is relatively trivial, but it requires the final number of negative (implicit) tracks in
    // each axis, and doing it up-front here means we don't have to keep repeating that calculation
    resolve_item_track_indexes(&mut items, final_col_counts, final_row_counts);
    // For each item, and in each axis, determine whether the item crosses any flexible (fr) tracks
    // Record this as a boolean (per-axis) on each item for later use in the track-sizing algorithm
    determine_if_item_crosses_flexible_or_intrinsic_tracks(&mut items, &columns, &rows);

    // Determine if the grid has any baseline aligned items
    let has_baseline_aligned_item = items.iter().any(|item| item.participates_in_baseline_alignment());

    // Run track sizing algorithm for Inline axis
    track_sizing_algorithm(
        tree,
        AbstractAxis::Inline,
        inner_min_size.get(AbstractAxis::Inline),
        inner_max_size.get(AbstractAxis::Inline),
        justify_content,
        align_content,
        available_grid_space,
        inner_node_size,
        &mut columns,
        &mut rows,
        &mut items,
        |track: &GridTrack, parent_size: Option<f32>, tree: &Tree| {
            track.max_track_sizing_function.definite_value(parent_size, |val, basis| tree.calc(val, basis))
        },
        has_baseline_aligned_item,
    );
    let initial_column_sum = columns.iter().map(|track| track.base_size).sum::<f32>();
    inner_node_size.width = inner_node_size.width.or_else(|| initial_column_sum.into());

    items.iter_mut().for_each(|item| item.grid_area_size_cache = None);

    // Run track sizing algorithm for Block axis
    track_sizing_algorithm(
        tree,
        AbstractAxis::Block,
        inner_min_size.get(AbstractAxis::Block),
        inner_max_size.get(AbstractAxis::Block),
        align_content,
        justify_content,
        available_grid_space,
        inner_node_size,
        &mut rows,
        &mut columns,
        &mut items,
        |track: &GridTrack, _, _| Some(track.base_size),
        false, // TODO: Support baseline alignment in the vertical axis
    );
    let initial_row_sum = rows.iter().map(|track| track.base_size).sum::<f32>();
    inner_node_size.height = inner_node_size.height.or_else(|| initial_row_sum.into());

    debug_log!("initial_column_sum", dbg:initial_column_sum);
    debug_log!(dbg: columns.iter().map(|track| track.base_size).collect::<Vec<_>>());
    debug_log!("initial_row_sum", dbg:initial_row_sum);
    debug_log!(dbg: rows.iter().map(|track| track.base_size).collect::<Vec<_>>());

    // 6. Compute container size
    let resolved_style_size = known_dimensions.or(preferred_size);
    let mut container_border_box = Size {
        width: resolved_style_size
            .get(AbstractAxis::Inline)
            .unwrap_or_else(|| initial_column_sum + content_box_inset.horizontal_axis_sum())
            .maybe_clamp(min_size.width, max_size.width)
            .max(padding_border_size.width),
        height: resolved_style_size
            .get(AbstractAxis::Block)
            .unwrap_or_else(|| initial_row_sum + content_box_inset.vertical_axis_sum())
            .maybe_clamp(min_size.height, max_size.height)
            .max(padding_border_size.height),
    };
    let mut container_content_box = Size {
        width: f32_max(0.0, container_border_box.width - content_box_inset.horizontal_axis_sum()),
        height: f32_max(0.0, container_border_box.height - content_box_inset.vertical_axis_sum()),
    };

    // If only the container's size has been requested
    if run_mode == RunMode::ComputeSize {
        return LayoutOutput::from_outer_size(container_border_box);
    }

    // 7. Resolve percentage track base sizes
    // In the case of an indefinitely sized container these resolve to zero during the "Initialise Tracks" step
    // and therefore need to be re-resolved here based on the content-sized content box of the container
    if !available_grid_space.width.is_definite() {
        for column in &mut columns {
            let min: Option<f32> = column
                .min_track_sizing_function
                .resolved_percentage_size(container_content_box.width, |val, basis| tree.calc(val, basis));
            let max: Option<f32> = column
                .max_track_sizing_function
                .resolved_percentage_size(container_content_box.width, |val, basis| tree.calc(val, basis));
            column.base_size = column.base_size.maybe_clamp(min, max);
        }
    }
    if !available_grid_space.height.is_definite() {
        for row in &mut rows {
            let min: Option<f32> = row
                .min_track_sizing_function
                .resolved_percentage_size(container_content_box.height, |val, basis| tree.calc(val, basis));
            let max: Option<f32> = row
                .max_track_sizing_function
                .resolved_percentage_size(container_content_box.height, |val, basis| tree.calc(val, basis));
            row.base_size = row.base_size.maybe_clamp(min, max);
        }
    }

    // Column sizing must be re-run (once) if:
    //   - The grid container's width was initially indefinite and there are any columns with percentage track sizing functions
    //   - Any grid item crossing an intrinsically sized track's min content contribution width has changed
    // TODO: Only rerun sizing for tracks that actually require it rather than for all tracks if any need it.
    let mut rerun_column_sizing;
    let mut intrinsic_column_contribution_changed = false;

    let has_percentage_column = columns.iter().any(|track| track.uses_percentage());
    let has_percentage_row = rows.iter().any(|track| track.uses_percentage());
    let parent_width_indefinite = !available_space.width.is_definite();
    rerun_column_sizing = parent_width_indefinite && has_percentage_column;

    if !rerun_column_sizing {
        intrinsic_column_contribution_changed =
            items.iter_mut().filter(|item| item.crosses_intrinsic_column).any(|item| {
                let grid_area_size = item.grid_area_size(
                    AbstractAxis::Inline,
                    &columns,
                    &rows,
                    inner_node_size,
                    |track: &GridTrack, _| Some(track.base_size),
                    &|val, basis| tree.calc(val, basis),
                );
                let available_space = grid_area_size.with(AbstractAxis::Inline, None);
                let new_min_content_contribution =
                    item.min_content_contribution(AbstractAxis::Inline, tree, grid_area_size, available_space);

                let has_changed = Some(new_min_content_contribution) != item.min_content_contribution_cache.width;

                item.grid_area_size_cache = Some(grid_area_size);
                item.min_content_contribution_cache.width = Some(new_min_content_contribution);
                item.max_content_contribution_cache.width = None;
                item.minimum_contribution_cache.width = None;

                has_changed
            });
        rerun_column_sizing = intrinsic_column_contribution_changed;
    } else {
        // Clear intrinsic width caches
        items.iter_mut().for_each(|item| {
            item.grid_area_size_cache = None;
            item.min_content_contribution_cache.width = None;
            item.max_content_contribution_cache.width = None;
            item.minimum_contribution_cache.width = None;
        });
    }

    let mut intrinsic_row_contribution_changed = false;

    if rerun_column_sizing {
        // Re-run track sizing algorithm for Inline axis
        track_sizing_algorithm(
            tree,
            AbstractAxis::Inline,
            inner_min_size.get(AbstractAxis::Inline),
            inner_max_size.get(AbstractAxis::Inline),
            justify_content,
            align_content,
            available_grid_space,
            inner_node_size,
            &mut columns,
            &mut rows,
            &mut items,
            |track: &GridTrack, _, _| Some(track.base_size),
            has_baseline_aligned_item,
        );

        // Row sizing must be re-run (once) if:
        //   - The grid container's height was initially indefinite and there are any rows with percentage track sizing functions
        //   - Any grid item crossing an intrinsically sized track's min content contribution height has changed
        // TODO: Only rerun sizing for tracks that actually require it rather than for all tracks if any need it.
        let mut rerun_row_sizing;

        let parent_height_indefinite = !available_space.height.is_definite();
        rerun_row_sizing = parent_height_indefinite && has_percentage_row;

        if !rerun_row_sizing {
            intrinsic_row_contribution_changed =
                items.iter_mut().filter(|item| item.crosses_intrinsic_column).any(|item| {
                    let grid_area_size = item.grid_area_size(
                        AbstractAxis::Block,
                        &rows,
                        &columns,
                        inner_node_size,
                        |track: &GridTrack, _| Some(track.base_size),
                        &|val, basis| tree.calc(val, basis),
                    );
                    let available_space = grid_area_size.with(AbstractAxis::Block, None);
                    let new_min_content_contribution =
                        item.min_content_contribution(AbstractAxis::Block, tree, grid_area_size, available_space);

                    let has_changed = Some(new_min_content_contribution) != item.min_content_contribution_cache.height;

                    item.grid_area_size_cache = Some(grid_area_size);
                    item.min_content_contribution_cache.height = Some(new_min_content_contribution);
                    item.max_content_contribution_cache.height = None;
                    item.minimum_contribution_cache.height = None;

                    has_changed
                });
            rerun_row_sizing = intrinsic_row_contribution_changed;
        } else {
            items.iter_mut().for_each(|item| {
                // Clear intrinsic height caches
                item.grid_area_size_cache = None;
                item.min_content_contribution_cache.height = None;
                item.max_content_contribution_cache.height = None;
                item.minimum_contribution_cache.height = None;
            });
        }

        if rerun_row_sizing {
            // Re-run track sizing algorithm for Block axis
            track_sizing_algorithm(
                tree,
                AbstractAxis::Block,
                inner_min_size.get(AbstractAxis::Block),
                inner_max_size.get(AbstractAxis::Block),
                align_content,
                justify_content,
                available_grid_space,
                inner_node_size,
                &mut rows,
                &mut columns,
                &mut items,
                |track: &GridTrack, _, _| Some(track.base_size),
                false, // TODO: Support baseline alignment in the vertical axis
            );
        }
    }

    if (intrinsic_column_contribution_changed && !has_percentage_column)
        || (intrinsic_row_contribution_changed && !has_percentage_row)
    {
        let final_column_sum = columns.iter().map(|track| track.base_size).sum::<f32>();
        let final_row_sum = rows.iter().map(|track| track.base_size).sum::<f32>();

        if intrinsic_column_contribution_changed && !has_percentage_column {
            container_border_box.width = resolved_style_size
                .get(AbstractAxis::Inline)
                .unwrap_or_else(|| final_column_sum + content_box_inset.horizontal_axis_sum())
                .maybe_clamp(min_size.width, max_size.width)
                .max(padding_border_size.width);
            container_content_box.width =
                f32_max(0.0, container_border_box.width - content_box_inset.horizontal_axis_sum());
        }

        if intrinsic_row_contribution_changed && !has_percentage_row {
            container_border_box.height = resolved_style_size
                .get(AbstractAxis::Block)
                .unwrap_or_else(|| final_row_sum + content_box_inset.vertical_axis_sum())
                .maybe_clamp(min_size.height, max_size.height)
                .max(padding_border_size.height);
            container_content_box.height =
                f32_max(0.0, container_border_box.height - content_box_inset.vertical_axis_sum());
        }
    }

    // If only the container's size has been requested
    if run_mode == RunMode::ComputeSize {
        return LayoutOutput::from_outer_size(container_border_box);
    }

    // 8. Track Alignment

    // Align columns
    let inline_size_without_scrollbar = f32_max(container_border_box.width - padding_border_size.width, 0.0);
    let inline_scrollbar_gutter_for_alignment = f32_min(scrollbar_gutter.x, inline_size_without_scrollbar);
    align_tracks(
        container_content_box.get(AbstractAxis::Inline),
        Line {
            start: padding.left + if direction.is_rtl() { inline_scrollbar_gutter_for_alignment } else { 0.0 },
            end: padding.right + if direction.is_rtl() { 0.0 } else { inline_scrollbar_gutter_for_alignment },
        },
        Line { start: border.left, end: border.right },
        &mut columns,
        justify_content,
        direction.is_rtl(),
    );
    // Align rows
    align_tracks(
        container_content_box.get(AbstractAxis::Block),
        Line { start: padding.top, end: padding.bottom },
        Line { start: border.top, end: border.bottom },
        &mut rows,
        align_content,
        false,
    );

    // 9. Size, Align, and Position Grid Items

    #[cfg_attr(not(feature = "content_size"), allow(unused_mut))]
    let mut item_overflow_rect = Rect::ZERO;
    #[cfg_attr(not(feature = "content_size"), allow(unused_mut, unused))]
    let mut absolute_overflow_rect = Rect::ZERO;

    // Sort items back into original order to allow them to be matched up with styles
    items.sort_by_key(|item| item.source_order);

    let container_alignment_styles = InBothAbsAxis { horizontal: justify_items, vertical: align_items };

    // Position in-flow children (stored in items vector)
    for (index, item) in items.iter_mut().enumerate() {
        // Tracks are stored in logical order. In RTL the physical offsets are assigned
        // right-to-left, so an item's physical left edge is derived from its logical end
        // line and its physical right edge from its logical start line.
        let grid_area = Rect {
            top: rows[item.row_indexes.start as usize + 1].offset,
            bottom: rows[item.row_indexes.end as usize].offset,
            left: if direction.is_rtl() {
                columns[item.column_indexes.end as usize - 1].offset
            } else {
                columns[item.column_indexes.start as usize + 1].offset
            },
            right: if direction.is_rtl() {
                columns[item.column_indexes.start as usize].offset
            } else {
                columns[item.column_indexes.end as usize].offset
            },
        };
        #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
        let (overflow_contribution, y_position, height) = align_and_position_item(
            tree,
            item.node,
            index as u32,
            grid_area,
            container_alignment_styles,
            item.baseline_shim,
            direction,
            container_border_box.width,
            border,
            #[cfg(feature = "content_size")]
            is_scroll_container,
        );
        item.y_position = y_position;
        item.height = height;

        #[cfg(feature = "content_size")]
        {
            item_overflow_rect = item_overflow_rect.union(overflow_contribution);
        }
    }

    // Position hidden and absolutely positioned children
    let mut order = items.len() as u32;
    (0..tree.child_count(node)).for_each(|index| {
        let child = tree.get_child_id(node, index);
        let child_style = tree.get_grid_child_style(child);

        // Position hidden child
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

        // Position absolutely positioned child
        if child_style.position() == Position::Absolute {
            // Convert grid-col-{start/end} into Option's of indexes into the columns vector
            // The Option is None if the style property is Auto and an unresolvable Span
            let maybe_col_indexes = name_resolver
                .resolve_column_names(&child_style.grid_column())
                .into_origin_zero(final_col_counts.explicit)
                .resolve_absolutely_positioned_grid_tracks()
                .map(|maybe_grid_line| {
                    maybe_grid_line.and_then(|line: OriginZeroLine| line.try_into_track_vec_index(final_col_counts))
                });
            // Convert grid-row-{start/end} into Option's of indexes into the row vector
            // The Option is None if the style property is Auto and an unresolvable Span
            let maybe_row_indexes = name_resolver
                .resolve_row_names(&child_style.grid_row())
                .into_origin_zero(final_row_counts.explicit)
                .resolve_absolutely_positioned_grid_tracks()
                .map(|maybe_grid_line| {
                    maybe_grid_line.and_then(|line: OriginZeroLine| line.try_into_track_vec_index(final_row_counts))
                });

            // Content alignment (align-content/justify-content) may distribute free space before, between,
            // or after tracks. Grid lines used by absolutely positioned items resolve to the edges of the
            // tracks adjacent to the line rather than to the raw gutter offset:
            //   - As a start edge, a line resolves to the start of the track that follows it
            //   - As an end edge, a line resolves to the end of the track that precedes it
            /// Resolve a grid line (by track vector index) used as a start edge to a position
            fn line_as_start_edge(tracks: &[GridTrack], index: usize) -> f32 {
                tracks.get(index + 1).unwrap_or(&tracks[index]).offset
            }
            /// Resolve a grid line (by track vector index) used as an end edge to a position
            fn line_as_end_edge(tracks: &[GridTrack], index: usize) -> f32 {
                if index == 0 {
                    tracks.get(1).unwrap_or(&tracks[0]).offset
                } else {
                    tracks[index].offset
                }
            }
            // In RTL, tracks remain in logical order but physical offsets are assigned
            // right-to-left: a line used as an inline-start edge resolves to the physical
            // *right* edge of the track that follows it (its gutter's offset), and a line
            // used as an inline-end edge resolves to the physical *left* edge (offset) of
            // the track that precedes it.
            /// Resolve a grid line used as an inline-start edge to a physical right x-position (RTL)
            fn rtl_line_as_start_edge(tracks: &[GridTrack], index: usize) -> f32 {
                if tracks.len() > index + 1 {
                    // The gutter's offset is the physical right edge of the track that follows the line
                    tracks[index].offset
                } else if index == 0 {
                    tracks[0].offset
                } else {
                    // No track follows the line: resolve to the line itself, which is the physical
                    // left edge of the track that precedes it (the trailing gutter is assigned its
                    // offset before any alignment offset is applied, so it cannot be used here)
                    tracks[index - 1].offset
                }
            }
            /// Resolve a grid line used as an inline-end edge to a physical left x-position (RTL)
            fn rtl_line_as_end_edge(tracks: &[GridTrack], index: usize) -> f32 {
                if index == 0 {
                    tracks[0].offset
                } else {
                    tracks[index - 1].offset
                }
            }

            // In RTL the item's physical left edge derives from its logical end line and its
            // physical right edge from its logical start line.
            let (grid_area_left, grid_area_right) = if direction.is_rtl() {
                (
                    maybe_col_indexes
                        .end
                        .map(|index| rtl_line_as_end_edge(&columns, index))
                        .unwrap_or(border.left + scrollbar_gutter.x),
                    maybe_col_indexes
                        .start
                        .map(|index| rtl_line_as_start_edge(&columns, index))
                        .unwrap_or(container_border_box.width - border.right),
                )
            } else {
                (
                    maybe_col_indexes.start.map(|index| line_as_start_edge(&columns, index)).unwrap_or(border.left),
                    maybe_col_indexes
                        .end
                        .map(|index| line_as_end_edge(&columns, index))
                        .unwrap_or(container_border_box.width - border.right - scrollbar_gutter.x),
                )
            };

            let grid_area = Rect {
                top: maybe_row_indexes.start.map(|index| line_as_start_edge(&rows, index)).unwrap_or(border.top),
                bottom: maybe_row_indexes
                    .end
                    .map(|index| line_as_end_edge(&rows, index))
                    .unwrap_or(container_border_box.height - border.bottom - scrollbar_gutter.y),
                left: grid_area_left,
                right: grid_area_right,
            };
            drop(child_style);

            // TODO: Baseline alignment support for absolutely positioned items (should check if is actually specified)
            #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
            let (overflow_contribution, _, _) = align_and_position_item(
                tree,
                child,
                order,
                grid_area,
                container_alignment_styles,
                0.0,
                direction,
                container_border_box.width,
                border,
                #[cfg(feature = "content_size")]
                is_scroll_container,
            );
            #[cfg(feature = "content_size")]
            {
                absolute_overflow_rect = absolute_overflow_rect.union(overflow_contribution);
            }

            order += 1;
        }
    });

    // Set detailed grid information
    #[cfg(feature = "detailed_layout_info")]
    tree.set_detailed_grid_info(
        node,
        DetailedGridInfo {
            rows: DetailedGridTracksInfo::from_grid_tracks_and_track_count(
                final_row_counts,
                rows,
                finalize_line_names(detailed_row_line_names, final_row_counts),
            ),
            columns: DetailedGridTracksInfo::from_grid_tracks_and_track_count(
                final_col_counts,
                columns,
                finalize_line_names(detailed_column_line_names, final_col_counts),
            ),
            items: items.iter().map(DetailedGridItemsInfo::from_grid_item).collect(),
        },
    );

    // If there are no in-flow items then return the container size and the overflow
    // contributed by absolutely positioned children (no baseline)
    if items.is_empty() {
        #[cfg(feature = "content_size")]
        {
            let mut overflow_rect = item_overflow_rect;
            if is_scroll_container {
                overflow_rect.right += if direction.is_rtl() { padding.left } else { padding.right };
                overflow_rect.bottom += padding.bottom;
            }
            return LayoutOutput::from_sizes(container_border_box, overflow_rect.union(absolute_overflow_rect));
        }
        #[cfg(not(feature = "content_size"))]
        return LayoutOutput::from_outer_size(container_border_box);
    }

    // Determine the grid container baseline(s) (currently we only compute the first baseline)
    // Layout containment suppresses the box's baseline for baseline-alignment purposes
    let grid_container_baseline: Option<f32> = if contain.suppresses_baseline() {
        None
    } else {
        // Sort items by row start position so that we can iterate items in groups which are in the same row
        items.sort_by_key(|item| item.row_indexes.start);

        // Get the row index of the first row containing items
        let first_row = items[0].row_indexes.start;

        // Create a slice of all of the items start in this row (taking advantage of the fact that we have just sorted the array)
        let first_row_items = &items[0..].split(|item| item.row_indexes.start != first_row).next().unwrap();

        // Check if any items in *this row* participate in baseline alignment
        // (items with an auto block-axis margin do not participate: https://www.w3.org/TR/css-align-3/#baseline-align-self)
        let item = first_row_items
            .iter()
            .find(|item| item.participates_in_baseline_alignment())
            .unwrap_or(&first_row_items[0]);

        Some(item.y_position + item.baseline.unwrap_or(item.height))
    };

    // A scroll container's own padding at the end of the content is part of its scrollable
    // overflow region, so it is included in the in-flow overflow rect. Boxes that are not
    // scroll containers do not extend their overflow region by their own padding.
    #[cfg(feature = "content_size")]
    let scrollable_overflow_rect = {
        let mut overflow_rect = item_overflow_rect;
        if is_scroll_container {
            overflow_rect.right += if direction.is_rtl() { padding.left } else { padding.right };
            overflow_rect.bottom += padding.bottom;
        }
        overflow_rect.union(absolute_overflow_rect)
    };
    #[cfg(not(feature = "content_size"))]
    let scrollable_overflow_rect = item_overflow_rect;

    LayoutOutput::from_sizes_and_baselines(
        container_border_box,
        scrollable_overflow_rect,
        Baselines::from_first(grid_container_baseline),
    )
}

/// Information from the computation of grid
#[derive(Debug, Clone, PartialEq)]
#[cfg(feature = "detailed_layout_info")]
pub struct DetailedGridInfo<S: CheapCloneStr = DefaultCheapStr> {
    /// <https://drafts.csswg.org/css-grid-1/#grid-row>
    pub rows: DetailedGridTracksInfo<S>,
    /// <https://drafts.csswg.org/css-grid-1/#grid-column>
    pub columns: DetailedGridTracksInfo<S>,
    /// <https://drafts.csswg.org/css-grid-1/#grid-items>
    pub items: Vec<DetailedGridItemsInfo>,
}

#[cfg(feature = "detailed_layout_info")]
impl<S: CheapCloneStr> DetailedGridInfo<S> {
    /// Write the used row track sizes and line names to the passed writer in the resolved value
    /// format of the `grid-template-rows` property
    /// (see <https://www.w3.org/TR/css-grid-1/#resolved-track-list>)
    pub fn write_grid_template_rows(&self, out: &mut impl core::fmt::Write) -> core::fmt::Result {
        self.rows.write_track_list(out)
    }

    /// Write the used column track sizes and line names to the passed writer in the resolved value
    /// format of the `grid-template-columns` property
    /// (see <https://www.w3.org/TR/css-grid-1/#resolved-track-list>)
    pub fn write_grid_template_columns(&self, out: &mut impl core::fmt::Write) -> core::fmt::Result {
        self.columns.write_track_list(out)
    }

    /// Serialize the used row track sizes and line names in the resolved value format of the
    /// `grid-template-rows` property (see <https://www.w3.org/TR/css-grid-1/#resolved-track-list>)
    pub fn grid_template_rows(&self) -> String {
        self.rows.to_track_list_string()
    }

    /// Serialize the used column track sizes and line names in the resolved value format of the
    /// `grid-template-columns` property (see <https://www.w3.org/TR/css-grid-1/#resolved-track-list>)
    pub fn grid_template_columns(&self) -> String {
        self.columns.to_track_list_string()
    }

    /// Compute the location and size of the grid area occupied by the item at `item_index` (an
    /// index into [`DetailedGridInfo::items`]), relative to the grid container's border box.
    ///
    /// The edges resolve to the edges of the tracks bounding the item's grid area (a start line
    /// resolves to the start of the track that follows it and an end line to the end of the track
    /// that precedes it), so the area excludes any gutter or content-alignment spacing around it.
    ///
    /// Returns `None` if `item_index` is out of bounds.
    pub fn item_grid_area(&self, item_index: usize) -> Option<(Point<f32>, Size<f32>)> {
        let item = self.items.get(item_index)?;
        let start_col = self.columns.positions[item.column_start as usize - 1];
        let end_col = self.columns.positions[item.column_end as usize - 2];
        let left = f32_min(start_col.start, end_col.start);
        let right = f32_max(start_col.end, end_col.end);
        let top = self.rows.positions[item.row_start as usize - 1].start;
        let bottom = self.rows.positions[item.row_end as usize - 2].end;
        Some((Point { x: left, y: top }, Size { width: right - left, height: bottom - top }))
    }
}

/// Information from the computation of grids tracks
#[derive(Debug, Clone, PartialEq)]
#[cfg(feature = "detailed_layout_info")]
pub struct DetailedGridTracksInfo<S: CheapCloneStr = DefaultCheapStr> {
    /// Number of leading implicit grid tracks
    pub negative_implicit_tracks: u16,
    /// Number of explicit grid tracks
    pub explicit_tracks: u16,
    /// Number of trailing implicit grid tracks
    pub positive_implicit_tracks: u16,

    /// The start and end position of each track relative to the grid container's border box.
    /// These positions account for the container's border and padding, the `gap` property,
    /// content alignment (`align-content`/`justify-content`), and collapsed tracks.
    pub positions: Vec<Line<f32>>,

    /// The names of each grid line. Line `i` (0-indexed) bounds the start of track `i`,
    /// so there is one more line than there are tracks. Empty if the grid has no named lines.
    pub line_names: GridLineNames<S>,
}

#[cfg(feature = "detailed_layout_info")]
impl<S: CheapCloneStr> DetailedGridTracksInfo<S> {
    /// Get the start and end position of each track relative to the grid container's border box
    fn positions_from_grid_track_layout(grid_tracks: &[GridTrack]) -> Vec<Line<f32>> {
        grid_tracks
            .iter()
            .filter(|track| track.kind == GridTrackKind::Track)
            .map(|track| Line { start: track.offset, end: track.offset + track.base_size })
            .collect()
    }

    /// Construct DetailedGridTracksInfo from TrackCounts and GridTracks
    fn from_grid_tracks_and_track_count(
        track_count: TrackCounts,
        grid_tracks: Vec<GridTrack>,
        line_names: GridLineNames<S>,
    ) -> Self {
        DetailedGridTracksInfo {
            negative_implicit_tracks: track_count.negative_implicit,
            explicit_tracks: track_count.explicit,
            positive_implicit_tracks: track_count.positive_implicit,
            positions: DetailedGridTracksInfo::<S>::positions_from_grid_track_layout(&grid_tracks),
            line_names,
        }
    }

    /// Write the used track sizes and line names of this axis to the passed writer in the
    /// resolved value format of the `grid-template-rows`/`grid-template-columns` properties
    /// (see <https://www.w3.org/TR/css-grid-1/#resolved-track-list>)
    pub fn write_track_list(&self, out: &mut impl core::fmt::Write) -> core::fmt::Result {
        /// Write a bracketed line name group (e.g. `[foo bar]`)
        fn write_line_names<S: CheapCloneStr>(out: &mut impl core::fmt::Write, names: &[S]) -> core::fmt::Result {
            out.write_char('[')?;
            for (i, name) in names.iter().enumerate() {
                if i != 0 {
                    out.write_char(' ')?;
                }
                out.write_str(name.as_ref())?;
            }
            out.write_char(']')
        }

        if self.positions.is_empty() {
            return out.write_str("none");
        }

        let mut needs_space = false;
        for (track_index, position) in self.positions.iter().enumerate() {
            let names = self.line_names.line(track_index);
            if !names.is_empty() {
                if needs_space {
                    out.write_char(' ')?;
                }
                write_line_names(out, names)?;
                needs_space = true;
            }
            if needs_space {
                out.write_char(' ')?;
            }
            write!(out, "{}px", position.end - position.start)?;
            needs_space = true;
        }
        let trailing_names = self.line_names.line(self.positions.len());
        if !trailing_names.is_empty() {
            out.write_char(' ')?;
            write_line_names(out, trailing_names)?;
        }
        Ok(())
    }

    /// Serialize the used track sizes and line names of this axis in the resolved value format of
    /// the `grid-template-rows`/`grid-template-columns` properties
    /// (see <https://www.w3.org/TR/css-grid-1/#resolved-track-list>)
    pub fn to_track_list_string(&self) -> String {
        let mut out = String::new();
        self.write_track_list(&mut out).expect("writing to a String cannot fail");
        out
    }
}

/// The names of each grid line in a single axis, stored in CSR (compressed sparse row) format:
/// a single flat `Vec` of names plus a `Vec` of offsets into it (one more offset than there
/// are lines). The names of line `i` (0-indexed) are `names[offsets[i]..offsets[i + 1]]`.
///
/// Iterate over per-line name groups with [`GridLineNames::iter`], or access a single line's
/// names with [`GridLineNames::line`]. A grid with no named lines is represented by two empty
/// `Vec`s (see [`GridLineNames::is_empty`]).
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg(feature = "detailed_layout_info")]
pub struct GridLineNames<S: CheapCloneStr = DefaultCheapStr> {
    /// The names of every grid line in the axis, concatenated in line order
    names: Vec<S>,
    /// Offsets into `names`: line `i`'s names are `names[offsets[i]..offsets[i + 1]]`.
    /// Either empty (no named lines) or of length `line count + 1`.
    offsets: Vec<u32>,
}

#[cfg(feature = "detailed_layout_info")]
impl<S: CheapCloneStr> GridLineNames<S> {
    /// Create an empty `GridLineNames` with pre-allocated capacity
    fn with_capacity(name_capacity: usize, offset_capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(offset_capacity);
        offsets.push(0);
        Self { names: Vec::with_capacity(name_capacity), offsets }
    }

    /// Start a new (initially empty) line
    fn start_line(&mut self) {
        self.offsets.push(self.names.len() as u32);
    }

    /// Append a name to the current (last) line
    fn push_name(&mut self, name: S) {
        self.names.push(name);
        *self.offsets.last_mut().unwrap() = self.names.len() as u32;
    }

    /// Whether the current (last) line already contains the passed name
    fn current_line_contains(&self, name: &str) -> bool {
        self.line(self.line_count().wrapping_sub(1)).iter().any(|n| n.as_ref() == name)
    }

    /// Whether the axis has any named lines at all
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// The number of grid lines represented (zero if the grid has no named lines)
    pub fn line_count(&self) -> usize {
        self.offsets.len().saturating_sub(1)
    }

    /// The names of the line with the passed 0-indexed line index.
    /// Returns an empty slice if the line has no names or the index is out of range.
    pub fn line(&self, line_index: usize) -> &[S] {
        match (self.offsets.get(line_index), self.offsets.get(line_index + 1)) {
            (Some(&start), Some(&end)) => &self.names[start as usize..end as usize],
            _ => &[],
        }
    }

    /// Iterate over the name group (`&[S]`) of each grid line in line order
    pub fn iter(&self) -> GridLineNamesIter<'_, S> {
        GridLineNamesIter { names: &self.names, offsets: self.offsets.windows(2) }
    }
}

#[cfg(feature = "detailed_layout_info")]
impl<'a, S: CheapCloneStr> IntoIterator for &'a GridLineNames<S> {
    type Item = &'a [S];
    type IntoIter = GridLineNamesIter<'a, S>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over the per-line name groups of a [`GridLineNames`]. Yields one `&[S]` per grid
/// line (which is empty for unnamed lines)
#[derive(Debug, Clone)]
#[cfg(feature = "detailed_layout_info")]
pub struct GridLineNamesIter<'a, S: CheapCloneStr> {
    /// The flat name storage being iterated over
    names: &'a [S],
    /// Iterator over adjacent pairs of offsets into `names`
    offsets: core::slice::Windows<'a, u32>,
}

#[cfg(feature = "detailed_layout_info")]
impl<'a, S: CheapCloneStr> Iterator for GridLineNamesIter<'a, S> {
    type Item = &'a [S];

    fn next(&mut self) -> Option<Self::Item> {
        let window = self.offsets.next()?;
        Some(&self.names[window[0] as usize..window[1] as usize])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.offsets.size_hint()
    }
}

#[cfg(feature = "detailed_layout_info")]
impl<S: CheapCloneStr> ExactSizeIterator for GridLineNamesIter<'_, S> {}

/// Expand the line names of the explicit grid in the passed axis into one name group per grid
/// line, expanding `repeat()`s (using the resolved auto-repetition count) and including the
/// implicit `<name>-start`/`<name>-end` names generated by `grid-template-areas`.
///
/// Line indices are relative to the explicit grid (line 0 = start of the first explicit track).
#[cfg(feature = "detailed_layout_info")]
fn expand_explicit_line_names<S: CheapCloneStr>(
    style: &impl GridContainerStyle<CustomIdent = S>,
    axis: AbsoluteAxis,
    auto_repetitions: u16,
    explicit_track_count: u16,
) -> GridLineNames<S> {
    // Collect (1-indexed line number, name) pairs for the axis.
    // This mirrors the expansion performed by `NamedLineResolver::new`.
    let mut pairs: Vec<(u32, S)> = Vec::new();

    let (template_tracks, template_line_names) = match axis {
        AbsoluteAxis::Horizontal => (style.grid_template_columns(), style.grid_template_column_names()),
        AbsoluteAxis::Vertical => (style.grid_template_rows(), style.grid_template_row_names()),
    };
    let mut current_line: u32 = 0;
    if let (Some(mut tracks), Some(line_names_iter)) = (template_tracks, template_line_names) {
        for line_names in line_names_iter {
            current_line += 1;
            for name in line_names {
                pairs.push((current_line, name.clone()));
            }

            if let Some(GenericGridTemplateComponent::Repeat(repeat)) = tracks.next() {
                let repeat_count = match repeat.count() {
                    RepetitionCount::Count(count) => count,
                    RepetitionCount::AutoFill | RepetitionCount::AutoFit => auto_repetitions,
                };
                let lines_per_repetition = repeat.track_count() as u32;

                for _ in 0..repeat_count {
                    for (line, line_name_set) in (current_line..).zip(repeat.lines_names()) {
                        for name in line_name_set {
                            pairs.push((line, name.clone()));
                        }
                    }
                    current_line += lines_per_repetition;

                    // Names for lines beyond the maximum track limit are never resolvable:
                    // stop generating them (the explicit grid is clamped to MAX_GRID_TRACKS)
                    if current_line > MAX_GRID_TRACKS as u32 {
                        break;
                    }
                }
                // Last line name set collapses with following line name set
                if repeat_count > 0 {
                    current_line = current_line.saturating_sub(1);
                }
            }
        }
    }

    if let Some(areas) = style.grid_template_areas() {
        for area in areas {
            let (start, end) = match axis {
                AbsoluteAxis::Horizontal => (area.column_start, area.column_end),
                AbsoluteAxis::Vertical => (area.row_start, area.row_end),
            };
            pairs.push((start as u32, S::from(format!("{}-start", area.name.as_ref()))));
            pairs.push((end as u32, S::from(format!("{}-end", area.name.as_ref()))));
        }
    }

    if pairs.is_empty() {
        return GridLineNames::default();
    }

    // Stable sort by line number preserves the source order of names within each line
    pairs.sort_by_key(|(line, _)| *line);

    let line_count = explicit_track_count as usize + 1;
    let mut line_names = GridLineNames::with_capacity(pairs.len(), line_count + 1);
    let mut pair_iter = pairs.into_iter().peekable();
    for line in 1..=(line_count as u32) {
        line_names.start_line();
        while let Some(&(pair_line, _)) = pair_iter.peek() {
            if pair_line != line {
                break;
            }
            let (_, name) = pair_iter.next().unwrap();
            if !line_names.current_line_contains(name.as_ref()) {
                line_names.push_name(name);
            }
        }
    }
    line_names
}

/// Convert explicit-grid-relative line names into final-grid line names by padding with
/// (unnamed) implicit grid lines, so indices match the logical track order used by
/// [`DetailedGridTracksInfo::positions`]
#[cfg(feature = "detailed_layout_info")]
fn finalize_line_names<S: CheapCloneStr>(
    explicit_line_names: GridLineNames<S>,
    track_counts: TrackCounts,
) -> GridLineNames<S> {
    if explicit_line_names.is_empty() {
        return GridLineNames::default();
    }

    let total_line_count = track_counts.len() + 1;
    let negative_implicit = track_counts.negative_implicit as usize;
    let explicit_line_count = track_counts.explicit as usize + 1;

    if negative_implicit == 0 && explicit_line_names.line_count() == total_line_count {
        return explicit_line_names;
    }

    let mut line_names = GridLineNames::with_capacity(explicit_line_names.names.len(), total_line_count + 1);
    for line_index in 0..total_line_count {
        line_names.start_line();
        if line_index >= negative_implicit && line_index < negative_implicit + explicit_line_count {
            for name in explicit_line_names.line(line_index - negative_implicit) {
                line_names.push_name(name.clone());
            }
        }
    }
    line_names
}

/// Grid area information from the placement algorithm
///
/// The values is 1-indexed grid line numbers bounding the area.
/// This matches the Chrome and Firefox's format as of 2nd Jan 2024.
#[derive(Debug, Clone, PartialEq)]
#[cfg(feature = "detailed_layout_info")]
pub struct DetailedGridItemsInfo {
    /// row-start with 1-indexed grid line numbers
    pub row_start: u16,
    /// row-end with 1-indexed grid line numbers
    pub row_end: u16,
    /// column-start with 1-indexed grid line numbers
    pub column_start: u16,
    /// column-end with 1-indexed grid line numbers
    pub column_end: u16,
}

/// Grid area information from the placement algorithm
#[cfg(feature = "detailed_layout_info")]
impl DetailedGridItemsInfo {
    /// Construct from GridItems
    #[inline(always)]
    fn from_grid_item(grid_item: &GridItem) -> Self {
        /// Conversion from the indexes of Vec<GridTrack> into 1-indexed grid line numbers. See [`GridItem::row_indexes`] or [`GridItem::column_indexes`]
        #[inline(always)]
        fn to_one_indexed_grid_line(grid_track_index: u16) -> u16 {
            grid_track_index / 2 + 1
        }

        DetailedGridItemsInfo {
            row_start: to_one_indexed_grid_line(grid_item.row_indexes.start),
            row_end: to_one_indexed_grid_line(grid_item.row_indexes.end),
            column_start: to_one_indexed_grid_line(grid_item.column_indexes.start),
            column_end: to_one_indexed_grid_line(grid_item.column_indexes.end),
        }
    }
}
