use crate::prelude::*;
use macroquad::prelude::draw_texture_ex;

#[derive(Debug, Copy, Clone)]
pub struct Texture {
    texture: Texture2D,
    height_map: Option<Texture2D>,
    normal_map: Option<Texture2D>,
}

impl Texture {
    pub fn new(
        texture: Texture2D,
        height_map: Option<Texture2D>,
        normal_map: Option<Texture2D>,
    ) -> Self {
        Texture {
            texture,
            height_map,
            normal_map,
        }
    }

    pub fn has_height_map(&self) -> bool {
        self.height_map.is_some()
    }

    pub fn has_normal_map(&self) -> bool {
        self.normal_map.is_some()
    }

    pub fn get(&self) -> Texture2D {
        self.texture
    }

    pub fn get_height_map(&self) -> Option<Texture2D> {
        self.normal_map
    }

    pub fn get_normal_map(&self) -> Option<Texture2D> {
        self.normal_map
    }

    pub fn draw(&self, position: Vec2, color: Option<Color>, params: DrawTextureParams) {
        draw_texture_ex(
            self.texture,
            position.x,
            position.y,
            color.unwrap_or(color::WHITE),
            params,
        )
    }
}

pub fn draw_texture(
    texture: &Texture,
    position: Vec2,
    color: Option<Color>,
    params: DrawTextureParams,
) {
    texture.draw(position, color, params)
}
