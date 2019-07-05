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
use coffee::Game;

struct Object<'a> {
    pub pos: Point,
    pub asset_name: Option<Cow<'a, str>>,
    pub visible: bool,
}

impl<'a> Object<'a> {
    pub fn new() -> Self {
        Self {
            pos: Point::new(0.0, 0.0),
            asset_name: None,
            visible: true,
        }
    }

    pub fn move_to(mut self, pos: Point) -> Object<'a> {
        self.pos = pos;
        self
    }

    pub fn with_asset<S>(mut self, asset_name: S) -> Object<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        self.asset_name = Some(asset_name.into());
        self
    }

    pub fn hide(mut self) -> Object<'a> {
        self.visible = false;
        self
    }
}

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
                let camera = Camera::new(Rectangle {
                    x: 0,
                    y: 0,
                    width: config.screen_width as i16,
                    height: config.screen_height as i16,
                })
                .with_bounds(Rectangle {
                    x: 0,
                    y: 0,
                    width: (map.width * config.tilesize) as i16,
                    height: (map.height * config.tilesize) as i16,
                });

                let mut objects = HashMap::new();
                objects.insert("player".into(), Object::new().with_asset("hillSmall"));

                Platformrs {
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
                    player.pos.x -= speed;
                }
                Some(Movement::Right) => {
                    player.pos.x += speed;
                }
                _ => {}
            }

            if input.jumping {
                player.pos.y -= speed;
            } else if input.crouched {
                player.pos.y += speed;
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &coffee::Timer) {
        frame.clear(Color::BLACK);

        for (x, y, cell) in self.map.iter() {
            let source = *self
                .assets
                .offsets
                .get(cell.asset_name.as_ref())
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
            if let Some(asset_name) = &object.asset_name {
                if let Some(offset) = self.assets.offsets.get(asset_name) {
                    self.batch.add(Sprite {
                        source: *offset,
                        position: Point::new(object.pos.x as f32, object.pos.y as f32),
                        scale: (1.0, 1.0),
                    });
                }
            }
        }

        let default = Object::new();
        let object = self.objects.get("player").unwrap_or(&default);
        let mut target = &self.assets.default_offset;
        if let Some(asset_name) = &object.asset_name {
            target = self.assets.offsets.get(asset_name).unwrap();
        }

        self.batch.draw(
            &mut frame
                .as_target()
                .transform(self.camera.update(Some(&Rectangle {
                    x: object.pos.x as i16,
                    y: object.pos.y as i16,
                    width: target.width as i16,
                    height: target.height as i16,
                }))),
        );
    }

    fn debug(&self, _input: &Self::Input, frame: &mut Frame<'_>, debug: &mut Debug) {
        let default = Object::new();
        let object = self.objects.get("player").unwrap_or(&default);
        let mut target = &self.assets.default_offset;
        if let Some(asset_name) = &object.asset_name {
            target = self.assets.offsets.get(asset_name).unwrap();
        }

        for position in self.map.collidable_tiles(&Rectangle {
            x: object.pos.x as i16,
            y: object.pos.y as i16,
            width: target.width as i16,
            height: target.height as i16,
        }) {
            self.debug_sheet.draw(
                Sprite {
                    source: Rectangle {
                        x: 0,
                        y: 0,
                        width: 72,
                        height: 72,
                    },
                    position,
                    scale: (1.0, 1.0),
                },
                &mut frame
                    .as_target()
                    .transform(self.camera.get_transform(Some(&Rectangle {
                        x: object.pos.x as i16,
                        y: object.pos.y as i16,
                        width: target.width as i16,
                        height: target.height as i16,
                    }))),
            );
        }

        debug.draw(frame);
    }

    fn on_close_request(&mut self) -> bool {
        self.map.write();
        return true;
    }
}
