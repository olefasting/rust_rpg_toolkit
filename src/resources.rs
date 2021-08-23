use std::{
    collections::HashMap,
    fs,
    iter::FromIterator,
};

use macroquad::{
    audio::{
        load_sound,
        Sound,
    },
    prelude::*,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    generate_id,
    nodes::{
        item::ItemParams,
        actor::ActorParams,
    },
    render::{
        LINEAR_FILTER_MODE,
        NEAREST_FILTER_MODE,
    },
    missions::MissionParams,
};
use crate::nodes::actor::ActorDialogue;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TextureData {
    pub id: String,
    pub filename: String,
    #[serde(default = "TextureData::default_filter_mode")]
    pub filter_mode: String,
}

impl TextureData {
    fn default_filter_mode() -> String {
        NEAREST_FILTER_MODE.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SoundData {
    pub id: String,
    pub filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResourcesData {
    textures: Vec<TextureData>,
    sound_effects: Vec<SoundData>,
    music: Vec<SoundData>,
}

pub struct Resources {
    pub textures: HashMap<String, Texture2D>,
    pub sound_effects: HashMap<String, Sound>,
    pub music: HashMap<String, Sound>,
    pub actors: HashMap<String, ActorParams>,
    pub items: HashMap<String, ItemParams>,
    pub missions: HashMap<String, MissionParams>,
    pub dialogue: HashMap<String, ActorDialogue>,
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

    const ITEMS_FILE_PATH: &'static str = "assets/items.json";
    const ACTORS_FILE_PATH: &'static str = "assets/actors.json";
    const MISSIONS_FILE_PATH: &'static str = "assets/missions.json";
    const DIALOGUE_FILE_PATH: &'static str = "assets/dialogue.json";

    pub async fn new() -> Result<Resources, FileError> {
        let mut textures = HashMap::new();

        let white_texture = load_texture("assets/textures/white_texture.png").await?;
        white_texture.set_filter(FilterMode::Nearest);
        textures.insert(Self::WHITE_TEXTURE_ID.to_string(), white_texture);

        let json = fs::read_to_string(Self::RESOURCES_FILE_PATH)
            .expect(&format!("Unable to find resources file '{}'!", Self::RESOURCES_FILE_PATH));
        let resources: ResourcesData = serde_json::from_str(&json)
            .expect(&format!("Error when parsing resource file '{}'!", Self::RESOURCES_FILE_PATH));

        for texture_data in &resources.textures {
            let texture = load_texture(&format!("{}/{}", Self::TEXTURES_FOLDER_PATH, &texture_data.filename)).await?;
            if texture_data.filter_mode == LINEAR_FILTER_MODE.to_string() {
                texture.set_filter(FilterMode::Linear)
            } else if texture_data.filter_mode == NEAREST_FILTER_MODE.to_string() {
                texture.set_filter(FilterMode::Nearest);
            } else {
                assert!(false, "Invalid filter mode '{}'", texture_data.filter_mode);
            }
            textures.insert(texture_data.id.clone(), texture);
        }

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

        let json = std::fs::read_to_string(Self::ACTORS_FILE_PATH)
            .expect(&format!("Unable to find actors file '{}'!", Self::ACTORS_FILE_PATH));
        let actor_data: Vec<ActorParams> = serde_json::from_str(&json)
            .expect(&format!("Error when parsing actors file '{}'!", Self::ACTORS_FILE_PATH));
        let actors = HashMap::from_iter(
            actor_data.into_iter().map(|params| (params.prototype_id.clone().unwrap_or(generate_id()), params)));

        let json = std::fs::read_to_string(Self::ITEMS_FILE_PATH)
            .expect(&format!("Unable to find items file '{}'!", Self::ITEMS_FILE_PATH));
        let items_data: Vec<ItemParams> = serde_json::from_str(&json)
            .expect(&format!("Error when parsing items file '{}'!", Self::ITEMS_FILE_PATH));
        let items = HashMap::from_iter(
            items_data.into_iter().map(|params| (params.prototype_id.clone(), params)));

        let json = std::fs::read_to_string(Self::MISSIONS_FILE_PATH)
            .expect(&format!("Unable to find missions file '{}'!", Self::MISSIONS_FILE_PATH));
        let missions_data: Vec<MissionParams> = serde_json::from_str(&json)
            .expect(&format!("Error when parsing missions file '{}'!", Self::MISSIONS_FILE_PATH));
        let missions = HashMap::from_iter(
            missions_data.into_iter().map(|mission| (mission.id.clone(), mission)));

        let json = std::fs::read_to_string(Self::DIALOGUE_FILE_PATH)
            .expect(&format!("Unable to find dialogue file '{}'!", Self::DIALOGUE_FILE_PATH));
        let dialogue_data: Vec<ActorDialogue> = serde_json::from_str(&json)
            .expect(&format!("Error when parsing dialogue file '{}'!", Self::DIALOGUE_FILE_PATH));
        let dialogue = HashMap::from_iter(
            dialogue_data.into_iter().map(|dialogue| (dialogue.id.clone(), dialogue)));

        Ok(Resources {
            textures,
            sound_effects,
            music,
            actors,
            items,
            missions,
            dialogue,
        })
    }
}
