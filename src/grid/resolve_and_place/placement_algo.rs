use super::super::types::{CssGrid, GridItem, RowColumn};
use super::{CellOccupancyMatrix, CellOccupancyState};
use crate::geometry::Line;
use crate::node::Node;
use crate::style::{GridAutoFlow, Style};
use crate::tree::LayoutTree;

/// 8.5. Grid Item Placement Algorithm
/// Place items into the grid, generating new rows/column into the implicit grid as required
///
/// [Specification](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)
pub(in crate::grid) fn place_grid_items(grid: &mut CssGrid, tree: &impl LayoutTree, node: Node) {
    let grid_auto_flow = tree.style(node).grid_auto_flow;
    let flow_direction = grid_auto_flow.flow_direction();

    // Create a vector of children's styles that we can iterate over multiple times
    let children = || tree.children(node).into_iter().copied();

    // 1. Place children with definite positions
    children()
        .map(|child_node| tree.style(child_node))
        .filter(|child_style| child_style.grid_row.is_definite() && child_style.grid_column.is_definite())
        .for_each(|child_style| {
            let (row_span, col_span) = place_definite_grid_item(child_style);
            record_grid_placement(grid, node, RowColumn::Row, row_span, col_span, CellOccupancyState::DefinitelyPlaced);
        });

    // 2. Place remaining children with definite primary axis positions
    children()
        .map(|child_node| tree.style(child_node))
        .filter(|child_style| {
            child_style.grid_placement(flow_direction).is_definite()
                && !child_style.grid_placement(flow_direction.opposite_axis()).is_definite()
        })
        .for_each(|child_style| {
            let com = &grid.cell_occupancy_matrix;
            let (primary_span, secondary_span) = place_definite_primary_axis_item(com, child_style, grid_auto_flow);

            let placement_type = CellOccupancyState::AutoPlaced;
            record_grid_placement(grid, node, flow_direction, primary_span, secondary_span, placement_type);
        });

    // 3. Determine the number of columns in the implicit grid
    // By the time we get to this point in the execution, this is actually already accounted for:
    //
    // 3.1 Start with the columns from the explicit grid
    //        => Handled by grid size estimate which is used to pre-size the GridOccupancyMatrix
    //
    // 3.2 Among all the items with a definite column position (explicitly positioned items, items positioned in the previous step,
    //     and items not yet positioned but with a definite column) add columns to the beginning and end of the implicit grid as necessary
    //     to accommodate those items.
    //        => Handled by expand_to_fit_range which expands the GridOccupancyMatrix as necessary
    //            -> Called by mark_area_as
    //            -> Called by record_grid_placement
    //
    // 3.3 If the largest column span among all the items without a definite column position is larger than the width of
    //     the implicit grid, add columns to the end of the implicit grid to accommodate that column span.
    //        => Handled by grid size estimate which is used to pre-size the GridOccupancyMatrix

    // 4. Position the remaining grid items
    // (which either have definite position only in the secondary axis or indefinite positions in both axis)
    let mut grid_position: (u16, u16) = (0, 0);
    children()
        .map(|child_node| tree.style(child_node))
        .filter(|child_style| !child_style.grid_row.is_definite() && !child_style.grid_column.is_definite())
        .for_each(|child_style| {
            // Compute placement
            let com = &grid.cell_occupancy_matrix;
            let (primary_span, secondary_span) =
                place_indefinitely_positioned_item(com, child_style, grid_auto_flow, grid_position);

            // Record item
            let placement_type = CellOccupancyState::AutoPlaced;
            record_grid_placement(grid, node, flow_direction, primary_span, secondary_span, placement_type);

            // If using the "dense" placement algorithm then reset the grid position back to (0, 0) ready for the next item
            // Otherwise set it to the position of the current item so that the next item it placed after it.
            grid_position = match grid_auto_flow.is_dense() {
                true => (0, 0),
                false => (primary_span.start as u16, secondary_span.end as u16),
            }
        });
}

/// 8.5. Grid Item Placement Algorithm
/// Place a single definitely placed item into the grid
fn place_definite_grid_item(style: &Style) -> (Line<i16>, Line<i16>) {
    // Resolve spans to tracks
    let row_span = style.grid_row.resolve_definite_grid_tracks();
    let column_span = style.grid_column.resolve_definite_grid_tracks();

    (row_span, column_span)
}

fn place_definite_primary_axis_item(
    cell_occupancy_matrix: &CellOccupancyMatrix,
    style: &Style,
    auto_flow: GridAutoFlow,
) -> (Line<i16>, Line<i16>) {
    let flow_direction = auto_flow.flow_direction();
    let primary_axis_placement = style.grid_placement(flow_direction).resolve_definite_grid_tracks();
    let starting_position = match auto_flow.is_dense() {
        true => 1,
        false => cell_occupancy_matrix
            .last_of_type(flow_direction, primary_axis_placement.start as i16, CellOccupancyState::AutoPlaced)
            .unwrap_or(1),
    };

    let mut position: i16 = starting_position;
    loop {
        let secondary_axis_placement =
            style.grid_placement(flow_direction.opposite_axis()).resolve_indefinite_grid_tracks(position);

        let does_fit = cell_occupancy_matrix.line_area_is_unoccupied(
            flow_direction,
            primary_axis_placement,
            secondary_axis_placement,
        );

        if does_fit {
            return (primary_axis_placement, secondary_axis_placement);
        } else {
            position += 1;
        }
    }
}

fn place_indefinitely_positioned_item(
    cell_occupancy_matrix: &CellOccupancyMatrix,
    style: &Style,
    auto_flow: GridAutoFlow,
    grid_position: (u16, u16),
) -> (Line<i16>, Line<i16>) {
    let flow_direction = auto_flow.flow_direction();

    let primary_placement_style = style.grid_placement(flow_direction);
    let secondary_placement_style = style.grid_placement(flow_direction.opposite_axis());

    let primary_span = primary_placement_style.indefinite_span();
    let secondary_span = secondary_placement_style.indefinite_span();
    let has_definite_secondary_axis_position = secondary_placement_style.is_definite();
    let secondary_axis_length = cell_occupancy_matrix.track_counts(flow_direction).len() as i16;

    let track_area_is_unoccupied = |primary_range, secondary_range| {
        cell_occupancy_matrix.track_area_is_unoccupied(flow_direction, primary_range, secondary_range)
    };
    let tracks_to_lines = |range| cell_occupancy_matrix.tracks_to_lines(flow_direction, range);

    let (mut primary_idx, mut secondary_idx) = grid_position;
    if has_definite_secondary_axis_position {
        let definite_secondary_placement = secondary_placement_style.resolve_definite_grid_tracks();
        secondary_idx = cell_occupancy_matrix
            .lines_to_tracks(flow_direction.opposite_axis(), definite_secondary_placement)
            .start as u16;
    }
    loop {
        let primary_range = (primary_idx as i16)..((primary_idx + primary_span) as i16);
        let secondary_range = (secondary_idx as i16)..((secondary_idx + secondary_span) as i16);

        // Check if item fits in it's current position, and if so place it there
        let secondary_out_of_bounds =
            !has_definite_secondary_axis_position && secondary_range.end >= secondary_axis_length;
        let place_here =
            !secondary_out_of_bounds && track_area_is_unoccupied(primary_range.clone(), secondary_range.clone());
        if place_here {
            let primary_span = tracks_to_lines(primary_range.clone());
            let secondary_span = tracks_to_lines(secondary_range.clone());
            return (primary_span, secondary_span);
        }

        // Else check the next position
        if has_definite_secondary_axis_position {
            primary_idx += 1;
        } else if secondary_out_of_bounds {
            primary_idx += 1;
            secondary_idx = 0;
        } else {
            secondary_idx += 1;
        }
    }
}

/// Record a grid item once the definite placement has been determined
fn record_grid_placement(
    grid: &mut CssGrid,
    node: Node,
    primary_axis: RowColumn,
    primary_span: Line<i16>,
    secondary_span: Line<i16>,
    placement_type: CellOccupancyState,
) {
    // Mark area of grid as occupied
    grid.cell_occupancy_matrix.mark_area_as(primary_axis, primary_span, secondary_span, placement_type);

    // Create grid item
    let (row_span, col_span) = primary_axis.into_row_column(primary_span, secondary_span);
    grid.items.push(GridItem {
        node,
        min_content_contribution: None,
        max_content_contribution: None,
        row: row_span,
        column: col_span,
    });
}
