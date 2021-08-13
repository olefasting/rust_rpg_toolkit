use macroquad::prelude::*;
use std::collections::HashMap;

pub struct Resources {
    textures: HashMap<String, Texture2D>,
}

impl Resources {
    pub const WHITE_TEXTURE_ID: &'static str = "__WHITE_TEXTURE__";

    pub const CHARACTERS_TEXTURE_ID: &'static str = "characters";
    pub const PROPS_TEXTURE_ID: &'static str = "props";
    pub const GROUND_TILES_TEXTURE_ID: &'static str = "ground_tiles";
    pub const ITEMS_TEXTURE_ID: &'static str = "items";

    pub async fn new() -> Result<Resources, FileError> {
        let mut textures= HashMap::new();

        let white_texture = load_texture("assets/sprites/white_texture.png").await?;
        white_texture.set_filter(FilterMode::Nearest);
        textures.insert(Self::WHITE_TEXTURE_ID.to_string(), white_texture);

        let characters = load_texture("assets/sprites/neo_zero_char_01.png").await?;
        characters.set_filter(FilterMode::Nearest);
        textures.insert(Self::CHARACTERS_TEXTURE_ID.to_string(), characters);

        let props = load_texture("assets/sprites/neo_zero_props_and_items_01.png").await?;
        props.set_filter(FilterMode::Nearest);
        textures.insert(Self::PROPS_TEXTURE_ID.to_string(), props);

        let ground_tiles = load_texture("assets/sprites/neo_zero_tiles_and_buildings_01.png").await?;
        ground_tiles.set_filter(FilterMode::Nearest);
        textures.insert(Self::GROUND_TILES_TEXTURE_ID.to_string(), ground_tiles);

        // https://rafazcruz.itch.io/cyberpunk-top-down-game-asset-pack
        // cyberpunk_city_pack_1.png
        // cyberpunk_city_pack_2.png

        // https://jeresikstus.itch.io/cyberpunk-items-16x16
        let items = load_texture("assets/sprites/items.png").await?;
        items.set_filter(FilterMode::Nearest);
        textures.insert(Self::ITEMS_TEXTURE_ID.to_string(), items);

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
