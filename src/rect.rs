use nalgebra::Point2;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Rectangle<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl Rectangle<f32> {
    pub fn contains(&self, point: Point2<f32>) -> bool {
        self.x <= point.x
            && point.x <= self.x + self.width
            && self.y <= point.y
            && point.y <= self.y + self.height
    }
}
