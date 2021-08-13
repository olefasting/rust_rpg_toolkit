use std::{
    collections::HashMap,
    fs,
};

use serde::{
    Deserialize,
    Serialize,
};

use macroquad::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
struct TextureData {
    pub id: String,
    pub filename: String,
    pub filter_mode: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct ResourcesData {
    pub textures: Vec<TextureData>,
}

pub struct Resources {
    textures: HashMap<String, Texture2D>,
}

impl Resources {
    const RESOURCES_FILE_PATH: &'static str = "assets/resources.json";
    const TEXTURES_FOLDER_PATH: &'static str = "assets/textures";

    const LINEAR_FILTER_MODE: &'static str = "Linear";
    const NEAREST_FILTER_MODE: &'static str = "Nearest";

    pub const WHITE_TEXTURE_ID: &'static str = "__WHITE_TEXTURE__";

    pub const CHARACTERS_TEXTURE_ID: &'static str = "characters";
    pub const PROPS_TEXTURE_ID: &'static str = "props";
    pub const GROUND_TILES_TEXTURE_ID: &'static str = "ground_tiles";
    pub const ITEMS_TEXTURE_ID: &'static str = "items";

    pub async fn new() -> Result<Resources, FileError> {
        let mut textures= HashMap::new();

        let white_texture = load_texture("assets/textures/white_texture.png").await?;
        white_texture.set_filter(FilterMode::Nearest);
        textures.insert(Self::WHITE_TEXTURE_ID.to_string(), white_texture);

        let json = fs::read_to_string(Self::RESOURCES_FILE_PATH)
            .expect(&format!("Unable to find resources file '{}'", Self::RESOURCES_FILE_PATH));
        let resources: ResourcesData = serde_json::from_str(&json)
            .expect(&format!("Error when parsing resource file '{}'", Self::RESOURCES_FILE_PATH));

        for texture_data in &resources.textures {
            let texture = load_texture(  &format!("{}/{}", Self::TEXTURES_FOLDER_PATH, &texture_data.filename)).await?;
            if texture_data.filter_mode == Self::LINEAR_FILTER_MODE.to_string() {
                texture.set_filter(FilterMode::Linear)
            } else if texture_data.filter_mode == Self::NEAREST_FILTER_MODE.to_string() {
                texture.set_filter(FilterMode::Nearest);
            } else {
                assert!(false, "Invalid filter mode '{}'", texture_data.filter_mode);
            }
            textures.insert(texture_data.id.clone(), texture);
        }

        // https://rafazcruz.itch.io/cyberpunk-top-down-game-asset-pack
        // cyberpunk_city_pack_1.png
        // cyberpunk_city_pack_2.png
        // https://jeresikstus.itch.io/cyberpunk-items-16x16

        Ok(Resources {
            textures,
        })
    }

    pub fn get_texture(&self, id: &str) -> &Texture2D {
        self.textures.get(id).unwrap()
    }

    pub fn try_get_texture(&self, id: &str) -> Option<&Texture2D> {
        self.textures.get(id)
    }
}
