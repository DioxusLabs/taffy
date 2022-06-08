//! Geometric primitives useful for layout

use core::ops::Add;

use crate::number::Number;
use crate::style::{Dimension, FlexDirection};

/// The spatial extent of an axis-aligned UI rectangle
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Rect<T> {
    /// The x-coordinate of the left edge
    pub start: T,
    /// The x-coordinate of the right edge
    pub end: T,
    /// The y-coordinate of the top edge
    pub top: T,
    /// the y-coordinate of the bottom edge
    pub bottom: T,
}

impl<T> Rect<T> {
    /// Applies the function `f` to all four sides of the [`Rect`]
    ///
    /// This is used to transform a `Rect<T>` into a `Rect<R>`.
    pub(crate) fn map<R, F>(self, f: F) -> Rect<R>
    where
        F: Fn(T) -> R,
    {
        Rect { start: f(self.start), end: f(self.end), top: f(self.top), bottom: f(self.bottom) }
    }

    /// Applies the function `f` to all four sides of the rect
    ///
    /// When applied to the left and right sides, the width is used
    /// as the second parameter of `f`.
    /// When applied to the top or bottom sides, the height is used instead.
    pub(crate) fn zip_size<R, F, U>(self, size: Size<U>, f: F) -> Rect<R>
    where
        F: Fn(T, U) -> R,
        U: Copy,
    {
        Rect {
            start: f(self.start, size.width),
            end: f(self.end, size.width),
            top: f(self.top, size.height),
            bottom: f(self.bottom, size.height),
        }
    }
}

impl<T> Rect<T>
where
    T: Add<Output = T> + Copy + Clone,
{
    /// The sum of the coordinates of the left and right side of the rectangle
    ///
    /// **NOTE:** this is *not* the width of the rectangle.
    pub(crate) fn horizontal(&self) -> T {
        self.start + self.end
    }

    /// The sum of the coordinates of the bottom and top side of the rectangle
    ///
    /// **NOTE:** this is *not* the height of the rectangle.
    pub(crate) fn vertical(&self) -> T {
        self.top + self.bottom
    }

    /// The main layout-axis sum of the rectangle
    ///
    /// This is always perpendicular to [`Rect::cross`].
    ///
    /// If the [`FlexDirection`] is [`FlexDirection::Row`] or [`FlexDirection::RowReverse`], this is [`Rect::horizontal`].
    /// Otherwise, this is [`Rect::vertical`].
    pub(crate) fn main(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.horizontal()
        } else {
            self.vertical()
        }
    }

    /// The cross layout-axis sum of the rectangle
    ///
    /// This is always perpendicular to [`Rect::main`].
    ///
    /// If the [`FlexDirection`] is [`FlexDirection::Row`] or [`FlexDirection::RowReverse`], this is [`Rect::vertical`].
    /// Otherwise, this is [`Rect::horizontal`].
    pub(crate) fn cross(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.vertical()
        } else {
            self.horizontal()
        }
    }
}

impl<T> Rect<T>
where
    T: Copy + Clone,
{
    /// The lowest coordinate of the rectangle, from the perspective of the main layout axis
    pub(crate) fn main_start(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.start
        } else {
            self.top
        }
    }

    /// The highest coordinate of the rectangle, from the perspective of the main layout axis
    pub(crate) fn main_end(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.end
        } else {
            self.bottom
        }
    }

    /// The lowest coordinate of the rectangle, from the perspective of the cross layout axis
    pub(crate) fn cross_start(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.top
        } else {
            self.start
        }
    }

    /// The highest coordinate of the rectangle, from the perspective of the cross layout axis
    pub(crate) fn cross_end(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.bottom
        } else {
            self.end
        }
    }
}

/// The width and height of a [`Rect`]
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Size<T> {
    /// The x extent of the rectangle
    pub width: T,
    /// The y extent of the rectangle
    pub height: T,
}

impl Size<()> {
    /// Generates a `Size<Number>` with undefined width and height
    pub fn undefined() -> Size<Number> {
        Size { width: Number::Undefined, height: Number::Undefined }
    }
}

impl<T> Size<T> {
    /// Applies the function `f` to both the width and height
    ///
    /// This is used to transform a `Rect<T>` into a `Rect<R>`.
    pub fn map<R, F>(self, f: F) -> Size<R>
    where
        F: Fn(T) -> R,
    {
        Size { width: f(self.width), height: f(self.height) }
    }

    /// Sets the extent of the main layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn set_main(&mut self, direction: FlexDirection, value: T) {
        if direction.is_row() {
            self.width = value
        } else {
            self.height = value
        }
    }

    /// Sets the extent of the cross layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn set_cross(&mut self, direction: FlexDirection, value: T) {
        if direction.is_row() {
            self.height = value
        } else {
            self.width = value
        }
    }

    /// Gets the extent of the main layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn main(self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.width
        } else {
            self.height
        }
    }

    /// Gets the extent of the cross layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn cross(self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.height
        } else {
            self.width
        }
    }
}

impl Size<f32> {
    /// A [`Size`] with zero width and height
    pub fn zero() -> Self {
        Self { width: 0.0, height: 0.0 }
    }
}

impl Size<Dimension> {
    pub(crate) fn resolve(&self, parent: Size<Number>) -> Size<Number> {
        Size { width: self.width.resolve(parent.width), height: self.height.resolve(parent.height) }
    }
}

/// A 2-dimensional coordinate.
///
/// When used in association with a [`Rect`], represents the bottom-left corner.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T> {
    /// The x-coordinate
    pub x: T,
    /// The y-coordinate
    pub y: T,
}

impl Point<f32> {
    /// A [`Point`] with values (0,0), representing the origin
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
