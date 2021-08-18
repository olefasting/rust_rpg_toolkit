use macroquad::prelude::*;

use crate::Camera;

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
    pub width: f32,
    pub height: f32,
    pub scale: f32,
}

impl Viewport {
    pub const FRUSTUM_PADDING: f32 = 25.0;

    pub fn to_screen(&self, coords: Vec2) -> Vec2 {
        to_screen_space(coords,vec2(self.x, self.y), self.scale)
    }

    pub fn to_world(&self, coords: Vec2) -> Vec2 {
        to_world_space(coords,vec2(self.x, self.y), self.scale)
    }

    pub fn get_mouse_in_world(&self) -> Vec2 {
        let (x, y) = mouse_position();
        self.to_world(vec2(x, y))
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(
            self.x,
            self.y,
            self.width,
            self.height,
        )
    }

    pub fn get_frustum(&self) -> Rect {
        let padding = Self::FRUSTUM_PADDING;
        let mut frustum = self.get_rect();
        frustum.x -= padding;
        frustum.y -= padding;
        frustum.w += padding * 2.0;
        frustum.h += padding * 2.0;
        frustum
    }
}
