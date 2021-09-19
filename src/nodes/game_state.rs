use crate::prelude::*;

const CHARACTER_SAVE_INTERVAL: f32 = 30.0;

pub struct GameState {
    pub chapter_index: usize,
    pub map_id: String,
    pub dead_actors: Vec<String>,
    pub character_name: String,
    pub is_permadeath: bool,
    pub in_debug_mode: bool,
    pub gui: GuiState,
    time_since_save: f32,
}

impl GameState {
    pub fn new(character: &Character) -> GameState {
        GameState {
            chapter_index: character.current_chapter_index,
            map_id: character.current_map_id.clone(),
            dead_actors: Vec::new(),
            character_name: character.actor.name.clone(),
            is_permadeath: character.is_permadeath,
            in_debug_mode: false,
            gui: GuiState::new(),
            time_since_save: 0.0,
        }
    }

    pub fn add_node(character: &Character) -> Handle<Self> {
        let game_state = Self::new(character);
        scene::add_node(game_state)
    }

    pub fn get_player_character(&self) -> Option<Character> {
        if let Some(actor) = get_player_actor() {
            let character = actor.to_character(self.chapter_index, &self.map_id, self.is_permadeath);
            return Some(character);
        }
        None
    }
}

impl Node for GameState {
    fn update(mut node: RefMut<Self>) {
        node.time_since_save += get_frame_time();
        if node.time_since_save >= CHARACTER_SAVE_INTERVAL {
            node.time_since_save = 0.0;
            dispatch_event(Event::SavePlayerCharacter);
        }

        if get_player_actor().is_none() {
            if node.is_permadeath {
                delete_character(&node.character_name).expect(&format!("Error when saving character '{}'!", &node.character_name));
            }
            dispatch_event(Event::MainMenu);
        }
    }
}
