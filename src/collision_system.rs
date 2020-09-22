use crate::entity::EntityManager;
use crate::input::Input;
use crate::map::Map;
use crate::rect::Rect;
use nalgebra::Vector2;
use uuid::Uuid;

enum CollisionDirection {
    Left,
    Right,
    Top,
    Bottom,
}

impl CollisionDirection {
    fn from_collision(rect: Rect<f32>, speed: Vector2<f32>) -> Self {
        if speed.x > 0.0 {
            CollisionDirection::Right
        } else if speed.x < 0.0 {
            CollisionDirection::Left
        } else if speed.y < 0.0 {
            CollisionDirection::Top
        } else {
            CollisionDirection::Bottom
        }
    }
}

pub struct CollisionSystem {}

impl CollisionSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, entity_manager: &mut EntityManager, map: &Map) {
        for uuid in entity_manager.get_entities() {
            self.update_entity(entity_manager, uuid, map);
        }
    }

    fn update_entity(&self, entity_manager: &mut EntityManager, uuid: Uuid, map: &Map) {
        let mut dx = 0.0;
        let mut dy = 0.0;
        if let Some(movement) = entity_manager.get_movement_mut(uuid) {
            dx = movement.dx();
            dy = movement.dy();
        }
        let mut hitx = false;
        let mut hity = false;
        let mut hit_from_top = false;
        if let Some(object) = entity_manager.get_object_mut(uuid) {
            object.move_by(dx, 0.0);
            if dx != 0.0 {
                for cell in map.collidable_tiles(&object.rect) {
                    if let Some(overlap) = object.overlap(&cell.object) {
                        hitx = true;
                        if dx > 0.0 {
                            object.move_by(-overlap.width, 0.0);
                        } else {
                            object.move_by(overlap.width, 0.0);
                        }
                    }
                }
            }

            object.move_by(0.0, dy);
            if dy != 0.0 {
                for cell in map.collidable_tiles(&object.rect) {
                    if let Some(overlap) = object.overlap(&cell.object) {
                        hity = true;
                        if dy > 0.0 {
                            hit_from_top = true;
                            object.move_by(0.0, -overlap.height);
                        } else {
                            object.move_by(0.0, overlap.height);
                        }
                    }
                }
            }
        }

        if (hitx || hity) && dy > 0.0 {
            if let Some(movement) = entity_manager.get_movement_mut(uuid) {
                movement.reset_speed_y();
            }
        }

        if hit_from_top {
            match entity_manager.get_input_mut(uuid) {
                Some(Input::Player(player_input)) => player_input.reset_jump(),
                _ => {}
            }
        }
    }
}
