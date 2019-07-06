use coffee::graphics::{Batch, Color, Frame, Image, Point, Rectangle, Sprite, Window};
use coffee::input::{keyboard, KeyboardAndMouse};
use coffee::load::loading_screen::ProgressBar;
use coffee::load::Join;
use coffee::load::Task;
use coffee::Debug;
use std::borrow::Cow;
use std::collections::HashMap;

use crate::assets::Assets;
use crate::camera::Camera;
use crate::config::Config;
use crate::map::Map;
use crate::object::Object;
use crate::rect::Rect;
use coffee::Game;

pub struct Platformrs<'a> {
    assets: Assets<'a>,
    map: Map<'a>,
    config: Config,
    camera: Camera,
    batch: Batch,
    debug_sheet: Image,
    input: Option<Input>,
    objects: HashMap<Cow<'a, str>, Object<'a>>,
}

enum Movement {
    Left,
    Right,
}

struct Input {
    movement: Option<Movement>,
    jumping: bool,
    crouched: bool,
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

                let mut objects = HashMap::new();
                objects.insert(
                    "player".into(),
                    Object::with_size(48.0, 106.0).with_asset("hillSmall"),
                );

                Self {
                    assets,
                    map,
                    config,
                    camera,
                    objects,
                    debug_sheet,
                    batch: Batch::new(spritesheet),
                    input: None,
                }
            })
    }

    fn interact(&mut self, input: &mut KeyboardAndMouse, _window: &mut Window) {
        let mut movement = None;
        if input.is_key_pressed(keyboard::KeyCode::A) {
            movement = Some(Movement::Left);
        }
        if input.is_key_pressed(keyboard::KeyCode::D) {
            movement = Some(Movement::Right);
        }

        let mut jumping = false;
        if input.is_key_pressed(keyboard::KeyCode::W) {
            jumping = true;
        }

        let mut crouched = false;
        if input.is_key_pressed(keyboard::KeyCode::S) {
            crouched = true;
        }
        self.input = Some(Input {
            movement,
            jumping,
            crouched,
        })
    }

    fn update(&mut self, _window: &Window) {
        let speed = 5.0;

        let player = self.objects.get_mut("player").unwrap();
        if let Some(input) = &self.input {
            match input.movement {
                Some(Movement::Left) => {
                    player.rect.x -= speed;
                }
                Some(Movement::Right) => {
                    player.rect.x += speed;
                }
                _ => {}
            }

            if input.jumping {
                player.rect.y -= speed;
            } else if input.crouched {
                player.rect.y += speed;
            }
        }

        // TODO: Move collision logic elsewhere
        for cell in self.map.collidable_tiles(&player.rect) {
            // player.collide(&cell.object);
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

        for object in self.objects.values() {
            if !object.visible {
                continue;
            }
            if let Some(asset_name) = &object.asset_name {
                if let Some(offset) = self.assets.offsets.get(asset_name) {
                    self.batch.add(Sprite {
                        source: *offset,
                        position: object.rect.point(),
                        scale: (1.0, 1.0),
                    });
                }
            }
        }

        let default = Object::with_size(70.0, 70.0);
        let object = self.objects.get("player").unwrap_or(&default);

        self.batch.draw(
            &mut frame
                .as_target()
                .transform(self.camera.update(Some(&object.rect))),
        );
        self.batch.clear();
    }

    fn debug(&self, input: &Self::Input, frame: &mut Frame<'_>, debug: &mut Debug) {
        let default = Object::with_size(70.0, 70.0);
        let player = self.objects.get("player").unwrap_or(&default);

        let mut batch = Batch::new(self.debug_sheet.clone());
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
