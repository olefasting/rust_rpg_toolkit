use serde::{
    Serialize,
    Deserialize,
};

use crate::json::{
    Vec2,
    UVec2,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct SpriteAnimationParams {
    pub offset: Vec2,
    pub texture_id: String,
    pub tile_size: Vec2,
    pub animations: Vec<Animation>,
    pub should_play: Option<bool>,
}

impl From<crate::render::SpriteAnimationParams> for SpriteAnimationParams {
    fn from(other: crate::render::SpriteAnimationParams) -> Self {
        SpriteAnimationParams {
            offset: Vec2::from(other.offset),
            texture_id: other.texture_id,
            tile_size: Vec2::from(other.tile_size),
            animations: other.animations.into_iter().map(|anim| Animation::from(anim)).collect(),
            should_play: if other.should_play { Some(true) } else { None },
        }
    }
}

impl From<SpriteAnimationParams> for crate::render::SpriteAnimationParams {
    fn from(other: SpriteAnimationParams) -> Self {
        crate::render::SpriteAnimationParams {
            offset: macroquad::prelude::Vec2::from(other.offset),
            texture_id: other.texture_id,
            tile_size: macroquad::prelude::Vec2::from(other.tile_size),
            animations: other.animations.into_iter().map(|anim| macroquad::prelude::animation::Animation::from(anim)).collect(),
            should_play: other.should_play.unwrap_or_default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub offset: Vec2,
    pub rotation: Option<f32>,
    pub flip_x: Option<bool>,
    pub flip_y: Option<bool>,
    pub tile_size: UVec2,
    pub texture_id: String,
    pub texture_coords: UVec2,
}

impl From<crate::render::Sprite> for Sprite {
    fn from(other: crate::render::Sprite) -> Self {
        Sprite {
            offset: Vec2::from(other.offset),
            rotation: if other.rotation == 0.0 { None } else { Some(other.rotation) },
            flip_x:  if other.flip_x { Some(other.flip_x) } else { None },
            flip_y:  if other.flip_y { Some(other.flip_y) } else { None },
            tile_size: UVec2::from(other.tile_size),
            texture_id: other.texture_id,
            texture_coords: UVec2::from(other.texture_coords),
        }
    }
}

impl From<Sprite> for crate::render::Sprite {
    fn from(other: Sprite) -> Self {
        crate::render::Sprite {
            offset: macroquad::prelude::Vec2::from(other.offset),
            rotation: other.rotation.unwrap_or_default(),
            flip_x: other.flip_x.unwrap_or_default(),
            flip_y: other.flip_y.unwrap_or_default(),
            tile_size: macroquad::prelude::UVec2::from(other.tile_size),
            texture_id: other.texture_id,
            texture_coords: macroquad::prelude::UVec2::from(other.texture_coords),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<macroquad::prelude::Color> for Color {
    fn from(other: macroquad::color::Color) -> Self {
        Color {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }
}

impl From<Color> for macroquad::prelude::Color {
    fn from(other: Color) -> Self {
        macroquad::color::Color {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub row: u32,
    pub frames: u32,
    pub fps: u32,
}

impl From<macroquad::prelude::animation::Animation> for Animation {
    fn from(other: macroquad::prelude::animation::Animation) -> Self {
        Animation {
            name: other.name,
            row: other.row,
            frames: other.frames,
            fps: other.fps,
        }
    }
}

impl From<Animation> for macroquad::prelude::animation::Animation {
    fn from(other: Animation) -> Self {
        macroquad::prelude::animation::Animation {
            name: other.name,
            row: other.row,
            frames: other.frames,
            fps: other.fps,
        }
    }
}
