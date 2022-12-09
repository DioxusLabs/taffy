//! Taffy uses two coordinate systems to refer to grid lines (the gaps/gutters between rows/columns):
//!
//!   "CSS Grid Line" coordinates are those used in grid-row/grid-column in the CSS grid spec:
//!     - The line at left hand (or top) edge of the explicit grid is line 1
//!       (and counts up from there)
//!     - The line at the right hand (or bottom) edge of the explicit grid in -1
//!       (and counts down from there)
//!     - 0 is not a valid index
//!
//!   "OriginZero" coordinates are a normalized form:
//!     - The line at left hand (or top) edge of the explicit grid is line 0
//!     - The next line to the right (or down) is 1, and so on
//!     - The next line to the left (or up) is -1, and so on
//!
use core::cmp::Ordering;

/// Convert from CSS Grid Line coordinates to our custom OriginZero coordinates
pub(crate) fn css_grid_line_into_origin_zero_coords(grid_line: i16, explicit_track_count: u16) -> i16 {
    let explicit_line_count = explicit_track_count + 1;
    match grid_line.cmp(&0) {
        Ordering::Greater => grid_line - 1,
        Ordering::Less => grid_line + explicit_line_count as i16,
        Ordering::Equal => panic!("Grid line of zero is invalid"),
    }
}

#[allow(dead_code)]
/// Convert from OriginZero coordinates to CSS Grid Line coordinates
pub(crate) fn origin_zero_coords_into_css_grid_line(origin_zero_line: i16, explicit_track_count: u16) -> i16 {
    let explicit_line_count = explicit_track_count + 1;
    if origin_zero_line >= 0 {
        origin_zero_line + 1
    } else {
        -(origin_zero_line + explicit_line_count as i16)
    }
}
