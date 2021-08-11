use macroquad::{
    color,
    prelude::*,
};

use crate::{get_global, Resources};

pub struct MapTile {
    tileset_coords: UVec2,
}

pub struct Map {
    pub size: UVec2,
    pub tile_size: UVec2,
    ground_layer: Vec<MapTile>,
}

impl Map {
    pub fn new(size: UVec2) -> Self {
        let tile_cnt = (size.x * size.y) as usize;
        let mut ground_layer = Vec::with_capacity(tile_cnt);
        for _ in 0..tile_cnt {
            ground_layer.push(MapTile{
                tileset_coords: uvec2(0, 0),
            });
        }

        Map {
            size,
            tile_size: uvec2(32, 32),
            ground_layer,
        }
    }

    pub fn draw(&self) {
        let resources = get_global::<Resources>();
        let tile_cnt = self.size.x * self.size.y;
        for i in 0..tile_cnt {
            let x = i % self.size.x;
            let y = i / self.size.x;
            draw_texture_ex(
                resources.ground_tiles,
                (x * self.tile_size.x) as f32,
                (y * self.tile_size.y) as f32,
                color::WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(32.0, 32.0)),
                    source: Some(Rect::new(0.0, 0.0, 32.0, 32.0)),
                    ..Default::default()
                },
            );
        }
    }
}
