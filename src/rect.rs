use coffee::graphics::Rectangle;
use std::fmt::Debug;

use nalgebra::Point2;
use serde::{Deserialize, Serialize};
use std::cmp::PartialOrd;
use std::ops::{Add, Sub};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T>
where
    T: 'static + Sub<Output = T> + Add<Output = T> + PartialOrd + From<i16> + Copy + Debug,
{
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn size(mut self, width: T, height: T) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn from_point(p: Point2<T>) -> Self {
        Self {
            x: p.x,
            y: p.y,
            width: T::from(0),
            height: T::from(0),
        }
    }

    pub fn point(&self) -> Point2<T> {
        Point2::new(self.x, self.y)
    }

    pub fn bottom_right(&self) -> Point2<T> {
        Point2::new(self.x + self.width, self.y + self.height)
    }

    pub fn has_overlap(&self, other: &Self) -> bool {
        // When one rect is fully left of the other.
        if self.x + self.width < other.x || other.x + other.width < self.x {
            return false;
        }

        // When one rect is fully above the other.
        if self.y + self.height < other.y || other.y + other.height < self.y {
            return false;
        }

        return true;
    }
}

impl Rect<f32> {
    #[cfg(allow_unused)]
    pub fn contains(&self, point: Point2<f32>) -> bool {
        self.x <= point.x
            && point.x <= self.x + self.width
            && self.y <= point.y
            && point.y <= self.y + self.height
    }
}

impl<T> From<Rectangle<T>> for Rect<T> {
    fn from(original: Rectangle<T>) -> Self {
        Self {
            x: original.x,
            y: original.y,
            width: original.width,
            height: original.height,
        }
    }
}

impl Default for Rect<f32> {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

impl Rect<f32> {
    pub fn overlap(&self, other: &Self) -> Option<Self> {
        if !self.has_overlap(other) {
            return None;
        }

        let x = f32::max(self.x, other.x);
        let y = f32::max(self.y, other.y);

        let bottom_right = self.bottom_right();
        let other_bottom_right = other.bottom_right();
        Some(Rect::new(
            x,
            y,
            f32::min(bottom_right.x, other_bottom_right.x) - x,
            f32::min(bottom_right.y, other_bottom_right.y) - y,
        ))
    }
}
