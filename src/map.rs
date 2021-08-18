mod tiled;

use std::collections::HashMap;
use std::iter::FromIterator;

use macroquad::{
    color,
    prelude::*,
};

pub use tiled::{
    TiledMap,
    TiledTileset,
};

use crate::{get_global, Resources, Collider, generate_id, MAP_LAYER_BARRIERS, MAP_LAYER_SOLIDS, draw_aligned_text, MAP_LAYER_GROUND};
use crate::physics::beam_collision_check;
use crate::render::{Viewport, HorizontalAlignment};
use crate::globals::DebugMode;
use crate::math::URect;
use std::thread::current;

#[derive(Debug, Clone)]
pub struct Map {
    pub world_offset: Vec2,
    pub grid_size: UVec2,
    pub tile_size: UVec2,
    pub layers: HashMap<String, MapLayer>,
    pub tilesets: HashMap<String, MapTileset>,
}

impl Map {
    pub fn new(path: &str) -> Self {
        let json = std::fs::read_to_string(path)
            .expect(&format!("Unable to find map file '{}'", path));
        let map: crate::json::Map = serde_json::from_str(&json)
            .expect(&format!("Error when parsing map file '{}'", path));
        Self::from(map)
    }

    pub fn to_grid_coords(&self, rect: Rect) -> URect {
        URect::new(
            ((rect.x as u32 - self.world_offset.x as u32) / self.tile_size.x).clamp(0, self.grid_size.x),
            ((rect.y as u32 - self.world_offset.y as u32) / self.tile_size.y).clamp(0, self.grid_size.y),
            (rect.w as u32 / self.tile_size.x).clamp(0, self.grid_size.x),
            (rect.h as u32 / self.tile_size.y).clamp(0, self.grid_size.y),
        )
    }

    pub fn get_tiles(&self, layer_id: &str, rect: Option<URect>) -> MapTileIterator {
        let rect = rect.unwrap_or(URect::new(0, 0, self.grid_size.x, self.grid_size.y));
        let layer = self.layers.get(layer_id)
            .expect(&format!("No layer with id '{}'!", layer_id));
        MapTileIterator::new(layer, rect)
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

    pub fn draw(&self, layer_ids: &[&str], rect: Option<URect>) {
        let resources = get_global::<Resources>();
        for layer_id in layer_ids {
            for (x, y, tile) in self.get_tiles(layer_id, rect) {
                if let Some(tile) = tile {
                    let texture = resources.textures
                        .get(&tile.texture_id)
                        .cloned()
                        .expect(&format!("No texture with id '{}'!", tile.texture_id));

                    let world_position = self.world_offset + vec2(
                        x as f32 * self.tile_size.x as f32,
                        y as f32 * self.tile_size.y as f32,
                    );

                    draw_texture_ex(
                        texture,
                        world_position.x,
                        world_position.y,
                        color::WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(
                                tile.texture_coords.x + 1.1,
                                tile.texture_coords.y + 1.1,
                                self.tile_size.x as f32 - 2.2,
                                self.tile_size.y as f32 - 2.2,
                            )),
                            dest_size: Some(vec2(
                                self.tile_size.x as f32,
                                self.tile_size.y as f32,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }

    pub fn get_beam_collision_point(&self, origin: Vec2, end: Vec2, width: f32, tolerance: f32, layer_ids: &[&str]) -> Vec2 {
        let coords = (
            uvec2((origin.x + self.world_offset.x) as u32 / self.tile_size.x, (origin.y + self.world_offset.y) as u32 / self.tile_size.y),
            uvec2((end.x + self.world_offset.x) as u32 / self.tile_size.x, (end.y + self.world_offset.y) as u32 / self.tile_size.y),
        );
        let ord_x = if coords.0.x > coords.1.x { (coords.1.x, coords.0.x) } else { (coords.0.x, coords.1.x) };
        let ord_y = if coords.0.y > coords.1.y { (coords.1.y, coords.0.y) } else { (coords.0.y, coords.1.y) };
        let mut collisions = Vec::new();
        for x in ord_x.0..ord_x.1 {
            for y in ord_y.0..ord_y.1 {
                let position = vec2(
                    ((x * self.tile_size.x) + self.tile_size.x / 2) as f32,
                    ((y * self.tile_size.y) + self.tile_size.y / 2) as f32,
                );
                for layer_id in layer_ids {
                    if self.get_tile(layer_id, x, y).is_some() {
                        if beam_collision_check(position, origin, end, width, tolerance) {
                            collisions.push(position);
                        }
                    }
                }
            }
        }
        if collisions.len() > 0 {
            collisions.sort_by(|a, b| a.distance(origin).partial_cmp(&b.distance(origin)).unwrap());
            *collisions.first().unwrap_or(&end)
        } else {
            end
        }
    }
}

impl From<TiledMap> for Map {
    fn from(other: TiledMap) -> Self {
        let raw_tiled_map = &other.tiled_map.raw_tiled_map;
        let tile_size = uvec2(raw_tiled_map.tilewidth, raw_tiled_map.tileheight);
        let grid_size = uvec2(raw_tiled_map.width, raw_tiled_map.height);

        let mut tileset_names = Vec::new();
        let tilesets = HashMap::from_iter(raw_tiled_map.tilesets.iter().map(|raw_tiled_tileset| {
            let tiled_tileset = other.tiled_tilesets
                .get(&raw_tiled_tileset.name)
                .expect(&format!("Unable to find definition for tileset with image '{}'! Did you remember to define all the tilesets when importing the TiledMap?", raw_tiled_tileset.image));
            let name = if tileset_names.contains(&raw_tiled_tileset.name) {
                format!("{}_{}", raw_tiled_tileset.name, generate_id())
            } else {
                tileset_names.push(raw_tiled_tileset.name.clone());
                raw_tiled_tileset.name.clone()
            };
            (name.clone(), MapTileset {
                id: name,
                texture_id: tiled_tileset.texture_id.clone(),
                texture_size: uvec2(raw_tiled_tileset.imagewidth as u32, raw_tiled_tileset.imageheight as u32),
                tile_size: uvec2(raw_tiled_tileset.tilewidth as u32, raw_tiled_tileset.tileheight as u32),
                grid_size: uvec2(raw_tiled_tileset.columns as u32, raw_tiled_tileset.tilecount / raw_tiled_tileset.columns as u32),
                first_tile_id: raw_tiled_tileset.firstgid,
                tile_cnt: raw_tiled_tileset.tilecount,
            })
        }));

        let layers = HashMap::from_iter(other.tiled_map.layers.iter().map(|(layer_id, tiled_layer)| {
            let (kind, tiles, objects) = if tiled_layer.objects.len() > 0 {
                let mut object_names = Vec::new();
                let objects = tiled_layer.objects.iter().map(|tiled_object| {
                    let id = if object_names.contains(&tiled_object.name) {
                        format!("{}_{}", tiled_object.name, generate_id())
                    } else {
                        object_names.push(tiled_object.name.clone());
                        tiled_object.name.clone()
                    };
                    MapObject {
                        id,
                        prototype_id: None,
                        position: vec2(tiled_object.world_x, tiled_object.world_y),
                    }
                }).collect();
                (MapLayerKind::ObjectLayer, Vec::new(), objects)
            } else {
                let tiles = tiled_layer.data.iter().map(|tiled_tile| {
                    if let Some(tiled_tile) = tiled_tile {
                        let tileset = tilesets.get(&tiled_tile.tileset)
                            .expect(&format!("Unable to find tiled tileset '{}'! Are you sure you defined all the tilesets when importing the map?", tiled_tile.tileset));
                        Some(MapTile {
                            tile_id: tiled_tile.id.clone(),
                            tileset_id: tileset.id.clone(),
                            texture_id: tileset.texture_id.clone(),
                            texture_coords: tileset.get_texture_position_from_tile_id(tiled_tile.id),
                        })
                    } else {
                        None
                    }
                }).collect();
                (MapLayerKind::TileLayer, tiles, Vec::new())
            };

            let layer = MapLayer {
                id: layer_id.clone(),
                kind,
                grid_size,
                tiles,
                objects,
            };

            (layer_id.clone(), layer)
        }));

        Map {
            world_offset: Vec2::ZERO,
            grid_size,
            tile_size,
            layers,
            tilesets,
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

#[derive(Debug, Clone)]
pub enum MapLayerKind {
    TileLayer,
    ObjectLayer,
}

#[derive(Debug, Clone)]
pub struct MapLayer {
    pub id: String,
    pub kind: MapLayerKind,
    pub grid_size: UVec2,
    pub tiles: Vec<Option<MapTile>>,
    pub objects: Vec<MapObject>,
}

#[derive(Debug, Clone)]
pub struct MapTile {
    pub tile_id: u32,
    pub tileset_id: String,
    pub texture_id: String,
    pub texture_coords: Vec2,
}

#[derive(Debug, Clone)]
pub struct MapObject {
    pub id: String,
    pub prototype_id: Option<String>,
    pub position: Vec2,
}

#[derive(Debug, Clone)]
pub struct MapTileset {
    pub id: String,
    pub texture_id: String,
    pub texture_size: UVec2,
    pub tile_size: UVec2,
    pub grid_size: UVec2,
    pub first_tile_id: u32,
    pub tile_cnt: u32,
}

impl MapTileset {
    pub fn get_texture_position_from_tile_id(&self, tile_id: u32) -> Vec2 {
        vec2(
            ((tile_id % self.grid_size.x) * self.tile_size.x) as f32,
            ((tile_id / self.grid_size.x) * self.tile_size.y) as f32,
        )
    }
}
