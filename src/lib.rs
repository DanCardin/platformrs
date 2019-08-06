use coffee::graphics::WindowSettings;
use coffee::Game;

mod assets;
mod camera;
mod config;
mod entity;
mod error;
mod game;
mod input;
mod map;
mod object;
mod rect;

use crate::config::Config;
pub use crate::error::Error;
pub use crate::game::Platformrs;

pub fn run() -> Result<(), Error> {
    let config = Config::new();
    Ok(Platformrs::run(WindowSettings {
        title: String::from("platformRS"),
        size: (config.screen_width, config.screen_height),
        resizable: true,
        fullscreen: false,
    })?)
}
