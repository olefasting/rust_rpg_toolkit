use macroquad::{
    color,
    prelude::*,
};

use macroquad_tiled as tiled;

use crate::{get_global, Resources};

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
                ("../sprites/neo_zero_tiles_and_buildings_01.png", resources.ground_tiles),
                ("../sprites/neo_zero_props_and_items_01.png", resources.props),
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
