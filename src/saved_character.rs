use std::{
    io,
};

use regex::Regex;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedCharacter {
    pub game_version: String,
    pub actor: ActorParams,
    pub items: Vec<ItemParams>,
    pub active_missions: Vec<String>,
    pub completed_missions: Vec<String>,
    pub current_chapter_index: usize,
    pub current_map_id: String,
    pub is_permadeath: bool,
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
pub async fn get_available_characters(characters_path: &str) -> io::Result<Vec<SavedCharacter>> {
    let regex = Regex::new(r".json$").unwrap();
    let mut res = Vec::new();
    for entry in fs::read_dir(characters_path)? {
        if let Ok(entry) = entry {
            if let Some(path) = entry.path().to_str() {
                if regex.is_match(path) {
                    let json = load_file(path)
                        .await
                        .expect(&format!("Error when parsing character file '{}'!", path));
                    let character: SavedCharacter = serde_json::from_slice(&json)?;
                    res.push(character);
                }
            }
        }
    }
    Ok(res)
}

#[cfg(target_family = "wasm")]
pub async fn get_available_characters(_: &str) -> io::Result<Vec<SavedCharacter>> {
    let game_params = storage::get::<GameParams>();
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    let save_name = format!("{}_character", game_params.game_name);
    if let Some(json) = storage.get(&save_name) {
        let character: SavedCharacter = serde_json::from_str(&json)?;
        return Ok(vec![character]);
    }
    Ok(Vec::new())
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
pub fn delete_character(name: &str) {
    let game_params = storage::get::<GameParams>();
    let path = format!("{}/{}.json", game_params.characters_path, name);
    fs::remove_file(path);
}

#[cfg(target_family = "wasm")]
pub fn delete_character(name: &str) {
    let game_params = storage::get::<GameParams>();
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    let save_name = format!("{}_character", game_params.game_name);
    storage.remove(save_name);
}
