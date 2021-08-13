use macroquad::{
    experimental::{
        animation::{
            AnimatedSprite,
            Animation,
        },
    },
    color,
    prelude::*,
};

use crate::{Resources, get_global};

#[derive(Clone)]
pub struct SpriteAnimationParams {
    pub offset: Vec2,
    pub texture_id: String,
    pub texture_color: Color,
    pub tile_size: Vec2,
    pub animations: Vec<Animation>,
    pub should_play: bool,
}

impl Default for SpriteAnimationParams {
    fn default() -> Self {
        SpriteAnimationParams {
            offset: vec2(-8.0, -8.0),
            texture_id: Resources::WHITE_TEXTURE_ID.to_string(),
            texture_color: color::WHITE,
            tile_size: vec2(16.0, 16.0),
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
    animated_sprite: AnimatedSprite,
}

impl SpriteAnimationPlayer {
    pub fn new(params: SpriteAnimationParams) -> Self {
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
            animated_sprite: sprite,
        }
    }

    pub fn is_playing(&self) -> bool {
        self.animated_sprite.playing
    }

    pub fn set_animation(&mut self, id: usize) {
        self.animated_sprite.set_animation(id);
    }

    pub fn start_animation(&mut self, id: usize) {
        self.set_animation(id);
        self.play();
    }

    pub fn restart_animation(&mut self) {
        self.animated_sprite.set_frame(0);
    }

    pub fn set_frame(&mut self, frame: u32) {
        self.animated_sprite.set_frame(frame);
    }

    pub fn play(&mut self) {
        self.animated_sprite.playing = true;
    }

    pub fn stop(&mut self) {
        self.animated_sprite.playing = false;
    }

    pub fn to_sprite_params(&self) -> SpriteAnimationParams {
        SpriteAnimationParams {
            offset: self.offset,
            texture_id: self.texture_id.to_string(),
            texture_color: self.texture_color,
            tile_size: self.tile_size,
            animations: self.animations.to_vec(),
            should_play: self.animated_sprite.playing,
        }
    }

    pub fn draw(&mut self, position: Vec2, rotation: f32) {
        self.animated_sprite.update();
        let resources = get_global::<Resources>();
        draw_texture_ex(
            resources.get_texture(&self.texture_id).clone(),
            position.x + self.offset.x,
            position.y + self.offset.y,
            self.texture_color,
            DrawTextureParams {
                source: Some(self.animated_sprite.frame().source_rect),
                dest_size: Some(self.animated_sprite.frame().dest_size),
                flip_x: self.flip_x,
                flip_y: self.flip_y,
                rotation: self.rotation + rotation,
                ..Default::default()
            },
        );
    }
}
