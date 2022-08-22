//!
//! Grid line coordinates are those used in grid-row/grid-column in the CSS grid spec
//!
//! In origin-zero coordinates:
//!   - The line at left hand (or top) edge of the explicit grid is line 0
//!   - The next line to the right (or down) is 1, and so on
//!   - The next line to the left (or up) is -1, and so on

use super::TrackCounts;

/// Convert from grid line coordinates to our custom origin-zero coordinates
pub(crate) fn into_origin_zero_coordinates(grid_line: i16, explicit_track_count: u16) -> i16 {
    let explicit_line_count = explicit_track_count + 1;
    if grid_line > 0 {
        grid_line - 1
    } else if grid_line < 0 {
        grid_line + explicit_line_count as i16
    } else {
        panic!("Grid line of zero is invalid");
    }
}

pub(crate) fn into_grid_coordinates(origin_zero_line: i16, explicit_track_count: u16) -> i16 {
    let explicit_line_count = explicit_track_count + 1;
    if origin_zero_line >= 0 {
        origin_zero_line + 1
    } else {
        -(origin_zero_line + explicit_line_count as i16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CssGridCoords;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct OriginZeroCoords;

trait GridLineCoords: Clone + Copy {}
impl GridLineCoords for CssGridCoords {}
impl GridLineCoords for OriginZeroCoords {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct GridLine<Coords: GridLineCoords = CssGridCoords>(i16, std::marker::PhantomData<Coords>);
