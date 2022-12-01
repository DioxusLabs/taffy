//! This module is a partial implementation of the CSS Grid Level 2 specification
//! https://www.w3.org/TR/css-grid-2/
use crate::compute::compute_node_layout;
use crate::geometry::{Line, Point, Size};
use crate::layout::{AvailableSpace, Layout};
use crate::node::Node;
use crate::resolve::MaybeResolve;
use crate::style::{AlignContent, AlignItems, AlignSelf, JustifyItems};
use crate::sys::{f32_max, Vec};
use crate::tree::LayoutTree;
use alignment::align_tracks;
use explicit_grid::{compute_explicit_grid_size, initialize_grid_tracks};
use placement::CellOccupancyMatrix;
use placement::{compute_grid_size_estimate, place_grid_items};
use track_sizing::{
    determine_if_item_crosses_flexible_tracks, resolve_item_track_indexes, track_sizing_algorithm, AvailableSpaceMode,
};
use types::{CssGrid, GridAxisTracks, GridItem, GridTrack};

mod alignment;
mod explicit_grid;
mod placement;
#[cfg(test)]
mod test_helpers;
mod track_sizing;
mod types;

pub(crate) use types::{AbsoluteAxis, GridAxis};

pub fn compute(tree: &mut impl LayoutTree, root: Node, available_space: Size<AvailableSpace>) -> Size<f32> {
    let get_child_styles_iter = |node| tree.children(node).into_iter().map(|child_node: &Node| tree.style(*child_node));
    let style = tree.style(root).clone();
    let child_styles_iter = get_child_styles_iter(root);

    // 1. Size Computation
    // Exactly compute the number of rows and columns in the explicit grid.
    // Estimate the number of rows and columns in the implicit grid (= the entire grid)
    // This is necessary as part of placement. Doing it early here is a perf optimisation to reduce allocations.
    let (explicit_col_count, explicit_row_count) = compute_explicit_grid_size(&style);
    let (est_col_counts, est_row_counts) =
        compute_grid_size_estimate(explicit_col_count, explicit_row_count, child_styles_iter);

    // 2a. Grid Item Placement
    // Match items (children) to a definite grid position (row start/end and column start/end position)
    let mut items = Vec::with_capacity(tree.child_count(root));
    let mut cell_occupancy_matrix = CellOccupancyMatrix::with_track_counts(est_col_counts, est_row_counts);
    let grid_auto_flow = style.grid_auto_flow;
    let children_iter =
        || tree.children(root).into_iter().copied().map(|child_node| (child_node, tree.style(child_node)));
    place_grid_items(&mut cell_occupancy_matrix, &mut items, children_iter, grid_auto_flow);

    // 2b. Extract final track counts
    // Extract track counts from previous step (auto-placement can expand the number of tracks)
    let final_col_counts = *cell_occupancy_matrix.track_counts(AbsoluteAxis::Horizontal);
    let final_row_counts = *cell_occupancy_matrix.track_counts(AbsoluteAxis::Vertical);

    // 3. Initialize Tracks
    // Initialize (explicit and implicit) grid tracks (and gutters)
    // This resolves the min and max track sizing functions for all tracks and gutters
    let mut columns = GridAxisTracks::with_counts(final_col_counts);
    let mut rows = GridAxisTracks::with_counts(final_row_counts);
    initialize_grid_tracks(
        &mut columns.tracks,
        final_col_counts,
        &style.grid_template_columns,
        &style.grid_auto_columns,
        style.gap.width.into(),
    );
    initialize_grid_tracks(
        &mut rows.tracks,
        final_row_counts,
        &style.grid_template_rows,
        &style.grid_auto_rows,
        style.gap.height.into(),
    );

    // 4. Track Sizing

    // Convert grid placements in origin-zero coordinates to indexes into the GridTrack (rows and columns) vectors
    // This computation is relatively trivial, but it requires the final number of negative (implicit) tracks in
    // each axis, and doing it up-front here means we don't have to keep repeating that calculation
    resolve_item_track_indexes(&mut items, final_col_counts, final_row_counts);

    // For each item, and in each axis, determine whether the item crosses any flexible (fr) tracks
    // Record this as a boolean (per-axis) on each item for later use in the track-sizing algorithm
    determine_if_item_crosses_flexible_tracks(&mut items, &columns.tracks, &rows.tracks);

    // Run track sizing algorithm for Inline axis
    track_sizing_algorithm(
        tree,
        available_space,
        AvailableSpaceMode::Estimates,
        GridAxis::Inline,
        &mut columns.tracks,
        &style,
        &mut rows.tracks,
        &mut items,
        compute_node_layout,
    );
    // Run track sizing algorithm for Block axis
    track_sizing_algorithm(
        tree,
        available_space,
        AvailableSpaceMode::Estimates,
        GridAxis::Block,
        &mut columns.tracks,
        &style,
        &mut rows.tracks,
        &mut items,
        compute_node_layout,
    );

    // 5. Alignment

    // Align tracks
    let resolved_style_size = style.size.maybe_resolve(available_space.as_options());
    align_tracks(
        resolved_style_size.get(GridAxis::Inline),
        &mut columns.tracks,
        style.justify_content.unwrap_or(AlignContent::Stretch),
    );
    align_tracks(
        resolved_style_size.get(GridAxis::Block),
        &mut rows.tracks,
        style.align_content.unwrap_or(AlignContent::Stretch),
    );

    let container_size = Size {
        width: resolved_style_size
            .get(GridAxis::Inline)
            .unwrap_or_else(|| columns.tracks.iter().map(|track| track.base_size).sum()),
        height: resolved_style_size
            .get(GridAxis::Block)
            .unwrap_or_else(|| rows.tracks.iter().map(|track| track.base_size).sum()),
    };

    perform_final_layout(
        tree,
        &mut items,
        &mut rows.tracks,
        &mut columns.tracks,
        available_space,
        style.align_items,
        style.justify_items,
    );

    let named_areas = Vec::new();
    let grid = CssGrid { available_space, cell_occupancy_matrix, named_areas, items, columns, rows };

    container_size
}

fn perform_final_layout(
    tree: &mut impl LayoutTree,
    items: &[GridItem],
    rows: &[GridTrack],
    columns: &[GridTrack],
    available_space: Size<AvailableSpace>,
    align_items: Option<AlignItems>,
    justify_items: Option<JustifyItems>,
) {
    let align_items = align_items;
    let justify_items = justify_items;

    items.iter().enumerate().for_each(|(i, item)| {
        let style = tree.style(item.node);
        let inherent_size = style.size.maybe_resolve(available_space.as_options());

        let (x, width) = align_and_size_item_within_area(
            columns,
            item.column_indexes,
            style.justify_self.or(justify_items),
            inherent_size.height,
            style.aspect_ratio,
        );

        let (y, height) = align_and_size_item_within_area(
            rows,
            item.row_indexes,
            style.align_self.or(align_items),
            inherent_size.width,
            style.aspect_ratio,
        );

        *tree.layout_mut(item.node) =
            Layout { order: i as u32, size: Size { width, height }, location: Point { x, y } };
    });
}

fn align_and_size_item_within_area(
    tracks: &[GridTrack],
    indexes: Line<u16>,
    alignment_style: Option<AlignSelf>,
    size: Option<f32>,
    aspect_ratio: Option<f32>,
) -> (f32, f32) {
    // Calculate grid area dimension in the axis
    let area_start = tracks[(indexes.start + 1) as usize].offset;
    let area_end = {
        let row = &tracks[(indexes.end - 1) as usize];
        row.offset + row.base_size
    };
    let free_space = area_end - area_start;
    let origin = f32_max(free_space, 0.0);

    // Compute default alignment style if it set on neither the parent or the node itself
    let alignment_style = alignment_style.unwrap_or_else(|| {
        if size.is_some() || aspect_ratio.is_some() {
            AlignSelf::Start
        } else {
            AlignSelf::Stretch
        }
    });

    // Compute size in the axis
    let size = match alignment_style {
        AlignItems::Stretch => free_space,
        _ => {
            size.unwrap_or_else(|| {
                // Todo: measure node
                0.0
            })
        }
    };

    // Compute offset in the axis
    let offset_within_area = match alignment_style {
        AlignSelf::Start => 0.0,
        AlignSelf::End => origin - size,
        AlignSelf::Center => (origin - size) / 2.0,
        // TODO: Add support for baseline alignment. For now we treat it as "start".
        AlignSelf::Baseline => 0.0,
        AlignSelf::Stretch => 0.0,
    };

    (area_start + offset_within_area, size)
}

fn populate_negative_grid_tracks(axis: &mut GridAxisTracks) {
    debug_assert!(
        axis.tracks.len() != 0,
        "populate_negative_grid_tracks should only ever be called on an empty grid axis"
    );
    debug_assert!(axis.origin % 2 != 0, "axis.origin should always be even");

    // If origin is zero then there are no negative grid tracks
    if axis.origin == 0 {
        return;
    }

    for _ in 0..axis.origin {
        axis.push(GridTrack::uninit());
    }
}
