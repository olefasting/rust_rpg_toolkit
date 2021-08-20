use std::{
    collections::HashMap,
    iter::FromIterator,
};

use macroquad::prelude::*;

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    Map,
    MapLayerKind,
    MapLayer,
    MapTile,
    MapObject,
    MapTileset,
};
use crate::map::MapCollisionKind;

#[derive(Clone, Serialize, Deserialize)]
pub struct MapDef {
    #[serde(with = "super::def_vec2", default)]
    pub world_offset: Vec2,
    #[serde(with = "super::def_uvec2")]
    pub grid_size: UVec2,
    #[serde(with = "super::def_vec2")]
    pub tile_size: Vec2,
    pub layers: Vec<MapLayerDef>,
    pub tilesets: Vec<MapTilesetDef>,
}

impl Into<MapDef> for Map {
    fn into(self) -> MapDef {
        let layers = self.layers.iter().map(|(_, layer)|  {
            let (tiles, objects) = match layer.kind {
                crate::MapLayerKind::TileLayer => {
                    println!("save layer_id: {}, vec_len: {}", layer.id, layer.tiles.len());
                    (Some(layer.tiles.iter().map(|opt| match opt {
                        Some(tile) => {
                            let tileset = self.tilesets.get(&tile.tileset_id)
                                .expect(&format!("Unable to find tileset with id '{}'!", tile.tileset_id));
                            tile.tile_id + tileset.first_tile_id
                        },
                        _ => 0,
                    }).collect()),
                     None)
                },
                MapLayerKind::ObjectLayer => (None, Some(layer.objects.clone())),
            };
            MapLayerDef {
                id: layer.id.clone(),
                kind: layer.kind.clone(),
                collision: layer.collision.clone(),
                objects,
                tiles,
            }
        }).collect();

        MapDef {
            world_offset: self.world_offset,
            grid_size: self.grid_size,
            tile_size: self.tile_size,
            layers,
            tilesets: self.tilesets.into_iter().map(|(_, tileset)| MapTilesetDef::from(tileset)).collect(),
        }
    }
}

impl From<MapDef> for Map {
    fn from(def: MapDef) -> Self {
        let tilesets = HashMap::from_iter(
            def.tilesets
                .clone()
                .into_iter()
                .map(|tileset| (tileset.id.clone(), crate::MapTileset::from(tileset))));

        let layers = HashMap::from_iter(
            def.layers
                .iter()
                .map(|layer| {
                    if let Some(tiles) = &layer.tiles {
                        println!("load: layer_id: {}, vec_len: {}", layer.id, tiles.len());
                    }
                    let tiles = layer.tiles
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|tile_id| if tile_id == 0 { None } else {
                            match tilesets
                                .iter()
                                .find(|(_, tileset)| tile_id >= tileset.first_tile_id
                                    && tile_id < tileset.first_tile_id + tileset.tile_cnt) {
                                Some((_, tileset)) => Some(crate::MapTile {
                                    tile_id: tile_id - tileset.first_tile_id,
                                    tileset_id: tileset.id.clone(),
                                    texture_id: tileset.texture_id.clone(),
                                    texture_coords: tileset.get_texture_coords(tile_id),
                                }),
                                _ => {
                                    panic!("Unable to determine tileset from tile_id '{}'", tile_id);
                                    None
                                }
                            }
                        }).collect();

                    let layer = MapLayer {
                        id: layer.id.clone(),
                        kind: layer.kind.clone(),
                        collision: layer.collision.clone(),
                        grid_size: def.grid_size,
                        tiles,
                        objects: layer.objects.clone().unwrap_or(Vec::new()),
                    };
                    (layer.id.clone(), layer)
                }));

        Map {
            world_offset: def.world_offset,
            grid_size: def.grid_size,
            tile_size: def.tile_size,
            layers,
            tilesets,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapLayerDef {
    pub id: String,
    #[serde(default, skip_serializing_if = "MapCollisionKind::is_none")]
    pub collision: MapCollisionKind,
    pub kind: MapLayerKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<MapObject>>,
}

impl From<&str> for crate::MapLayerKind {
    fn from(other: &str) -> Self {
        match other {
            TILE_LAYER_KIND => MapLayerKind::TileLayer,
            OBJECT_LAYER_KIND => MapLayerKind::ObjectLayer,
            _ => {
                panic!("Invalid map layer kind '{}'!", other);
                MapLayerKind::TileLayer
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapObjectDef {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prototype_id: Option<String>,
    #[serde(with = "super::def_vec2")]
    pub position: Vec2,
}

impl From<crate::MapObject> for MapObjectDef {
    fn from(other: crate::MapObject) -> Self {
        MapObjectDef {
            id: other.id.clone(),
            prototype_id: other.prototype_id.clone(),
            position: other.position,
        }
    }
}


impl From<MapObjectDef> for crate::MapObject {
    fn from(other: MapObjectDef) -> Self {
        crate::MapObject {
            id: other.id.clone(),
            prototype_id: other.prototype_id.clone(),
            position: other.position,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapTilesetDef {
    pub id: String,
    pub texture_id: String,
    #[serde(with = "super::def_uvec2")]
    pub texture_size: UVec2,
    #[serde(with = "super::def_uvec2")]
    pub tile_size: UVec2,
    #[serde(with = "super::def_uvec2")]
    pub grid_size: UVec2,
    pub first_tile_id: u32,
    pub tile_cnt: u32,
}

impl From<crate::MapTileset> for MapTilesetDef {
    fn from(other: crate::MapTileset) -> Self {
        MapTilesetDef {
            id: other.id.clone(),
            texture_id: other.texture_id.clone(),
            texture_size: other.texture_size,
            tile_size: other.tile_size,
            grid_size: other.grid_size,
            first_tile_id: other.first_tile_id,
            tile_cnt: other.tile_cnt,
        }
    }
}

impl From<MapTilesetDef> for crate::MapTileset {
    fn from(other: MapTilesetDef) -> Self {
        crate::MapTileset {
            id: other.id.clone(),
            texture_id: other.texture_id.clone(),
            texture_size: other.texture_size,
            tile_size: other.tile_size,
            grid_size: other.grid_size,
            first_tile_id: other.first_tile_id,
            tile_cnt: other.tile_cnt,
        }
    }
}
