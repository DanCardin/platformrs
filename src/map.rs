use crate::object::Object;
use crate::rect::Rect;
use itertools::iproduct;
use serde::{Deserialize, Serialize};
use serde_json;
use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cell<'a> {
    pub asset_name: Option<Cow<'a, str>>,
    pub object: Object,
}

impl<'a> Cell<'a> {
    pub fn with_size(width: f32, height: f32) -> Self {
        Self {
            object: Object::with_size(width, height),
            asset_name: None,
        }
    }

    pub fn collision(mut self, collides: bool) -> Self {
        self.object = self.object.collision(collides);
        self
    }

    pub fn at(mut self, x: f32, y: f32) -> Self {
        self.object = self.object.at(x, y);
        self
    }

    pub fn with_asset<S>(mut self, asset_name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.asset_name = Some(asset_name.into());
        self
    }

    pub fn get_name(&self) -> Option<&Cow<'a, str>> {
        self.asset_name.as_ref()
    }

    pub fn get_rect(&self) -> &Rect<f32> {
        &self.object.rect
    }
}

impl<'a> Default for Cell<'a> {
    fn default() -> Self {
        Self {
            object: Object::default(),
            asset_name: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Map<'a> {
    cells: Vec<Cell<'a>>,
    pub width: u16,
    pub height: u16,
    tilesize: u16,
}

impl<'a> Default for Map<'a> {
    fn default() -> Map<'a> {
        let size = 70.0;
        let width = 30;
        let height = 15;
        let mut cells = Vec::new();
        let wall = Cell::with_size(size, size).with_asset("box");
        let empty = Cell::with_size(size, size)
            .with_asset("dirtCenter")
            .collision(false);

        for y in 0..height {
            for x in 0..width {
                let mut cell = empty.clone();
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    cell = wall.clone();
                }
                cells.push(cell.at(x as f32 * size, y as f32 * size));
            }
        }
        Self {
            cells: cells,
            width: width,
            height: height,
            tilesize: size as u16,
        }
    }
}

impl<'a> Map<'a> {
    pub fn load() -> Self {
        let path = Path::new("assets/map.map");
        if !path.exists() {
            let file = File::create(path).unwrap();
            let writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &Map::default()).unwrap();
        }

        let file = File::open("assets/map.map").unwrap();
        let reader = io::BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }

    pub fn iter(&'a self) -> IterMap<'a> {
        IterMap {
            map: self,
            index: 0,
        }
    }

    pub fn write(&self) {
        let path = Path::new("assets/map.map");
        let file = File::create(path).unwrap();
        let writer = io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)
            .unwrap_or_else(|e| println!("Unable to write the map file: {}.", e));
    }

    pub fn collidable_tiles(&self, target: &Rect<f32>) -> Vec<&Cell> {
        let x = f32::max(target.x, 0.0);
        let minx = (x / self.tilesize as f32) as usize;
        let maxx = ((x + target.width) / self.tilesize as f32).ceil() as usize;

        let y = f32::max(target.y, 0.0);
        let miny = (y / self.tilesize as f32) as usize;
        let maxy = ((y + target.height) / self.tilesize as f32).ceil() as usize;

        iproduct!(minx..maxx, miny..maxy)
            .filter_map(|(x, y)| {
                self.cells
                    .get(std::cmp::max(0, y * self.width as usize + x))
            })
            .collect()
    }
}

pub struct IterMap<'a> {
    map: &'a Map<'a>,
    index: usize,
}

impl<'a> Iterator for IterMap<'a> {
    type Item = (u16, u16, &'a Cell<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.map.cells.len() {
            return None;
        }
        let x = self.index as u16 % self.map.width;
        let y = self.index as u16 / self.map.width;
        let cell = &self.map.cells[self.index];
        self.index += 1;
        Some((x as u16, y as u16, cell))
    }
}
