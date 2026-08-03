//! Contains CellOccupancyMatrix used to track occupied cells during grid placement
use super::TrackCounts;
use crate::compute::grid::OriginZeroLine;
use crate::geometry::AbsoluteAxis;
use crate::geometry::Line;
use crate::util::sys::{new_vec_with_capacity, Vec};
use core::cmp::{max, min};
use core::fmt::Debug;
use core::ops::Range;
use grid::Grid;

/// The occupancy state of a single grid cell
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub(crate) enum CellOccupancyState {
    #[default]
    /// Indicates that a grid cell is unoccupied
    Unoccupied,
    /// Indicates that a grid cell is occupied by a definitely placed item
    DefinitelyPlaced,
    /// Indicates that a grid cell is occupied by an item that was placed by the auto placement algorithm
    AutoPlaced,
}

/// The "hull" of the occupied cells within a single track of the `CellOccupancyMatrix`:
/// the range from the first occupied cell to one past the last occupied cell (`None` if the
/// track is entirely unoccupied). Cells at `start` and `end - 1` are always occupied, but
/// cells in between may or may not be.
type OccupiedHull = Option<Range<i16>>;

/// Extends `hull` to cover `range`
fn merge_hull(hull: &mut OccupiedHull, range: Range<i16>) {
    if range.start >= range.end {
        return;
    }
    *hull = match hull.take() {
        Some(existing) => Some(min(existing.start, range.start)..max(existing.end, range.end)),
        None => Some(range),
    };
}

/// A dynamically sized matrix (2d grid) which tracks the occupancy of each grid cell during auto-placement
/// It also keeps tabs on how many tracks there are and which tracks are implicit and which are explicit.
pub(crate) struct CellOccupancyMatrix {
    /// The grid of occupancy states
    inner: Grid<CellOccupancyState>,
    /// The counts of implicit and explicit columns
    columns: TrackCounts,
    /// The counts of implicit and explicit rows
    rows: TrackCounts,
    /// For each row: the hull of the occupied cells in that row (as column indices)
    row_hulls: Vec<OccupiedHull>,
    /// For each column: the hull of the occupied cells in that column (as row indices)
    column_hulls: Vec<OccupiedHull>,
}

/// Debug impl that represents the matrix in a compact 2d text format
impl Debug for CellOccupancyMatrix {
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
            writeln!(f)?;
        }

        Ok(())
    }
}

impl CellOccupancyMatrix {
    /// Create a CellOccupancyMatrix given a set of provisional track counts. The grid can expand as needed to fit more tracks,
    /// the provisional track counts represent a best effort attempt to avoid the extra allocations this requires.
    pub fn with_track_counts(columns: TrackCounts, rows: TrackCounts) -> Self {
        let mut row_hulls = new_vec_with_capacity(rows.len());
        row_hulls.resize(rows.len(), None);
        let mut column_hulls = new_vec_with_capacity(columns.len());
        column_hulls.resize(columns.len(), None);
        Self { inner: Grid::new(rows.len(), columns.len()), rows, columns, row_hulls, column_hulls }
    }

    /// Determines whether the specified area fits within the tracks currently represented by the matrix
    pub fn is_area_in_range(
        &self,
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
        // Calculate number of rows and columns missing to accommodate ranges (if any)
        let req_negative_rows = max(-row_range.start, 0);
        let req_positive_rows = max(row_range.end - self.rows.len() as i16, 0);
        let req_negative_cols = max(-col_range.start, 0);
        let req_positive_cols = max(col_range.end - self.columns.len() as i16, 0);

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
                data.push(*self.inner.get(row, col).unwrap());
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

        // Shift the per-track occupied hulls to account for the newly inserted negative tracks
        let mut row_hulls = new_vec_with_capacity(new_row_count);
        row_hulls.resize(new_row_count, None);
        for (index, hull) in self.row_hulls.drain(..).enumerate() {
            row_hulls[index + req_negative_rows as usize] =
                hull.map(|hull| (hull.start + req_negative_cols)..(hull.end + req_negative_cols));
        }
        self.row_hulls = row_hulls;
        let mut column_hulls = new_vec_with_capacity(new_col_count);
        column_hulls.resize(new_col_count, None);
        for (index, hull) in self.column_hulls.drain(..).enumerate() {
            column_hulls[index + req_negative_cols as usize] =
                hull.map(|hull| (hull.start + req_negative_rows)..(hull.end + req_negative_rows));
        }
        self.column_hulls = column_hulls;

        self.rows.negative_implicit += req_negative_rows as u16;
        self.rows.positive_implicit += req_positive_rows as u16;
        self.columns.negative_implicit += req_negative_cols as u16;
        self.columns.positive_implicit += req_positive_cols as u16;
    }

    /// Mark an area of the matrix as occupied, expanding the allocated space as necessary to accommodate the passed area.
    pub fn mark_area_as(
        &mut self,
        primary_axis: AbsoluteAxis,
        primary_span: Line<OriginZeroLine>,
        secondary_span: Line<OriginZeroLine>,
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

        for x in row_range.clone() {
            merge_hull(&mut self.row_hulls[x as usize], col_range.clone());
            for y in col_range.clone() {
                *self.inner.get_mut(x as usize, y as usize).unwrap() = value;
            }
        }
        for y in col_range {
            merge_hull(&mut self.column_hulls[y as usize], row_range.clone());
        }
    }

    /// Whether the cell at (`primary_index`, `secondary_index`) (indices into this matrix, with
    /// `primary_index` indexing along `primary_axis`) is occupied. Out of bounds cells are unoccupied.
    fn is_cell_occupied(&self, primary_axis: AbsoluteAxis, primary_index: i16, secondary_index: i16) -> bool {
        let (row, col) = match primary_axis {
            AbsoluteAxis::Horizontal => (secondary_index, primary_index),
            AbsoluteAxis::Vertical => (primary_index, secondary_index),
        };
        !matches!(self.inner.get(row as usize, col as usize), None | Some(CellOccupancyState::Unoccupied))
    }

    /// Checks the specified area for occupied cells. Returns `None` if the area is entirely
    /// unoccupied. Otherwise returns the index (into this matrix, along `primary_axis`) of an
    /// occupied cell within the area such that every search position between the current one and
    /// the position just past the returned cell would also collide with that cell. When searching
    /// forwards (`reversed == false`) this is the maximum such index; when searching backwards it
    /// is the minimum. This allows the auto-placement search cursor to jump past the collision
    /// rather than advancing one track at a time.
    ///
    /// Uses the per-track occupied hulls so that fully-vacant and fully-occupied tracks are
    /// handled in O(1) per track, only falling back to a dense cell scan when a track's hull
    /// straddles the edge of the area being checked.
    pub fn area_collision_extent(
        &self,
        primary_axis: AbsoluteAxis,
        primary_range: Range<i16>,
        secondary_range: Range<i16>,
        reversed: bool,
    ) -> Option<i16> {
        let hulls: &[OccupiedHull] = match primary_axis {
            AbsoluteAxis::Horizontal => &self.row_hulls,
            AbsoluteAxis::Vertical => &self.column_hulls,
        };

        // Out of bounds cells are considered unoccupied, so clamp the secondary range to the
        // tracks which actually exist
        let secondary_start = max(secondary_range.start, 0);
        let secondary_end = min(secondary_range.end, hulls.len() as i16);

        let mut extent: Option<i16> = None;
        for secondary_index in secondary_start..secondary_end {
            let Some(hull) = hulls[secondary_index as usize].clone() else { continue };
            let overlap_start = max(hull.start, primary_range.start);
            let overlap_end = min(hull.end, primary_range.end);
            if overlap_start >= overlap_end {
                continue;
            }
            let found = if !reversed {
                if hull.end <= primary_range.end {
                    // The last cell of the hull is always occupied and is within the area
                    Some(hull.end - 1)
                } else {
                    (overlap_start..overlap_end)
                        .rev()
                        .find(|&index| self.is_cell_occupied(primary_axis, index, secondary_index))
                }
            } else if hull.start >= primary_range.start {
                // The first cell of the hull is always occupied and is within the area
                Some(hull.start)
            } else {
                (overlap_start..overlap_end).find(|&index| self.is_cell_occupied(primary_axis, index, secondary_index))
            };
            if let Some(index) = found {
                extent = Some(match extent {
                    None => index,
                    Some(best) => {
                        if reversed {
                            min(best, index)
                        } else {
                            max(best, index)
                        }
                    }
                });
            }
        }
        extent
    }

    /// Like `area_collision_extent`, but takes bounding grid lines in OriginZero coordinates and
    /// returns the next search position (also in OriginZero coordinates, along `primary_axis`)
    /// that is not guaranteed to collide with the occupied cells found in the area.
    /// Returns `None` if the area is entirely unoccupied.
    pub fn line_area_collision_jump(
        &self,
        primary_axis: AbsoluteAxis,
        primary_span: Line<OriginZeroLine>,
        secondary_span: Line<OriginZeroLine>,
        reversed: bool,
    ) -> Option<OriginZeroLine> {
        let primary_counts = self.track_counts(primary_axis);
        let primary_range = primary_counts.oz_line_range_to_track_range(primary_span);
        let secondary_range = self.track_counts(primary_axis.other_axis()).oz_line_range_to_track_range(secondary_span);
        self.area_collision_extent(primary_axis, primary_range, secondary_range, reversed).map(|track_index| {
            let line = primary_counts.track_to_prev_oz_line(track_index as u16);
            if reversed {
                OriginZeroLine(line.0 - 1)
            } else {
                line + 1
            }
        })
    }

    /// Given a span of tracks in `axis` (in OriginZero coordinates), returns the next search
    /// position past all non-empty tracks within the span, or `None` if all tracks within the
    /// span are entirely unoccupied. Used to place items which span every track in the other
    /// axis (such items can only fit in a stripe of entirely unoccupied tracks).
    pub fn occupied_track_jump(
        &self,
        axis: AbsoluteAxis,
        span: Line<OriginZeroLine>,
        reversed: bool,
    ) -> Option<OriginZeroLine> {
        let counts = self.track_counts(axis);
        let hulls: &[OccupiedHull] = match axis {
            AbsoluteAxis::Horizontal => &self.column_hulls,
            AbsoluteAxis::Vertical => &self.row_hulls,
        };
        let range = counts.oz_line_range_to_track_range(span);
        let start = max(range.start, 0);
        let end = min(range.end, hulls.len() as i16);
        let found = if !reversed {
            (start..end).rev().find(|&index| hulls[index as usize].is_some())
        } else {
            (start..end).find(|&index| hulls[index as usize].is_some())
        };
        found.map(|track_index| {
            let line = counts.track_to_prev_oz_line(track_index as u16);
            if reversed {
                OriginZeroLine(line.0 - 1)
            } else {
                line + 1
            }
        })
    }

    /// Determines whether the specified row contains any items
    pub fn row_is_occupied(&self, row_index: usize) -> bool {
        self.row_hulls.get(row_index).is_some_and(|hull| hull.is_some())
    }

    /// Determines whether the specified column contains any items
    pub fn column_is_occupied(&self, column_index: usize) -> bool {
        self.column_hulls.get(column_index).is_some_and(|hull| hull.is_some())
    }

    /// Returns the track counts of this CellOccunpancyMatrix in the relevant axis
    pub fn track_counts(&self, track_type: AbsoluteAxis) -> &TrackCounts {
        match track_type {
            AbsoluteAxis::Horizontal => &self.columns,
            AbsoluteAxis::Vertical => &self.rows,
        }
    }

    /// Given an axis and a track index
    /// Search backwards from the end of the track and find the last grid cell matching the specified state (if any)
    /// Return the index of that cell or None.
    pub fn last_of_type(
        &self,
        track_type: AbsoluteAxis,
        start_at: OriginZeroLine,
        kind: CellOccupancyState,
    ) -> Option<OriginZeroLine> {
        let track_counts = self.track_counts(track_type.other_axis());
        let track_computed_index = track_counts.oz_line_to_next_track(start_at);

        let maybe_index = match track_type {
            AbsoluteAxis::Horizontal => {
                if track_computed_index < 0 || track_computed_index >= self.inner.rows() as i16 {
                    // Index out of bounds: no tracks to search
                    None
                } else {
                    self.inner.iter_row(track_computed_index as usize).rposition(|item| *item == kind)
                }
            }
            AbsoluteAxis::Vertical => {
                if track_computed_index < 0 || track_computed_index >= self.inner.cols() as i16 {
                    // Index out of bounds: no tracks to search
                    None
                } else {
                    self.inner.iter_col(track_computed_index as usize).rposition(|item| *item == kind)
                }
            }
        };

        maybe_index.map(|idx| track_counts.track_to_prev_oz_line(idx as u16))
    }

    /// Given an axis and a track index
    /// Search forwards from the start of the track and find the first grid cell matching the specified state (if any)
    /// Return the index of that cell or None.
    pub fn first_of_type(
        &self,
        track_type: AbsoluteAxis,
        start_at: OriginZeroLine,
        kind: CellOccupancyState,
    ) -> Option<OriginZeroLine> {
        let track_counts = self.track_counts(track_type.other_axis());
        let track_computed_index = track_counts.oz_line_to_next_track(start_at);

        let maybe_index = match track_type {
            AbsoluteAxis::Horizontal => {
                if track_computed_index < 0 || track_computed_index >= self.inner.rows() as i16 {
                    // Index out of bounds: no tracks to search
                    None
                } else {
                    self.inner.iter_row(track_computed_index as usize).position(|item| *item == kind)
                }
            }
            AbsoluteAxis::Vertical => {
                if track_computed_index < 0 || track_computed_index >= self.inner.cols() as i16 {
                    // Index out of bounds: no tracks to search
                    None
                } else {
                    self.inner.iter_col(track_computed_index as usize).position(|item| *item == kind)
                }
            }
        };

        maybe_index.map(|idx| track_counts.track_to_prev_oz_line(idx as u16))
    }
}
