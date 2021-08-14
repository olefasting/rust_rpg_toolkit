use macroquad::{
    prelude::*,
};

use macroquad_tiled as tiled;

use crate::{get_global, Resources};
use crate::physics::Collider;

pub struct Map {
    tile_size: UVec2,
    tiled_map: tiled::Map,
}

impl Map {
    pub const GROUND_LAYER: &'static str = "ground";
    pub const SOLIDS_LAYER: &'static str = "solids";
    pub const BARRIERS_LAYER: &'static str = "barriers";

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

    pub fn draw(&self) {
        if let Some(layer) = self.tiled_map.layers.get(Self::GROUND_LAYER) {
            self.tiled_map.draw_tiles(
                Self::GROUND_LAYER,
                Rect::new(
                    0.0,
                    0.0,
                    (self.tile_size.x * layer.width) as f32,
                    (self.tile_size.y * layer.height) as f32,
                ),
                None,
            );
        }
        if let Some(layer) = self.tiled_map.layers.get(Self::SOLIDS_LAYER) {
            self.tiled_map.draw_tiles(
                Self::SOLIDS_LAYER,
                Rect::new(
                    0.0,
                    0.0,
                    (self.tile_size.x * layer.width) as f32,
                    (self.tile_size.y * layer.height) as f32,
                ),
                None,
            );
        }
        if let Some(layer) = self.tiled_map.layers.get(Self::BARRIERS_LAYER) {
            self.tiled_map.draw_tiles(
                Self::BARRIERS_LAYER,
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
