use macroquad::prelude::*;

pub struct Resources {
    pub white_texture: Texture2D,
    pub ground_tiles: Texture2D,
}

impl Resources {
    pub const WHITE_TEXTURE_ID: &'static str = "__WHITE_TEXTURE__";

    pub async fn new() -> Result<Resources, FileError> {
        let white_texture = load_texture("assets/white_texture.png").await?;
        white_texture.set_filter(FilterMode::Nearest);

        let ground_tiles = load_texture("assets/tileset/TX Tileset Grass.png").await?;
        ground_tiles.set_filter(FilterMode::Nearest);

        Ok(Resources {
            white_texture,
            ground_tiles,
        })
    }

    pub fn get_texture_by_id(&self, id: &str) -> Option<Texture2D> {
        match id {
            Self::WHITE_TEXTURE_ID => Some(self.white_texture),
            _ => None,
        }
    }
}
