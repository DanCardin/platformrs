use coffee::graphics::{Batch, Color, Frame, Image, Point, Rectangle, Sprite, Window};
use coffee::input::KeyboardAndMouse;
use coffee::load::loading_screen::ProgressBar;
use coffee::load::Join;
use coffee::load::Task;
use coffee::Debug;
use nalgebra::Vector2;
use std::borrow::Cow;
use std::collections::HashMap;
use uuid::Uuid;

use crate::assets::Assets;
use crate::camera::Camera;
use crate::config::Config;
use crate::entity::{EntityBuilder, EntityManager};
use crate::input::{Input, PlayerInput};
use crate::map::Map;
use crate::object::{Movement, Object};
use crate::rect::Rect;
use coffee::Game;

pub struct Platformrs<'a> {
    assets: Assets<'a>,
    map: Map<'a>,
    config: Config,
    camera: Camera,
    batch: Batch,
    debug_sheet: Image,
    entity_manager: EntityManager<'a>,
}

impl<'a> Game for Platformrs<'a> {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ProgressBar;

    fn load(_window: &Window) -> Task<Platformrs<'a>> {
        (
            Task::stage(
                "Loading assets...",
                Task::using_gpu(|_gpu| Assets::load().map_err(|e| coffee::Error::from(e))),
            ),
            Task::stage("Loading map data...", Task::new(|| Map::load())),
            Task::stage(
                "Loading spritesheet",
                Task::using_gpu(|mut gpu| Image::new(&mut gpu, "assets/tiles.png")),
            ),
            Task::stage(
                "Loading image",
                Task::using_gpu(|mut gpu| Image::new(&mut gpu, "assets/debug.png")),
            ),
        )
            .join()
            .map(|(assets, map, spritesheet, debug_sheet)| {
                let config = Config::new();
                let camera = Camera::new(
                    Rect::default().size(config.screen_width as f32, config.screen_height as f32),
                )
                .with_bounds(Rect::default().size(
                    (map.width * config.tilesize) as f32,
                    (map.height * config.tilesize) as f32,
                ));

                let mut entity_manager = EntityManager::new();

                entity_manager.add(
                    EntityBuilder::new()
                        .with_name("player")
                        .with_asset("hillSmall")
                        .with_object(Object::with_size(48.0, 106.0).at(100.0, 100.0))
                        .with_movement(
                            Movement::new()
                                .with_max_speed((Some(10.0), Some(20.0)))
                                .with_force(Vector2::new(0.0, 0.5)),
                        )
                        .with_input(Input::Player(PlayerInput::new())),
                );

                Self {
                    assets,
                    map,
                    config,
                    camera,
                    debug_sheet,
                    batch: Batch::new(spritesheet),
                    entity_manager,
                }
            })
    }

    fn interact(&mut self, input: &mut KeyboardAndMouse, _window: &mut Window) {
        for (uuid, entity_input) in self.entity_manager.get_inputs_mut() {
            entity_input.update(input);
        }
    }

    fn update(&mut self, _window: &Window) {
        let object_forces = self
            .entity_manager
            .get_inputs()
            .iter()
            .map(|(uuid, input)| (**uuid, input.get_force()))
            .collect::<HashMap<Uuid, Vector2<f32>>>();

        for (uuid, movement) in self.entity_manager.get_movements_mut() {
            if let Some(force) = object_forces.get(&uuid) {
                movement.add_instantaneous_force(*force);
            }
        }

        for uuid in self.entity_manager.get_entities() {
            self.entity_manager.update(uuid, &self.map);
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &coffee::Timer) {
        frame.clear(Color::BLACK);

        for (x, y, cell) in self.map.iter() {
            let source = *self
                .assets
                .offsets
                .get(cell.get_name().unwrap_or(&Cow::Borrowed("")).as_ref())
                .unwrap_or(&self.assets.default_offset);
            self.batch.add(Sprite {
                source,
                position: Point::new(
                    (x * self.config.tilesize) as f32,
                    (y * self.config.tilesize) as f32,
                ),
                scale: (self.config.scale, self.config.scale),
            });
        }

        for (uuid, asset) in self.entity_manager.get_assets() {
            if let Some(object) = self.entity_manager.get_object(uuid) {
                if !object.visible {
                    continue;
                }

                if let Some(offset) = self.assets.offsets.get(&asset) {
                    self.batch.add(Sprite {
                        source: *offset,
                        position: object.rect.point(),
                        scale: (1.0, 1.0),
                    });
                }
            }
        }

        let default = Object::with_size(70.0, 70.0);
        let object = self
            .entity_manager
            .get_object(self.entity_manager.by_name("player"))
            .unwrap_or(&default);

        self.batch.draw(
            &mut frame
                .as_target()
                .transform(self.camera.update(Some(&object.rect))),
        );
        self.batch.clear();
    }

    fn debug(&self, input: &Self::Input, frame: &mut Frame<'_>, debug: &mut Debug) {
        let mut batch = Batch::new(self.debug_sheet.clone());

        let default = Object::with_size(70.0, 70.0);
        let player = self
            .entity_manager
            .get_object(self.entity_manager.by_name("player"))
            .unwrap_or(&default);

        for cell in self.map.collidable_tiles(&player.rect) {
            batch.add(Sprite {
                source: Rectangle {
                    x: 0,
                    y: 0,
                    width: 70,
                    height: 70,
                },
                position: cell.get_rect().point(),
                scale: (1.0, 1.0),
            });

            if let Some(overlap) = player.overlap(&cell.object) {
                batch.add(Sprite {
                    source: Rectangle {
                        x: 0,
                        y: 0,
                        width: 1,
                        height: 1,
                    },
                    position: overlap.point(),
                    scale: (overlap.width, overlap.height),
                });
            }
        }

        for cell in self
            .map
            .collidable_tiles(&Rect::from_point(input.cursor_position()))
        {
            batch.add(Sprite {
                source: Rectangle {
                    x: 0,
                    y: 0,
                    width: 70,
                    height: 70,
                },
                position: cell.get_rect().point(),
                scale: (1.0, 1.0),
            });
        }
        batch.draw(
            &mut frame
                .as_target()
                .transform(self.camera.get_transform(Some(&player.rect))),
        );

        debug.draw(frame);
    }

    fn on_close_request(&mut self) -> bool {
        self.map.write();
        return true;
    }
}
