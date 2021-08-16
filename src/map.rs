use std::collections::HashMap;
use std::iter::FromIterator;

use macroquad::{
    color,
    prelude::*,
};

use crate::{get_global, Resources, Collider};
use crate::physics::beam_collision_check;

#[derive(Clone)]
pub struct Map {
    pub world_offset: Vec2,
    pub grid_size: UVec2,
    pub tile_size: UVec2,
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
                .map(|tileset| (tileset.texture_id.clone(), *resource.textures.get(&tileset.texture_id).unwrap())));
        for layer in &self.layers {
            for i in 0..layer.tiles.len() {
                if let Some(Some(tile)) = layer.tiles.get(i) {
                    let (x, y) = (i as u32 % self.grid_size.x, i as u32 / self.grid_size.x);
                    draw_texture_ex(
                        textures.get(&tile.texture_id).cloned().unwrap(),
                        self.world_offset.x + x as f32 * self.tile_size.x as f32,
                        self.world_offset.y + y as f32 * self.tile_size.y as f32,
                        color::WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(
                                tile.tileset_position.x,
                                tile.tileset_position.y,
                                self.tile_size.x as f32,
                                self.tile_size.y as f32,
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

    pub fn get_tile_at_coords(&self, coords: UVec2, layer_id: &str) -> Option<MapTile> {
        if let Some(layer) = self.layers.iter().find(|layer| layer.id == layer_id) {
            if coords.x >= self.grid_size.x
                || coords.y >= self.grid_size.y
                || coords.x >= self.grid_size.x
                || coords.y >= self.grid_size.y {
                return None;
            }
            let i = coords.y * self.grid_size.x + coords.x;
            layer.tiles.get(i as usize).cloned().unwrap_or(None)
        } else {
            None
        }
    }

    pub fn get_tile_at_position(&self, position: Vec2, layer_id: &str) -> Option<MapTile> {
        let offset_position = position - self.world_offset;
        let map_size = self.grid_size * self.tile_size;
        if offset_position.x < 0.0 || offset_position.x >= map_size.x as f32 || offset_position.y < 0.0 || offset_position.y >= map_size.y as f32 {
            None
        } else {
            self.get_tile_at_coords(uvec2(offset_position.x as u32, offset_position.y as u32) / self.tile_size, layer_id)
        }
    }

    pub fn is_tile_at_position(&self, position: Vec2, layer_ids: &[&str]) -> bool {
        for layer_id in layer_ids {
            if self.get_tile_at_position(position, layer_id).is_some() {
                return true;
            }
        }
        false
    }

    pub fn is_tile_at_collider(&self, collider: Collider, layer_ids: &[&str]) -> bool {
        let offset_position = collider.get_position() - self.world_offset;
        let map_size = self.grid_size * self.tile_size;
        if offset_position.x < 0.0 || offset_position.x >= map_size.x as f32 || offset_position.y < 0.0 || offset_position.y >= map_size.y as f32 {
            false
        } else {
            let coords = match collider {
                Collider::Rectangle(rect) => {
                    (uvec2(
                        ((rect.x - self.world_offset.x) / self.tile_size.x as f32 - (rect.w / self.tile_size.x as f32) / 2.0) as u32,
                        ((rect.y - self.world_offset.y) / self.tile_size.y as f32 - (rect.w / self.tile_size.y as f32) / 2.0) as u32,
                    ),
                     uvec2(
                         ((rect.x - self.world_offset.x) / self.tile_size.x as f32 + (rect.h / self.tile_size.x as f32) / 2.0) as u32,
                         ((rect.y - self.world_offset.y) / self.tile_size.y as f32 + (rect.h / self.tile_size.y as f32) / 2.0) as u32,
                     ))
                },
                Collider::Circle(circle) => {
                    (uvec2(
                        ((circle.x - self.world_offset.x) / self.tile_size.x as f32 - (circle.r / self.tile_size.x as f32)) as u32,
                        ((circle.y - self.world_offset.y) / self.tile_size.y as f32 - (circle.r / self.tile_size.y as f32)) as u32,
                    ),
                     uvec2(
                         ((circle.x - self.world_offset.x) / self.tile_size.x as f32 + (circle.r / self.tile_size.x as f32)) as u32,
                         ((circle.y - self.world_offset.y) / self.tile_size.y as f32 + (circle.r / self.tile_size.y as f32)) as u32,
                     ))
                }
            };
            if coords.0.x >= self.grid_size.x
                || coords.0.y >= self.grid_size.y
                || coords.1.x >= self.grid_size.x
                || coords.1.y >= self.grid_size.y {
                return false;
            }
            for x in coords.0.x..coords.1.x + 1 {
                for y in coords.0.y..coords.1.y + 1 {
                    for layer_id in layer_ids {
                        if self.get_tile_at_coords(uvec2(x, y), layer_id).is_some() {
                            return true;
                        }
                    }
                }
            }
            false
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
                    if self.get_tile_at_coords(uvec2(x, y), layer_id).is_some() {
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

#[derive(Clone)]
pub enum MapLayerKind {
    TileLayer,
    ObjectLayer,
}

#[derive(Clone)]
pub struct MapLayer {
    pub id: String,
    pub kind: MapLayerKind,
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
    pub prototype_id: Option<String>,
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
        assert_ne!(tile_id, 0, "Invalid tile id '{}' (tile id can not be zero)!", tile_id);
        assert!(
            tile_id >= self.first_tile_id && tile_id <= self.first_tile_id + self.grid_size.x * self.grid_size.y,
            "The specified tile id '{}' does not belong to this tileset!",
            tile_id,
        );
        let i = tile_id - self.first_tile_id;
        vec2(
            ((i % self.grid_size.x) * self.tile_size.x) as f32,
            ((i / self.grid_size.x) * self.tile_size.y) as f32,
        )
    }
}
