use std::collections::HashMap;
use std::iter::FromIterator;

use macroquad::{
    color,
    prelude::*,
};

pub mod tiled;

pub use tiled::TiledMap;
use crate::{get_global, Resources, Collider};

#[derive(Clone)]
pub struct Map {
    pub world_offset: Vec2,
    pub grid_size: UVec2,
    pub tile_size: Vec2,
    pub layers: Vec<MapLayer>,
    pub tilesets: Vec<MapTileset>,
}

impl Map {
    pub fn new(path: &str) -> Self {
        let json = std::fs::read_to_string(path)
            .expect(&format!("Unable to find map file '{}'", path));
        let map: crate::json::Map = serde_json::from_str(&json)
            .expect(&format!("Error when parsing map file '{}'", path));
        Self::from(map)
    }

    pub fn draw(&self) {
        let resource = get_global::<Resources>();
        let textures: HashMap<String, Texture2D> = HashMap::from_iter(
            self.tilesets
                .iter()
                .map(|tileset| (tileset.texture_id.clone(), *resource.get_texture(&tileset.texture_id))));
        for layer in &self.layers {
            let total_offset = self.world_offset + layer.offset;
            for i in 0..layer.tiles.len() {
                if let Some(Some(tile)) = layer.tiles.get(i) {
                    let (x, y) = (i as u32 % self.grid_size.x, i as u32 / self.grid_size.x);
                    draw_texture_ex(
                        textures.get(&tile.texture_id).cloned().unwrap(),
                        total_offset.x + x as f32 * self.tile_size.x,
                        total_offset.y + y as f32 * self.tile_size.y,
                        color::WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(
                                tile.tileset_position.x,
                                tile.tileset_position.y,
                                self.tile_size.x,
                                self.tile_size.y,
                            )),
                            dest_size: Some(self.tile_size),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }

    pub fn solid_at_collider(&self, collider: Collider, include_barriers: bool) -> bool {
        false
    }

    pub fn get_beam_collision_point(
        &self,
        origin: Vec2,
        end: Vec2,
        width: f32,
        tolerance: f32,
        include_barriers: bool,
    ) -> Vec2 {
        end
    }
}

#[derive(Clone)]
pub enum MapLayerKind {
    TileLayer,
    ObjectLayer,
}

#[derive(Clone)]
pub struct MapLayer {
    pub id: String,
    pub kind: MapLayerKind,
    pub offset: Vec2,
    pub tiles: Vec<Option<MapTile>>,
    pub objects: Vec<MapObject>,
}

#[derive(Clone)]
pub struct MapTile {
    pub tile_id: u32,
    pub texture_id: String,
    pub tileset_position: Vec2,
}

#[derive(Clone)]
pub struct MapObject {
    pub id: String,
    pub prototype_id: String,
    pub position: Vec2,
}

#[derive(Clone)]
pub struct MapTileset {
    pub id: String,
    pub texture_id: String,
    pub texture_size: UVec2,
    pub tile_size: UVec2,
    pub grid_size: UVec2,
    pub first_tile_id: u32,
}

impl MapTileset {
    pub fn get_texture_position_from_tile_id(&self, tile_id: u32) -> Vec2 {
        assert!(
            tile_id >= self.first_tile_id && tile_id <= self.first_tile_id + self.grid_size.x * self.grid_size.y,
            "The specified tile_id '{}' does not belong to this tileset!",
            tile_id,
        );
        let i = tile_id - self.first_tile_id;
        vec2(
            ((i % self.grid_size.x) * self.tile_size.x) as f32,
            ((i / self.grid_size.x) * self.tile_size.y) as f32,
        )
    }
}
