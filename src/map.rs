use macroquad::{
    prelude::*,
};

use macroquad_tiled as tiled;

use crate::{get_global, Resources};
use crate::physics::{Collider, beam_collision_check};

pub struct Map {
    tile_size: UVec2,
    pub tiled_map: tiled::Map,
}

impl Map {
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
            ],
            &[],
        ).unwrap();
        Map {
            tile_size,
            tiled_map,
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

    pub fn get_beam_collision_point(&self, origin: Vec2, end: Vec2, width: f32, include_barriers: bool) -> Vec2 {
        let coords = (
            uvec2(origin.x as u32 / self.tile_size.x, origin.y as u32 / self.tile_size.y),
            uvec2(end.x as u32 / self.tile_size.x, end.y as u32 / self.tile_size.y),
        );
        let ord_x = if coords.0.x > coords.1.x { (coords.1.x, coords.0.x) } else { (coords.0.x, coords.1.x) };
        let ord_y = if coords.0.y > coords.1.y { (coords.1.y, coords.0.y) } else { (coords.0.y, coords.1.y) };
        let mut collisions = Vec::new();
        for x in ord_x.0-1..ord_x.1+1 {
            for y in ord_y.0-1..ord_y.1+1 {
                let position = vec2((x * self.tile_size.x) as f32, (y * self.tile_size.y) as f32);
                if self.tiled_map.get_tile(Self::SOLIDS_LAYER, x, y).is_some() {
                    if beam_collision_check(position, origin, end, width) {
                        collisions.push(position);
                    }
                } else if include_barriers && self.tiled_map.get_tile(Self::BARRIERS_LAYER, x, y).is_some() {
                    if beam_collision_check(position, origin, end, width) {
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
            Self::BARRIERS_LAYER,
            Self::SOLIDS_LAYER,
        ] {
            if let Some(layer) = self.tiled_map.layers.get(layer_name) {
                self.tiled_map.draw_tiles(
                    layer_name,
                    Rect::new(
                        0.0,
                        0.0,
                        (self.tile_size.x * layer.width) as f32,
                        (self.tile_size.y * layer.height) as f32,
                    ),
                    None,
                );
            }
        }
    }
}
