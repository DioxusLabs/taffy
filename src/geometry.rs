//! Geometric primitives useful for layout

use crate::{
    math::MaybeMath,
    style::{Dimension, FlexDirection},
};

/// An axis-aligned UI rectangle
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Rect<T> {
    /// This can represent either the x-coordinate of the starting edge,
    /// or the amount of padding on the starting side.
    ///
    /// The starting edge is the left edge when working with LTR text,
    /// and the right edge when working with RTL text.
    pub start: Option<T>,
    /// This can represent either the x-coordinate of the ending edge,
    /// or the amount of padding on the ending side.
    ///
    /// The ending edge is the right edge when working with LTR text,
    /// and the left edge when working with RTL text.
    pub end: Option<T>,
    /// This can represent either the y-coordinate of the top edge,
    /// or the amount of padding on the top side.
    pub top: Option<T>,
    /// This can represent either the y-coordinate of the bottom edge,
    /// or the amount of padding on the bottom side.
    pub bottom: Option<T>,
}

impl<T> Rect<T> {
    /// Applies the function `f` to all four sides of the [`Rect`]
    ///
    /// This is used to transform a `Rect<T>` into a `Rect<R>`.
    pub(crate) fn map<R, F>(self, f: F) -> Rect<R>
    where
        F: Fn(Option<T>) -> Option<R>,
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
        F: Fn(Option<T>, Option<U>) -> Option<R>,
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
    T: MaybeMath<Option<T>, Option<T>> + Copy + Clone,
{
    /// The sum of [`Rect.start`](Rect) and [`Rect.end`](Rect)
    ///
    /// This is typically used when computing total padding.
    ///
    /// **NOTE:** this is *not* the width of the rectangle.
    pub(crate) fn horizontal_axis_sum(&self) -> Option<T> {
        self.start.and_then(|start| start.maybe_add(self.end))
        // self.start.maybe_add(self.end)
    }

    /// The sum of [`Rect.top`](Rect) and [`Rect.bottom`](Rect)
    ///
    /// This is typically used when computing total padding.
    ///
    /// **NOTE:** this is *not* the height of the rectangle.
    pub(crate) fn vertical_axis_sum(&self) -> Option<T> {
        self.top.and_then(|top| top.maybe_add(self.bottom))
    }

    /// The sum of the two fields of the [`Rect`] representing the main axis.
    ///
    /// This is typically used when computing total padding.
    ///
    /// If the [`FlexDirection`] is [`FlexDirection::Row`] or [`FlexDirection::RowReverse`], this is [`Rect::horizontal`].
    /// Otherwise, this is [`Rect::vertical`].
    pub(crate) fn main_axis_sum(&self, direction: FlexDirection) -> Option<T> {
        if direction.is_row() {
            self.horizontal_axis_sum()
        } else {
            self.vertical_axis_sum()
        }
    }

    /// The sum of the two fields of the [`Rect`] representing the cross axis.
    ///
    /// If the [`FlexDirection`] is [`FlexDirection::Row`] or [`FlexDirection::RowReverse`], this is [`Rect::vertical`].
    /// Otherwise, this is [`Rect::horizontal`].
    pub(crate) fn cross_axis_sum(&self, direction: FlexDirection) -> Option<T> {
        if direction.is_row() {
            self.vertical_axis_sum()
        } else {
            self.horizontal_axis_sum()
        }
    }
}

impl<T> Rect<T>
where
    T: Copy + Clone,
{
    /// The `start` or `top` value of the [`Rect`], from the perspective of the main layout axis
    pub(crate) fn main_start(&self, direction: FlexDirection) -> Option<T> {
        if direction.is_row() {
            self.start
        } else {
            self.top
        }
    }

    /// The `end` or `bottom` value of the [`Rect`], from the perspective of the main layout axis
    pub(crate) fn main_end(&self, direction: FlexDirection) -> Option<T> {
        if direction.is_row() {
            self.end
        } else {
            self.bottom
        }
    }

    /// The `start` or `top` value of the [`Rect`], from the perspective of the cross layout axis
    pub(crate) fn cross_start(&self, direction: FlexDirection) -> Option<T> {
        if direction.is_row() {
            self.top
        } else {
            self.start
        }
    }

    /// The `end` or `bottom` value of the [`Rect`], from the perspective of the main layout axis
    pub(crate) fn cross_end(&self, direction: FlexDirection) -> Option<T> {
        if direction.is_row() {
            self.bottom
        } else {
            self.end
        }
    }
}

// 2 dimensional
/// The width and height of a [`Rect`]
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Size<T> {
    /// The x extent of the rectangle
    pub width: Option<T>,
    /// The y extent of the rectangle
    pub height: Option<T>,
}

impl Size<()> {
    /// Generates a `Size<f32>` with undefined width and height
    #[must_use]
    pub fn undefined() -> Size<f32> {
        Size { width: None, height: None }
    }
}

impl<T> Size<T> {
    /// Applies the function `f` to both the width and height
    ///
    /// This is used to transform a `Rect<T>` into a `Rect<R>`.
    pub fn map<R, F>(self, f: F) -> Size<R>
    where
        F: Fn(Option<T>) -> Option<R>,
    {
        Size { width: f(self.width), height: f(self.height) }
    }

    /// Sets the extent of the main layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn set_main(&mut self, direction: FlexDirection, value: Option<T>) {
        if direction.is_row() {
            self.width = value
        } else {
            self.height = value
        }
    }

    /// Sets the extent of the cross layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn set_cross(&mut self, direction: FlexDirection, value: Option<T>) {
        if direction.is_row() {
            self.height = value
        } else {
            self.width = value
        }
    }

    /// Gets the extent of the main layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn main(self, direction: FlexDirection) -> Option<T> {
        if direction.is_row() {
            self.width
        } else {
            self.height
        }
    }

    /// Gets the extent of the cross layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn cross(self, direction: FlexDirection) -> Option<T> {
        if direction.is_row() {
            self.height
        } else {
            self.width
        }
    }
}

impl Size<f32> {
    /// A [`Size<f32>`] with zero width and height
    #[must_use]
    pub fn zero() -> Self {
        Self { width: Some(0.0), height: Some(0.0) }
    }
}

impl Size<Dimension> {
    /// Converts any `parent`-relative values for size into an absolute size
    pub(crate) fn resolve(&self, parent: Size<f32>) -> Size<f32> {
        Size {
            width: if let Some(width) = self.width { width.resolve(parent.width) } else { None },
            height: if let Some(height) = self.height { height.resolve(parent.height) } else { None },
        }
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
    #[must_use]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
