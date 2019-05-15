use std::ops::Add;

use number::Number;
use style::FlexDirection;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub origin: RectOrigin,
    pub size: RectSize,
}

impl Rect {
    pub const fn undefined() -> Self {
        Self {
            origin: RectOrigin::undefined(),
            size: RectSize::undefined(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RectOrigin {
    pub x: Number,
    pub y: Number,
}

impl RectOrigin {
    pub const fn undefined() -> Self {
        Self {
            x: Number::Undefined,
            y: Number::Undefined,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RectSize {
    pub width: Number,
    pub height: Number,
}

impl RectSize {
    pub const fn undefined() -> Self {
        Self {
            width: Number::Undefined,
            height: Number::Undefined,
        }
    }

    pub(crate) fn main(self, direction: FlexDirection) -> Number {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.width,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.height,
        }
    }

    pub(crate) fn cross(self, direction: FlexDirection) -> Number {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.width,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Offsets<T> {
    pub top: T,
    pub left: T,
    pub bottom: T,
    pub right: T,
}

impl<T> Offsets<T> {
    pub(crate) fn map<R, F: Fn(T) -> R>(self, f: F) -> Offsets<R> {
        Offsets { left: f(self.left), right: f(self.right), top: f(self.top), bottom: f(self.bottom) }
    }
}

impl<T> Offsets<T>
where
    T: Add<Output = T> + Copy + Clone,
{
    pub(crate) fn horizontal(&self) -> T {
        self.left + self.right
    }

    pub(crate) fn vertical(&self) -> T {
        self.top + self.bottom
    }

    pub(crate) fn main(&self, direction: FlexDirection) -> T {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.left + self.right,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.top + self.bottom,
        }
    }

    pub(crate) fn cross(&self, direction: FlexDirection) -> T {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.top + self.bottom,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.left + self.right,
        }
    }
}

impl<T> Offsets<T>
where
    T: Copy + Clone,
{
    pub(crate) fn main_start(&self, direction: FlexDirection) -> T {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.left,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.top,
        }
    }

    pub(crate) fn main_end(&self, direction: FlexDirection) -> T {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.right,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.bottom,
        }
    }

    pub(crate) fn cross_start(&self, direction: FlexDirection) -> T {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.top,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.left,
        }
    }

    pub(crate) fn cross_end(&self, direction: FlexDirection) -> T {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.bottom,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.right,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub(crate) fn map<R, F>(self, f: F) -> Size<R>
    where
        F: Fn(T) -> R,
    {
        Size { width: f(self.width), height: f(self.height) }
    }

    pub(crate) fn set_main(&mut self, direction: FlexDirection, value: T) {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.width = value,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.height = value,
        }
    }

    pub(crate) fn set_cross(&mut self, direction: FlexDirection, value: T) {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.height = value,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.width = value,
        }
    }

    pub(crate) fn main(self, direction: FlexDirection) -> T {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.width,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.height,
        }
    }

    pub(crate) fn cross(self, direction: FlexDirection) -> T {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.width,
        }
    }
}