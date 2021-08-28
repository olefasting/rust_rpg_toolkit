use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprite {
    #[serde(default, with = "json::def_vec2")]
    pub offset: Vec2,
    #[serde(default)]
    pub rotation: f32,
    #[serde(default)]
    pub flip_x: bool,
    #[serde(default)]
    pub flip_y: bool,
    #[serde(with = "json::def_uvec2")]
    pub tile_size: UVec2,
    pub texture_id: String,
    #[serde(with = "json::def_uvec2")]
    pub texture_coords: UVec2,
}

impl Sprite {
    pub fn draw(&self, position: Vec2, rotation: f32) {
        let resources = storage::get::<Resources>();
        draw_texture_ex(
            resources.textures.get(&self.texture_id).cloned().unwrap(),
            position.x + self.offset.x,
            position.y + self.offset.y,
            color::WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    (self.texture_coords.x * self.tile_size.x) as f32,
                    (self.texture_coords.y * self.tile_size.y) as f32,
                    self.tile_size.x as f32,
                    self.tile_size.y as f32,
                )),
                dest_size: Some(vec2(
                    self.tile_size.x as f32,
                    self.tile_size.y as f32,
                )),
                flip_x: self.flip_x,
                flip_y: self.flip_y,
                rotation: self.rotation + rotation,
                ..Default::default()
            },
        );
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite {
            offset: Vec2::ZERO,
            rotation: 0.0,
            texture_id: Resources::WHITE_TEXTURE_ID.to_string(),
            tile_size: uvec2(16, 16),
            texture_coords: uvec2(0, 0),
            flip_x: false,
            flip_y: false,
        }
    }
}
