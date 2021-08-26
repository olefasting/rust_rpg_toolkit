use std::{
    collections::HashMap,
    iter::FromIterator,
};

use macroquad::{
    audio::{
        load_sound,
        Sound,
    },
    experimental::{
        collections::storage,
        coroutines::start_coroutine,
    },
    color,
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
        actor::ActorDialogue,
    },
    render::{
        LINEAR_FILTER_MODE,
        NEAREST_FILTER_MODE,
        draw_aligned_text,
        HorizontalAlignment,
        VerticalAlignment,
    },
    missions::MissionParams,
    ability::AbilityParams,
};
use crate::modules::load_modules;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialInfo {
    pub id: String,
    pub fragment_shader_path: String,
    pub vertex_shader_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureInfo {
    pub id: String,
    pub path: String,
    #[serde(default = "TextureInfo::default_filter_mode")]
    pub filter_mode: String,
}

impl TextureInfo {
    fn default_filter_mode() -> String {
        NEAREST_FILTER_MODE.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundInfo {
    pub id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesInfo {
    materials: Vec<MaterialInfo>,
    textures: Vec<TextureInfo>,
    sound_effects: Vec<SoundInfo>,
    music: Vec<SoundInfo>,
}

pub struct Resources {
    pub materials: HashMap<String, Material>,
    pub textures: HashMap<String, Texture2D>,
    pub sound_effects: HashMap<String, Sound>,
    pub music: HashMap<String, Sound>,
    pub actors: HashMap<String, ActorParams>,
    pub items: HashMap<String, ItemParams>,
    pub abilities: HashMap<String, AbilityParams>,
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

    const ITEMS_FILE_PATH: &'static str = "assets/items.json";
    const ABILITIES_FILE_PATH: &'static str = "assets/abilities.json";
    const ACTORS_FILE_PATH: &'static str = "assets/actors.json";
    const MISSIONS_FILE_PATH: &'static str = "assets/missions.json";
    const DIALOGUE_FILE_PATH: &'static str = "assets/dialogue.json";

    pub async fn new() -> Result<Resources, FileError> {
        let mut textures = HashMap::new();
        let white_texture = load_texture("assets/textures/white_texture.png").await?;
        white_texture.set_filter(FilterMode::Nearest);
        textures.insert(Self::WHITE_TEXTURE_ID.to_string(), white_texture);

        let bytes = load_file(Self::RESOURCES_FILE_PATH).await
            .expect(&format!("Unable to find resources file '{}'!", Self::RESOURCES_FILE_PATH));
        let resources: ResourcesInfo = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing resource file '{}'!", Self::RESOURCES_FILE_PATH));

        let mut materials = HashMap::new();
        for material_info in &resources.materials {
            let vertex_shader = load_file(&format!("assets/{}", material_info.vertex_shader_path)).await?;
            let fragment_shader = load_file(&format!("assets/{}", material_info.fragment_shader_path)).await?;

            let material = load_material(
                &String::from_utf8(vertex_shader).unwrap(),
                &String::from_utf8(fragment_shader).unwrap(),
                MaterialParams {
                    ..Default::default()
                }
            ).unwrap();

            materials.insert(material_info.id.clone(), material);
        }

        for texture_info in &resources.textures {
            let texture = load_texture(&format!("assets/{}", texture_info.path)).await?;
            if texture_info.filter_mode == LINEAR_FILTER_MODE.to_string() {
                texture.set_filter(FilterMode::Linear)
            } else if texture_info.filter_mode == NEAREST_FILTER_MODE.to_string() {
                texture.set_filter(FilterMode::Nearest);
            } else {
                assert!(false, "Invalid filter mode '{}'", texture_info.filter_mode);
            }
            textures.insert(texture_info.id.clone(), texture);
        }

        let mut sound_effects = HashMap::new();
        for sound_info in &resources.sound_effects {
            let sound = load_sound(&format!("assets/{}", sound_info.path)).await.unwrap();
            sound_effects.insert(sound_info.id.clone(), sound);
        }

        let mut music = HashMap::new();
        for music_info in &resources.music {
            let track = load_sound(&format!("assets/{}", music_info.path)).await.unwrap();
            music.insert(music_info.id.clone(), track);
        }

        let bytes = load_file(Self::ACTORS_FILE_PATH).await
            .expect(&format!("Unable to find actors file '{}'!", Self::ACTORS_FILE_PATH));
        let actor_data: Vec<ActorParams> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing actors file '{}'!", Self::ACTORS_FILE_PATH));
        let mut actors = HashMap::from_iter(
            actor_data.into_iter().map(|params| (params.prototype_id.clone().unwrap_or(generate_id()), params)));

        let bytes = load_file(Self::ITEMS_FILE_PATH).await
            .expect(&format!("Unable to find items file '{}'!", Self::ITEMS_FILE_PATH));
        let items_data: Vec<ItemParams> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing items file '{}'!", Self::ITEMS_FILE_PATH));
        let mut items = HashMap::from_iter(
            items_data.into_iter().map(|params| (params.prototype_id.clone(), params)));

        let bytes = load_file(Self::MISSIONS_FILE_PATH).await
            .expect(&format!("Unable to find missions file '{}'!", Self::MISSIONS_FILE_PATH));
        let missions_data: Vec<MissionParams> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing missions file '{}'!", Self::MISSIONS_FILE_PATH));
        let mut missions = HashMap::from_iter(
            missions_data.into_iter().map(|mission| (mission.id.clone(), mission)));

        let bytes = load_file(Self::DIALOGUE_FILE_PATH).await
            .expect(&format!("Unable to find dialogue file '{}'!", Self::DIALOGUE_FILE_PATH));
        let dialogue_data: Vec<ActorDialogue> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing dialogue file '{}'!", Self::DIALOGUE_FILE_PATH));
        let mut dialogue = HashMap::from_iter(
            dialogue_data.into_iter().map(|dialogue| (dialogue.id.clone(), dialogue)));

        let bytes = load_file(Self::ABILITIES_FILE_PATH).await
            .expect(&format!("Unable to find dialogue file '{}'!", Self::ABILITIES_FILE_PATH));
        let ability_data: Vec<AbilityParams> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing dialogue file '{}'!", Self::ABILITIES_FILE_PATH));
        let mut abilities = HashMap::from_iter(
            ability_data.into_iter().map(|ability| (ability.id.clone(), ability)));

        let mut resources = Resources {
            materials,
            textures,
            sound_effects,
            music,
            actors,
            items,
            abilities,
            missions,
            dialogue,
        };

        load_modules(&mut resources).await;

        Ok(resources)
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub async fn load() {
        let load_resources = start_coroutine(async move {
            let resources = Resources::new().await.unwrap();
            storage::store(resources);
        });

        while load_resources.is_done() == false {
            clear_background(BLACK);
            draw_aligned_text(
                &format!("Loading resources"),
                screen_width() / 2.0,
                screen_height() / 2.0,
                HorizontalAlignment::Center,
                VerticalAlignment::Center,
                TextParams {
                    font_size: 40,
                    color: color::WHITE,
                    ..Default::default()
                },
            );

            next_frame().await;
        }
    }

    #[cfg(target_family = "wasm")]
    pub async fn load() {
        let resources = Resources::new().await.unwrap();
        storage::store(resources);
    }
}
