use std::{
    collections::HashMap,
    iter::FromIterator,
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    MapLayerKind,
    json::{
        Vec2,
        UVec2,
    }
};

const TILE_LAYER_KIND: &'static str = "tile_layer";
const OBJECT_LAYER_KIND: &'static str = "object_layer";

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub world_offset: Option<Vec2>,
    pub grid_size: UVec2,
    pub tile_size: Vec2,
    pub layers: Vec<MapLayer>,
    pub tilesets: Vec<MapTileset>,
}

impl From<crate::Map> for Map {
    fn from(other: crate::Map) -> Self {
        let layers = other.layers.iter().map(|(_, layer)|  {
            let (kind, tiles, objects) = match layer.kind {
                crate::MapLayerKind::TileLayer => {
                    (TILE_LAYER_KIND.to_string(),
                     Some(layer.tiles.iter().map(|opt| match opt {
                         Some(tile) => {
                             let tileset = other.tilesets.get(&tile.tileset_id)
                                 .expect(&format!("Unable to find tileset with id '{}'!", tile.tileset_id));
                             tile.tile_id + tileset.first_tile_id
                         },
                         _ => 0,
                     }).collect()),
                     None)
                }
                crate::MapLayerKind::ObjectLayer => {
                    (OBJECT_LAYER_KIND.to_string(),
                     None,
                     Some(layer.objects.iter().map(|object| MapObject::from(object.clone())).collect()))
                }
            };
            MapLayer {
                id: layer.id.clone(),
                kind,
                objects,
                tiles,
            }
        }).collect();

        Map {
            world_offset: if other.world_offset != macroquad::prelude::Vec2::ZERO { Some(Vec2::from(other.world_offset)) } else { None },
            grid_size: UVec2::from(other.grid_size),
            tile_size: Vec2::from(other.tile_size),
            layers,
            tilesets: other.tilesets.into_iter().map(|(_, tileset)| MapTileset::from(tileset)).collect(),
        }
    }
}

impl From<Map> for crate::Map {
    fn from(other: Map) -> Self {
        let tilesets = HashMap::from_iter(
            other.tilesets
                .into_iter()
                .map(|tileset| (tileset.id.clone(), crate::MapTileset::from(tileset))));

        let world_offset = macroquad::prelude::Vec2::from(other.world_offset.unwrap_or_default());
        let grid_size = macroquad::prelude::UVec2::from(other.grid_size);
        let tile_size = macroquad::prelude::Vec2::from(other.tile_size);

        let layers = HashMap::from_iter(
            other.layers
                .into_iter()
                .map(|layer| {
                    let tiles = layer.tiles
                        .unwrap_or_default()
                        .into_iter()
                        .map(|tile_id| if tile_id == 0 { None } else {
                            match tilesets
                                .iter()
                                .find(|(_, tileset)| tile_id >= tileset.first_tile_id
                                    && tile_id <= tileset.first_tile_id + tileset.grid_size.x * tileset.grid_size.y) {
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

                    let layer = crate::MapLayer {
                        id: layer.id.clone(),
                        kind: MapLayerKind::from(&*layer.kind),
                        grid_size,
                        tiles,
                        objects: layer.objects
                            .unwrap_or_default()
                            .into_iter()
                            .map(|object| crate::MapObject::from(object))
                            .collect(),
                    };
                    (layer.id.clone(), layer)
                }));

        crate::Map {
            world_offset,
            grid_size,
            tile_size,
            layers,
            tilesets,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapLayer {
    pub id: String,
    pub kind: String,
    pub tiles: Option<Vec<u32>>,
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
pub struct MapObject {
    pub id: String,
    pub prototype_id: Option<String>,
    pub position: Vec2,
}

impl From<crate::MapObject> for MapObject {
    fn from(other: crate::MapObject) -> Self {
        MapObject {
            id: other.id.clone(),
            prototype_id: other.prototype_id.clone(),
            position: Vec2::from(other.position),
        }
    }
}


impl From<MapObject> for crate::MapObject {
    fn from(other: MapObject) -> Self {
        crate::MapObject {
            id: other.id.clone(),
            prototype_id: other.prototype_id.clone(),
            position: macroquad::prelude::Vec2::from(other.position),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapTileset {
    pub id: String,
    pub texture_id: String,
    pub texture_size: UVec2,
    pub tile_size: UVec2,
    pub grid_size: UVec2,
    pub first_tile_id: u32,
    pub tile_cnt: u32,
}

impl From<crate::MapTileset> for MapTileset {
    fn from(other: crate::MapTileset) -> Self {
        MapTileset {
            id: other.id.clone(),
            texture_id: other.texture_id.clone(),
            texture_size: UVec2::from(other.texture_size),
            tile_size: UVec2::from(other.tile_size),
            grid_size: UVec2::from(other.grid_size),
            first_tile_id: other.first_tile_id,
            tile_cnt: other.tile_cnt,
        }
    }
}

impl From<MapTileset> for crate::MapTileset {
    fn from(other: MapTileset) -> Self {
        crate::MapTileset {
            id: other.id.clone(),
            texture_id: other.texture_id.clone(),
            texture_size: crate::UVec2::from(other.texture_size),
            tile_size: crate::UVec2::from(other.tile_size),
            grid_size: crate::UVec2::from(other.grid_size),
            first_tile_id: other.first_tile_id,
            tile_cnt: other.tile_cnt,
        }
    }
}
