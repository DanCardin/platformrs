// use crate::rect::Rect;
// use nalgebra::Vector2;
// use serde::{Deserialize, Serialize};
use crate::input::Input;
use crate::map::Map;
use crate::object::{Movement, Object};
use std::borrow::Cow;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Entity {
    pub id: Uuid,
}

impl Entity {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

pub struct EntityManager<'a> {
    entities: HashMap<Uuid, Entity>,
    names: HashMap<Cow<'a, str>, Uuid>,
    objects: HashMap<Uuid, Object>,
    assets: HashMap<Uuid, Cow<'a, str>>,
    movements: HashMap<Uuid, Movement>,
    inputs: HashMap<Uuid, Input>,
}

impl<'a> EntityManager<'a> {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            names: HashMap::new(),
            objects: HashMap::new(),
            assets: HashMap::new(),
            movements: HashMap::new(),
            inputs: HashMap::new(),
        }
    }

    pub fn add(&mut self, entity_builder: EntityBuilder<'a>) {
        let entity = entity_builder.entity;
        let id = entity.id;

        self.entities.insert(id, entity);

        if let Some(name) = entity_builder.name {
            self.names.insert(name, id);
        }

        if let Some(object) = entity_builder.object {
            self.objects.insert(id, object);
        }

        if let Some(asset_name) = entity_builder.asset_name {
            self.assets.insert(id, asset_name);
        }

        if let Some(movement) = entity_builder.movement {
            self.movements.insert(id, movement);
        }

        if let Some(input) = entity_builder.input {
            self.inputs.insert(id, input);
        }
    }

    pub fn by_name<S>(&self, name: S) -> Uuid
    where
        S: Into<Cow<'a, str>>,
    {
        self.names.get(&name.into()).unwrap_or(&Uuid::nil()).clone()
    }

    pub fn get_entities(&self) -> Vec<Uuid> {
        self.objects.keys().map(|uuid| *uuid).collect()
    }

    pub fn get_object(&self, uuid: Uuid) -> Option<&Object> {
        self.objects.get(&uuid)
    }

    pub fn get_object_mut(&mut self, uuid: Uuid) -> Option<&mut Object> {
        self.objects.get_mut(&uuid)
    }

    pub fn get_objects(&self) -> Vec<(&Uuid, &Object)> {
        self.objects.iter().collect()
    }

    pub fn get_asset(&mut self, uuid: Uuid) -> Option<&Cow<'a, str>> {
        self.assets.get(&uuid)
    }

    pub fn get_assets(&self) -> Vec<(Uuid, Cow<'a, str>)> {
        self.assets
            .iter()
            .map(|(uuid, name)| (uuid.clone(), name.clone()))
            .collect()
    }

    pub fn get_movements_mut(&mut self) -> Vec<(&Uuid, &mut Movement)> {
        self.movements.iter_mut().collect()
    }

    pub fn get_movement_mut(&mut self, uuid: Uuid) -> Option<&mut Movement> {
        self.movements.get_mut(&uuid)
    }

    pub fn get_inputs(&self) -> Vec<(&Uuid, &Input)> {
        self.inputs.iter().collect()
    }

    pub fn get_inputs_mut(&mut self) -> Vec<(&Uuid, &mut Input)> {
        self.inputs.iter_mut().collect()
    }

    pub fn get_input(&self, uuid: Uuid) -> &Input {
        self.inputs.get(&uuid).unwrap_or(&Input::None)
    }

    pub fn update(&mut self, uuid: Uuid, map: &Map) {
        let mut dx = 0.0;
        let mut dy = 0.0;
        if let Some(movement) = self.movements.get_mut(&uuid) {
            dx = movement.dx();
            dy = movement.dy();
        }

        let mut hitx = false;
        let mut hity = false;
        if let Some(object) = self.get_object_mut(uuid) {
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
                            object.move_by(0.0, -overlap.height);
                        } else {
                            object.move_by(0.0, overlap.height);
                        }
                    }
                }
            }
        }

        if hitx || hity {
            if let Some(movement) = self.movements.get_mut(&uuid) {
                movement.reset_speed();
            }
        }
    }
}

pub struct EntityBuilder<'a> {
    entity: Entity,
    name: Option<Cow<'a, str>>,
    asset_name: Option<Cow<'a, str>>,
    object: Option<Object>,
    movement: Option<Movement>,
    input: Option<Input>,
}

impl<'a> EntityBuilder<'a> {
    pub fn new() -> Self {
        Self {
            entity: Entity::new(),
            name: None,
            asset_name: None,
            object: None,
            movement: None,
            input: None,
        }
    }

    pub fn with_name<S>(mut self, name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.name = Some(name.into());
        self
    }

    pub fn with_asset<S>(mut self, asset_name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.asset_name = Some(asset_name.into());
        self
    }

    pub fn with_object(mut self, object: Object) -> Self {
        self.object = Some(object);
        self
    }

    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.movement = Some(movement);
        self
    }

    pub fn with_input(mut self, input: Input) -> Self {
        self.input = Some(input);
        self
    }
}
