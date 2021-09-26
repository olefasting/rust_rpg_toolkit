use crate::prelude::*;

const CHARACTER_SAVE_INTERVAL: f32 = 30.0;

pub struct GameState {
    pub chapter_index: usize,
    pub map_id: String,
    pub dead_actors: Vec<String>,
    pub player_spawn_point: Vec2,
    pub character_name: String,
    pub is_permadeath: bool,
    pub in_debug_mode: bool,
    pub gui_state: GuiState,
    time_since_save: f32,
}

impl GameState {
    pub fn new(player_spawn_point: Vec2, character: &Character) -> GameState {
        GameState {
            chapter_index: character.chapter_index,
            map_id: character.map_id.clone(),
            dead_actors: Vec::new(),
            player_spawn_point,
            character_name: character.actor.name.clone(),
            is_permadeath: character.is_permadeath,
            in_debug_mode: false,
            gui_state: GuiState::new(),
            time_since_save: 0.0,
        }
    }

    pub fn add_node(player_spawn_point: Vec2, character: &Character) -> Handle<Self> {
        let game_state = Self::new(player_spawn_point, character);
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

        if get_player_actor().is_none() {
            if node.is_permadeath {
                delete_character(&node.character_name).unwrap();
                dispatch_event(Event::OpenMainMenu);
            } else {
                dispatch_event(Event::Respawn);
            }
        } else if node.time_since_save >= CHARACTER_SAVE_INTERVAL {
            node.time_since_save = 0.0;
            dispatch_event(Event::Save);
        }
    }
}
