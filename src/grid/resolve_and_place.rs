use super::types::{CssGrid, GridAxisTracks, GridItem, GridTrack, RowColumn};
use crate::geometry::Line;
use crate::node::Node;
use crate::style::{Dimension, GridAutoFlow, GridPlacement, Style, TrackSizingFunction};
use crate::sys::GridTrackVec;
use crate::tree::LayoutTree;
use grid::Grid;
use std::cmp::{max, min};
use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub(super) enum CellOccupancyState {
    #[default]
    /// Indicates that a grid cell is unoccupied
    Unoccupied,
    // Indicates that a grid cell is occupied by a definitely placed item
    DefinitelyPlaced,
    // Indicates that a grid cell is occupied by an item that is definitely placed in the primary axis
    PrimaryAxisPlaced,
    // Indicates that a grid cell is occupied by an item that is definitely placed in the secondary axis
    SeconaryAxisPlaced,
}

impl CellOccupancyState {
    pub fn is_occupied(&self) -> bool {
        match self {
            Self::Unoccupied => false,
            _ => true,
        }
    }
}

pub(crate) struct TrackCounts {
    pub negative_implicit: u16,
    pub explicit: u16,
    pub positive_implicit: u16,
}

impl TrackCounts {
    pub fn from_raw(negative_implicit: u16, explicit: u16, positive_implicit: u16) -> Self {
        Self { negative_implicit, explicit, positive_implicit }
    }
}

impl Default for TrackCounts {
    fn default() -> Self {
        Self { explicit: 0, negative_implicit: 0, positive_implicit: 0 }
    }
}

//  0  1  2  3  4  5  6  7  8  9
// [I, I, I, E, E, E, E, I, I, I]

// negative_implicit_track_count=3;
// explicit_track_count=4;
// positive_implicit_track_count=3

// -2 =>

impl TrackCounts {
    fn len(&self) -> usize {
        return (self.negative_implicit + self.explicit + self.positive_implicit) as usize;
    }

    fn line_index_to_proceeding_track_index(&self, index: i16) -> i16 {
        use std::cmp::Ordering;
        match index.cmp(&0) {
            Ordering::Equal => 0,
            Ordering::Less => self.negative_implicit as i16 + self.explicit as i16 + index,
            Ordering::Greater => self.negative_implicit as i16 + index,
        }
    }

    fn track_index_to_preceeding_line_index(&self, index: u16) -> i16 {
        if index < self.negative_implicit {
            -(self.negative_implicit as i16 + self.explicit as i16 + 1 - index as i16)
        } else {
            (index + 1 - self.negative_implicit) as i16
        }
    }

    pub fn convert_grid_line_range_to_track_index_range(&self, input: Line<i16>) -> Range<i16> {
        let start = self.line_index_to_proceeding_track_index(input.start);
        let end = self.line_index_to_proceeding_track_index(input.end);
        start..end
    }
}

pub(super) struct CellOccupancyMatrix {
    inner: Grid<CellOccupancyState>,
    rows: TrackCounts,
    columns: TrackCounts,
}
impl CellOccupancyMatrix {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self { inner: Grid::new(rows, columns), rows: TrackCounts::default(), columns: TrackCounts::default() }
    }

    pub fn with_track_counts(rows: TrackCounts, columns: TrackCounts) -> Self {
        Self { inner: Grid::new(rows.len(), columns.len()), rows, columns }
    }

    /// Expands the grid (potentially in all 4 directions) in order to ensure
    /// That
    fn ensure_area_is_in_range(&mut self, row_range: Range<i16>, col_range: Range<i16>) -> bool {
        let old_row_count = self.rows.negative_implicit + self.rows.explicit + self.rows.positive_implicit;
        let old_col_count = self.columns.negative_implicit + self.columns.explicit + self.columns.positive_implicit;

        // Calculate number of rows and columns missing to accomodate ranges (if any)
        let req_negative_rows = min(row_range.start, 0);
        let req_positive_rows =
            max(row_range.end + 1 - self.rows.explicit as i16 - self.rows.negative_implicit as i16, 0);
        let req_negative_cols = min(col_range.start, 0);
        let req_positive_cols =
            max(col_range.end - self.columns.explicit as i16 - self.columns.negative_implicit as i16, 0);

        // Return early (with true to indicated "already in range") if no new rows or columns are required
        if (req_negative_rows + req_positive_rows + req_negative_cols + req_positive_cols == 0) {
            return true;
        }

        let new_row_count = self.inner.rows() + (req_negative_rows + req_positive_rows) as usize;
        let new_col_count = self.inner.cols() + (req_negative_cols + req_positive_cols) as usize;

        let mut data = Vec::with_capacity(new_row_count * new_col_count);

        // Push new negative rows
        for _ in 0..(req_negative_rows as usize * new_col_count) {
            data.push(CellOccupancyState::Unoccupied);
        }

        // Push existing rows
        for row in 0..old_row_count {
            // Push new negative columns
            for _ in 0..req_negative_cols {
                data.push(CellOccupancyState::Unoccupied);
            }
            // Push existing columns
            for col in 0..old_col_count {
                data.push(*self.inner.get(row as usize, col as usize).unwrap());
            }
            // Push new positive columns
            for _ in 0..req_positive_cols {
                data.push(CellOccupancyState::Unoccupied);
            }
        }

        // Push new negative rows
        for _ in 0..(req_positive_rows as usize * new_col_count) {
            data.push(CellOccupancyState::Unoccupied);
        }

        // Update self with new data
        self.inner = Grid::from_vec(data, new_col_count);
        self.rows.negative_implicit += req_negative_rows as u16;
        self.rows.positive_implicit += req_positive_rows as u16;
        self.columns.negative_implicit += req_negative_cols as u16;
        self.columns.positive_implicit += req_positive_cols as u16;

        // Return false to indicate "not already in range"
        false
    }

    pub fn mark_area_as(
        &mut self,
        primary_axis: RowColumn,
        primary_span: Line<i16>,
        secondary_span: Line<i16>,
        value: CellOccupancyState,
    ) {
        let (row_span, column_span) = match primary_axis {
            RowColumn::Row => (primary_span, secondary_span),
            RowColumn::Column => (secondary_span, primary_span),
        };

        let mut row_range = self.rows.convert_grid_line_range_to_track_index_range(row_span);
        let mut col_range = self.columns.convert_grid_line_range_to_track_index_range(column_span);

        // Ensure resolve ranges fit within the allocated grid. And if they didn't already fit then re-resolve the ranges
        // once the grid has been expanded as the resolved indexes may have changed
        let was_in_range = self.ensure_area_is_in_range(row_range.clone(), col_range.clone());
        if !was_in_range {
            row_range = self.rows.convert_grid_line_range_to_track_index_range(row_span);
            col_range = self.columns.convert_grid_line_range_to_track_index_range(column_span);
        }

        for x in row_range {
            for y in col_range.clone() {
                *self.inner.get_mut(x as usize, y as usize).unwrap() = value;
            }
        }
    }

    pub fn area_is_unoccupied(
        &self,
        primary_axis: RowColumn,
        primary_span: Line<i16>,
        secondary_span: Line<i16>,
    ) -> bool {
        let (row_span, column_span) = match primary_axis {
            RowColumn::Row => (primary_span, secondary_span),
            RowColumn::Column => (secondary_span, primary_span),
        };

        let row_range = self.rows.convert_grid_line_range_to_track_index_range(row_span);
        let col_range = self.columns.convert_grid_line_range_to_track_index_range(column_span);

        // Search for occupied cells in the specified area. Out of bounds cells are considered unoccupied.
        for x in row_range {
            for y in col_range.clone() {
                match self.inner.get(x as usize, y as usize) {
                    None | Some(CellOccupancyState::Unoccupied) => continue,
                    _ => return false,
                }
            }
        }

        true
    }

    fn track_counts(&self, track_type: RowColumn) -> &TrackCounts {
        match track_type {
            RowColumn::Row => &self.rows,
            RowColumn::Column => &self.columns,
        }
    }

    pub fn next_of_type(
        &self,
        track_type: RowColumn,
        primary_track_index: i16,
        kind: CellOccupancyState,
        start_after: i16,
    ) -> Option<i16> {
        let track_counts = self.track_counts(track_type);
        let primary_track_computed_index = track_counts.line_index_to_proceeding_track_index(primary_track_index);

        let maybe_index = match track_type {
            RowColumn::Row => self
                .inner
                .iter_row(primary_track_computed_index as usize)
                .skip(start_after as usize)
                .position(|item| *item == kind),
            RowColumn::Column => self
                .inner
                .iter_col(primary_track_computed_index as usize)
                .skip(start_after as usize)
                .position(|item| *item == kind),
        };

        maybe_index.map(|idx| track_counts.track_index_to_preceeding_line_index(idx as u16))
    }

    pub fn last_of_type(
        &self,
        track_type: RowColumn,
        primary_track_index: i16,
        kind: CellOccupancyState,
    ) -> Option<i16> {
        let track_counts = self.track_counts(track_type);
        let primary_track_computed_index = track_counts.line_index_to_proceeding_track_index(primary_track_index);

        let maybe_index = match track_type {
            RowColumn::Row => {
                self.inner.iter_row(primary_track_computed_index as usize).rposition(|item| *item == kind)
            }
            RowColumn::Column => {
                self.inner.iter_col(primary_track_computed_index as usize).rposition(|item| *item == kind)
            }
        };

        maybe_index.map(|idx| track_counts.track_index_to_preceeding_line_index(idx as u16))
    }

    pub fn next_unoccupied(&self, track_type: RowColumn, index: i16, start_after: i16) -> Option<i16> {
        self.next_of_type(track_type, index, CellOccupancyState::Unoccupied, start_after)
    }
    pub fn first_unoccupied(&self, track_type: RowColumn, index: i16) -> Option<i16> {
        self.next_of_type(track_type, index, CellOccupancyState::Unoccupied, 0)
    }
}

/// 7.1. The Explicit Grid
/// Initialise the `rows` and `columns` fields of the `Grid` based on following style properties:
/// - `grid-template-rows`
/// - `grid-template-columns`
pub(super) fn resolve_explicit_grid_track(
    axis: &mut GridAxisTracks,
    track_template: &GridTrackVec<TrackSizingFunction>,
    gap: Dimension,
) {
    debug_assert!(
        axis.len() == axis.origin,
        "Number of populated tracks should be equal to origin when calling resolve_explicit_grid_track"
    );

    let mut track_count = 0;
    track_template.iter().enumerate().for_each(|(index, track_sizing_function): (usize, &TrackSizingFunction)| {
        // Generate gutter in between each track
        if index != 0 {
            axis.push(GridTrack::gutter(gap))
        }

        // Generate track
        axis.push(GridTrack::new(
            track_sizing_function.min_sizing_function(),
            track_sizing_function.max_sizing_function(),
        ));

        // Count track
        track_count += 1;
    });

    axis.explicit_track_count = track_count;
}

/// 8.5. Grid Item Placement Algorithm
/// Place items into the grid, generating new rows/column into the implicit grid as required
///
/// [Specification](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)
pub(super) fn place_grid_items(grid: &mut CssGrid, tree: &impl LayoutTree, node: Node) {
    let grid_auto_flow = tree.style(node).grid_auto_flow;
    let flow_direction = grid_auto_flow.flow_direction();

    // Create a vector of children's styles that we can iterate over multiple times
    let children = || tree.children(node).into_iter().copied();

    // 1. Place children with definite positions
    children()
        .map(|child_node| tree.style(child_node))
        .filter(|child_style| child_style.grid_row.is_definite() && child_style.grid_column.is_definite())
        .for_each(|child_style| place_definite_grid_item(grid, node, child_style));

    // 2. Place remaining children with definite primary axis positions
    children()
        .map(|child_node| tree.style(child_node))
        .filter(|child_style| {
            child_style.grid_placement(flow_direction).is_definite()
                && !child_style.grid_placement(flow_direction.opposite_axis()).is_definite()
        })
        .for_each(|child_style| place_definite_primary_axis_item(grid, node, child_style, grid_auto_flow));

    // 3. Determine the number of columns in the implicit grid
    // By the time we get to this point in the execution, this is actually already accounted for:
    //
    // 3.1 Start with the columns from the explicit grid
    //        => Handled by grid size estimate which is used to pre-size the GridOccupancyMatrix
    //
    // 3.2 Among all the items with a definite column position (explicitly positioned items, items positioned in the previous step,
    //     and items not yet positioned but with a definite column) add columns to the beginning and end of the implicit grid as necessary
    //     to accommodate those items.
    //        => Handled by ensure_area_is_in_range which expands the GridOccupancyMatrix as necessary
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
            grid_position = place_indefinitely_positioned_item(grid, node, child_style, grid_auto_flow, grid_position);
        });
}

/// 8.5. Grid Item Placement Algorithm
/// Place a single definitely placed item into the grid
pub(super) fn place_definite_grid_item(grid: &mut CssGrid, node: Node, style: &Style) {
    // Resolve spans to tracks
    let row_span = style.grid_row.resolve_definite_grid_tracks();
    let column_span = style.grid_column.resolve_definite_grid_tracks();

    record_grid_placement(grid, node, RowColumn::Row, row_span, column_span, CellOccupancyState::DefinitelyPlaced);
}

pub(super) fn place_definite_primary_axis_item(grid: &mut CssGrid, node: Node, style: &Style, auto_flow: GridAutoFlow) {
    let flow_direction = auto_flow.flow_direction();
    let primary_axis_placement = style.grid_placement(flow_direction).resolve_definite_grid_tracks();
    let starting_position = match auto_flow.is_dense() {
        true => 1,
        false => grid
            .cell_occupancy_matrix
            .last_of_type(flow_direction, primary_axis_placement.start as i16, CellOccupancyState::PrimaryAxisPlaced)
            .unwrap_or(1),
    };

    let mut position: i16 = starting_position;
    loop {
        let secondary_axis_placement =
            style.grid_placement(flow_direction.opposite_axis()).resolve_indefinite_grid_tracks(position);

        let does_fit = grid.cell_occupancy_matrix.area_is_unoccupied(
            flow_direction,
            primary_axis_placement,
            secondary_axis_placement,
        );

        if does_fit {
            record_grid_placement(
                grid,
                node,
                flow_direction,
                primary_axis_placement,
                secondary_axis_placement,
                CellOccupancyState::PrimaryAxisPlaced,
            );
            break;
        } else {
            position += 1;
        }
    }
}

pub(super) fn place_indefinitely_positioned_item(
    grid: &mut CssGrid,
    node: Node,
    style: &Style,
    auto_flow: GridAutoFlow,
    grid_position: (u16, u16),
) -> (u16, u16) {
    let flow_direction = auto_flow.flow_direction();

    grid_position
}

/// Record a grid item once the definite placement has been determined
pub(super) fn record_grid_placement(
    grid: &mut CssGrid,
    node: Node,
    primary_axis: RowColumn,
    row_span: Line<i16>,
    column_span: Line<i16>,
    placement_type: CellOccupancyState,
) {
    // Mark area of grid as occupied
    grid.cell_occupancy_matrix.mark_area_as(primary_axis, row_span, column_span, placement_type);

    // Create grid item
    grid.items.push(GridItem {
        node,
        min_content_contribution: None,
        max_content_contribution: None,
        row: row_span,
        column: column_span,
    });
}
