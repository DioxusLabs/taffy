use slotmap::secondary;

use super::super::types::{AbsoluteAxis, CssGrid, GridItem};
use super::coordinates::into_origin_zero_coordinates;
use super::{CellOccupancyMatrix, CellOccupancyState};
use crate::geometry::Line;
use crate::node::Node;
use crate::style::{GridAutoFlow, GridPlacement, Style};
use crate::tree::LayoutTree;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InBothAbsAxis<T> {
    inline: T,
    block: T,
}

impl<T: Copy> InBothAbsAxis<T> {
    // Bit of a hack to mix absolute axis with grid axis like this, but it'll do for now
    pub fn get(&self, axis: AbsoluteAxis) -> T {
        match axis {
            AbsoluteAxis::Horizontal => self.inline,
            AbsoluteAxis::Vertical => self.block,
        }
    }
}

/// 8.5. Grid Item Placement Algorithm
/// Place items into the grid, generating new rows/column into the implicit grid as required
///
/// [Specification](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)
pub(in crate::grid) fn place_grid_items(grid: &mut CssGrid, tree: &impl LayoutTree, node: Node) {
    let grid_auto_flow = tree.style(node).grid_auto_flow;
    let children_iter =
        || tree.children(node).into_iter().copied().map(|child_node| (child_node, tree.style(child_node)));
    place_grid_items_inner(&mut grid.cell_occupancy_matrix, &mut grid.items, children_iter, grid_auto_flow)
}

/// 8.5. Grid Item Placement Algorithm
/// Place items into the grid, generating new rows/column into the implicit grid as required
///
/// [Specification](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)
pub(in crate::grid) fn place_grid_items_inner<'a, ChildIter>(
    cell_occupancy_matrix: &mut CellOccupancyMatrix,
    items: &mut Vec<GridItem>,
    children_iter: impl Fn() -> ChildIter,
    grid_auto_flow: GridAutoFlow,
) where
    ChildIter: Iterator<Item = (Node, &'a Style)>,
{
    let primary_axis = grid_auto_flow.primary_axis();
    let secondary_axis = primary_axis.other_axis();

    let map_child_style_to_origin_zero_placement = {
        let explicit_col_count = cell_occupancy_matrix.track_counts(AbsoluteAxis::Horizontal).explicit;
        let explicit_row_count = cell_occupancy_matrix.track_counts(AbsoluteAxis::Vertical).explicit;
        move |(node, style): (Node, &Style)| {
            let origin_zero_placement = InBothAbsAxis {
                inline: style.grid_column.map(|placement| {
                    placement.map_track(|track| into_origin_zero_coordinates(track, explicit_col_count))
                }),
                block: style.grid_row.map(|placement| {
                    placement.map_track(|track| into_origin_zero_coordinates(track, explicit_row_count))
                }),
            };
            (node, origin_zero_placement)
        }
    };

    // 1. Place children with definite positions
    children_iter()
        .filter(|(_, child_style)| child_style.grid_row.is_definite() && child_style.grid_column.is_definite())
        .map(map_child_style_to_origin_zero_placement)
        .for_each(|(child_node, child_placement)| {
            let (row_span, col_span) = place_definite_grid_item(child_placement, primary_axis);
            record_grid_placement(
                cell_occupancy_matrix,
                items,
                child_node,
                primary_axis,
                row_span,
                col_span,
                CellOccupancyState::DefinitelyPlaced,
            );
        });

    // 2. Place remaining children with definite primary axis positions
    children_iter()
        .filter(|(_, child_style)| {
            child_style.grid_placement(primary_axis).is_definite()
                && !child_style.grid_placement(secondary_axis).is_definite()
        })
        .map(map_child_style_to_origin_zero_placement)
        .for_each(|(child_node, child_placement)| {
            let (primary_span, secondary_span) =
                place_definite_primary_axis_item(&*cell_occupancy_matrix, child_placement, grid_auto_flow);

            record_grid_placement(
                cell_occupancy_matrix,
                items,
                child_node,
                primary_axis,
                primary_span,
                secondary_span,
                CellOccupancyState::AutoPlaced,
            );
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
    let mut idx = 0;
    children_iter()
        .filter(|(_, child_style)| !child_style.grid_row.is_definite() && !child_style.grid_column.is_definite())
        .map(map_child_style_to_origin_zero_placement)
        .for_each(|(child_node, child_placement)| {
            idx += 1;
            #[cfg(test)]
            println!("Item {idx}\n==============");

            // Compute placement
            let (primary_span, secondary_span) = place_indefinitely_positioned_item(
                &*cell_occupancy_matrix,
                child_placement,
                grid_auto_flow,
                grid_position,
            );

            // Record item
            record_grid_placement(
                cell_occupancy_matrix,
                items,
                child_node,
                primary_axis,
                primary_span,
                secondary_span,
                CellOccupancyState::AutoPlaced,
            );

            // If using the "dense" placement algorithm then reset the grid position back to (0, 0) ready for the next item
            // Otherwise set it to the position of the current item so that the next item it placed after it.
            grid_position = match grid_auto_flow.is_dense() {
                true => (0, 0),
                false => (primary_span.end as u16, secondary_span.start as u16),
            }
        });
}

/// 8.5. Grid Item Placement Algorithm
/// Place a single definitely placed item into the grid
fn place_definite_grid_item(
    placement: InBothAbsAxis<Line<GridPlacement>>,
    primary_axis: AbsoluteAxis,
) -> (Line<i16>, Line<i16>) {
    // Resolve spans to tracks
    let primary_span = placement.get(primary_axis).resolve_definite_grid_tracks();
    let secondary_span = placement.get(primary_axis.other_axis()).resolve_definite_grid_tracks();

    (primary_span, secondary_span)
}

fn place_definite_primary_axis_item(
    cell_occupancy_matrix: &CellOccupancyMatrix,
    placement: InBothAbsAxis<Line<GridPlacement>>,
    auto_flow: GridAutoFlow,
) -> (Line<i16>, Line<i16>) {
    let primary_axis = auto_flow.primary_axis();
    let secondary_axis = primary_axis.other_axis();

    let primary_axis_placement = placement.get(primary_axis).resolve_definite_grid_tracks();
    let starting_position = match auto_flow.is_dense() {
        true => 1,
        false => cell_occupancy_matrix
            .last_of_type(primary_axis, primary_axis_placement.start as i16, CellOccupancyState::AutoPlaced)
            .unwrap_or(1),
    };

    let mut position: i16 = starting_position;
    loop {
        let secondary_axis_placement = placement.get(secondary_axis).resolve_indefinite_grid_tracks(position);

        let does_fit = cell_occupancy_matrix.line_area_is_unoccupied(
            primary_axis,
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
    placement: InBothAbsAxis<Line<GridPlacement>>,
    auto_flow: GridAutoFlow,
    grid_position: (u16, u16),
) -> (Line<i16>, Line<i16>) {
    let primary_axis = auto_flow.primary_axis();

    let primary_placement_style = placement.get(primary_axis);
    let secondary_placement_style = placement.get(primary_axis.other_axis());

    let primary_span = primary_placement_style.indefinite_span();
    let secondary_span = secondary_placement_style.indefinite_span();
    let has_definite_secondary_axis_position = secondary_placement_style.is_definite();
    let primary_axis_length = cell_occupancy_matrix.track_counts(primary_axis).len() as i16;

    let track_area_is_unoccupied = |primary_range, secondary_range| {
        cell_occupancy_matrix.track_area_is_unoccupied(primary_axis, primary_range, secondary_range)
    };
    let tracks_to_lines = |range| cell_occupancy_matrix.tracks_to_lines(primary_axis, range);

    let (mut primary_idx, mut secondary_idx) = grid_position;

    if has_definite_secondary_axis_position {
        let definite_secondary_placement = secondary_placement_style.resolve_definite_grid_tracks();
        secondary_idx =
            cell_occupancy_matrix.lines_to_tracks(primary_axis.other_axis(), definite_secondary_placement).start as u16;
    }
    #[cfg(test)]
    dbg!(has_definite_secondary_axis_position);
    #[cfg(test)]
    dbg!(primary_axis, primary_axis_length);
    #[cfg(test)]
    dbg!(primary_idx);
    #[cfg(test)]
    dbg!(secondary_idx);
    // let mut loop_idx = 0;

    loop {
        let primary_range = (primary_idx as i16)..((primary_idx + primary_span) as i16);
        let secondary_range = (secondary_idx as i16)..((secondary_idx + secondary_span) as i16);

        #[cfg(test)]
        dbg!(&primary_range);
        #[cfg(test)]
        dbg!(&secondary_range);

        // Check if item fits in it's current position, and if so place it there
        let primary_out_of_bounds =
            !has_definite_secondary_axis_position && primary_range.end >= primary_axis_length + 1;
        let place_here =
            !primary_out_of_bounds && track_area_is_unoccupied(primary_range.clone(), secondary_range.clone());
        if place_here {
            let primary_span = tracks_to_lines(primary_range.clone());
            let secondary_span = tracks_to_lines(secondary_range.clone());
            return (primary_span, secondary_span);
        }

        // Else check the next position
        if primary_out_of_bounds && !has_definite_secondary_axis_position {
            secondary_idx += 1;
            primary_idx = 0;
        } else {
            primary_idx += 1;
        }

        // loop_idx += 1;
        // if loop_idx > 4 {
        //     let primary_span = tracks_to_lines(primary_range.clone());
        //     let secondary_span = tracks_to_lines(secondary_range.clone());
        //     return (primary_span, secondary_span);
        // }
    }
}

/// Record a grid item once the definite placement has been determined
fn record_grid_placement(
    cell_occupancy_matrix: &mut CellOccupancyMatrix,
    items: &mut Vec<GridItem>,
    node: Node,
    primary_axis: AbsoluteAxis,
    primary_span: Line<i16>,
    secondary_span: Line<i16>,
    placement_type: CellOccupancyState,
) {
    #[cfg(test)]
    println!("BEFORE placement:");
    #[cfg(test)]
    println!("{:?}", cell_occupancy_matrix);
    // #[cfg(test)] println!("{:?}", primary_span);
    // #[cfg(test)] println!("{:?}", secondary_span);

    // Mark area of grid as occupied
    cell_occupancy_matrix.mark_area_as(primary_axis, primary_span, secondary_span, placement_type);

    #[cfg(test)]
    println!("AFTER placement:");
    #[cfg(test)]
    println!("{:?}", cell_occupancy_matrix);

    // Create grid item
    let (row_span, col_span) = primary_axis.into_row_column(primary_span, secondary_span);
    items.push(GridItem {
        node,
        min_content_contribution: None,
        max_content_contribution: None,
        row: row_span,
        column: col_span,
    });

    #[cfg(test)]
    println!("");
    #[cfg(test)]
    println!("");
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {

    mod test_placement_algorithm {
        use crate::grid::resolve_and_place::TrackCounts;
        use crate::grid::test_helpers::*;
        use crate::grid::CellOccupancyMatrix;
        use crate::prelude::*;
        use crate::style::{GridAutoFlow, GridPlacement::*};
        use slotmap::SlotMap;

        use super::super::place_grid_items_inner;

        #[test]
        fn explicit_grid_auto_placement() {
            let mut cell_occupancy_matrix = CellOccupancyMatrix::with_track_counts(
                TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 0 },
                TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 0 },
            );
            let mut items = Vec::new();
            let children = {
                let mut sm = SlotMap::new();
                let auto_child = (Auto, Auto, Auto, Auto).into_grid_child();
                vec![
                    // node, style, expected_placement
                    (sm.insert(()), auto_child.clone(), (0, 1, 0, 1)),
                    (sm.insert(()), auto_child.clone(), (0, 1, 1, 2)),
                    (sm.insert(()), auto_child.clone(), (1, 2, 0, 1)),
                    (sm.insert(()), auto_child.clone(), (1, 2, 1, 2)),
                    (sm.insert(()), auto_child.clone(), (2, 3, 0, 1)),
                    (sm.insert(()), auto_child.clone(), (2, 3, 1, 2)),
                    (sm.insert(()), auto_child.clone(), (3, 4, 0, 1)),
                    (sm.insert(()), auto_child.clone(), (3, 4, 1, 2)),
                ]
            };
            let children_iter = || children.iter().map(|(node, style, _)| (*node, style));

            place_grid_items_inner(&mut cell_occupancy_matrix, &mut items, children_iter, GridAutoFlow::Row);

            assert_eq!(
                *cell_occupancy_matrix.track_counts(crate::grid::AbsoluteAxis::Vertical),
                TrackCounts::from_raw(0, 2, 2),
                "row track counts"
            );
            assert_eq!(
                *cell_occupancy_matrix.track_counts(crate::grid::AbsoluteAxis::Horizontal),
                TrackCounts::from_raw(0, 2, 0),
                "column track counts"
            );
            for (idx, ((node, _style, expected_placement), item)) in children.iter().zip(items.iter()).enumerate() {
                assert_eq!(item.node, *node);
                assert_eq!(
                    (item.column.start, item.column.end, item.row.start, item.row.end),
                    *expected_placement,
                    "Item {idx} (0-indexed)"
                );
            }
        }
    }
}
