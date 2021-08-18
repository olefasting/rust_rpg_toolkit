use serde::{
    Serialize,
    Deserialize,
};

pub use crate::math::{
    URect,
    Circle,
};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 {
            x,
            y,
        }
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2 {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl From<macroquad::prelude::Vec2> for Vec2 {
    fn from(other: macroquad::prelude::Vec2) -> Self {
        Vec2 {
            x: other.x,
            y: other.y,
        }
    }
}

impl From<Vec2> for macroquad::prelude::Vec2 {
    fn from(other: Vec2) -> Self {
        macroquad::prelude::vec2(
            other.x,
            other.y,
        )
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

impl UVec2 {
    pub fn new(x: u32, y: u32) -> Self {
        UVec2 {
            x,
            y,
        }
    }
}

impl Default for UVec2 {
    fn default() -> Self {
        UVec2 {
            x: 0,
            y: 0,
        }
    }
}

impl From<macroquad::prelude::UVec2> for UVec2 {
    fn from(other: macroquad::prelude::UVec2) -> Self {
        UVec2 {
            x: other.x,
            y: other.y,
        }
    }
}

impl From<UVec2> for macroquad::prelude::UVec2 {
    fn from(other: UVec2) -> Self {
        macroquad::prelude::uvec2(
            other.x,
            other.y,
        )
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl From<macroquad::math::Rect> for Rect {
    fn from(other: macroquad::math::Rect) -> Self {
        Rect {
            x: other.x,
            y: other.y,
            w: other.w,
            h: other.h,
        }
    }
}

impl From<Rect> for macroquad::math::Rect {
    fn from(other: Rect) -> Self {
        macroquad::math::Rect {
            x: other.x,
            y: other.y,
            w: other.w,
            h: other.h,
        }
    }
}
