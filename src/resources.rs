use macroquad::prelude::*;

pub struct Resources {
    pub white_texture: Texture2D,
    pub characters: Texture2D,
    pub props: Texture2D,
    pub ground_tiles: Texture2D,
}

impl Resources {
    pub const WHITE_TEXTURE_ID: &'static str = "__WHITE_TEXTURE__";

    pub const CHARACTERS_TEXTURE_ID: &'static str = "characters";
    pub const PROPS_TEXTURE_ID: &'static str = "props";
    pub const GROUND_TILES_TEXTURE_ID: &'static str = "ground_tiles";

    pub async fn new() -> Result<Resources, FileError> {
        let white_texture = load_texture("assets/sprites/white_texture.png").await?;
        white_texture.set_filter(FilterMode::Nearest);

        let characters = load_texture("assets/sprites/neo_zero_char_01.png").await?;
        characters.set_filter(FilterMode::Nearest);

        let props = load_texture("assets/sprites/neo_zero_props_and_items_01.png").await?;
        props.set_filter(FilterMode::Nearest);

        let ground_tiles = load_texture("assets/sprites/neo_zero_tiles_and_buildings_01.png").await?;
        ground_tiles.set_filter(FilterMode::Nearest);

        Ok(Resources {
            white_texture,
            characters,
            props,
            ground_tiles,
        })
    }

    pub fn get_texture_by_id(&self, id: &str) -> Option<Texture2D> {
        match id {
            Self::WHITE_TEXTURE_ID => Some(self.white_texture),
            Self::CHARACTERS_TEXTURE_ID => Some(self.characters),
            Self::PROPS_TEXTURE_ID => Some(self.props),
            Self::GROUND_TILES_TEXTURE_ID => Some(self.ground_tiles),
            _ => {
                assert!(false, "Invalid texture id '{}", id);
                None
            },
        }
    }
}
