use regex::Regex;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub gamepad_id: Option<GamepadId>,
    pub actor_id: Option<String>,
    pub actor_name: Option<String>,
    pub actor_handle: Option<Handle<Actor>>,
    pub is_permadeath: bool,
}

impl Player {
    pub fn new(id: &str, gamepad_id: Option<GamepadId>) -> Self {
        Player {
            id: id.to_string(),
            gamepad_id,
            actor_id: None,
            actor_name: None,
            actor_handle: None,
            is_permadeath: false,
        }
    }

    pub fn set_actor(&mut self, actor_id: &str, actor_name: &str, actor_handle: Handle<Actor>, is_permadeath: bool) {
        self.actor_id = Some(actor_id.to_string());
        self.actor_name = Some(actor_name.to_string());
        self.actor_handle = Some(actor_handle);
        self.is_permadeath = is_permadeath;
    }

    pub fn is_actor_set(&self) -> bool {
        self.actor_handle.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerCharacter {
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
pub async fn get_available_characters(characters_path: &str) -> io::Result<Vec<PlayerCharacter>> {
    let regex = Regex::new(r".json$").unwrap();
    let mut res = Vec::new();
    for entry in fs::read_dir(characters_path)? {
        if let Ok(entry) = entry {
            if let Some(path) = entry.path().to_str() {
                if regex.is_match(path) {
                    let json = load_file(path)
                        .await
                        .expect(&format!("Error when parsing character file '{}'!", path));
                    let character: PlayerCharacter = serde_json::from_slice(&json)?;
                    res.push(character);
                }
            }
        }
    }
    Ok(res)
}

#[cfg(target_family = "wasm")]
pub async fn get_available_characters(_: &str) -> io::Result<Vec<PlayerCharacter>> {
    let game_params = storage::get::<GameParams>();
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    let save_name = format!("{}_character", game_params.game_name);
    if let Some(json) = storage.get(&save_name) {
        let character: PlayerCharacter = serde_json::from_str(&json)?;
        return Ok(vec![character]);
    }
    Ok(Vec::new())
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
pub fn save_character(character: PlayerCharacter) -> Result<()> {
    let game_params = storage::get::<GameParams>();
    let json = serde_json::to_string_pretty(&character)?;
    let path = format!("{}/{}.json", game_params.characters_path, &character.actor.name);
    fs::write(&path, json)?;
    Ok(())
}

#[cfg(target_family = "wasm")]
pub fn save_character(character: &PlayerCharacter) -> Result<()> {
    let game_params = storage::get::<GameParams>();
    let json = serde_json::to_string_pretty(&character)?;
    let mut storage = quad_storage::STORAGE.lock().unwrap();
    storage.set(&format!("{}_character", game_params.game_name), &json);
    Ok(())
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
pub fn delete_character(name: &str) -> Result<()> {
    let game_params = storage::get::<GameParams>();
    let path = format!("{}/{}.json", game_params.characters_path, name);
    fs::remove_file(path)?;
    Ok(())
}

#[cfg(target_family = "wasm")]
pub fn delete_character(name: &str) -> Result<()> {
    let game_params = storage::get::<GameParams>();
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    let save_name = format!("{}_character", game_params.game_name);
    storage.remove(save_name);
    Ok(())
}