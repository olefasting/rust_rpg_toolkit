use std::{
    collections::HashMap,
    fs,
};

use serde::{
    Deserialize,
    Serialize,
};

use macroquad::{
    prelude::*,
};

use crate::{
    nodes::{
        ItemParams,
        ActorParams,
    },
};
use macroquad::audio::{Sound, load_sound};

#[derive(Clone, Serialize, Deserialize)]
struct TextureData {
    pub id: String,
    pub filename: String,
    pub filter_mode: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct SoundData {
    pub id: String,
    pub filename: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct ResourcesData {
    textures: Vec<TextureData>,
    sound_effects: Vec<SoundData>,
    music: Vec<SoundData>,
}

pub struct Resources {
    textures: HashMap<String, Texture2D>,
    sound_effects: HashMap<String, Sound>,
    music: HashMap<String, Sound>,
    actors: HashMap<String, ActorParams>,
    items: HashMap<String, ItemParams>,
}

impl Resources {
    pub const WHITE_TEXTURE_ID: &'static str = "__WHITE_TEXTURE__";

    pub const CHARACTERS_TEXTURE_ID: &'static str = "characters";
    pub const PROPS_TEXTURE_ID: &'static str = "props";
    pub const GROUND_TILES_TEXTURE_ID: &'static str = "tiles";
    pub const ITEMS_TEXTURE_ID: &'static str = "items";

    const RESOURCES_FILE_PATH: &'static str = "assets/resources.json";
    const TEXTURES_FOLDER_PATH: &'static str = "assets/textures";
    const SOUND_EFFECTS_FOLDER_PATH: &'static str = "assets/sound_effects";
    const MUSIC_FOLDER_PATH: &'static str = "assets/music";

    const LINEAR_FILTER_MODE: &'static str = "linear";
    const NEAREST_FILTER_MODE: &'static str = "nearest_neighbor";

    const ITEMS_FILE_PATH: &'static str = "assets/items.json";
    const ACTORS_FILE_PATH: &'static str = "assets/actors.json";

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



        let mut sound_effects = HashMap::new();

        for sound_data in &resources.sound_effects {
            let sound = load_sound(&format!("{}/{}", Self::SOUND_EFFECTS_FOLDER_PATH, sound_data.filename)).await.unwrap();
            sound_effects.insert(sound_data.id.clone(), sound);
        }

        let mut music = HashMap::new();

        for music_data in &resources.music {
            let track = load_sound(&format!("{}/{}", Self::MUSIC_FOLDER_PATH, music_data.filename)).await.unwrap();
            music.insert(music_data.id.clone(), track);
        }

        let mut actors= HashMap::new();

        let json = std::fs::read_to_string(Self::ACTORS_FILE_PATH)
            .expect(&format!("Unable to find actors file '{}'", Self::ACTORS_FILE_PATH));
        let actor_data: Vec<ActorParams> = serde_json::from_str(&json)
            .expect(&format!("Error when parsing actors file '{}'", Self::ACTORS_FILE_PATH));

        for actor in actor_data {
            actors.insert(actor.id.clone(), actor);
        }

        let mut items= HashMap::new();

        let json = std::fs::read_to_string(Self::ITEMS_FILE_PATH)
            .expect(&format!("Unable to find items file '{}'", Self::ITEMS_FILE_PATH));
        let items_data: Vec<ItemParams> = serde_json::from_str(&json)
            .expect(&format!("Error when parsing items file '{}'", Self::ITEMS_FILE_PATH));

        for item in items_data {
            items.insert(item.id.clone(), item);
        }

        Ok(Resources {
            textures,
            sound_effects,
            music,
            actors,
            items,
        })
    }

    pub fn get_texture(&self, id: &str) -> &Texture2D {
        self.textures.get(id).unwrap()
    }

    pub fn try_get_texture(&self, id: &str) -> Option<&Texture2D> {
        self.textures.get(id)
    }

    pub fn get_actor(&self, id: &str) -> &ActorParams {
        self.actors.get(id).unwrap()
    }

    pub fn try_get_actor(&self, id: &str) -> Option<&ActorParams> {
        self.actors.get(id)
    }

    pub fn get_item(&self, id: &str) -> &ItemParams {
        self.items.get(id).unwrap()
    }

    pub fn try_get_item(&self, id: &str) -> Option<&ItemParams> {
        self.items.get(id)
    }
}
