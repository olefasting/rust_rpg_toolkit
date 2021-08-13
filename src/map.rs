use macroquad::{
    prelude::*,
};

use macroquad_tiled as tiled;

use crate::{get_global, Resources};
use crate::physics::Collider;

pub struct Map {
    tile_size: Vec2,
    tiled_map: tiled::Map,
}

impl Map {
    pub async fn new(tile_size: Vec2, path: &str) -> Self {
        let resources = get_global::<Resources>();
        let tiled_map_json = load_string(path).await.unwrap();
        let tiled_map = tiled::load_map(
            &tiled_map_json,
            &[
                ("../sprites/neo_zero_tiles_and_buildings_01.png", resources.get_texture(Resources::GROUND_TILES_TEXTURE_ID).clone()),
                ("../sprites/neo_zero_props_and_items_01.png", resources.get_texture(Resources::PROPS_TEXTURE_ID).clone()),
            ],
            &[],
        ).unwrap();

        Map {
            tile_size,
            tiled_map,
        }
    }

    pub fn solid_at(&self, position: Vec2) -> bool {
        let coords = position / self.tile_size;
        self.tiled_map.get_tile("solids", coords.x as u32, coords.y as u32).is_some()
    }

    pub fn solid_at_collider(&self, collider: Collider) -> bool {
        let coords = match collider {
            Collider::Rectangle(rect) => {
                (uvec2(
                    (rect.x / self.tile_size.x - (rect.w / self.tile_size.x) / 2.0) as u32,
                    (rect.y / self.tile_size.y - (rect.w / self.tile_size.y) / 2.0) as u32,
                ),
                uvec2(
                    (rect.x / self.tile_size.x + (rect.h / self.tile_size.x) / 2.0) as u32,
                    (rect.y / self.tile_size.y + (rect.h / self.tile_size.y) / 2.0) as u32,
                ))
            },
            Collider::Circle(circle) => {
                (uvec2(
                    (circle.x / self.tile_size.x - (circle.r / self.tile_size.x)) as u32,
                    (circle.y / self.tile_size.y - (circle.r / self.tile_size.y)) as u32,
                ),
                uvec2(
                    (circle.x / self.tile_size.x + (circle.r / self.tile_size.x)) as u32,
                    (circle.y / self.tile_size.y + (circle.r / self.tile_size.y)) as u32,
                ))
            }
        };
        for x in coords.0.x..coords.1.x+1 {
            for y in coords.0.y..coords.1.y+1 {
                if self.tiled_map.get_tile("solids", x, y).is_some() {
                    return true;
                }
            }
        }
        false
    }

    pub fn draw(&self) {
        if let Some(layer) = self.tiled_map.layers.get("ground") {
            self.tiled_map.draw_tiles(
                "ground",
                Rect::new(
                    0.0,
                    0.0,
                    self.tile_size.x * layer.width as f32,
                    self.tile_size.y * layer.height as f32,
                ),
                None,
            );
        }
        if let Some(layer) = self.tiled_map.layers.get("solids") {
            self.tiled_map.draw_tiles(
                "solids",
                Rect::new(
                    0.0,
                    0.0,
                    self.tile_size.x * layer.width as f32,
                    self.tile_size.y * layer.height as f32,
                ),
                None,
            );
        }
    }
}
