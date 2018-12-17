use std::ops::Add;

use crate::style;

#[derive(Debug, Copy, Clone)]
pub struct Rect<T> {
    pub start: T,
    pub end: T,
    pub top: T,
    pub bottom: T,
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
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.start + self.end,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.top + self.bottom,
        }
    }

    pub(crate) fn cross(&self, direction: style::FlexDirection) -> T {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.top + self.bottom,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.start + self.end,
        }
    }
}

impl<T> Rect<T>
where
    T: Copy + Clone,
{
    pub(crate) fn main_start(&self, direction: style::FlexDirection) -> T {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.start,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.top,
        }
    }

    pub(crate) fn main_end(&self, direction: style::FlexDirection) -> T {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.end,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.bottom,
        }
    }

    pub(crate) fn cross_start(&self, direction: style::FlexDirection) -> T {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.top,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.start,
        }
    }

    pub(crate) fn cross_end(&self, direction: style::FlexDirection) -> T {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.bottom,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.end,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub(crate) fn set_main(&mut self, direction: style::FlexDirection, value: T) {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.width = value,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.height = value,
        }
    }

    pub(crate) fn set_cross(&mut self, direction: style::FlexDirection, value: T) {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.height = value,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.width = value,
        }
    }

    pub(crate) fn main(self, direction: style::FlexDirection) -> T {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.width,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.height,
        }
    }

    pub(crate) fn cross(self, direction: style::FlexDirection) -> T {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.height,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.width,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}
