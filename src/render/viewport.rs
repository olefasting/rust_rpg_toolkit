use macroquad::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Viewport {
    pub position: Vec2,
    pub size: Vec2,
    pub scale: f32,
}

impl Viewport {
    pub const FRUSTUM_PADDING: f32 = 25.0;

    pub fn to_screen_space(&self, position: Vec2) -> Vec2 {
        (position - self.position) * self.scale
    }

    pub fn to_world_space(&self, position: Vec2) -> Vec2 {
        self.position + (position / self.scale)
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
        )
    }

    pub fn get_center(&self) -> Vec2 {
        self.position + self.size / 2.0
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
