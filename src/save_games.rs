use std::{
    fs,
};

use macroquad::{
    experimental::{
        collections::storage,
    },
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    nodes::{
        Actor,
        ActorParams,
        GameState,
        Item,
        ItemParams,
    },
    scenario::CurrentChapter,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    pub chapter: u32,
    #[serde(rename = "map")]
    pub map_id: String,
    pub player_actor_id: String,
    pub actors: Vec<ActorParams>,
    pub items: Vec<ItemParams>,
}

impl SaveGame {
    const SAVE_FOLDER_PATH: &'static str = "saved_games";

    pub fn create_from_scene(game_state: &GameState) -> Self {
        let player_actor_id = {
            let player = Actor::find_by_player_id(&game_state.local_player_id).unwrap();
            player.id.clone()
        };

        let mut actors = Vec::new();
        let mut items = Vec::new();

        for actor in scene::find_nodes_by_type::<Actor>() {
            for item in &actor.inventory.items {
                items.push(item.params.clone())
            }
            actors.push(actor.to_save());
        }

        for item in scene::find_nodes_by_type::<Item>() {
            items.push(item.to_params());
        }

        let current_chapter = storage::get::<CurrentChapter>();

        SaveGame {
            chapter: current_chapter.chapter_index as u32 + 1,
            map_id: current_chapter.current_map_id.clone(),
            player_actor_id,
            actors,
            items,
        }
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn save_scene_to_file(name: &str, game_state: &GameState) {
        let save_game = Self::create_from_scene(game_state);
        let path = &format!("{}/{}", Self::SAVE_FOLDER_PATH, name);
        let json = serde_json::to_string_pretty(&save_game)
            .expect("Unable to serialize scene into JSON!");
        fs::write(path, &json)
            .expect(&format!("Unable to save game '{}' to disk!", path));
    }

    #[cfg(target_family = "wasm")]
    pub fn save_scene_to_file(name: &str, game_state: &GameState) {
        todo!()
    }
}
