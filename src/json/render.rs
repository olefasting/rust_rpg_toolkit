use macroquad::{
    prelude::animation::Animation,
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::json::{
    Vec2Def,
    UVec2Def,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct SpriteAnimationParams {
    pub offset: Vec2Def,
    pub texture_id: String,
    pub tile_size: Vec2Def,
    #[serde(with = "super::vec_animation")]
    pub animations: Vec<Animation>,
    pub should_play: Option<bool>,
}

impl From<crate::render::SpriteAnimationParams> for SpriteAnimationParams {
    fn from(other: crate::render::SpriteAnimationParams) -> Self {
        SpriteAnimationParams {
            offset: Vec2Def::from(other.offset),
            texture_id: other.texture_id,
            tile_size: Vec2Def::from(other.tile_size),
            animations: other.animations,
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
    pub offset: Vec2Def,
    pub rotation: Option<f32>,
    pub flip_x: Option<bool>,
    pub flip_y: Option<bool>,
    pub tile_size: UVec2Def,
    pub texture_id: String,
    pub texture_coords: UVec2Def,
}

impl From<crate::render::Sprite> for Sprite {
    fn from(other: crate::render::Sprite) -> Self {
        Sprite {
            offset: Vec2Def::from(other.offset),
            rotation: if other.rotation == 0.0 { None } else { Some(other.rotation) },
            flip_x:  if other.flip_x { Some(other.flip_x) } else { None },
            flip_y:  if other.flip_y { Some(other.flip_y) } else { None },
            tile_size: UVec2Def::from(other.tile_size),
            texture_id: other.texture_id,
            texture_coords: UVec2Def::from(other.texture_coords),
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
#[serde(remote = "Color")]
pub struct ColorDef {
    #[serde(alias = "red")]
    pub r: f32,
    #[serde(alias = "green")]
    pub g: f32,
    #[serde(alias = "blue")]
    pub b: f32,
    #[serde(alias = "alpha")]
    pub a: f32,
}

impl From<Color> for ColorDef {
    fn from(other: Color) -> Self {
        ColorDef {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }
}

impl From<ColorDef> for Color {
    fn from(other: ColorDef) -> Self {
        Color {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(remote = "Animation")]
pub struct AnimationDef {
    pub name: String,
    pub row: u32,
    pub frames: u32,
    pub fps: u32,
}

impl From<&Animation> for AnimationDef {
    fn from(other: &Animation) -> Self {
        AnimationDef {
            name: other.name.clone(),
            row: other.row,
            frames: other.frames,
            fps: other.fps,
        }
    }
}

impl From<AnimationDef> for Animation {
    fn from(other: AnimationDef) -> Self {
        Animation {
            name: other.name,
            row: other.row,
            frames: other.frames,
            fps: other.fps,
        }
    }
}

pub mod vec_animation {
    use super::{Animation, AnimationDef};
    use serde::{Serialize, Serializer, Deserialize, Deserializer};

    pub fn serialize<S>(value: &Vec<Animation>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "AnimationDef")] &'a Animation);

        value
            .into_iter()
            .map(Helper)
            .collect::<Vec<Helper>>()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Animation>, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "AnimationDef")] Animation);

        let helper = Vec::deserialize(deserializer)?;
        Ok(helper.iter().map(|Helper(external)| external.clone()).collect())
    }
}
