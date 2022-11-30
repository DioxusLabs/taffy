use super::super::types::AbsoluteAxis;
use crate::geometry::Line;
use crate::sys::Vec;
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

#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub fn from_explicit(count: u16) -> Self {
        Self { negative_implicit: 0, explicit: count, positive_implicit: 0 }
    }

    pub fn len(&self) -> usize {
        return (self.negative_implicit + self.explicit + self.positive_implicit) as usize;
    }

    pub fn oz_line_to_next_track(&self, index: i16) -> i16 {
        index + (self.negative_implicit as i16)
    }

    pub fn oz_line_to_grid_track_vec_index(&self, index: i16) -> u16 {
        assert!(
            index >= -(self.negative_implicit as i16),
            "origin-zero grid line cannot be less than the number of negative grid lines"
        );
        assert!(
            index <= (self.explicit + self.positive_implicit) as i16,
            "origin-zero grid line cannot be more than the number of positive grid lines"
        );
        2 * ((index + self.negative_implicit as i16) as u16)
    }

    pub fn track_to_prev_oz_line(&self, index: u16) -> i16 {
        (index as i16) - (self.negative_implicit as i16)
    }

    pub fn oz_line_range_to_track_range(&self, input: Line<i16>) -> Range<i16> {
        let start = self.oz_line_to_next_track(input.start);
        let end = self.oz_line_to_next_track(input.end); // Don't subtract 1 as output range is exclusive
        start..end
    }

    pub fn track_range_to_oz_line_range(&self, input: Range<i16>) -> Line<i16> {
        let start = self.track_to_prev_oz_line(input.start as u16);
        let end = self.track_to_prev_oz_line(input.end as u16); // Don't add 1 as input range is exclusive
        Line { start, end }
    }
}

pub(crate) struct CellOccupancyMatrix {
    inner: Grid<CellOccupancyState>,
    columns: TrackCounts,
    rows: TrackCounts,
}

impl core::fmt::Debug for CellOccupancyMatrix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "Rows: neg_implicit={} explicit={} pos_implicit={}",
            self.rows.negative_implicit, self.rows.explicit, self.rows.positive_implicit
        )?;
        writeln!(
            f,
            "Cols: neg_implicit={} explicit={} pos_implicit={}",
            self.columns.negative_implicit, self.columns.explicit, self.columns.positive_implicit
        )?;
        writeln!(f, "State:")?;

        for row_idx in 0..self.inner.rows() {
            for cell in self.inner.iter_row(row_idx) {
                let letter = match *cell {
                    CellOccupancyState::Unoccupied => '_',
                    CellOccupancyState::DefinitelyPlaced => 'D',
                    CellOccupancyState::AutoPlaced => 'A',
                };
                write!(f, "{letter}")?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl CellOccupancyMatrix {
    pub fn with_track_counts(columns: TrackCounts, rows: TrackCounts) -> Self {
        Self { inner: Grid::new(rows.len(), columns.len()), rows, columns }
    }

    pub fn with_explicit_track_counts(columns: u16, rows: u16) -> Self {
        Self {
            inner: Grid::new(rows as usize, columns as usize),
            columns: TrackCounts::from_explicit(columns),
            rows: TrackCounts::from_explicit(rows),
        }
    }

    pub fn is_area_in_range(
        &mut self,
        primary_axis: AbsoluteAxis,
        primary_range: Range<i16>,
        secondary_range: Range<i16>,
    ) -> bool {
        if primary_range.start < 0 || primary_range.end > self.track_counts(primary_axis).len() as i16 {
            return false;
        }
        if secondary_range.start < 0 || secondary_range.end > self.track_counts(primary_axis.other_axis()).len() as i16
        {
            return false;
        }
        true
    }

    /// Expands the grid (potentially in all 4 directions) in order to ensure that the specified range fits within the allocated space
    fn expand_to_fit_range(&mut self, row_range: Range<i16>, col_range: Range<i16>) {
        // Calculate number of rows and columns missing to accomodate ranges (if any)
        let req_negative_rows = min(row_range.start, 0);
        let req_positive_rows = max(row_range.end - self.rows.explicit as i16 - self.rows.positive_implicit as i16, 0);
        let req_negative_cols = min(col_range.start, 0);
        let req_positive_cols =
            max(col_range.end - self.columns.explicit as i16 - self.columns.positive_implicit as i16, 0);

        let old_row_count = self.rows.len();
        let old_col_count = self.columns.len();
        let new_row_count = old_row_count + (req_negative_rows + req_positive_rows) as usize;
        let new_col_count = old_col_count + (req_negative_cols + req_positive_cols) as usize;

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
        primary_axis: AbsoluteAxis,
        primary_span: Line<i16>,
        secondary_span: Line<i16>,
        value: CellOccupancyState,
    ) {
        let (row_span, column_span) = match primary_axis {
            AbsoluteAxis::Horizontal => (secondary_span, primary_span),
            AbsoluteAxis::Vertical => (primary_span, secondary_span),
        };

        let mut col_range = self.columns.oz_line_range_to_track_range(column_span);
        let mut row_range = self.rows.oz_line_range_to_track_range(row_span);

        // Check that if the resolved ranges fit within the allocated grid. And if they don't then expand the grid to fit
        // and then re-resolve the ranges once the grid has been expanded as the resolved indexes may have changed
        let is_in_range = self.is_area_in_range(AbsoluteAxis::Horizontal, col_range.clone(), row_range.clone());
        if !is_in_range {
            self.expand_to_fit_range(row_range.clone(), col_range.clone());
            col_range = self.columns.oz_line_range_to_track_range(column_span);
            row_range = self.rows.oz_line_range_to_track_range(row_span);
        }

        for x in row_range {
            for y in col_range.clone() {
                *self.inner.get_mut(x as usize, y as usize).unwrap() = value;
            }
        }
    }

    pub fn lines_to_tracks(&self, axis: AbsoluteAxis, span: Line<i16>) -> Range<i16> {
        self.track_counts(axis).oz_line_range_to_track_range(span)
    }

    pub fn tracks_to_lines(&self, axis: AbsoluteAxis, span: Range<i16>) -> Line<i16> {
        self.track_counts(axis).track_range_to_oz_line_range(span)
    }

    pub fn line_area_is_unoccupied(
        &self,
        primary_axis: AbsoluteAxis,
        primary_span: Line<i16>,
        secondary_span: Line<i16>,
    ) -> bool {
        let primary_range = self.track_counts(primary_axis).oz_line_range_to_track_range(primary_span);
        let secondary_range = self.track_counts(primary_axis.other_axis()).oz_line_range_to_track_range(secondary_span);
        self.track_area_is_unoccupied(primary_axis, primary_range, secondary_range)
    }

    pub fn track_area_is_unoccupied(
        &self,
        primary_axis: AbsoluteAxis,
        primary_range: Range<i16>,
        secondary_range: Range<i16>,
    ) -> bool {
        let (row_range, col_range) = match primary_axis {
            AbsoluteAxis::Horizontal => (secondary_range, primary_range),
            AbsoluteAxis::Vertical => (primary_range, secondary_range),
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

    pub fn track_counts(&self, track_type: AbsoluteAxis) -> &TrackCounts {
        match track_type {
            AbsoluteAxis::Horizontal => &self.columns,
            AbsoluteAxis::Vertical => &self.rows,
        }
    }

    pub fn get(
        &self,
        track_type: AbsoluteAxis,
        primary_index: u16,
        secondary_index: u16,
    ) -> Option<&CellOccupancyState> {
        match track_type {
            AbsoluteAxis::Horizontal => self.inner.get(secondary_index as usize, primary_index as usize),
            AbsoluteAxis::Vertical => self.inner.get(primary_index as usize, secondary_index as usize),
        }
    }

    pub fn next_of_type(
        &self,
        track_type: AbsoluteAxis,
        primary_track_index: i16,
        kind: CellOccupancyState,
        start_after: i16,
    ) -> Option<i16> {
        let track_counts = self.track_counts(track_type);
        let primary_track_computed_index = track_counts.oz_line_to_next_track(primary_track_index);

        let maybe_index = match track_type {
            AbsoluteAxis::Horizontal => self
                .inner
                .iter_row(primary_track_computed_index as usize)
                .skip(start_after as usize)
                .position(|item| *item == kind),
            AbsoluteAxis::Vertical => self
                .inner
                .iter_col(primary_track_computed_index as usize)
                .skip(start_after as usize)
                .position(|item| *item == kind),
        };

        maybe_index.map(|idx| track_counts.track_to_prev_oz_line(idx as u16))
    }

    pub fn last_of_type(&self, track_type: AbsoluteAxis, track_index: i16, kind: CellOccupancyState) -> Option<i16> {
        let track_counts = self.track_counts(track_type.other_axis());
        let track_computed_index = track_counts.oz_line_to_next_track(track_index);

        let maybe_index = match track_type {
            AbsoluteAxis::Horizontal => {
                self.inner.iter_row(track_computed_index as usize).rposition(|item| *item == kind)
            }
            AbsoluteAxis::Vertical => {
                self.inner.iter_col(track_computed_index as usize).rposition(|item| *item == kind)
            }
        };

        maybe_index.map(|idx| track_counts.track_to_prev_oz_line(idx as u16))
    }

    pub fn next_unoccupied(&self, track_type: AbsoluteAxis, index: i16, start_after: i16) -> Option<i16> {
        self.next_of_type(track_type, index, CellOccupancyState::Unoccupied, start_after)
    }
    pub fn first_unoccupied(&self, track_type: AbsoluteAxis, index: i16) -> Option<i16> {
        self.next_of_type(track_type, index, CellOccupancyState::Unoccupied, 0)
    }
}
