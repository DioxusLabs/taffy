//! This module is a partial implementation of the CSS Grid Level 1 specification
//! <https://www.w3.org/TR/css-grid-1>
use crate::axis::{AbsoluteAxis, AbstractAxis, InBothAbsAxis};
use crate::geometry::{Line, Point, Rect, Size};
use crate::layout::{Layout, RunMode, SizeAndBaselines, SizingMode};
use crate::math::MaybeMath;
use crate::resolve::{MaybeResolve, ResolveOrZero};
use crate::style::{AlignContent, AlignItems, AlignSelf, AvailableSpace, Display, Position};
use crate::style_helpers::*;
use crate::sys::{GridTrackVec, Vec};
use crate::tree::LayoutTree;
use alignment::{align_and_position_item, align_tracks};
use explicit_grid::{compute_explicit_grid_size_in_axis, initialize_grid_tracks};
use implicit_grid::compute_grid_size_estimate;
use placement::place_grid_items;
use track_sizing::{
    determine_if_item_crosses_flexible_or_intrinsic_tracks, resolve_item_track_indexes, track_sizing_algorithm,
};
use types::{CellOccupancyMatrix, GridItem, GridTrack};

pub(crate) use types::{GridCoordinate, GridLine, OriginZeroLine};

#[cfg(feature = "debug")]
use crate::debug::NODE_LOGGER;

use super::LayoutAlgorithm;

mod alignment;
mod explicit_grid;
mod implicit_grid;
mod placement;
mod track_sizing;
mod types;
mod util;

/// The public interface to Taffy's CSS Grid algorithm implementation
pub struct CssGridAlgorithm;
impl LayoutAlgorithm for CssGridAlgorithm {
    const NAME: &'static str = "CSS GRID";

    #[inline(always)]
    fn perform_layout(
        tree: &mut impl LayoutTree,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
    ) -> SizeAndBaselines {
        compute(tree, known_dimensions, parent_size, available_space, RunMode::PeformLayout)
    }

    #[inline(always)]
    fn measure_size(
        tree: &mut impl LayoutTree,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
    ) -> Size<f32> {
        compute(tree, known_dimensions, parent_size, available_space, RunMode::ComputeSize).size
    }
}

/// Grid layout algorithm
/// This consists of a few phases:
///   - Resolving the explicit grid
///   - Placing items (which also resolves the implicit grid)
///   - Track (row/column) sizing
///   - Alignment & Final item placement
pub fn compute<Tree: LayoutTree>(
    tree: &mut Tree,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
) -> SizeAndBaselines {
    let get_child_styles_iter = || tree.children().map(|child_node: Tree::ChildId| tree.child_style(child_node));
    let style = tree.style().clone();
    let child_styles_iter = get_child_styles_iter();

    // 1. Resolve the explicit grid
    // Exactly compute the number of rows and columns in the explicit grid.
    let explicit_col_count = compute_explicit_grid_size_in_axis(&style, AbsoluteAxis::Horizontal);
    let explicit_row_count = compute_explicit_grid_size_in_axis(&style, AbsoluteAxis::Vertical);

    // 2. Implicit Grid: Estimate Track Counts
    // Estimate the number of rows and columns in the implicit grid (= the entire grid)
    // This is necessary as part of placement. Doing it early here is a perf optimisation to reduce allocations.
    let (est_col_counts, est_row_counts) =
        compute_grid_size_estimate(explicit_col_count, explicit_row_count, child_styles_iter);

    // 2. Grid Item Placement
    // Match items (children) to a definite grid position (row start/end and column start/end position)
    let mut items: Vec<GridItem<Tree>> = Vec::with_capacity(tree.child_count());
    let mut cell_occupancy_matrix = CellOccupancyMatrix::with_track_counts(est_col_counts, est_row_counts);
    let in_flow_children_iter = || {
        tree.children()
            .enumerate()
            .map(|(index, child_node)| (index, child_node, tree.child_style(child_node)))
            .filter(|(_, _, style)| style.display != Display::None && style.position != Position::Absolute)
    };
    place_grid_items(
        &mut cell_occupancy_matrix,
        &mut items,
        in_flow_children_iter,
        style.grid_auto_flow,
        style.align_items.unwrap_or(AlignItems::Stretch),
        style.justify_items.unwrap_or(AlignItems::Stretch),
    );

    // Extract track counts from previous step (auto-placement can expand the number of tracks)
    let final_col_counts = *cell_occupancy_matrix.track_counts(AbsoluteAxis::Horizontal);
    let final_row_counts = *cell_occupancy_matrix.track_counts(AbsoluteAxis::Vertical);

    // 3. Initialize Tracks
    // Initialize (explicit and implicit) grid tracks (and gutters)
    // This resolves the min and max track sizing functions for all tracks and gutters
    let mut columns = GridTrackVec::new();
    let mut rows = GridTrackVec::new();
    initialize_grid_tracks(
        &mut columns,
        final_col_counts,
        &style.grid_template_columns,
        &style.grid_auto_columns,
        style.gap.width,
        |column_index| cell_occupancy_matrix.column_is_occupied(column_index),
    );
    initialize_grid_tracks(
        &mut rows,
        final_row_counts,
        &style.grid_template_rows,
        &style.grid_auto_rows,
        style.gap.height,
        |row_index| cell_occupancy_matrix.row_is_occupied(row_index),
    );

    // 4. Compute "available grid space"
    // https://www.w3.org/TR/css-grid-1/#available-grid-space
    let padding = style.padding.resolve_or_zero(parent_size.width);
    let border = style.border.resolve_or_zero(parent_size.width);
    let aspect_ratio = style.aspect_ratio;
    let min_size = style.min_size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);
    let max_size = style.max_size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);
    let size = style.size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);

    let constrained_available_space = known_dimensions
        .or(size)
        .maybe_clamp(min_size, max_size)
        .map(|size| size.map(AvailableSpace::Definite))
        .unwrap_or(available_space.map(|space| match space {
            // Available grid space should not depend on Definite available space as a grid is allowed
            // to expand beyond it's available space
            AvailableSpace::Definite(_) => AvailableSpace::MaxContent,
            _ => space,
        }));

    let available_grid_space = Size {
        width: constrained_available_space
            .width
            .map_definite_value(|space| space - padding.horizontal_axis_sum() - border.horizontal_axis_sum()),
        height: constrained_available_space
            .height
            .map_definite_value(|space| space - padding.vertical_axis_sum() - border.vertical_axis_sum()),
    };

    let outer_node_size = known_dimensions.or(size.maybe_clamp(min_size, max_size).or(min_size));
    let mut inner_node_size = Size {
        width: outer_node_size.width.map(|space| space - padding.horizontal_axis_sum() - border.horizontal_axis_sum()),
        height: outer_node_size.height.map(|space| space - padding.vertical_axis_sum() - border.vertical_axis_sum()),
    };

    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("parent_size", parent_size);
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("outer_node_size", outer_node_size);
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("inner_node_size", inner_node_size);

    // 5. Track Sizing

    // Convert grid placements in origin-zero coordinates to indexes into the GridTrack (rows and columns) vectors
    // This computation is relatively trivial, but it requires the final number of negative (implicit) tracks in
    // each axis, and doing it up-front here means we don't have to keep repeating that calculation
    resolve_item_track_indexes(&mut items, final_col_counts, final_row_counts);

    // For each item, and in each axis, determine whether the item crosses any flexible (fr) tracks
    // Record this as a boolean (per-axis) on each item for later use in the track-sizing algorithm
    determine_if_item_crosses_flexible_or_intrinsic_tracks(&mut items, &columns, &rows);

    // Determine if the grid has any baseline aligned items
    let has_baseline_aligned_item = items.iter().any(|item| item.align_self == AlignSelf::Baseline);

    // Run track sizing algorithm for Inline axis
    track_sizing_algorithm(
        tree,
        AbstractAxis::Inline,
        min_size.get(AbstractAxis::Inline),
        max_size.get(AbstractAxis::Inline),
        style.grid_align_content(AbstractAxis::Block),
        available_grid_space,
        inner_node_size,
        &mut columns,
        &mut rows,
        &mut items,
        |track: &GridTrack, parent_size: Option<f32>| track.max_track_sizing_function.definite_value(parent_size),
        has_baseline_aligned_item,
    );
    let initial_column_sum = columns.iter().map(|track| track.base_size).sum::<f32>();
    inner_node_size.width = inner_node_size.width.or_else(|| initial_column_sum.into());

    items.iter_mut().for_each(|item| item.available_space_cache = None);

    // Run track sizing algorithm for Block axis
    track_sizing_algorithm(
        tree,
        AbstractAxis::Block,
        min_size.get(AbstractAxis::Block),
        max_size.get(AbstractAxis::Block),
        style.grid_align_content(AbstractAxis::Inline),
        available_grid_space,
        inner_node_size,
        &mut rows,
        &mut columns,
        &mut items,
        |track: &GridTrack, _| Some(track.base_size),
        false, // TODO: Support baseline alignment in the vertical axis
    );
    let initial_row_sum = rows.iter().map(|track| track.base_size).sum::<f32>();
    inner_node_size.height = inner_node_size.height.or_else(|| initial_row_sum.into());

    // 6. Compute container size
    let resolved_style_size = known_dimensions.or(style.size.maybe_resolve(parent_size));
    let container_border_box = Size {
        width: resolved_style_size
            .get(AbstractAxis::Inline)
            .unwrap_or_else(|| initial_column_sum + padding.horizontal_axis_sum() + border.horizontal_axis_sum()),
        height: resolved_style_size
            .get(AbstractAxis::Block)
            .unwrap_or_else(|| initial_row_sum + padding.vertical_axis_sum() + border.vertical_axis_sum()),
    };
    let container_content_box = Size {
        width: container_border_box.width - padding.horizontal_axis_sum() - border.horizontal_axis_sum(),
        height: container_border_box.height - padding.vertical_axis_sum() - border.vertical_axis_sum(),
    };

    // If only the container's size has been requested
    if run_mode == RunMode::ComputeSize {
        return container_border_box.into();
    }

    // 7. Resolve percentage track base sizes
    // In the case of an indefinitely sized container these resolve to zero during the "Initialise Tracks" step
    // and therefore need to be re-resolved here based on the content-sized content box of the container
    if !available_grid_space.width.is_definite() {
        for column in &mut columns {
            let min: Option<f32> =
                column.min_track_sizing_function.resolved_percentage_size(container_content_box.width);
            let max: Option<f32> =
                column.max_track_sizing_function.resolved_percentage_size(container_content_box.width);
            column.base_size = column.base_size.maybe_clamp(min, max);
        }
    }
    if !available_grid_space.height.is_definite() {
        for row in &mut rows {
            let min: Option<f32> = row.min_track_sizing_function.resolved_percentage_size(container_content_box.height);
            let max: Option<f32> = row.max_track_sizing_function.resolved_percentage_size(container_content_box.height);
            row.base_size = row.base_size.maybe_clamp(min, max);
        }
    }

    // Column sizing must be re-run (once) if:
    //   - The grid container's width was initially indefinite and there are any columns with percentage track sizing functions
    //   - Any grid item crossing an intrinsically sized track's min content contribution width has changed
    // TODO: Only rerun sizing for tracks that actually require it rather than for all tracks if any need it.
    let mut rerun_column_sizing;

    let has_percentage_column = columns.iter().any(|track| track.uses_percentage());
    let parent_width_indefinite = !available_space.width.is_definite();
    rerun_column_sizing = parent_width_indefinite && has_percentage_column;

    if !rerun_column_sizing {
        let min_content_contribution_changed = items
            .iter_mut()
            .filter(|item| item.crosses_intrinsic_column)
            .map(|item| {
                let available_space = item.available_space(
                    AbstractAxis::Inline,
                    &rows,
                    inner_node_size.height,
                    |track: &GridTrack, _| Some(track.base_size),
                );
                let new_min_content_contribution =
                    item.min_content_contribution(AbstractAxis::Inline, tree, available_space, inner_node_size);

                let has_changed = Some(new_min_content_contribution) != item.min_content_contribution_cache.width;

                item.available_space_cache = Some(available_space);
                item.min_content_contribution_cache.width = Some(new_min_content_contribution);
                item.max_content_contribution_cache.width = None;
                item.minimum_contribution_cache.width = None;

                has_changed
            })
            .any(|has_changed| has_changed);
        rerun_column_sizing = min_content_contribution_changed;
    } else {
        // Clear intrisic width caches
        items.iter_mut().for_each(|item| {
            item.available_space_cache = None;
            item.min_content_contribution_cache.width = None;
            item.max_content_contribution_cache.width = None;
            item.minimum_contribution_cache.width = None;
        });
    }

    if rerun_column_sizing {
        // Re-run track sizing algorithm for Inline axis
        track_sizing_algorithm(
            tree,
            AbstractAxis::Inline,
            min_size.get(AbstractAxis::Inline),
            max_size.get(AbstractAxis::Inline),
            style.grid_align_content(AbstractAxis::Block),
            available_grid_space,
            inner_node_size,
            &mut columns,
            &mut rows,
            &mut items,
            |track: &GridTrack, _| Some(track.base_size),
            has_baseline_aligned_item,
        );

        // Row sizing must be re-run (once) if:
        //   - The grid container's height was initially indefinite and there are any rows with percentage track sizing functions
        //   - Any grid item crossing an intrinsically sized track's min content contribution height has changed
        // TODO: Only rerun sizing for tracks that actually require it rather than for all tracks if any need it.
        let mut rerun_row_sizing;

        let has_percentage_row = rows.iter().any(|track| track.uses_percentage());
        let parent_height_indefinite = !available_space.height.is_definite();
        rerun_row_sizing = parent_height_indefinite && has_percentage_row;

        if !rerun_row_sizing {
            let min_content_contribution_changed = items
                .iter_mut()
                .filter(|item| item.crosses_intrinsic_column)
                .map(|item| {
                    let available_space = item.available_space(
                        AbstractAxis::Block,
                        &columns,
                        inner_node_size.width,
                        |track: &GridTrack, _| Some(track.base_size),
                    );
                    let new_min_content_contribution =
                        item.min_content_contribution(AbstractAxis::Block, tree, available_space, inner_node_size);

                    let has_changed = Some(new_min_content_contribution) != item.min_content_contribution_cache.height;

                    item.available_space_cache = Some(available_space);
                    item.min_content_contribution_cache.height = Some(new_min_content_contribution);
                    item.max_content_contribution_cache.height = None;
                    item.minimum_contribution_cache.height = None;

                    has_changed
                })
                .any(|has_changed| has_changed);
            rerun_row_sizing = min_content_contribution_changed;
        } else {
            items.iter_mut().for_each(|item| {
                // Clear intrisic height caches
                item.available_space_cache = None;
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
                min_size.get(AbstractAxis::Block),
                max_size.get(AbstractAxis::Block),
                style.grid_align_content(AbstractAxis::Inline),
                available_grid_space,
                inner_node_size,
                &mut rows,
                &mut columns,
                &mut items,
                |track: &GridTrack, _| Some(track.base_size),
                false, // TODO: Support baseline alignment in the vertical axis
            );
        }
    }

    // 8. Track Alignment

    // Align columns
    align_tracks(
        container_content_box.get(AbstractAxis::Inline),
        Line { start: padding.left, end: padding.right },
        Line { start: border.left, end: border.right },
        &mut columns,
        style.justify_content.unwrap_or(AlignContent::Stretch),
    );
    // Align rows
    align_tracks(
        container_content_box.get(AbstractAxis::Block),
        Line { start: padding.top, end: padding.bottom },
        Line { start: border.top, end: border.bottom },
        &mut rows,
        style.align_content.unwrap_or(AlignContent::Stretch),
    );

    // 9. Size, Align, and Position Grid Items

    // Sort items back into original order to allow them to be matched up with styles
    items.sort_by_key(|item| item.source_order);

    let container_alignment_styles = InBothAbsAxis { horizontal: style.justify_items, vertical: style.align_items };

    // Position in-flow children (stored in items vector)
    for (index, item) in items.iter().enumerate() {
        let grid_area = Rect {
            top: rows[item.row_indexes.start as usize + 1].offset,
            bottom: rows[item.row_indexes.end as usize].offset,
            left: columns[item.column_indexes.start as usize + 1].offset,
            right: columns[item.column_indexes.end as usize].offset,
        };
        align_and_position_item(
            tree,
            item.node,
            index as u32,
            grid_area,
            container_alignment_styles,
            item.baseline_shim,
        );
    }

    // Position hidden and absolutely positioned children
    let mut order = items.len() as u32;
    (0..tree.child_count()).for_each(|index| {
        let child = tree.child(index);
        let child_style = tree.child_style(child);

        // Position hidden child
        if child_style.display == Display::None {
            *tree.layout_mut() = Layout::with_order(order);
            tree.perform_child_layout(child, Size::NONE, Size::NONE, Size::MAX_CONTENT, SizingMode::InherentSize);
            order += 1;
            return;
        }

        // Position absolutely positioned child
        if child_style.position == Position::Absolute {
            // Convert grid-col-{start/end} into Option's of indexes into the columns vector
            // The Option is None if the style property is Auto and an unresolvable Span
            let maybe_col_indexes = child_style
                .grid_column
                .into_origin_zero(final_col_counts.explicit)
                .resolve_absolutely_positioned_grid_tracks()
                .map(|maybe_grid_line| {
                    maybe_grid_line.map(|line: OriginZeroLine| line.into_track_vec_index(final_col_counts))
                });
            // Convert grid-row-{start/end} into Option's of indexes into the row vector
            // The Option is None if the style property is Auto and an unresolvable Span
            let maybe_row_indexes = child_style
                .grid_row
                .into_origin_zero(final_row_counts.explicit)
                .resolve_absolutely_positioned_grid_tracks()
                .map(|maybe_grid_line| {
                    maybe_grid_line.map(|line: OriginZeroLine| line.into_track_vec_index(final_row_counts))
                });

            let grid_area = Rect {
                top: maybe_row_indexes.start.map(|index| rows[index].offset).unwrap_or(border.top),
                bottom: maybe_row_indexes
                    .end
                    .map(|index| rows[index].offset)
                    .unwrap_or(container_border_box.height - border.bottom),
                left: maybe_col_indexes.start.map(|index| columns[index].offset).unwrap_or(border.left),
                right: maybe_col_indexes
                    .end
                    .map(|index| columns[index].offset)
                    .unwrap_or(container_border_box.width - border.right),
            };
            // TODO: Baseline alignment support for absolutely positioned items (should check if is actuallty specified)
            align_and_position_item(tree, child, order, grid_area, container_alignment_styles, 0.0);
            order += 1;
        }
    });

    // If there are not items then return just the container size (no baseline)
    if items.is_empty() {
        return SizeAndBaselines { size: container_border_box, first_baselines: Point::NONE };
    }

    // Determine the grid container baseline(s) (currently we only compute the first baseline)
    let grid_container_baseline: f32 = {
        // Sort items by row start position so that we can iterate items in groups which are in the same row
        items.sort_by_key(|item| item.row_indexes.start);

        // Get the row index of the first row containing items
        let first_row = items[0].row_indexes.start;

        // Create a slice of all of the items start in this row (taking advantage of the fact that we have just sorted the array)
        let first_row_items = &items[0..].split(|item| item.row_indexes.start != first_row).next().unwrap();

        // Check if any items in *this row* are baseline aligned
        let row_has_baseline_item = first_row_items.iter().any(|item| item.align_self == AlignSelf::Baseline);

        let item = if row_has_baseline_item {
            first_row_items.iter().find(|item| item.align_self == AlignSelf::Baseline).unwrap()
        } else {
            &first_row_items[0]
        };

        let layout = tree.child_layout_mut(item.node);
        layout.location.y + item.baseline.unwrap_or(layout.size.height)
    };

    SizeAndBaselines {
        size: container_border_box,
        first_baselines: Point { x: None, y: Some(grid_container_baseline) },
    }
}
