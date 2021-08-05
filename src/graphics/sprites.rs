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
use crate::graphics::Drawable;

#[derive(Clone)]
pub struct SpriteParams {
    pub offset: Vec2,
    pub texture_id: String,
    pub texture_color: Color,
    pub tile_size: Vec2,
    pub animations: Vec<Animation>,
    pub should_play: bool,
}

impl Default for SpriteParams {
    fn default() -> Self {
        SpriteParams {
            offset: vec2(-32.0, -32.0),
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
            should_play: false,
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
    animations: Vec<Animation>,
    sprite: AnimatedSprite,
}

impl SpriteAnimationPlayer {
    pub fn new(params: SpriteParams) -> Self {
        let sprite = AnimatedSprite::new(
            params.tile_size.x as u32,
            params.tile_size.y as u32,
            &params.animations,
            params.should_play,
        );

        SpriteAnimationPlayer {
            offset: params.offset,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            texture_id: params.texture_id.to_string(),
            texture_color: params.texture_color,
            tile_size: params.tile_size,
            animations: params.animations,
            sprite,
        }
    }

    #[allow(dead_code)]
    pub fn is_playing(&self) -> bool {
        self.sprite.playing
    }

    #[allow(dead_code)]
    pub fn to_sprite_params(&self) -> SpriteParams {
        SpriteParams {
            offset: self.offset,
            texture_id: self.texture_id.to_string(),
            texture_color: self.texture_color,
            tile_size: self.tile_size,
            animations: self.animations.to_vec(),
            should_play: self.sprite.playing,
        }
    }
}

impl Drawable for SpriteAnimationPlayer {
    fn draw(&mut self, position: Vec2, rotation: f32, flip_x: bool, flip_y: bool) {
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
