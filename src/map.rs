use macroquad::{
    color,
    prelude::*,
};

use macroquad_tiled as tiled;

use crate::{get_global, Resources};

pub struct MapTile {
    tileset_coords: UVec2,
}

pub struct Map {
    pub size: UVec2,
    pub tile_size: UVec2,
    ground_layer: Vec<MapTile>,
    tiled_map: tiled::Map,
}

impl Map {
    pub async fn new(size: UVec2) -> Self {
        let tile_cnt = (size.x * size.y) as usize;
        let mut ground_layer = Vec::with_capacity(tile_cnt);
        for _ in 0..tile_cnt {
            ground_layer.push(MapTile{
                tileset_coords: uvec2(1, 8),
            });
        }

        let resources = get_global::<Resources>();
        let tiled_map_json = load_string("assets/maps/map_01.json").await.unwrap();
        let tiled_map = tiled::load_map(
            &tiled_map_json,
            &[
                ("../sprites/neo_zero_tiles_and_buildings_01.png", resources.ground_tiles),
                ("../sprites/neo_zero_props_and_items_01.png", resources.props),
            ],
            &[],
        ).unwrap();

        Map {
            size,
            tile_size: uvec2(16, 16),
            ground_layer,
            tiled_map,
        }
    }

    pub fn draw(&self) {
        // let resources = get_global::<Resources>();
        // let tile_cnt = self.size.x * self.size.y;
        // for i in 0..tile_cnt {
        //     let tile = &self.ground_layer[i as usize];
        //     let x = i % self.size.x;
        //     let y = i / self.size.x;
        //     draw_texture_ex(
        //         resources.ground_tiles,
        //         (x * self.tile_size.x) as f32,
        //         (y * self.tile_size.y) as f32,
        //         color::WHITE,
        //         DrawTextureParams {
        //             dest_size: Some(vec2(
        //                 self.tile_size.x as f32,
        //                 self.tile_size.y as f32,
        //             )),
        //             source: Some(Rect::new(
        //                 (tile.tileset_coords.x * self.tile_size.x) as f32,
        //                 (tile.tileset_coords.y * self.tile_size.y) as f32,
        //                 self.tile_size.x as f32,
        //                 self.tile_size.y as f32,
        //             )),
        //             ..Default::default()
        //         },
        //     );
        // }

        let resources = get_global::<Resources>();
        for (x, y, tile) in self.tiled_map.tiles("ground", None) {
            let tileset_coords = uvec2(1, 8);
            draw_texture_ex(
                resources.ground_tiles,
                (x * self.tile_size.x) as f32,
                (y * self.tile_size.y) as f32,
                color::WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        self.tile_size.x as f32,
                        self.tile_size.y as f32,
                    )),
                    source: Some(Rect::new(
                        (tileset_coords.x * self.tile_size.x) as f32,
                        (tileset_coords.y * self.tile_size.y) as f32,
                        self.tile_size.x as f32,
                        self.tile_size.y as f32,
                    )),
                    ..Default::default()
                },
            );
        }
    }
}
