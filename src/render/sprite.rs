use macroquad::{
    color,
    prelude::*,
};

use crate::{get_global, Resources};

#[derive(Clone)]
pub struct SpriteParams {
    pub offset: Vec2,
    pub texture_id: String,
    pub texture_color: Color,
    pub tile_size: Vec2,
    pub texture_coords: UVec2,
}

impl Default for SpriteParams {
    fn default() -> Self {
        SpriteParams {
            offset: Vec2::ZERO,
            texture_id: Resources::WHITE_TEXTURE_ID.to_string(),
            texture_color: color::WHITE,
            tile_size: vec2(16.0, 16.0),
            texture_coords: UVec2::ZERO,
        }
    }
}

#[derive(Clone)]
pub struct Sprite {
    pub offset: Vec2,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    tile_size: Vec2,
    texture_id: String,
    texture_coords: UVec2,
    texture_color: Color,
}

impl Sprite {
    pub fn new(params: SpriteParams) -> Self {
        Sprite {
            offset: params.offset,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            tile_size: params.tile_size,
            texture_id: params.texture_id,
            texture_coords: params.texture_coords,
            texture_color: params.texture_color,
        }
    }

    pub fn draw(&self, position: Vec2, rotation: f32) {
        let resources = get_global::<Resources>();
        draw_texture_ex(
            resources.get_texture(&self.texture_id).clone(),
            position.x + self.offset.x,
            position.y + self.offset.y,
            self.texture_color,
            DrawTextureParams {
                source: Some(Rect::new(
                    self.texture_coords.x as f32 * self.tile_size.x,
                    self.texture_coords.y as f32 * self.tile_size.y,
                    self.tile_size.x,
                    self.tile_size.y,
                )),
                dest_size: Some(self.tile_size),
                flip_x: self.flip_x,
                flip_y: self.flip_y,
                rotation: self.rotation + rotation,
                ..Default::default()
            },
        );
    }
}
