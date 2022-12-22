//! This module is a partial implementation of the CSS Grid Level 1 specification
//! https://www.w3.org/TR/css-grid-1/
use crate::axis::{AbsoluteAxis, AbstractAxis, InBothAbsAxis};
use crate::geometry::{Line, Rect, Size};
use crate::layout::{Layout, RunMode, SizingMode};
use crate::math::MaybeMath;
use crate::node::Node;
use crate::resolve::{MaybeResolve, ResolveOrZero};
use crate::style::{AlignContent, AvailableSpace, Display, PositionType};
use crate::style_helpers::*;
use crate::sys::{GridTrackVec, Vec};
use crate::tree::LayoutTree;
use alignment::{align_and_position_item, align_tracks};
use explicit_grid::{compute_explicit_grid_size_in_axis, initialize_grid_tracks};
use implicit_grid::compute_grid_size_estimate;
use placement::place_grid_items;
use track_sizing::{determine_if_item_crosses_flexible_tracks, resolve_item_track_indexes, track_sizing_algorithm};
use types::{CellOccupancyMatrix, GridTrack};
use util::coordinates::css_grid_line_into_origin_zero_coords;

use super::compute_node_layout;

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
pub fn compute(tree: &mut impl LayoutTree, node: Node, available_space: Size<AvailableSpace>) -> Size<f32> {
    let get_child_styles_iter = |node| tree.children(node).into_iter().map(|child_node: &Node| tree.style(*child_node));
    let style = tree.style(node).clone();
    let child_styles_iter = get_child_styles_iter(node);

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
    let mut items = Vec::with_capacity(tree.child_count(node));
    let mut cell_occupancy_matrix = CellOccupancyMatrix::with_track_counts(est_col_counts, est_row_counts);
    let grid_auto_flow = style.grid_auto_flow;
    let in_flow_children_iter = || {
        tree.children(node)
            .into_iter()
            .copied()
            .map(|child_node| (child_node, tree.style(child_node)))
            .filter(|(_, style)| style.display != Display::None && style.position_type != PositionType::Absolute)
    };
    place_grid_items(&mut cell_occupancy_matrix, &mut items, in_flow_children_iter, grid_auto_flow);

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
    let padding = style.padding.resolve_or_zero(available_space.width.into_option());
    let border = style.border.resolve_or_zero(available_space.width.into_option());
    let min_size = style.min_size.maybe_resolve(available_space.into_options());
    let max_size = style.max_size.maybe_resolve(available_space.into_options());
    let size = style.size.maybe_resolve(available_space.into_options());

    let constrained_available_space = size
        .maybe_clamp(min_size, max_size)
        .map(|size| size.map(AvailableSpace::Definite))
        .unwrap_or(available_space.maybe_clamp(min_size, max_size));

    let available_grid_space = Size {
        width: constrained_available_space
            .width
            .map_definite_value(|space| space - padding.horizontal_axis_sum() - border.horizontal_axis_sum()),
        height: constrained_available_space
            .height
            .map_definite_value(|space| space - padding.vertical_axis_sum() - border.vertical_axis_sum()),
    };

    // 5. Track Sizing

    // Convert grid placements in origin-zero coordinates to indexes into the GridTrack (rows and columns) vectors
    // This computation is relatively trivial, but it requires the final number of negative (implicit) tracks in
    // each axis, and doing it up-front here means we don't have to keep repeating that calculation
    resolve_item_track_indexes(&mut items, final_col_counts, final_row_counts);

    // For each item, and in each axis, determine whether the item crosses any flexible (fr) tracks
    // Record this as a boolean (per-axis) on each item for later use in the track-sizing algorithm
    determine_if_item_crosses_flexible_tracks(&mut items, &columns, &rows);

    // Run track sizing algorithm for Inline axis
    track_sizing_algorithm(
        tree,
        AbstractAxis::Inline,
        available_space,
        available_grid_space,
        &style,
        &mut columns,
        &mut rows,
        &mut items,
        |track: &GridTrack, available_space: AvailableSpace| {
            track.max_track_sizing_function.definite_value(available_space)
        },
    );
    // Run track sizing algorithm for Block axis
    track_sizing_algorithm(
        tree,
        AbstractAxis::Block,
        available_space,
        available_grid_space,
        &style,
        &mut rows,
        &mut columns,
        &mut items,
        |track: &GridTrack, available_space: AvailableSpace| {
            track.max_track_sizing_function.definite_value(available_space)
        },
    );
    // Re-run track sizing algorithm for Inline axis
    track_sizing_algorithm(
        tree,
        AbstractAxis::Inline,
        available_space,
        available_grid_space,
        &style,
        &mut columns,
        &mut rows,
        &mut items,
        |track: &GridTrack, _| Some(track.base_size),
    );
    // Re-run track sizing algorithm for Block axis
    track_sizing_algorithm(
        tree,
        AbstractAxis::Block,
        available_space,
        available_grid_space,
        &style,
        &mut rows,
        &mut columns,
        &mut items,
        |track: &GridTrack, _| Some(track.base_size),
    );

    // 6. Compute container size
    let resolved_style_size = style.size.maybe_resolve(available_space.into_options());
    let container_border_box = Size {
        width: resolved_style_size.get(AbstractAxis::Inline).unwrap_or_else(|| {
            columns.iter().map(|track| track.base_size).sum::<f32>()
                + padding.horizontal_axis_sum()
                + border.horizontal_axis_sum()
        }),
        height: resolved_style_size.get(AbstractAxis::Block).unwrap_or_else(|| {
            rows.iter().map(|track| track.base_size).sum::<f32>()
                + padding.vertical_axis_sum()
                + border.vertical_axis_sum()
        }),
    };
    let container_content_box = Size {
        width: container_border_box.width - padding.horizontal_axis_sum() - border.horizontal_axis_sum(),
        height: container_border_box.height - padding.vertical_axis_sum() - border.vertical_axis_sum(),
    };

    // 7. Track Alignment

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

    // 8. Size, Align, and Position Grid Items

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
            container_content_box,
            container_alignment_styles,
        );
    }

    // Position hidden and absolutely positioned children
    let mut order = items.len() as u32;
    (0..tree.child_count(node)).for_each(|index| {
        let child = tree.child(node, index);
        let child_style = tree.style(child);

        // Position hidden child
        if child_style.display == Display::None {
            *tree.layout_mut(node) = Layout::with_order(order);
            compute_node_layout(
                tree,
                child,
                Size::NONE,
                Size::MAX_CONTENT,
                RunMode::PeformLayout,
                SizingMode::InherentSize,
            );
            order += 1;
            return;
        }

        // Position absolutely positioned child
        if child_style.position_type == PositionType::Absolute {
            // Convert grid-col-{start/end} into Option's of indexes into the columns vector
            // The Option is None if the style property is Auto and an unresolvable Span
            let maybe_grid_cols = child_style.grid_column.resolve_absolutely_positioned_grid_tracks();
            let maybe_col_indexes = maybe_grid_cols.map(|maybe_grid_line| {
                maybe_grid_line.map(|grid_line| {
                    let oz_line = css_grid_line_into_origin_zero_coords(grid_line, final_col_counts.explicit);
                    final_col_counts.oz_line_to_grid_track_vec_index(oz_line) as usize
                })
            });
            // Convert grid-row-{start/end} into Option's of indexes into the row vector
            // The Option is None if the style property is Auto and an unresolvable Span
            let maybe_grid_rows = child_style.grid_row.resolve_absolutely_positioned_grid_tracks();
            let maybe_row_indexes = maybe_grid_rows.map(|maybe_grid_line| {
                maybe_grid_line.map(|grid_line| {
                    let oz_line = css_grid_line_into_origin_zero_coords(grid_line, final_row_counts.explicit);
                    final_row_counts.oz_line_to_grid_track_vec_index(oz_line) as usize
                })
            });

            let grid_area = Rect {
                top: maybe_row_indexes.start.map(|index| rows[index].offset).unwrap_or(0.0),
                bottom: maybe_row_indexes.end.map(|index| rows[index].offset).unwrap_or(container_border_box.height),
                left: maybe_col_indexes.start.map(|index| columns[index].offset).unwrap_or(0.0),
                right: maybe_col_indexes.end.map(|index| columns[index].offset).unwrap_or(container_border_box.width),
            };
            align_and_position_item(tree, child, order, grid_area, container_content_box, container_alignment_styles);
            order += 1;
            return;
        }
    });

    container_border_box
}
