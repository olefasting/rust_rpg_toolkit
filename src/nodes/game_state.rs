use crate::prelude::*;

const CHARACTER_SAVE_INTERVAL: f32 = 30.0;

pub struct GameState {
    pub map: Map,
    pub player: Player,
    pub dead_actors: Vec<String>,
    pub should_show_character_window: bool,
    pub should_show_inventory_window: bool,
    pub should_show_game_menu: bool,
    pub in_debug_mode: bool,
    time_since_save: f32,
}

impl GameState {
    pub fn new(player: Player, character: CharacterExport, map: Map) -> Result<GameState> {
        save_character(character)?;

        let game_state = GameState {
            map,
            player,
            dead_actors: Vec::new(),
            should_show_character_window: false,
            should_show_inventory_window: false,
            should_show_game_menu: false,
            in_debug_mode: false,
            time_since_save: 0.0,
        };

        Ok(game_state)
    }

    pub fn add_node(player: Player, character: CharacterExport, map: Map) -> Result<Handle<Self>> {
        let game_state = Self::new(player, character, map)?;
        Ok(scene::add_node(game_state))
    }

    pub fn is_player_actor_in_scene(&self) -> bool {
        if let Some(handle) = self.player.actor_handle {
            return scene::try_get_node(handle).is_some()
                || Actor::find_by_player_id(&self.player.id).is_some();
        }
        false
    }

    pub fn get_player_actor(&self) -> Option<RefMut<Actor>> {
        if let Some(handle) = self.player.actor_handle {
            scene::try_get_node(handle)
        } else {
            Actor::find_by_player_id(&self.player.id)
        }
    }

    pub fn get_player_character(&self) -> Option<CharacterExport> {
        if let Some(actor) = self.get_player_actor() {
            let character = actor.to_export(self.player.is_permadeath);
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

        if node.is_player_actor_in_scene() == false {
            if node.dead_actors.contains(node.player.actor_id.as_ref().unwrap()) {
                if node.player.is_permadeath {
                    let name = node.player.actor_name.as_ref().unwrap();
                    delete_character(name).expect(&format!("Error when saving character '{}'!", name));
                }
            } else {
                println!("WARNING: Unable to find local players actor node, yet it does not appear to be dead!");
            }

            dispatch_event(Event::ShowMainMenu);
        }
    }

    fn draw(node: RefMut<Self>) {
        let viewport = storage::get::<Viewport>();
        let rect = node.map.to_grid(viewport.get_frustum());
        node.map.draw( Some(rect));
    }
}
