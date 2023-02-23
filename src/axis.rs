//! Axis enums representing CSS Grid axis
use crate::geometry::{Rect, Size};
use core::ops::Add;

/// The simple absolute horizontal and vertical axis
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AbsoluteAxis {
    /// The horizontal axis
    Horizontal,
    /// The vertical axis
    Vertical,
}

impl AbsoluteAxis {
    /// Returns the other variant of the enum
    #[inline]
    pub const fn other_axis(&self) -> Self {
        match *self {
            AbsoluteAxis::Horizontal => AbsoluteAxis::Vertical,
            AbsoluteAxis::Vertical => AbsoluteAxis::Horizontal,
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
}

impl<T: Add> Rect<T> {
    #[inline(always)]
    /// Get either the width or height depending on the AbsoluteAxis passed in
    pub fn grid_axis_sum(self, axis: AbsoluteAxis) -> <T as Add>::Output {
        match axis {
            AbsoluteAxis::Horizontal => self.left + self.right,
            AbsoluteAxis::Vertical => self.top + self.bottom,
        }
    }
}

/// The CSS abstract axis
/// <https://www.w3.org/TR/css-writing-modes-3/#abstract-axes>
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AbstractAxis {
    /// The axis in the inline dimension, i.e. the horizontal axis in horizontal writing modes and the vertical axis in vertical writing modes.
    Inline,
    /// The axis in the block dimension, i.e. the vertical axis in horizontal writing modes and the horizontal axis in vertical writing modes.
    Block,
}

impl AbstractAxis {
    /// Returns the other variant of the enum
    pub fn other(&self) -> AbstractAxis {
        match *self {
            AbstractAxis::Inline => AbstractAxis::Block,
            AbstractAxis::Block => AbstractAxis::Inline,
        }
    }
}

/// Container that holds an item in each absolute axis without specifying
/// what kind of item it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct InBothAbsAxis<T> {
    /// The item in the horizontal axis
    pub horizontal: T,
    /// The item in the vertical axis
    pub vertical: T,
}

impl<T: Copy> InBothAbsAxis<T> {
    #[cfg(feature = "grid")]
    /// Get the contained item based on the AbsoluteAxis passed
    pub fn get(&self, axis: AbsoluteAxis) -> T {
        match axis {
            AbsoluteAxis::Horizontal => self.horizontal,
            AbsoluteAxis::Vertical => self.vertical,
        }
    }
}
