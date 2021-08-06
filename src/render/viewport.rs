use macroquad::prelude::*;

pub fn get_aspect_ratio() -> f32 {
    screen_width() / screen_height()
}

pub fn to_screen_space(coords: Vec2, viewport_pos: Vec2, scale: f32) -> Vec2 {
    (coords / scale) - viewport_pos
}

pub fn to_world_space(coords: Vec2, viewport_pos: Vec2, scale: f32) -> Vec2 {
    viewport_pos + (coords / scale)
}

#[derive(Copy, Clone)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub s: f32,
}

impl Viewport {
    pub fn to_screen_space(&self, coords: Vec2) -> Vec2 {
        to_screen_space(coords,vec2(self.x, self.y), self.s)
    }

    pub fn to_world_space(&self, coords: Vec2) -> Vec2 {
        to_world_space(coords,vec2(self.x, self.y), self.s)
    }

    pub fn get_mouse_world_coords(&self) -> Vec2 {
        let (x, y) = mouse_position();
        self.to_world_space(vec2(x, y))
    }
}