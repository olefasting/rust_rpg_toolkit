use std::{
    fs,
    io,
};

use regex::Regex;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    pub game_version: String,
    pub filename: String,
    pub chapter: u32,
    #[serde(rename = "map")]
    pub map_id: String,
    pub player_actor: ActorParams,
    pub actors: Vec<ActorParams>,
    pub items: Vec<ItemParams>,
    pub active_missions: Vec<String>,
    pub completed_missions: Vec<String>,
}

impl SaveGame {
    pub fn create_from_scene(filename: &str, game_state: &GameState) -> Self {
        let game_params = storage::get::<GameParams>();

        let player_actor = Actor::find_by_player_id(&game_state.local_player_id).unwrap();

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

        let active_missions = player_actor.active_missions
            .iter()
            .map(|mission| mission.id.clone())
            .collect();

        let completed_missions = player_actor.completed_missions
            .iter()
            .map(|mission| mission.id.clone())
            .collect();

        SaveGame {
            game_version: game_params.game_version.clone(),
            filename: filename.to_string(),
            chapter: current_chapter.chapter_index as u32 + 1,
            map_id: current_chapter.current_map_id.clone(),
            player_actor: player_actor.to_save(),
            actors,
            items,
            active_missions,
            completed_missions,
        }
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn save_scene_to_file(filename: &str, game_state: &GameState) {
        let game_params = storage::get::<GameParams>();
        let save_game = Self::create_from_scene(filename, game_state);
        let path = &format!("{}/{}", game_params.saves_path, filename);
        let json = serde_json::to_string_pretty(&save_game)
            .expect("Unable to serialize scene into JSON!");
        fs::write(path, &json)
            .expect(&format!("Unable to save game '{}' to disk!", path));
    }

    #[cfg(target_family = "wasm")]
    pub fn save_scene_to_file(name: &str, game_state: &GameState) {
        todo!("Implement wasm save games")
    }
}

#[cfg(any(target_family = "unix", target_family = "windows"))]
pub async fn get_available_save_games(saves_path: &str) -> io::Result<Vec<SaveGame>> {
    let regex = Regex::new(r".json$").unwrap();
    let mut res = Vec::new();
    for entry in fs::read_dir(saves_path)? {
        if let Ok(entry) = entry {
            if let Some(path) = entry.path().to_str() {
                if regex.is_match(path) {
                    let json = load_file(path)
                        .await
                        .expect(&format!("Error when parsing save game '{}'!", path));
                    let save_game: SaveGame = serde_json::from_slice(&json)?;
                    res.push(save_game);
                }
            }
        }
    }
    Ok(res)
}

#[cfg(target_family = "wasm")]
pub async fn get_available_save_games(saves_path: &str) -> io::Result<Vec<SaveGame>> {
    Ok(Vec::new())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedCharacter {
    pub game_version: String,
    pub actor: ActorParams,
    pub items: Vec<ItemParams>,
    pub active_missions: Vec<String>,
    pub completed_missions: Vec<String>,
}

#[cfg(any(target_family = "unix", target_family = "windows"))]
pub async fn get_available_characters(characters_path: &str) -> io::Result<Vec<ExportedCharacter>> {
    let regex = Regex::new(r".json$").unwrap();
    let mut res = Vec::new();
    for entry in fs::read_dir(characters_path)? {
        if let Ok(entry) = entry {
            if let Some(path) = entry.path().to_str() {
                if regex.is_match(path) {
                    let json = load_file(path)
                        .await
                        .expect(&format!("Error when parsing character file '{}'!", path));
                    let character: ExportedCharacter = serde_json::from_slice(&json)?;
                    res.push(character);
                }
            }
        }
    }
    Ok(res)
}

#[cfg(target_family = "wasm")]
pub async fn get_available_characters(characters_path: &str) -> io::Result<Vec<ExportedCharacter>> {
    Ok(Vec::new())
}
