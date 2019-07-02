use coffee::graphics::{Batch, Color, Frame, Image, Point, Rectangle, Sprite, Window};
use coffee::input::{keyboard, KeyboardAndMouse};
use coffee::load::loading_screen::ProgressBar;
use coffee::load::Join;
use coffee::load::Task;

use crate::assets::Assets;
use crate::camera::Camera;
use crate::config::Config;
use crate::map::Map;
use coffee::Game;

pub struct Platformrs {
    assets: Assets,
    map: Map,
    config: Config,
    camera: Camera,
    pos: Point,
    batch: Batch,
    palette: Image,
}

impl Game for Platformrs {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ProgressBar;

    fn load(_window: &Window) -> Task<Platformrs> {
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
                Task::using_gpu(|mut gpu| {
                    Image::from_colors(&mut gpu, &[Color::from_rgb(255, 0, 0)])
                }),
            ),
        )
            .join()
            .map(|(assets, map, spritesheet, palette)| {
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
                Platformrs {
                    assets: assets,
                    map,
                    config: config,
                    camera: camera,
                    pos: Point::new(0.0, 0.0),
                    batch: Batch::new(spritesheet),
                    palette,
                }
            })
    }

    fn interact(&mut self, input: &mut KeyboardAndMouse, _window: &mut Window) {
        let speed = 5.0;
        if input.is_key_pressed(keyboard::KeyCode::D) {
            self.pos.x += speed;
        }
        if input.is_key_pressed(keyboard::KeyCode::A) {
            self.pos.x -= speed;
        }
        if input.is_key_pressed(keyboard::KeyCode::W) {
            self.pos.y -= speed;
        }
        if input.is_key_pressed(keyboard::KeyCode::S) {
            self.pos.y += speed;
        }
    }

    fn update(&mut self, _window: &Window) {
        //
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &coffee::Timer) {
        frame.clear(Color::BLACK);

        let target = self
            .assets
            .offsets
            .get("hillSmall")
            .unwrap_or(&self.assets.default_offset);

        for (x, y, cell) in self.map.iter() {
            let source = *self
                .assets
                .offsets
                .get(&cell.asset_name)
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
        self.batch.draw(
            &mut frame
                .as_target()
                .transform(self.camera.get_transform(Some(&Rectangle {
                    x: self.pos.x as i16,
                    y: self.pos.y as i16,
                    width: target.width as i16,
                    height: target.height as i16,
                }))),
        );

        for position in self.map.collidable_tiles(&Rectangle {
            x: self.pos.x as i16,
            y: self.pos.y as i16,
            width: target.width as i16,
            height: target.height as i16,
        }) {
            self.palette.draw(
                Sprite {
                    source: Rectangle {
                        x: 0,
                        y: 0,
                        width: 1,
                        height: 1,
                    },
                    position,
                    scale: (self.config.tilesize as f32, self.config.tilesize as f32),
                },
                &mut frame.as_target(),
            );
        }

        self.batch.clear();
        self.batch.add(Sprite {
            source: *target,
            position: Point::new(self.pos.x as f32, self.pos.y as f32),
            scale: (1.0, 1.0),
        });

        self.batch.draw(
            &mut frame
                .as_target()
                .transform(self.camera.get_transform(Some(&Rectangle {
                    x: self.pos.x as i16,
                    y: self.pos.y as i16,
                    width: target.width as i16,
                    height: target.height as i16,
                }))),
        );
    }

    fn on_close_request(&mut self) -> bool {
        self.map.write();
        return true;
    }
}
