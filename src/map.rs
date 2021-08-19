mod tiled;

use std::{
    collections::HashMap,
    iter::FromIterator,
    io,
};

use macroquad::{
    color,
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

pub use tiled::{
    TiledMap,
    TiledTileset,
};

use crate::{
    Resources,
    Collider,
    generate_id,
    MAP_LAYER_BARRIERS,
    MAP_LAYER_SOLIDS,
    draw_aligned_text,
    MAP_LAYER_GROUND,
    get_global,
    json,
};
use crate::physics::beam_collision_check;
use crate::render::{Viewport, HorizontalAlignment};
use crate::math::URect;
use crate::json::MapDef;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "json::MapDef", from = "json::MapDef")]
pub struct Map {
    #[serde(with = "json::def_vec2")]
    pub world_offset: Vec2,
    #[serde(with = "json::def_uvec2")]
    pub grid_size: UVec2,
    #[serde(with = "json::def_vec2")]
    pub tile_size: Vec2,
    pub layers: HashMap<String, MapLayer>,
    pub tilesets: HashMap<String, MapTileset>,
}

impl Map {
    pub fn load(path: &str) -> io::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let map = serde_json::from_str(&json)?;
        Ok(map)
    }

    pub fn load_tiled(path: &str, export_path: Option<&str>, tiled_tilesets: &[(&str, &str, &str)]) -> io::Result<Self> {
        let map = Map::from(TiledMap::new(path, tiled_tilesets));
        if let Some(export_path) = export_path {
           let json = serde_json::to_string_pretty(&map)?;
            std::fs::write(export_path, json)?;
        }
        Ok(map)
    }

    pub fn to_map_grid(&self, rect: Rect) -> URect {
            let p = self.to_map_point(rect.point());
            let w = ((rect.w / self.tile_size.x) as u32).clamp(0, self.grid_size.x - p.x);
            let h = ((rect.h / self.tile_size.y) as u32).clamp(0, self.grid_size.y - p.y);
        URect::new(p.x, p.y, w, h)
    }

    pub fn to_map_point(&self, position: Vec2) -> UVec2 {
        let x = ((position.x - self.world_offset.x) as u32 / self.tile_size.x as u32).clamp(0, self.grid_size.x - 1);
        let y = ((position.y - self.world_offset.y) as u32 / self.tile_size.y as u32).clamp(0, self.grid_size.y - 1);
        uvec2(x, y)
    }

    pub fn get_tile(&self, layer_id: &str, x: u32, y: u32) -> &Option<MapTile> {
        let layer = self.layers
            .get(layer_id)
            .expect(&format!("No layer with id '{}'!", layer_id));

        if x >= self.grid_size.x || y >= self.grid_size.y {
            return &None;
        };

        let i = (y * self.grid_size.x + x) as usize;
        &layer.tiles[i]
    }

    pub fn get_tiles(&self, layer_id: &str, rect: Option<URect>) -> MapTileIterator {
        let rect = rect.unwrap_or(URect::new(0, 0, self.grid_size.x, self.grid_size.y));
        let layer = self.layers.get(layer_id)
            .expect(&format!("No layer with id '{}'!", layer_id));

        MapTileIterator::new(layer, rect)
    }

    pub fn draw(&mut self, layer_ids: &[&str], rect: Option<URect>) {
        let resources = get_global::<Resources>();
        for layer_id in layer_ids {
            for (x, y, tile) in self.get_tiles(layer_id, rect) {
                if let Some(tile) = tile {
                    let world_position = self.world_offset + vec2(
                        x as f32 * self.tile_size.x,
                        y as f32 * self.tile_size.y,
                    );

                    let texture = resources.textures
                        .get(&tile.texture_id)
                        .cloned()
                        .expect(&format!("No texture with id '{}'!", tile.texture_id));

                    draw_texture_ex(
                        texture,
                        world_position.x,
                        world_position.y,
                        color::WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(
                                tile.texture_coords.x + 1.1,
                                tile.texture_coords.y + 1.1,
                                self.tile_size.x - 2.2,
                                self.tile_size.y - 2.2,
                            )),
                            dest_size: Some(vec2(
                                self.tile_size.x,
                                self.tile_size.y,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}

pub struct MapTileIterator<'a> {
    rect: URect,
    current: (u32, u32),
    layer: &'a MapLayer,
}

impl<'a> MapTileIterator<'a> {
    fn new(layer: &'a MapLayer, rect: URect) -> Self {
        let current = (rect.x, rect.y);
        MapTileIterator {
            layer,
            rect,
            current,
        }
    }
}

impl<'a> Iterator for MapTileIterator<'a> {
    type Item = (u32, u32, &'a Option<MapTile>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.current.0 + 1 >= self.rect.x + self.rect.w {
            (self.rect.x, self.current.1 + 1)
        } else {
            (self.current.0 + 1, self.current.1)
        };

        if self.current.1 >= self.rect.y + self.rect.h {
            return None;
        }

        let i = (self.current.1 * self.layer.grid_size.x + self.current.0) as usize;
        let res = Some((
            self.current.0,
            self.current.1,
            &self.layer.tiles[i],
        ));

        self.current = next;
        return res;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapLayerKind {
    #[serde(alias = "tile_layer")]
    TileLayer,
    #[serde(alias = "object_layer")]
    ObjectLayer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLayer {
    pub id: String,
    pub kind: MapLayerKind,
    #[serde(with = "json::def_uvec2")]
    pub grid_size: UVec2,
    pub tiles: Vec<Option<MapTile>>,
    pub objects: Vec<MapObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapTile {
    pub tile_id: u32,
    pub tileset_id: String,
    pub texture_id: String,
    #[serde(with = "json::def_vec2")]
    pub texture_coords: Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapObject {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prototype_id: Option<String>,
    #[serde(with = "json::def_vec2")]
    pub position: Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapTileset {
    pub id: String,
    pub texture_id: String,
    #[serde(with = "json::def_uvec2")]
    pub texture_size: UVec2,
    #[serde(with = "json::def_uvec2")]
    pub tile_size: UVec2,
    #[serde(with = "json::def_uvec2")]
    pub grid_size: UVec2,
    pub first_tile_id: u32,
    pub tile_cnt: u32,
}

impl MapTileset {
    pub fn get_texture_coords(&self, tile_id: u32) -> Vec2 {
        let x = ((tile_id % self.grid_size.x) * self.tile_size.x) as f32;
        let y = ((tile_id / self.grid_size.x) * self.tile_size.y) as f32;
        vec2(x, y)
    }
}
