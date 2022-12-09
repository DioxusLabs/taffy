//! Axis enums representing CSS Grid axis
use crate::geometry::Size;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AbsoluteAxis {
    Horizontal,
    Vertical,
}

impl AbsoluteAxis {
    #[inline]
    pub const fn other_axis(&self) -> Self {
        match *self {
            AbsoluteAxis::Horizontal => AbsoluteAxis::Vertical,
            AbsoluteAxis::Vertical => AbsoluteAxis::Horizontal,
        }
    }

    #[inline]
    pub fn into_column_row<T>(&self, primary: T, secondary: T) -> (T, T) {
        match *self {
            AbsoluteAxis::Horizontal => (primary, secondary),
            AbsoluteAxis::Vertical => (secondary, primary),
        }
    }

    #[inline]
    pub fn into_primary_secondary<T>(&self, row: T, column: T) -> (T, T) {
        match *self {
            AbsoluteAxis::Horizontal => (row, column),
            AbsoluteAxis::Vertical => (column, row),
        }
    }
}

impl<T> Size<T> {
    #[inline(always)]
    /// Get either the width or height depending on the AbsoluteAxis passed in
    pub fn get_abs(self, axis: AbsoluteAxis) -> T {
        match axis {
            AbsoluteAxis::Horizontal => self.width,
            AbsoluteAxis::Vertical => self.height,
        }
    }

    #[inline(always)]
    /// Get either the width or height depending on the AbsoluteAxis passed in
    pub fn get_abs_other(self, axis: AbsoluteAxis) -> T {
        match axis {
            AbsoluteAxis::Horizontal => self.height,
            AbsoluteAxis::Vertical => self.width,
        }
    }
}

/// The abstract axis in CSS Grid
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GridAxis {
    /// The axis in the inline dimension, i.e. the horizontal axis in horizontal writing modes and the vertical axis in vertical writing modes.
    Inline,
    /// The axis in the block dimension, i.e. the vertical axis in horizontal writing modes and the horizontal axis in vertical writing modes.
    Block,
}

impl GridAxis {
    pub fn other(&self) -> GridAxis {
        match *self {
            GridAxis::Inline => GridAxis::Block,
            GridAxis::Block => GridAxis::Inline,
        }
    }
}
