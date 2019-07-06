use crate::rect::Rect;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object<'a> {
    pub rect: Rect<f32>,
    pub asset_name: Option<Cow<'a, str>>,
    pub visible: bool,
    pub collides: bool,
}

impl<'a> Object<'a> {
    #[cfg(allow_unused)]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            asset_name: None,
            visible: true,
            collides: true,
        }
    }

    pub fn with_size(width: f32, height: f32) -> Self {
        Self {
            rect: Rect::default().size(width, height),
            asset_name: None,
            visible: true,
            collides: true,
        }
    }

    pub fn collision(mut self, collides: bool) -> Self {
        self.collides = collides;
        self
    }

    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.rect.x = x;
        self.rect.y = y;
        self
    }

    pub fn with_asset<S>(mut self, asset_name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.asset_name = Some(asset_name.into());
        self
    }

    #[cfg(allow_unused)]
    pub fn hide(mut self) -> Self {
        self.visible = false;
        self
    }

    pub fn overlap(&self, other: &Self) -> Option<Rect<f32>> {
        if !self.collides || !other.collides {
            return None;
        }
        self.rect.overlap(&other.rect)
    }

    pub fn collide(&mut self, other: &Self) {
        let overlap = self.overlap(other);
    }
}

impl<'a> Default for Object<'a> {
    fn default() -> Self {
        Self {
            rect: Rect::default(),
            asset_name: None,
            visible: true,
            collides: true,
        }
    }
}
