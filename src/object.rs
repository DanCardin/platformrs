use crate::rect::Rect;
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

pub struct Movement {
    speed: Vector2<f32>,
    forces: Vec<Vector2<f32>>,
    instantaneous_forces: Vec<Vector2<f32>>,
    max_speed: (Option<f32>, Option<f32>),
    dirty: bool,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            speed: Vector2::new(0.0, 0.0),
            forces: Vec::new(),
            instantaneous_forces: Vec::new(),
            max_speed: (None, None),
            dirty: true,
        }
    }

    pub fn with_max_speed(mut self, max_speed: (Option<f32>, Option<f32>)) -> Self {
        self.max_speed = max_speed;
        self
    }

    pub fn with_force(mut self, force: Vector2<f32>) -> Self {
        self.forces.push(force);
        self
    }

    pub fn update(&mut self) {
        if !self.dirty {
            return;
        }

        let force_total: Vector2<f32> = self.forces.iter().sum();
        let instantaneous_force_total: Vector2<f32> = self.instantaneous_forces.iter().sum();
        self.speed += instantaneous_force_total;
        self.speed += force_total;

        if let Some(x) = self.max_speed.0 {
            self.speed.x = self.speed.x.signum() * (f32::min(self.speed.x.abs(), x));
        }

        if let Some(y) = self.max_speed.1 {
            self.speed.y = self.speed.y.signum() * (f32::min(self.speed.y.abs(), y));
        }

        self.instantaneous_forces.clear();

        let drag = -1.0 * self.speed.x.signum() * 0.4;
        self.speed.x = if drag.abs() <= self.speed.x.abs() {
            self.speed.x + drag
        } else {
            0.0
        };

        // let drag = -1.0 * self.speed.y.signum() * 0.1;
        // self.speed.y = if drag.abs() <= self.speed.y.abs() {
        //     self.speed.y + drag
        // } else {
        //     0.0
        // };
    }

    pub fn add_force(&mut self, force: Vector2<f32>) {
        self.forces.push(force);
        self.dirty = true;
    }

    pub fn add_instantaneous_force(&mut self, force: Vector2<f32>) {
        self.instantaneous_forces.push(force);
        self.dirty = true;
    }

    pub fn reset_speed(&mut self) {
        self.speed = Vector2::new(0.0, 0.0);
    }

    pub fn dx(&mut self) -> f32 {
        self.update();
        self.speed.x
    }

    pub fn dy(&mut self) -> f32 {
        self.update();
        self.speed.y
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    pub rect: Rect<f32>,
    pub visible: bool,
    is_solid: bool,
}

impl Object {
    #[cfg(allow_unused)]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            visible: true,
            is_solid: true,
        }
    }

    pub fn with_size(width: f32, height: f32) -> Self {
        Self {
            rect: Rect::default().size(width, height),
            visible: true,
            is_solid: true,
        }
    }

    pub fn collision(mut self, is_solid: bool) -> Self {
        self.is_solid = is_solid;
        self
    }

    pub fn at(mut self, x: f32, y: f32) -> Self {
        self.rect.x = x;
        self.rect.y = y;
        self
    }

    #[cfg(allow_unused)]
    pub fn hide(mut self) -> Self {
        self.visible = false;
        self
    }

    pub fn move_by(&mut self, x: f32, y: f32) {
        self.rect.x += x;
        self.rect.y += y;
    }

    pub fn overlap(&self, other: &Self) -> Option<Rect<f32>> {
        if !self.is_solid || !other.is_solid {
            return None;
        }
        self.rect.overlap(&other.rect)
    }
}

impl Default for Object {
    fn default() -> Self {
        Self {
            rect: Rect::default(),
            visible: true,
            is_solid: true,
        }
    }
}
