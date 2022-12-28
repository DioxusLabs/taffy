//! Taffy uses two coordinate systems to refer to grid lines (the gaps/gutters between rows/columns):
use super::super::types::TrackCounts;
use core::num::NonZeroI16;
use core::ops::{Add, AddAssign, Sub};

/// Represents a grid line position in "CSS Grid Line" coordinates
///
/// "CSS Grid Line" coordinates are those used in grid-row/grid-column in the CSS grid spec:
///   - The line at left hand (or top) edge of the explicit grid is line 1
///     (and counts up from there)
///   - The line at the right hand (or bottom) edge of the explicit grid in -1
///     (and counts down from there)
///   - 0 is not a valid index
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct GridLine(pub NonZeroI16);

impl From<NonZeroI16> for GridLine {
    fn from(value: NonZeroI16) -> Self {
        Self(value)
    }
}
impl TryFrom<i16> for GridLine {
    type Error = ();
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        NonZeroI16::new(value).map(GridLine).ok_or(())
    }
}

impl GridLine {
    /// Returns the underlying i16
    pub fn as_i16(self) -> i16 {
        self.0.get()
    }

    /// Convert into OriginZero coordinates using the specified explicit track count
    pub(crate) fn into_origin_zero_line(self, explicit_track_count: u16) -> OriginZeroLine {
        let grid_line = self.as_i16();

        // Note: grid_line cannot be zero because it comes from a NonZeroI16
        let oz_line = if grid_line > 0 {
            grid_line - 1
        } else {
            let explicit_line_count = explicit_track_count + 1;
            grid_line + explicit_line_count as i16
        };

        OriginZeroLine(oz_line)
    }
}

/// Represents a grid line position in "OriginZero" coordinates
///
/// "OriginZero" coordinates are a normalized form:
///   - The line at left hand (or top) edge of the explicit grid is line 0
///   - The next line to the right (or down) is 1, and so on
///   - The next line to the left (or up) is -1, and so on
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct OriginZeroLine(pub i16);

// Add and Sub with Self
impl Add<OriginZeroLine> for OriginZeroLine {
    type Output = Self;
    fn add(self, rhs: OriginZeroLine) -> Self::Output {
        OriginZeroLine(self.0 + rhs.0)
    }
}
impl Sub<OriginZeroLine> for OriginZeroLine {
    type Output = Self;
    fn sub(self, rhs: OriginZeroLine) -> Self::Output {
        OriginZeroLine(self.0 - rhs.0)
    }
}

// Add and Sub with u16
impl Add<u16> for OriginZeroLine {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        OriginZeroLine(self.0 + rhs as i16)
    }
}
impl AddAssign<u16> for OriginZeroLine {
    fn add_assign(&mut self, rhs: u16) {
        self.0 += rhs as i16;
    }
}
impl Sub<u16> for OriginZeroLine {
    type Output = Self;
    fn sub(self, rhs: u16) -> Self::Output {
        OriginZeroLine(self.0 - rhs as i16)
    }
}

impl OriginZeroLine {
    /// Converts a grid line in OriginZero coordinates into the index of that same grid line in the GridTrackVec.
    pub(crate) fn into_track_vec_index(self, track_counts: TrackCounts) -> usize {
        assert!(
            self.0 >= -(track_counts.negative_implicit as i16),
            "OriginZero grid line cannot be less than the number of negative grid lines"
        );
        assert!(
            self.0 <= (track_counts.explicit + track_counts.positive_implicit) as i16,
            "OriginZero grid line cannot be more than the number of positive grid lines"
        );
        2 * ((self.0 + track_counts.negative_implicit as i16) as usize)
    }
}

/// A trait for the different coordinates used to define grid lines.
pub trait GridCoordinate: Copy {}
impl GridCoordinate for GridLine {}
impl GridCoordinate for OriginZeroLine {}
