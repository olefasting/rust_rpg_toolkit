use std::{
    collections::HashMap,
    iter::FromIterator,
};

use macroquad::prelude::*;

use serde::{
    Serialize,
    Deserialize,
};

use crate::map::{
    Map,
    MapLayerKind,
    MapLayer,
    MapTile,
    MapObject,
    MapTileset,
    MapCollisionKind,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct MapDef {
    #[serde(with = "super::def_vec2", default)]
    pub world_offset: Vec2,
    #[serde(with = "super::def_uvec2")]
    pub grid_size: UVec2,
    #[serde(with = "super::def_vec2")]
    pub tile_size: Vec2,
    pub layers: Vec<MapLayerDef>,
    pub tilesets: Vec<MapTileset>,
}

impl Into<MapDef> for Map {
    fn into(self) -> MapDef {
        let layers = self.draw_order.iter().filter_map(|layer_id|  {
            if let Some(layer) = self.layers.get(layer_id) {
                let (tiles, objects) = match layer.kind {
                    MapLayerKind::TileLayer => {
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

                Some(MapLayerDef {
                    id: layer.id.clone(),
                    kind: layer.kind.clone(),
                    collision: layer.collision.clone(),
                    objects,
                    tiles,
                })
            } else {
                None
            }
        }).collect();

        MapDef {
            world_offset: self.world_offset,
            grid_size: self.grid_size,
            tile_size: self.tile_size,
            layers,
            tilesets: self.tilesets
                .into_iter()
                .map(|(_, tileset)| tileset)
                .collect(),
        }
    }
}

impl From<MapDef> for Map {
    fn from(def: MapDef) -> Self {
        let tilesets = HashMap::from_iter(
            def.tilesets
                .clone()
                .into_iter()
                .map(|tileset| (tileset.id.clone(), tileset)));

        let draw_order = def.layers
            .iter()
            .map(|layer| layer.id.clone())
            .collect();

        let layers = HashMap::from_iter(
            def.layers
                .iter()
                .map(|layer| {
                    let tiles = layer.tiles
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|tile_id| if tile_id == 0 { None } else {
                            let tile = match tilesets
                                .iter()
                                .find(|(_, tileset)| tile_id >= tileset.first_tile_id
                                    && tile_id < tileset.first_tile_id + tileset.tile_cnt) {
                                Some((_, tileset)) => Some(MapTile {
                                    tile_id: tile_id - tileset.first_tile_id,
                                    tileset_id: tileset.id.clone(),
                                    texture_id: tileset.texture_id.clone(),
                                    texture_coords: tileset.get_texture_coords(tile_id),
                                }),
                                _ => {
                                    None
                                }
                            };
                            assert!(tile.is_some(), "Unable to determine tileset from tile_id '{}'", tile_id);
                            tile
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
            draw_order,
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
