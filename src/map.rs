// use crate::rect::Rectangle;
use coffee::graphics::{Point, Rectangle};
use itertools::iproduct;
use serde::{Deserialize, Serialize};
use serde_json;
use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cell<'a> {
    pub asset_name: Cow<'a, str>,
}

impl<'a> Cell<'a> {
    pub fn new<S>(asset_name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            asset_name: asset_name.into(),
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
        let width = 30;
        let height = 15;
        let mut cells = Vec::new();
        let wall = Cell::new("box");
        let empty = Cell::new("dirtCenter");
        for _ in 0..width {
            cells.push(wall.clone());
        }
        for _ in 0..(height - 2) {
            cells.push(wall.clone());
            for _ in 0..(width - 2) {
                cells.push(empty.clone());
            }
            cells.push(wall.clone());
        }
        for _ in 0..(width) {
            cells.push(wall.clone());
        }
        Self {
            cells: cells,
            width: width,
            height: height,
            tilesize: 70,
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

    pub fn collidable_tiles(&self, target: &Rectangle<i16>) -> Vec<Point> {
        let minx = (target.x as f32 / self.tilesize as f32) as usize;
        let maxx = ((target.x + target.width) as f32 / self.tilesize as f32).ceil() as usize;

        let miny = (target.y as f32 / self.tilesize as f32) as usize;
        let maxy = ((target.y + target.height) as f32 / self.tilesize as f32).ceil() as usize;

        iproduct!(minx..maxx, miny..maxy)
            .map(|(x, y)| {
                Point::new(
                    x as f32 * self.tilesize as f32,
                    y as f32 * self.tilesize as f32,
                )
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
