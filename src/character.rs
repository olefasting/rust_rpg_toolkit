use regex::Regex;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub game_version: String,
    pub actor: ActorParams,
    pub items: Vec<ItemParams>,
    pub active_missions: Vec<String>,
    pub completed_missions: Vec<String>,
    pub chapter_index: usize,
    pub map_id: String,
    pub is_permadeath: bool,
}

impl Character {
    pub fn with_map(self, chapter_index: usize, map_id: &str) -> Self {
        Character {
            chapter_index,
            map_id: map_id.to_string(),
            ..self
        }
    }

    #[cfg(not(any(target_family = "wasm", target_os = "android")))]
    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let path = character_name_to_path(&self.actor.name);
        fs::write(path, json)?;

        Ok(())
    }

    #[cfg(target_family = "wasm")]
    pub fn save(&self) -> Result<()> {
        let game_params = storage::get::<GameParams>();
        let json = serde_json::to_string_pretty(self)?;
        let mut storage = quad_storage::STORAGE.lock().unwrap();
        storage.set(&format!("{}_character", game_params.name), &json);

        Ok(())
    }

    pub(crate) fn spawn(&self, game_state: Handle<GameState>, position: Vec2) {
        let player = storage::get::<LocalPlayer>();
        let mut actor = Actor::from_saved(
            game_state,
            position,
            ActorControllerKind::local_player(&player.id),
            self,
        );

        actor.stats.recalculate_derived();
        actor.stats.restore_vitals();

        scene::add_node(actor);
    }
}

pub fn character_name_to_path(name: &str) -> PathBuf {
    let game_params = storage::get::<GameParams>();
    Path::new(&game_params.characters_path).join(&format!("{}.json", name))
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
pub fn get_available_characters<P: AsRef<Path>>(path: P) -> Result<Vec<Character>> {
    let regex = Regex::new(r".json$")?;
    let mut res = Vec::new();
    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            if regex.is_match(&entry.path().to_string_lossy()) {
                let character = load_character(entry.path())?;
                res.push(character);
            }
        }
    }

    Ok(res)
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
pub fn load_character<P: AsRef<Path>>(path: P) -> Result<Character> {
    let json = fs::read(path)?;
    let character: Character = serde_json::from_slice(&json)?;
    Ok(character)
}

#[cfg(target_family = "wasm")]
pub fn get_available_characters(_: &str) -> io::Result<Vec<Character>> {
    let game_params = storage::get::<GameParams>();
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    let save_name = format!("{}_character", game_params.game_name);
    if let Some(json) = storage.get(&save_name) {
        let character: Character = serde_json::from_str(&json)?;
        return Ok(vec![character]);
    }

    Ok(Vec::new())
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
pub fn delete_character(name: &str) -> Result<()> {
    let path = character_name_to_path(name);
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