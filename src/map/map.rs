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

use crate::{
    resources::Resources,
    physics::Collider,
    math::URect,
    json,
};

use super::TiledMap;

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

    pub async fn load_tiled(
        path: &str,
        export_path: Option<&str>,
        collisions: Option<&[(&str, MapCollisionKind)]>,
        tiled_tilesets: &[(&str, &str, &str)],
    ) -> io::Result<Self> {
        let map: Map = TiledMap::load(path, collisions, tiled_tilesets).await.into();
        if let Some(export_path) = export_path {
            map.save(export_path)?;
        }
        Ok(map)
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
                        },
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
    pub fn save(&self, path: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    #[cfg(target_family = "wasm")]
    pub fn save(&self, _: &str) -> io::Result<()> {
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
            is_visible: true
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
    pub properties: HashMap<String, String>,
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
