use macroquad::{
    color,
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize
};

use crate::{
    get_global,
    Resources,
    json,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct SpriteParams {
    pub offset: json::Vec2,
    pub texture_id: String,
    pub tile_size: json::UVec2,
    pub texture_coords: json::UVec2,
}

impl Default for SpriteParams {
    fn default() -> Self {
        SpriteParams {
            offset: json::Vec2::from(Vec2::ZERO),
            texture_id: Resources::WHITE_TEXTURE_ID.to_string(),
            tile_size: json::UVec2::new(16, 16),
            texture_coords: json::UVec2::new(0, 0),
        }
    }
}

#[derive(Clone)]
pub struct Sprite {
    pub offset: Vec2,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    tile_size: UVec2,
    texture_id: String,
    texture_coords: UVec2,
}

impl Sprite {
    pub fn new(params: SpriteParams) -> Self {
        Sprite {
            offset: Vec2::from(params.offset),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            tile_size: UVec2::from(params.tile_size),
            texture_id: params.texture_id,
            texture_coords: UVec2::from(params.texture_coords),
        }
    }

    pub fn draw(&self, position: Vec2, rotation: f32) {
        let resources = get_global::<Resources>();
        draw_texture_ex(
            resources.textures.get(&self.texture_id).cloned().unwrap(),
            position.x + self.offset.x,
            position.y + self.offset.y,
            color::WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    (self.texture_coords.x * self.tile_size.x) as f32,
                    (self.texture_coords.y * self.tile_size.y) as f32,
                    self.tile_size.x as f32,
                    self.tile_size.y as f32,
                )),
                dest_size: Some(vec2(
                    self.tile_size.x as f32,
                    self.tile_size.y as f32,
                )),
                flip_x: self.flip_x,
                flip_y: self.flip_y,
                rotation: self.rotation + rotation,
                ..Default::default()
            },
        );
    }
}
