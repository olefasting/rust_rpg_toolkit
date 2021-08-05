use macroquad::prelude::*;

#[allow(dead_code)]
pub fn get_aspect_ratio() -> f32 {
    screen_width() / screen_height()
}

#[allow(dead_code)]
pub fn to_screen_space(coords: Vec2, viewport_pos: Vec2, scale: f32) -> Vec2 {
    (coords / scale) - viewport_pos
}

#[allow(dead_code)]
pub fn to_world_space(coords: Vec2, viewport_pos: Vec2, scale: f32) -> Vec2 {
    viewport_pos + (coords / scale)
}

pub trait Drawable {
    fn draw(&mut self, position: Vec2, rotation: f32, flip_x: bool, flip_y: bool);
}
