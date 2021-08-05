use macroquad::prelude::*;
use std::collections::HashMap;

pub struct Resources {
    textures: HashMap<String, Texture2D>,
}

impl Resources {
    pub const WHITE_TEXTURE_ID: &'static str = "__WHITE_TEXTURE__";

    pub async fn new() -> Result<Resources, FileError> {
        let white_texture = load_texture("assets/white_texture.png").await?;
        white_texture.set_filter(FilterMode::Nearest);

        let mut textures = HashMap::new();
        textures.insert(Self::WHITE_TEXTURE_ID.to_string(), white_texture);

        Ok(Resources {
            textures,
        })
    }

    pub fn get_texture(&self, id: &str) -> Texture2D {
        self.textures.get(id).cloned().unwrap()
    }

    pub fn get_texture_ref(&self, id: &str) -> &Texture2D {
        self.textures.get(id).unwrap()
    }
}
