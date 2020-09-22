use crate::assets::Assets;
use crate::config::Config;
use crate::entity::EntityManager;
use crate::map::Map;
use coffee::graphics::{Batch, Point, Sprite};
use std::borrow::Cow;

pub trait Drawable {
    fn draw(&self, config: &Config, assets: &Assets, batch: &mut Batch);
    // fn debug(&self, config: &Config, assets: &Assets, batch: &mut Batch);
}

impl<'a> Drawable for Map<'a> {
    fn draw(&self, config: &Config, assets: &Assets, batch: &mut Batch) {
        for (x, y, cell) in self.iter() {
            let source = *assets
                .offsets
                .get(cell.get_name().unwrap_or(&Cow::Borrowed("")).as_ref())
                .unwrap_or(&assets.default_offset);

            batch.add(Sprite {
                source,
                position: Point::new((x * config.tilesize) as f32, (y * config.tilesize) as f32),
                scale: (config.scale, config.scale),
            });
        }
    }

    // fn debug(&self, config: &Config, assets: &Assets, batch: &mut Batch) {
    // for cell in self.collidable_tiles(&player.rect) {
    //     batch.add(Sprite {
    //         source: Rectangle {
    //             x: 0,
    //             y: 0,
    //             width: 70,
    //             height: 70,
    //         },
    //         position: cell.get_rect().point(),
    //         scale: (1.0, 1.0),
    //     });
    // }
    // }
}

impl<'a> Drawable for EntityManager<'a> {
    fn draw(&self, config: &Config, assets: &Assets, batch: &mut Batch) {
        for (uuid, asset) in self.get_assets() {
            if let Some(object) = self.get_object(uuid) {
                if !object.visible {
                    continue;
                }

                if let Some(offset) = assets.offsets.get(&asset) {
                    batch.add(Sprite {
                        source: *offset,
                        position: object.rect.point(),
                        scale: (1.0, 1.0),
                    });
                }
            }
        }
    }
}
