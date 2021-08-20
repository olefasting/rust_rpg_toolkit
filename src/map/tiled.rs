use macroquad::{
    experimental::{
        collections::storage,
    },
    prelude::*,
};
use macroquad_tiled as tiled;

use std::{
    collections::HashMap,
    ops::Deref,
    iter::FromIterator,
};

use crate::{
    generate_id,
    Resources,
    Map,
    MapLayerKind,
    MapLayer,
    MapTile,
    MapObject,
    MapTileset,
};
use crate::map::MapCollisionKind;

pub struct TiledTileset {
    pub relative_path: String,
    pub texture_id: String,
}

pub struct TiledMap {
    pub tiled_map: tiled::Map,
    pub tiled_tilesets: HashMap<String, TiledTileset>,
    pub collisions: HashMap<String, MapCollisionKind>,
}

impl TiledMap {
    pub fn load(
        path: &str,
        collisions: Option<&[(&str, MapCollisionKind)]>,
        tiled_tilesets: &[(&str, &str, &str)],
    ) -> Self {
        let tiled_tilesets = HashMap::from_iter(tiled_tilesets
                .iter()
                .map(|(name, relative_path, texture_id)| (name.to_string(), TiledTileset {
                    relative_path: relative_path.to_string(),
                    texture_id: texture_id.to_string(),
                })));
        let resources = storage::get::<Resources>();
        let mut tileset_textures = Vec::new();
        for (_, tileset) in &tiled_tilesets {
            let texture = resources.textures
                .get(&tileset.texture_id)
                .expect(&format!("Unable to find texture with id '{}'", tileset.texture_id));
            tileset_textures.push((tileset.relative_path.deref(), texture.clone()));
        }
        let json = std::fs::read_to_string(path)
            .expect(&format!("Error loading tiled map file '{}'!", path));
        let tiled_map = tiled::load_map(&json, tileset_textures.deref(), &[])
            .expect(&format!("Error parsing tiled map '{}'!", path));

        let collisions = match collisions {
            Some(collisions) => HashMap::from_iter(collisions
                .into_iter()
                .map(|(layer_id, kind)| (layer_id.to_string(), kind.clone()))),
            _ => HashMap::new(),
        };

        TiledMap {
            tiled_map,
            tiled_tilesets,
            collisions,
        }
    }
}

impl Into<Map> for TiledMap {
    fn into(self) -> Map {
        let raw_tiled_map = &self.tiled_map.raw_tiled_map;
        let tile_size = vec2(raw_tiled_map.tilewidth as f32, raw_tiled_map.tileheight as f32);
        let grid_size = uvec2(raw_tiled_map.width, raw_tiled_map.height);

        let mut tileset_ids = Vec::new();
        let tilesets = HashMap::from_iter(
            raw_tiled_map.tilesets
                .iter()
                .map(|raw_tiled_tileset| {
                    let tiled_tileset = self.tiled_tilesets
                        .get(&raw_tiled_tileset.name)
                        .expect(&format!("Unable to find definition for tileset with image '{}'! Did you remember to define all the tilesets when importing the TiledMap?", raw_tiled_tileset.image));

                    let id = if tileset_ids.contains(&raw_tiled_tileset.name) {
                        format!("{}_{}", raw_tiled_tileset.name, generate_id())
                    } else {
                        tileset_ids.push(raw_tiled_tileset.name.clone());
                        raw_tiled_tileset.name.clone()
                    };

                    let tileset = MapTileset {
                        id: id.clone(),
                        texture_id: tiled_tileset.texture_id.clone(),
                        texture_size: uvec2(raw_tiled_tileset.imagewidth as u32, raw_tiled_tileset.imageheight as u32),
                        tile_size: uvec2(raw_tiled_tileset.tilewidth as u32, raw_tiled_tileset.tileheight as u32),
                        grid_size: uvec2(raw_tiled_tileset.columns as u32, raw_tiled_tileset.tilecount / raw_tiled_tileset.columns as u32),
                        first_tile_id: raw_tiled_tileset.firstgid,
                        tile_cnt: raw_tiled_tileset.tilecount,
                    };

                    (id, tileset)
                }));

        let layers = HashMap::from_iter(
            self.tiled_map.layers
                .iter()
                .map(|(layer_id, tiled_layer)| {
                let (kind, tiles, objects) = if tiled_layer.objects.len() > 0 {
                    let mut object_ids = Vec::new();
                    let objects = tiled_layer.objects
                        .iter()
                        .map(|tiled_object| {
                        let id = if object_ids.contains(&tiled_object.name) {
                            format!("{}_{}", tiled_object.name, generate_id())
                        } else {
                            object_ids.push(tiled_object.name.clone());
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
                    let tiles = tiled_layer.data
                        .iter()
                        .map(|tiled_tile| {
                        if let Some(tiled_tile) = tiled_tile {
                            let tileset = tilesets.get(&tiled_tile.tileset)
                                .expect(&format!("Unable to find tiled tileset '{}'! Are you sure you defined all the tilesets when importing the map?", tiled_tile.tileset));

                            let tile = MapTile {
                                tile_id: tiled_tile.id.clone() - 1,
                                tileset_id: tileset.id.clone(),
                                texture_id: tileset.texture_id.clone(),
                                texture_coords: tileset.get_texture_coords(tiled_tile.id),
                            };

                            Some(tile)
                        } else {
                            None
                        }
                    }).collect();

                    (MapLayerKind::TileLayer, tiles, Vec::new())
                };

                let collision = match self.collisions.get(layer_id) {
                    Some(collision) => collision.clone(),
                    _ => MapCollisionKind::None,
                };

                let layer = MapLayer {
                    id: layer_id.clone(),
                    kind,
                    collision,
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