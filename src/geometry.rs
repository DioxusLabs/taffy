use core::ops::Add;

use crate::number::Number;
use crate::style;

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
pub struct Rect<T> {
    pub start: T,
    pub end: T,
    pub top: T,
    pub bottom: T,
}

impl<T> Rect<T> {
    pub(crate) fn map<R, F>(self, f: F) -> Rect<R>
    where
        F: Fn(T) -> R,
    {
        Rect { start: f(self.start), end: f(self.end), top: f(self.top), bottom: f(self.bottom) }
    }
}

impl<T> Rect<T>
where
    T: Add<Output = T> + Copy + Clone,
{
    pub(crate) fn horizontal(&self) -> T {
        self.start + self.end
    }

    pub(crate) fn vertical(&self) -> T {
        self.top + self.bottom
    }

    pub(crate) fn main(&self, direction: style::FlexDirection) -> T {
        if direction.is_row() {
            self.horizontal()
        } else {
            self.vertical()
        }
    }

    pub(crate) fn cross(&self, direction: style::FlexDirection) -> T {
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
    pub(crate) fn main_start(&self, direction: style::FlexDirection) -> T {
        if direction.is_row() {
            self.start
        } else {
            self.top
        }
    }

    pub(crate) fn main_end(&self, direction: style::FlexDirection) -> T {
        if direction.is_row() {
            self.end
        } else {
            self.bottom
        }
    }

    pub(crate) fn cross_start(&self, direction: style::FlexDirection) -> T {
        if direction.is_row() {
            self.top
        } else {
            self.start
        }
    }

    pub(crate) fn cross_end(&self, direction: style::FlexDirection) -> T {
        if direction.is_row() {
            self.bottom
        } else {
            self.end
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(all(feature = "serde", feature = "serde_kebab_case"), serde(rename_all = "kebab-case"))]
#[cfg_attr(all(feature = "serde", feature = "serde_camel_case"), serde(rename_all = "camelCase"))]
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

    pub(crate) fn set_main(&mut self, direction: style::FlexDirection, value: T) {
        if direction.is_row() {
            self.width = value
        } else {
            self.height = value
        }
    }

    pub(crate) fn set_cross(&mut self, direction: style::FlexDirection, value: T) {
        if direction.is_row() {
            self.height = value
        } else {
            self.width = value
        }
    }

    pub(crate) fn main(self, direction: style::FlexDirection) -> T {
        if direction.is_row() {
            self.width
        } else {
            self.height
        }
    }

    pub(crate) fn cross(self, direction: style::FlexDirection) -> T {
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

impl Size<style::Dimension> {
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
