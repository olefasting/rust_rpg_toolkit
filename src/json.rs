use serde::{
    Serialize,
    Deserialize,
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

    pub fn from(other: macroquad::prelude::Vec2) -> Self {
        Vec2 {
            x: other.x,
            y: other.y,
        }
    }

    pub fn to_macroquad(&self) -> macroquad::prelude::Vec2 {
        macroquad::prelude::vec2(
            self.x,
            self.y,
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

    pub fn from(other: macroquad::prelude::UVec2) -> Self {
        UVec2 {
            x: other.x,
            y: other.y,
        }
    }

    pub fn to_macroquad(&self) -> macroquad::prelude::UVec2 {
        macroquad::prelude::uvec2(
            self.x,
            self.y,
        )
    }
}
