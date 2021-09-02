use crate::prelude::*;

const CHARACTER_SAVE_INTERVAL: f32 = 30.0;

pub struct GameState {
    pub map: Map,
    pub dead_actors: Vec<String>,
    pub local_player_id: String,
    pub should_show_character_window: bool,
    pub should_show_inventory_window: bool,
    pub should_show_game_menu: bool,
    pub in_debug_mode: bool,
    pub scene_transition: Option<SceneTransitionParams>,
    pub should_save_character: bool,
    pub should_go_to_main_menu: bool,
    pub should_quit: bool,
    character_save_timer: f32,
}

impl GameState {
    pub fn new(local_player_id: &str, map: Map) -> GameState {
        GameState {
            map,
            dead_actors: Vec::new(),
            local_player_id: local_player_id.to_string(),
            should_show_character_window: false,
            should_show_inventory_window: false,
            should_show_game_menu: false,
            in_debug_mode: false,
            scene_transition: None,
            should_quit: false,
            should_save_character: false,
            should_go_to_main_menu: false,
            character_save_timer: 0.0,
        }
    }

    pub fn add_node(local_player_id: &str, map: Map) -> Handle<Self> {
        scene::add_node(Self::new(local_player_id, map))
    }

    #[cfg(not(any(target_family = "wasm", target_os = "android")))]
    pub fn save_player_character(&mut self) {
        let game_params = storage::get::<GameParams>();
        if let Some(player) = Actor::find_by_player_id(&self.local_player_id) {
            let json = serde_json::to_string_pretty(&player.to_export()).unwrap();
            let path = format!("{}/{}.json", game_params.characters_path, player.name);
            fs::write(&path, json).unwrap()
        }
    }

    #[cfg(target_family = "wasm")]
    pub fn save_player_character(&self) {
        let game_params = storage::get::<GameParams>();
        if let Some(player) = Actor::find_by_player_id(&self.local_player_id) {
            let json = serde_json::to_string_pretty(&player.to_export()).unwrap();
            let mut storage = quad_storage::STORAGE.lock().unwrap();
            storage.set(&format!("{}_character", game_params.game_name), &json);
        }
    }
}

impl Node for GameState {
    fn update(mut node: RefMut<Self>) where Self: Sized {
        if Actor::find_by_player_id(&node.local_player_id).is_none() {
            node.should_show_game_menu = is_key_released(KeyCode::Escape);
        }

        node.character_save_timer += get_frame_time();
        if node.character_save_timer >= CHARACTER_SAVE_INTERVAL {
            node.should_save_character = true;
            node.character_save_timer = 0.0;
        }
    }

    fn draw(node: RefMut<Self>) {
        let viewport = storage::get::<Viewport>();
        let rect = node.map.to_grid(viewport.get_frustum());
        node.map.draw( Some(rect));
    }
}
