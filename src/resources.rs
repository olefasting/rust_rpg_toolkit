use std::{
    collections::HashMap,
    fs,
};

use serde::{
    Deserialize,
    Serialize,
};

use macroquad::{
    color,
    prelude::*,
};
use crate::nodes::{ItemParams, Projectiles, ActorParams};
use crate::generate_id;

#[derive(Clone, Serialize, Deserialize)]
struct TextureData {
    pub id: String,
    pub filename: String,
    pub filter_mode: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct ResourcesData {
    textures: Vec<TextureData>,
}

pub struct Resources {
    textures: HashMap<String, Texture2D>,
    actors: HashMap<String, ActorParams>,
    items: HashMap<String, ItemParams>,
    action_funcs: HashMap<String, ActionFunc>,
}

impl Resources {
    pub const WHITE_TEXTURE_ID: &'static str = "__WHITE_TEXTURE__";

    pub const CHARACTERS_TEXTURE_ID: &'static str = "characters";
    pub const PROPS_TEXTURE_ID: &'static str = "props";
    pub const GROUND_TILES_TEXTURE_ID: &'static str = "tiles";
    pub const ITEMS_TEXTURE_ID: &'static str = "items";

    const RESOURCES_FILE_PATH: &'static str = "assets/resources.json";
    const TEXTURES_FOLDER_PATH: &'static str = "assets/textures";

    const LINEAR_FILTER_MODE: &'static str = "linear";
    const NEAREST_FILTER_MODE: &'static str = "nearest_neighbor";

    const ITEMS_FILE_PATH: &'static str = "assets/items.json";
    const ACTORS_FILE_PATH: &'static str = "assets/actors.json";

    const PROJECTILE_ACTION_ID: &'static str = "projectile";
    const MAGIC_SPHERE_ACTION_ID: &'static str = "magic_sphere";

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

        let mut action_funcs: HashMap<String, ActionFunc> = HashMap::new();

        action_funcs.insert(Self::PROJECTILE_ACTION_ID.to_string(), projectile_action);
        action_funcs.insert(Self::MAGIC_SPHERE_ACTION_ID.to_string(), magic_sphere_action);

        Ok(Resources {
            textures,
            actors,
            items,
            action_funcs,
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

    pub fn get_action_func(&self, id: &str) -> ActionFunc {
        *self.action_funcs.get(id).unwrap()
    }

    pub fn try_get_action_func(&self, id: &str) -> Option<ActionFunc> {
        let func = self.action_funcs.get(id);
        match func {
            Some(action) => Some(*action),
            None => None,
        }
    }
}


#[derive(Clone, Serialize, Deserialize)]
pub struct ActionParams {
    pub id: String,
    pub action_kind: String,
    pub action_func_id: String,
    pub cooldown: f32,
    pub health_cost: f32,
    pub stamina_cost: f32,
    pub energy_cost: f32,
    pub speed: f32,
    pub spread: f32,
    pub range: f32,
    pub damage: f32,
}

impl ActionParams {
    pub const PRIMARY_ABILITY: &'static str = "primary";
    pub const SECONDARY_ABILITY: &'static str = "secondary";
}

impl Default for ActionParams {
    fn default() -> Self {
        ActionParams {
            id: generate_id(),
            action_kind: Self::PRIMARY_ABILITY.to_string(),
            action_func_id: Resources::PROJECTILE_ACTION_ID.to_string(),
            cooldown: 0.0,
            health_cost: 0.0,
            stamina_cost: 0.0,
            energy_cost: 0.0,
            speed: 75.0,
            spread: 0.0,
            range: 100.0,
            damage: 0.0,
        }
    }
}

pub type ActionFunc = fn (actor_id: &str, origin: Vec2, target: Vec2, speed: f32, spread: f32, range: f32, damage: f32);

fn projectile_action(actor_id: &str, origin: Vec2, target: Vec2, speed: f32, spread: f32, range: f32, damage: f32) {
    let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
    let ttl = range / speed;
    projectiles.spawn(actor_id, damage, color::YELLOW, 2.0, origin, target, speed, spread, ttl);
}

fn magic_sphere_action(actor_id: &str, origin: Vec2, target: Vec2, speed: f32, spread: f32, range: f32, damage: f32) {
    let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
    let ttl = range / speed;
    projectiles.spawn(actor_id, damage, color::BLUE, 100.0, origin, target, speed, spread, ttl);
}

pub struct ActionFuncs {
    actions: HashMap<String, ActionFunc>
}
