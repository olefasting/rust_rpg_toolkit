use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    render::{
        LINEAR_FILTER_MODE,
        NEAREST_FILTER_MODE,
    },
    prelude::*,
};

use crate::modules::load_modules;
use crate::dialogue::Dialogue;
use crate::map::{TiledMapDeclaration, Map};
use crate::scenario::Scenario;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialInfo {
    pub id: String,
    pub fragment_shader_path: String,
    pub vertex_shader_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureParams {
    pub id: String,
    pub path: String,
    #[serde(default = "TextureParams::default_filter_mode")]
    pub filter_mode: String,
}

impl TextureParams {
    fn default_filter_mode() -> String {
        NEAREST_FILTER_MODE.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundParams {
    pub id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesParams {
    materials: Vec<MaterialInfo>,
    textures: Vec<TextureParams>,
    sound_effects: Vec<SoundParams>,
    music: Vec<SoundParams>,
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
    pub dialogue: HashMap<String, Dialogue>,
}

impl Resources {
    pub const WHITE_TEXTURE_ID: &'static str = "__WHITE_TEXTURE__";

    pub const CHARACTERS_TEXTURE_ID: &'static str = "characters";
    pub const PROPS_TEXTURE_ID: &'static str = "props";
    pub const GROUND_TILES_TEXTURE_ID: &'static str = "tiles";
    pub const ITEMS_TEXTURE_ID: &'static str = "items";

    pub async fn new() -> Result<Self, FileError> {
        let game_params = storage::get::<GameParams>();
        let resources_file_path = format!("{}/resources.json", game_params.assets_path);
        let actors_file_path = format!("{}/actors.json", game_params.assets_path);
        let items_file_path = format!("{}/items.json", game_params.assets_path);
        let abilities_file_path = format!("{}/abilities.json", game_params.assets_path);
        let missions_file_path = format!("{}/missions.json", game_params.assets_path);
        let dialogue_file_path = format!("{}/dialogue.json", game_params.assets_path);

        let mut textures = HashMap::new();
        let white_texture = load_texture("assets/textures/white_texture.png").await?;
        white_texture.set_filter(FilterMode::Nearest);
        textures.insert(Self::WHITE_TEXTURE_ID.to_string(), white_texture);

        let bytes = load_file(&resources_file_path).await?;
        let resources: ResourcesParams = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing resource file '{}'!", resources_file_path));

        let mut materials = HashMap::new();
        for material_params in &resources.materials {
            let vertex_shader = load_file(&format!("{}/{}", game_params.assets_path, material_params.vertex_shader_path)).await?;
            let fragment_shader = load_file(&format!("{}/{}", game_params.assets_path, material_params.fragment_shader_path)).await?;

            let material = load_material(
                &String::from_utf8(vertex_shader).unwrap(),
                &String::from_utf8(fragment_shader).unwrap(),
                MaterialParams {
                    ..Default::default()
                },
            ).unwrap();

            materials.insert(material_params.id.clone(), material);
        }

        for texture_params in &resources.textures {
            let texture = load_texture(&format!("{}/{}", game_params.assets_path, texture_params.path)).await?;
            if texture_params.filter_mode == LINEAR_FILTER_MODE.to_string() {
                texture.set_filter(FilterMode::Linear)
            } else if texture_params.filter_mode == NEAREST_FILTER_MODE.to_string() {
                texture.set_filter(FilterMode::Nearest);
            } else {
                assert!(false, "Invalid filter mode '{}'", texture_params.filter_mode);
            }
            textures.insert(texture_params.id.clone(), texture);
        }

        let mut sound_effects = HashMap::new();
        for sound_params in &resources.sound_effects {
            let sound = load_sound(&format!("{}/{}", game_params.assets_path, sound_params.path)).await?;
            sound_effects.insert(sound_params.id.clone(), sound);
        }

        let mut music = HashMap::new();
        for music_params in &resources.music {
            let track = load_sound(&format!("assets/{}", music_params.path)).await?;
            music.insert(music_params.id.clone(), track);
        }

        let bytes = load_file(&actors_file_path).await?;
        let actor_data: Vec<ActorParams> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing actors file '{}'!", &actors_file_path));
        let mut actors = HashMap::from_iter(
            actor_data.into_iter().map(|params| (params.id.clone(), params)));

        let bytes = load_file(&items_file_path).await?;
        let items_data: Vec<ItemParams> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing items file '{}'!", items_file_path));
        let mut items = HashMap::from_iter(
            items_data.into_iter().map(|params| (params.id.clone(), params)));

        let bytes = load_file(&missions_file_path).await?;
        let missions_data: Vec<MissionParams> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing missions file '{}'!", &missions_file_path));
        let mut missions = HashMap::from_iter(
            missions_data.into_iter().map(|mission| (mission.id.clone(), mission)));

        let bytes = load_file(&dialogue_file_path).await?;
        let dialogue_data: Vec<Dialogue> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing dialogue file '{}'!", &dialogue_file_path));
        let mut dialogue = HashMap::from_iter(
            dialogue_data.into_iter().map(|dialogue| (dialogue.id.clone(), dialogue)));

        let bytes = load_file(&abilities_file_path).await?;
        let ability_data: Vec<AbilityParams> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing dialogue file '{}'!", &abilities_file_path));
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

        Ok(resources)
    }
}
