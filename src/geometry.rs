use core::ops::Add;

use crate::number::Number;
use crate::style::{Dimension, FlexDirection};

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Rect<T> {
    pub main_start: T,
    pub main_end: T,
    pub cross_start: T,
    pub cross_end: T,
}

impl<T> Rect<T> {
    pub(crate) fn map<R, F>(self, f: F) -> Rect<R>
    where
        F: Fn(T) -> R,
    {
        Rect {
            main_start: f(self.main_start),
            main_end: f(self.main_end),
            cross_start: f(self.cross_start),
            cross_end: f(self.cross_end),
        }
    }
    pub(crate) fn zip_size<R, F, U>(self, size: Size<U>, f: F) -> Rect<R>
    where
        F: Fn(T, U) -> R,
        U: Copy,
    {
        Rect {
            main_start: f(self.main_start, size.width),
            main_end: f(self.main_end, size.width),
            cross_start: f(self.cross_start, size.height),
            cross_end: f(self.cross_end, size.height),
        }
    }
}

impl<T> Rect<T>
where
    T: Add<Output = T> + Copy + Clone,
{
    pub(crate) fn horizontal(&self) -> T {
        self.main_start + self.main_end
    }

    pub(crate) fn vertical(&self) -> T {
        self.cross_start + self.cross_end
    }

    pub(crate) fn main(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.horizontal()
        } else {
            self.vertical()
        }
    }

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
    pub(crate) fn main_start(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.main_start
        } else {
            self.cross_start
        }
    }

    pub(crate) fn main_end(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.main_end
        } else {
            self.cross_end
        }
    }

    pub(crate) fn cross_start(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.cross_start
        } else {
            self.main_start
        }
    }

    pub(crate) fn cross_end(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.cross_end
        } else {
            self.main_end
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl Size<()> {
    pub fn undefined() -> Size<Number> {
        Size { width: Number::Undefined, height: Number::Undefined }
    }
}

impl<T> Size<T> {
    pub fn map<R, F>(self, f: F) -> Size<R>
    where
        F: Fn(T) -> R,
    {
        Size { width: f(self.width), height: f(self.height) }
    }

    pub(crate) fn set_main(&mut self, direction: FlexDirection, value: T) {
        if direction.is_row() {
            self.width = value
        } else {
            self.height = value
        }
    }

    pub(crate) fn set_cross(&mut self, direction: FlexDirection, value: T) {
        if direction.is_row() {
            self.height = value
        } else {
            self.width = value
        }
    }

    pub(crate) fn main(self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.width
        } else {
            self.height
        }
    }

    pub(crate) fn cross(self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.height
        } else {
            self.width
        }
    }
}

impl Size<f32> {
    pub fn zero() -> Self {
        Self { width: 0.0, height: 0.0 }
    }
}

impl Size<Dimension> {
    pub(crate) fn resolve(&self, parent: Size<Number>) -> Size<Number> {
        Size { width: self.width.resolve(parent.width), height: self.height.resolve(parent.height) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<f32> {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
