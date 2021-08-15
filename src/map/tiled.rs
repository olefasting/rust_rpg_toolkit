use std::collections::HashMap;

use macroquad::{
    prelude::*,
};

use macroquad_tiled as tiled;
use crate::{get_global, Resources};
use crate::physics::{Collider, beam_collision_check};

#[derive(Clone)]
pub struct SpawnPoint {
    pub id: String,
    pub position: Vec2,
}

#[derive(Clone)]
pub struct MapItem {
    pub id: String,
    pub position: Vec2,
}

pub struct TiledMap {
    pub map_size: UVec2,
    pub tile_size: UVec2,
    pub items: HashMap<String, MapItem>,
    pub spawn_points: HashMap<String, SpawnPoint>,
    tiled_map: tiled::Map,
}

impl TiledMap {
    pub const GROUND_LAYER: &'static str = "ground";
    pub const SOLIDS_LAYER: &'static str = "solids";
    pub const BARRIERS_LAYER: &'static str = "barriers";
    pub const ITEMS_LAYER: &'static str = "items";
    pub const SPAWN_POINTS_LAYER: &'static str = "spawn_points";

    pub const PLAYER_SPAWN_POINT_NAME: &'static str = "player_spawn";

    pub async fn new(tile_size: UVec2, path: &str) -> Self {
        let resources = get_global::<Resources>();
        let tiled_map_json = load_string(path).await.unwrap();
        let tiled_map = tiled::load_map(
            &tiled_map_json,
            &[
                ("../textures/neo_zero_tiles.png", resources.get_texture(Resources::GROUND_TILES_TEXTURE_ID).clone()),
                ("../textures/neo_zero_props.png", resources.get_texture(Resources::PROPS_TEXTURE_ID).clone()),
                ("../textures/items.png", resources.get_texture(Resources::ITEMS_TEXTURE_ID).clone()),
            ],
            &[],
        ).expect(&format!("Error when parsing Tiled map '{}'", path));

        let ground_layer = tiled_map.layers.get(Self::GROUND_LAYER)
            .expect(&format!("No ground layer defined in Tiled map and ground layer ('{}') size is used to define map size...", Self::GROUND_LAYER));
        let map_size = uvec2(ground_layer.width, ground_layer.height);

        let mut items = HashMap::new();
        for item in tiled_map.layers[TiledMap::ITEMS_LAYER].objects.clone() {
            items.insert(item.name.clone(), MapItem {
                id: item.name,
                position: vec2(
                    item.world_x,
                    item.world_y,
                ),
            });
        }

        let mut spawn_points = HashMap::new();
        for spawn_point in tiled_map.layers[TiledMap::SPAWN_POINTS_LAYER].objects.clone() {
            spawn_points.insert(spawn_point.name.clone(), SpawnPoint {
                id: spawn_point.name,
                position: vec2(
                    spawn_point.world_x,
                    spawn_point.world_y,
                ),
            });
        }

        TiledMap {
            map_size,
            tile_size,
            tiled_map,
            items,
            spawn_points,
        }
    }

    pub fn solid_at(&self, position: Vec2, include_barriers: bool) -> bool {
        let coords = uvec2(
            position.x as u32 / self.tile_size.x,
            position.y as u32 / self.tile_size.y,
        );
        let barriers = if include_barriers {
            self.tiled_map.get_tile(Self::BARRIERS_LAYER, coords.x, coords.y).is_some()
        } else {
            false
        };
        barriers || self.tiled_map.get_tile(Self::SOLIDS_LAYER, coords.x, coords.y).is_some()
    }

    pub fn solid_at_collider(&self, collider: Collider, include_barriers: bool) -> bool {
        let coords = match collider {
            Collider::Rectangle(rect) => {
                (uvec2(
                    (rect.x / self.tile_size.x as f32 - (rect.w / self.tile_size.x as f32) / 2.0) as u32,
                    (rect.y / self.tile_size.y as f32 - (rect.w / self.tile_size.y as f32) / 2.0) as u32,
                ),
                 uvec2(
                     (rect.x / self.tile_size.x as f32 + (rect.h / self.tile_size.x as f32) / 2.0) as u32,
                     (rect.y / self.tile_size.y as f32 + (rect.h / self.tile_size.y as f32) / 2.0) as u32,
                 ))
            },
            Collider::Circle(circle) => {
                (uvec2(
                    (circle.x / self.tile_size.x as f32 - (circle.r / self.tile_size.x as f32)) as u32,
                    (circle.y / self.tile_size.y as f32 - (circle.r / self.tile_size.y as f32)) as u32,
                ),
                 uvec2(
                     (circle.x / self.tile_size.x as f32 + (circle.r / self.tile_size.x as f32)) as u32,
                     (circle.y / self.tile_size.y as f32 + (circle.r / self.tile_size.y as f32)) as u32,
                 ))
            }
        };
        for x in coords.0.x..coords.1.x+1 {
            for y in coords.0.y..coords.1.y+1 {
                if self.tiled_map.get_tile(Self::SOLIDS_LAYER, x, y).is_some() {
                    return true;
                }
                if include_barriers && self.tiled_map.get_tile(Self::BARRIERS_LAYER, x, y).is_some() {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_beam_collision_point(&self, origin: Vec2, end: Vec2, width: f32, tolerance: f32, include_barriers: bool) -> Vec2 {
        let coords = (
            uvec2(origin.x as u32 / self.tile_size.x, origin.y as u32 / self.tile_size.y),
            uvec2(end.x as u32 / self.tile_size.x, end.y as u32 / self.tile_size.y),
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
                if self.tiled_map.get_tile(Self::SOLIDS_LAYER, x, y).is_some() {
                    if beam_collision_check(position, origin, end, width, tolerance) {
                        collisions.push(position);
                    }
                } else if include_barriers && self.tiled_map.get_tile(Self::BARRIERS_LAYER, x, y).is_some() {
                    if beam_collision_check(position, origin, end, width, tolerance) {
                        collisions.push(position);
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

    pub fn draw(&self) {
        for layer_name in [
            Self::GROUND_LAYER,
            Self::SOLIDS_LAYER,
            Self::BARRIERS_LAYER,
        ] {
            self.tiled_map.draw_tiles(
                layer_name,
                Rect::new(
                    0.0,
                    0.0,
                    (self.tile_size.x * self.map_size.x) as f32,
                    (self.tile_size.y * self.map_size.y) as f32,
                ),
                None,
            );
        }
    }

    pub fn try_get_item(&self, id: &str) -> Option<MapItem> {
        match self.items.get(id) {
            Some(item) => Some(item.clone()),
            None => None,
        }
    }

    pub fn get_item(&self, id: &str) -> MapItem {
        self.try_get_item(id).unwrap()
    }

    pub fn try_get_spawn_point(&self, id: &str) -> Option<SpawnPoint> {
        match self.spawn_points.get(id) {
            Some(spawn_point) => Some(spawn_point.clone()),
            None => None,
        }
    }

    pub fn get_spawn_point(&self, id: &str) -> SpawnPoint {
        self.try_get_spawn_point(id).unwrap()
    }
}
