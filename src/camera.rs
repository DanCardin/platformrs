use coffee::graphics::{Rectangle, Transformation, Vector};

pub struct Camera {
    area: Rectangle<i16>,
    margin: Rectangle<i16>,
    max_bounds: Option<Rectangle<i16>>,
    zoom: f32,
}

impl Camera {
    pub fn new(rect: Rectangle<i16>) -> Self {
        Self {
            area: rect,
            margin: Rectangle {
                x: 100,
                y: 100,
                width: 100,
                height: 100,
            },
            max_bounds: None,
            zoom: 1.0,
        }
    }

    pub fn with_bounds(mut self, rect: Rectangle<i16>) -> Self {
        self.max_bounds = Some(rect);
        self
    }

    pub fn get_transform(self: &mut Self, target: Option<&Rectangle<i16>>) -> Transformation {
        let mut x = self.area.x;
        let mut y = self.area.y;

        // Adjust the position based on the target, if it exists.
        if let Some(target) = target {
            let max_x_bound = target.x - self.margin.x;
            if x > max_x_bound {
                x = max_x_bound;
            }

            let min_x_bound = target.x + target.width + self.margin.width - self.area.width;
            if x < min_x_bound {
                x = min_x_bound;
            }

            let max_y_bound = target.y - self.margin.y;
            if y > max_y_bound {
                y = max_y_bound;
            }

            let min_y_bound = target.y + target.height + self.margin.height - self.area.height;
            if y < min_y_bound {
                y = min_y_bound;
            }
        }

        // Adjusts x and y relative to the maximum/minimum bounds.
        if let Some(max_bounds) = self.max_bounds {
            if x < max_bounds.x {
                x = max_bounds.x;
            }

            if x + self.area.width > max_bounds.width {
                x = max_bounds.width - self.area.width;
            }

            if (x < 0) && (x + self.area.width > max_bounds.width) {
                if let Some(target) = target {
                    x = target.x + target.width / 2 - self.area.width / 2;
                }
            }

            if y < max_bounds.y {
                y = max_bounds.y;
            }

            if y + self.area.height > max_bounds.height {
                y = max_bounds.height - self.area.height;
            }

            if (y < 0) && (y + self.area.height > max_bounds.height) {
                if let Some(target) = target {
                    y = target.y + target.height / 2 - self.area.height / 2;
                }
            }
        }

        self.area.x = x;
        self.area.y = y;

        Transformation::identity()
            * Transformation::scale(self.zoom)
            * Transformation::translate(Vector::new(-1.0 * x as f32, -1.0 * y as f32))
    }
}
