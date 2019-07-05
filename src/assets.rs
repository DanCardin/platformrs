use coffee::graphics::Rectangle;
use serde::Deserialize;
use serde_json;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io;

use crate::error::Error;

#[derive(Debug, Deserialize)]
struct SubTexture {
    name: String,
    path: String,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

#[derive(Debug, Deserialize)]
struct TextureAtlas {
    path: String,
    items: Vec<SubTexture>,
}

pub struct Assets<'a> {
    pub offsets: HashMap<Cow<'a, str>, Rectangle<u16>>,
    pub default_offset: Rectangle<u16>,
}

impl<'a> Assets<'a> {
    pub fn load() -> Result<Assets<'a>, Error> {
        let file = File::open("assets/tiles.json")?;
        let reader = io::BufReader::new(file);
        let foo: TextureAtlas = serde_json::from_reader(reader)?;
        Ok(Assets {
            offsets: foo
                .items
                .iter()
                .map(|st| {
                    (
                        Cow::Owned(st.name.clone()),
                        Rectangle {
                            x: st.x,
                            y: st.y,
                            width: st.width,
                            height: st.height,
                        },
                    )
                })
                .collect(),
            default_offset: Rectangle {
                x: 0,
                y: 0,
                width: 70,
                height: 70,
            },
        })
    }
}
