//! This module is a partial implementation of the CSS Grid Level 2 specification
//! https://www.w3.org/TR/css-grid-2/
use crate::geometry::Size;
use crate::layout::AvailableSpace;
use crate::node::Node;
use crate::sys::Vec;
use crate::tree::LayoutTree;
use explicit_grid::{compute_explicit_grid_size, initialize_grid_tracks};
use placement::CellOccupancyMatrix;
use placement::{compute_grid_size_estimate, place_grid_items};
use types::{CssGrid, GridAxisTracks, GridTrack};

mod explicit_grid;
mod placement;
#[cfg(test)]
mod test_helpers;
mod types;

pub use types::AbsoluteAxis;

pub fn compute(tree: &mut impl LayoutTree, root: Node, available_space: Size<AvailableSpace>) {
    let get_child_styles_iter = |node| tree.children(node).into_iter().map(|child_node: &Node| tree.style(*child_node));
    let style = tree.style(root);
    let child_styles_iter = get_child_styles_iter(root);

    // Resolve the number of rows and columns in the explicit grid.
    let (explicit_col_count, explicit_row_count) = compute_explicit_grid_size(style);

    // Estimate the number of rows and columns in the grid
    // This is necessary as part of placement. Doing it early here is a perf optimisation to reduce allocations.
    let grid_size_est = compute_grid_size_estimate(explicit_col_count, explicit_row_count, child_styles_iter);

    // Place Grid Items
    // Matches items to a definite grid position (row start/end and column start/end position)
    // https://www.w3.org/TR/css-grid-1/#placement
    let mut items = Vec::with_capacity(tree.child_count(root));
    let mut cell_occupancy_matrix = CellOccupancyMatrix::with_track_counts(grid_size_est.height, grid_size_est.width);
    let grid_auto_flow = style.grid_auto_flow;
    let children_iter =
        || tree.children(root).into_iter().copied().map(|child_node| (child_node, tree.style(child_node)));
    place_grid_items(&mut cell_occupancy_matrix, &mut items, children_iter, grid_auto_flow);

    // Extract track counts from previous step (auto-placement can expand the number of tracks)
    let final_col_counts = *cell_occupancy_matrix.track_counts(AbsoluteAxis::Horizontal);
    let final_row_counts = *cell_occupancy_matrix.track_counts(AbsoluteAxis::Vertical);

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

    let named_areas = Vec::new();
    let grid = CssGrid { available_space, cell_occupancy_matrix, named_areas, items, columns, rows };
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
