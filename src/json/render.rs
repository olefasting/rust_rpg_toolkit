use macroquad::{
    prelude::animation::Animation,
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

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
