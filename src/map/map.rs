use std::{
    collections::HashMap,
    io,
};

use macroquad::{
    color,
    experimental::{
        collections::storage,
    },
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::prelude::*;

use super::tiled::TiledMapDeclaration;

use crate::json::tiled::{
    RawTiledMap,
    RawTiledPropertyType,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "json::MapDef", from = "json::MapDef")]
pub struct Map {
    #[serde(default = "Map::default_background_color", with = "json::ColorDef")]
    pub background_color: Color,
    #[serde(with = "json::def_vec2")]
    pub world_offset: Vec2,
    #[serde(with = "json::def_uvec2")]
    pub grid_size: UVec2,
    #[serde(with = "json::def_vec2")]
    pub tile_size: Vec2,
    pub layers: HashMap<String, MapLayer>,
    pub tilesets: HashMap<String, MapTileset>,
    #[serde(skip)]
    pub draw_order: Vec<String>,
}

impl Map {
    pub async fn load(path: &str) -> io::Result<Self> {
        let bytes = load_file(path).await.unwrap();
        let map = serde_json::from_str(&String::from_utf8(bytes).unwrap())?;
        Ok(map)
    }

    // pub async fn load_tiled(assets_path: &str, decl: TiledMapDeclaration) -> Result<Self, FileError> {
    //     let map: Map = TiledMap::load(assets_path, decl.clone()).await?.into();
    //     map.save(&format!("{}/{}", assets_path, &decl.export_path))?;
    //     Ok(map)
    // }

    pub async fn load_tiled(assets_path: &str, decl: TiledMapDeclaration) -> Result<Self, FileError> {
        let in_path = format!("{}/{}", assets_path, &decl.path);
        let out_path = format!("{}/{}", assets_path, &decl.export_path);
        let bytes = load_file(&in_path).await?;
        let tiled_map: RawTiledMap = serde_json::from_slice(&bytes).unwrap();
        let map = Map::from_tiled(tiled_map, decl);
        map.save(&out_path)?;
        Ok(map)
    }

    fn from_tiled(tiled_map: RawTiledMap, decl: TiledMapDeclaration) -> Map {
        let background_color = if let Some(background_color) = tiled_map.backgroundcolor {
            color_from_hex_string(&background_color)
        } else {
            color::BLACK
        };

        let mut tilesets = HashMap::new();
        for tileset_decl in decl.tilesets {
            let id = tileset_decl.name.clone();
            let tiled_tileset = tiled_map.tilesets
                .iter()
                .find(|tileset| tileset.name == id)
                .unwrap();

            let texture_size = uvec2(tiled_tileset.imagewidth as u32, tiled_tileset.imageheight as u32);
            let tile_size = uvec2(tiled_tileset.tilewidth as u32, tiled_tileset.tileheight as u32);
            let grid_size = uvec2(tiled_tileset.columns as u32, tiled_tileset.tilecount as u32 / tiled_tileset.columns as u32);

            let tileset = MapTileset {
                id: id.clone(),
                texture_id: tileset_decl.texture_id,
                texture_size,
                tile_size,
                grid_size,
                first_tile_id: tiled_tileset.firstgid,
                tile_cnt: tiled_tileset.tilecount,
            };

            tilesets.insert(id, tileset);
        }

        let mut layers = HashMap::new();
        let mut draw_order = Vec::new();
        for tiled_layer in tiled_map.layers {
            let collision = decl.collisions
                .iter()
                .find_map(|decl| {
                    if decl.layer_id == tiled_layer.name {
                        return Some(decl.collision_kind.clone());
                    }
                    None
                })
                .unwrap_or(MapCollisionKind::None);

            let mut tiles = Vec::new();
            for tile_id in tiled_layer.data {
                let res = if tile_id != 0 {
                    let tileset = tilesets
                        .iter()
                        .find_map(|(_, tileset)| {
                            if tile_id >= tileset.first_tile_id
                                && tile_id <= tileset.first_tile_id + tileset.tile_cnt {
                                return Some(tileset);
                            }
                            None
                        })
                        .unwrap();

                    let tile_id = tile_id - tileset.first_tile_id;
                    let tile = MapTile {
                        tile_id,
                        tileset_id: tileset.id.clone(),
                        texture_id: tileset.texture_id.clone(),
                        texture_coords: tileset.get_texture_coords(tile_id),
                    };

                    Some(tile)
                } else {
                    None
                };

                tiles.push(res);
            }


            let mut objects = Vec::new();
            for object in tiled_layer.objects {
                let position = vec2(object.x, object.y);
                let size = {
                    let size = vec2(object.width, object.height);
                    if size != Vec2::ZERO {
                        Some(size)
                    } else {
                        None
                    }
                };

                let mut properties = HashMap::new();
                if let Some(tiled_props) = object.properties {
                    for tiled_property in tiled_props {
                        let value_type = match tiled_property.value_type {
                            RawTiledPropertyType::BoolType => MapPropertyType::BoolType,
                            RawTiledPropertyType::FloatType => MapPropertyType::FloatType,
                            RawTiledPropertyType::IntType => MapPropertyType::IntType,
                            RawTiledPropertyType::StringType => MapPropertyType::StringType,
                            RawTiledPropertyType::ColorType => MapPropertyType::ColorType,
                            RawTiledPropertyType::ObjectType => MapPropertyType::ObjectType,
                            RawTiledPropertyType::FileType => MapPropertyType::FileType,
                        };

                        let property = MapProperty {
                            value: tiled_property.value,
                            value_type,
                        };

                        properties.insert(tiled_property.name, property);
                    }
                }

                let object = MapObject {
                    name: object.name,
                    position,
                    size,
                    properties,
                };

                objects.push(object);
            }

            let kind = if tiled_layer.layer_type == "tilelayer".to_string() {
                MapLayerKind::TileLayer
            } else {
                MapLayerKind::ObjectLayer
            };

            let grid_size = uvec2(tiled_map.width, tiled_map.height);
            let layer = MapLayer {
                id: tiled_layer.name,
                kind,
                collision,
                grid_size,
                tiles,
                objects,
                is_visible: tiled_layer.visible,
            };

            draw_order.push(layer.id.clone());
            layers.insert(layer.id.clone(), layer);
        }

        let grid_size = uvec2(tiled_map.width, tiled_map.height);
        Map {
            background_color,
            world_offset: Vec2::ZERO,
            grid_size,
            tile_size: vec2(tiled_map.tilewidth as f32, tiled_map.tileheight as f32),
            layers,
            tilesets,
            draw_order,
        }
    }

    pub fn to_grid(&self, rect: Rect) -> URect {
        let p = self.to_coords(rect.point());
        let w = ((rect.w / self.tile_size.x) as u32 + 1).clamp(0, self.grid_size.x - p.x);
        let h = ((rect.h / self.tile_size.y) as u32 + 1).clamp(0, self.grid_size.y - p.y);
        URect::new(p.x, p.y, w, h)
    }

    pub fn to_coords(&self, position: Vec2) -> UVec2 {
        let x = ((position.x - self.world_offset.x) as u32 / self.tile_size.x as u32).clamp(0, self.grid_size.x);
        let y = ((position.y - self.world_offset.y) as u32 / self.tile_size.y as u32).clamp(0, self.grid_size.y);
        uvec2(x, y)
    }

    pub fn to_position(&self, point: UVec2) -> Vec2 {
        vec2(
            point.x as f32 * self.tile_size.x + self.world_offset.x,
            point.y as f32 * self.tile_size.y + self.world_offset.y,
        )
    }

    pub fn get_collisions(&self, collider: Collider) -> Vec<(Vec2, MapCollisionKind)> {
        let rect = self.to_grid(collider.with_padding(self.tile_size.x).into());
        let mut collisions = Vec::new();
        'layers: for (_, layer) in &self.layers {
            if layer.is_visible {
                match layer.collision {
                    MapCollisionKind::None => continue 'layers,
                    _ => for (x, y, tile) in self.get_tiles(&layer.id, Some(rect)) {
                        if let Some(_) = tile {
                            if Collider::rect(
                                x as f32 * self.tile_size.x,
                                y as f32 * self.tile_size.y,
                                self.tile_size.x as f32,
                                self.tile_size.y as f32,
                            ).overlaps(collider) {
                                collisions.push((
                                    self.to_position(uvec2(x, y)),
                                    layer.collision.clone(),
                                ));
                            }
                        }
                    }
                }
            }
        }
        collisions
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

    pub fn draw(&self, rect: Option<URect>) {
        let rect = rect.unwrap_or(URect::new(0, 0, self.grid_size.x, self.grid_size.y));
        draw_rectangle(
            self.world_offset.x + (rect.x as f32 * self.tile_size.x),
            self.world_offset.y + (rect.y as f32 * self.tile_size.y),
            rect.w as f32 * self.tile_size.x,
            rect.h as f32 * self.tile_size.y,
            self.background_color,
        );

        let resources = storage::get::<Resources>();
        for layer_id in &self.draw_order {
            if let Some(layer) = self.layers.get(layer_id) {
                if layer.is_visible {
                    match layer.kind {
                        MapLayerKind::TileLayer => {
                            for (x, y, tile) in self.get_tiles(layer_id, Some(rect)) {
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
                                                tile.texture_coords.x, // + 0.1,
                                                tile.texture_coords.y, // + 0.1,
                                                self.tile_size.x, // - 0.2,
                                                self.tile_size.y, // - 0.2,
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
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn default_background_color() -> Color {
        color::BLACK
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn save(&self, path: &str) -> Result<(), FileError> {
        let json = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(path, json).unwrap();
        Ok(())
    }

    #[cfg(target_family = "wasm")]
    pub fn save(&self, _: &str) -> Result<(), FileError> {
        Ok(())
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
    #[serde(rename = "tile_layer")]
    TileLayer,
    #[serde(rename = "object_layer")]
    ObjectLayer,
}

impl Default for MapLayerKind {
    fn default() -> Self {
        MapLayerKind::TileLayer
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLayer {
    pub id: String,
    pub kind: MapLayerKind,
    #[serde(default, skip_serializing_if = "MapCollisionKind::is_none")]
    pub collision: MapCollisionKind,
    #[serde(with = "json::def_uvec2")]
    pub grid_size: UVec2,
    pub tiles: Vec<Option<MapTile>>,
    pub objects: Vec<MapObject>,
    #[serde(default)]
    pub is_visible: bool,
}

impl Default for MapLayer {
    fn default() -> Self {
        MapLayer {
            id: "".to_string(),
            collision: MapCollisionKind::None,
            kind: MapLayerKind::TileLayer,
            grid_size: UVec2::ZERO,
            tiles: Vec::new(),
            objects: Vec::new(),
            is_visible: true,
        }
    }
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
    pub name: String,
    #[serde(with = "json::def_vec2")]
    pub position: Vec2,
    #[serde(default, with = "json::opt_vec2", skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec2>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, MapProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapPropertyType {
    #[serde(rename = "bool")]
    BoolType,
    #[serde(rename = "float")]
    FloatType,
    #[serde(rename = "integer")]
    IntType,
    #[serde(rename = "string")]
    StringType,
    #[serde(rename = "color")]
    ColorType,
    #[serde(rename = "object")]
    ObjectType,
    #[serde(rename = "file")]
    FileType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapProperty {
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: MapPropertyType,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MapCollisionKind {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "barrier")]
    Barrier,
    #[serde(rename = "solid")]
    Solid,
}

impl MapCollisionKind {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
}

impl Default for MapCollisionKind {
    fn default() -> Self {
        MapCollisionKind::None
    }
}
