use super::super::types::RowColumn;
use crate::geometry::Line;
use core::cmp::{max, min};
use core::ops::Range;
use grid::Grid;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub(crate) enum CellOccupancyState {
    #[default]
    /// Indicates that a grid cell is unoccupied
    Unoccupied,
    // Indicates that a grid cell is occupied by a definitely placed item
    DefinitelyPlaced,
    // Indicates that a grid cell is occupied by an item that was placed by the auto placement algorithm
    AutoPlaced,
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
    pub fn len(&self) -> usize {
        return (self.negative_implicit + self.explicit + self.positive_implicit) as usize;
    }

    fn line_index_to_proceeding_track_index(&self, index: i16) -> i16 {
        use core::cmp::Ordering;
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

    pub fn convert_track_index_range_to_line_range(&self, input: Range<i16>) -> Line<i16> {
        let start = self.track_index_to_preceeding_line_index(input.start as u16);
        let end = self.track_index_to_preceeding_line_index(input.end as u16);
        Line { start, end }
    }
}

pub(crate) struct CellOccupancyMatrix {
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

    pub fn is_area_in_range(
        &mut self,
        primary_axis: RowColumn,
        primary_range: Range<i16>,
        secondary_range: Range<i16>,
    ) -> bool {
        if primary_range.start < 0 || primary_range.end >= self.track_counts(primary_axis).len() as i16 {
            return false;
        }
        if secondary_range.start < 0
            || secondary_range.end >= self.track_counts(primary_axis.opposite_axis()).len() as i16
        {
            return false;
        }
        true
    }

    /// Expands the grid (potentially in all 4 directions) in order to ensure that the specified range fits within the allocated space
    fn expand_to_fit_range(&mut self, row_range: Range<i16>, col_range: Range<i16>) {
        let old_row_count = self.rows.negative_implicit + self.rows.explicit + self.rows.positive_implicit;
        let old_col_count = self.columns.negative_implicit + self.columns.explicit + self.columns.positive_implicit;

        // Calculate number of rows and columns missing to accomodate ranges (if any)
        let req_negative_rows = min(row_range.start, 0);
        let req_positive_rows =
            max(row_range.end + 1 - self.rows.explicit as i16 - self.rows.negative_implicit as i16, 0);
        let req_negative_cols = min(col_range.start, 0);
        let req_positive_cols =
            max(col_range.end - self.columns.explicit as i16 - self.columns.negative_implicit as i16, 0);

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
    }

    /// Mark an area of the matrix as occupied, expanding the allocated space as necessary to accomodate the passed area.
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

        // Check that if the resolved ranges fit within the allocated grid. And if they don't then expand the grid to fit
        // and then re-resolve the ranges once the grid has been expanded as the resolved indexes may have changed
        let is_in_range = self.is_area_in_range(RowColumn::Row, row_range.clone(), col_range.clone());
        if !is_in_range {
            self.expand_to_fit_range(row_range.clone(), col_range.clone());
            row_range = self.rows.convert_grid_line_range_to_track_index_range(row_span);
            col_range = self.columns.convert_grid_line_range_to_track_index_range(column_span);
        }

        for x in row_range {
            for y in col_range.clone() {
                *self.inner.get_mut(x as usize, y as usize).unwrap() = value;
            }
        }
    }

    pub fn lines_to_tracks(&self, axis: RowColumn, span: Line<i16>) -> Range<i16> {
        self.track_counts(axis).convert_grid_line_range_to_track_index_range(span)
    }

    pub fn tracks_to_lines(&self, axis: RowColumn, span: Range<i16>) -> Line<i16> {
        self.track_counts(axis).convert_track_index_range_to_line_range(span)
    }

    pub fn line_area_is_unoccupied(
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

        self.track_area_is_unoccupied(RowColumn::Row, row_range, col_range)
    }

    pub fn track_area_is_unoccupied(
        &self,
        primary_axis: RowColumn,
        primary_range: Range<i16>,
        secondary_range: Range<i16>,
    ) -> bool {
        let (row_range, col_range) = match primary_axis {
            RowColumn::Row => (primary_range, secondary_range),
            RowColumn::Column => (secondary_range, primary_range),
        };

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

    pub fn track_counts(&self, track_type: RowColumn) -> &TrackCounts {
        match track_type {
            RowColumn::Row => &self.rows,
            RowColumn::Column => &self.columns,
        }
    }

    pub fn get(&self, track_type: RowColumn, primary_index: u16, secondary_index: u16) -> Option<&CellOccupancyState> {
        match track_type {
            RowColumn::Row => self.inner.get(primary_index as usize, secondary_index as usize),
            RowColumn::Column => self.inner.get(secondary_index as usize, primary_index as usize),
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
