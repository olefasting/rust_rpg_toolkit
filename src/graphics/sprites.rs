use macroquad::{
    experimental::{
        animation::{
            AnimatedSprite,
            Animation,
        },
        collections::storage,
    },
    color,
    prelude::*,
};

use crate::{
    Resources,
};

#[derive(Clone)]
pub struct SpriteParams {
    pub offset: Vec2,
    pub texture_id: String,
    pub texture_color: Color,
    pub tile_size: Vec2,
    pub animations: Vec<Animation>,
    pub playing: bool,
}

impl Default for SpriteParams {
    fn default() -> Self {
        SpriteParams {
            offset: Vec2::ZERO,
            texture_id: Resources::WHITE_TEXTURE_ID.to_string(),
            texture_color: color::WHITE,
            tile_size: vec2(64.0, 64.0),
            animations: vec!(
                Animation {
                    name: "idle".to_string(),
                    row: 0,
                    frames: 1,
                    fps: 8
                },
            ),
            playing: false,
        }
    }
}

#[derive(Clone)]
pub struct SpriteAnimationPlayer {
    pub offset: Vec2,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    texture_id: String,
    texture_color: Color,
    tile_size: Vec2,
    sprite: AnimatedSprite,
    params: SpriteParams,
}

impl SpriteAnimationPlayer {
    pub fn new(params: SpriteParams) -> Self {
        SpriteAnimationPlayer {
            offset: params.offset,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            texture_id: params.texture_id.to_string(),
            texture_color: params.texture_color,
            tile_size: params.tile_size,
            sprite: AnimatedSprite::new(
                params.tile_size.x as u32,
                params.tile_size.y as u32,
                &params.animations,
                params.playing,
            ),
            params: params.clone(),
        }
    }

    pub fn draw(&mut self, position: Vec2, rotation: f32, flip_x: bool, flip_y: bool) {
        self.sprite.update();
        let resources = storage::get::<Resources>();
        draw_texture_ex(
            resources.get_texture(&self.texture_id),
            position.x + self.offset.x,
            position.y + self.offset.y,
            self.texture_color,
            DrawTextureParams {
                source: Some(self.sprite.frame().source_rect),
                dest_size: Some(self.sprite.frame().dest_size),
                flip_x: self.flip_x && !flip_x,
                flip_y: self.flip_y && !flip_y,
                rotation: self.rotation + rotation,
                ..Default::default()
            },
        );
    }
}
