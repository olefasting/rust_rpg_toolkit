use macroquad::prelude::*;

mod inventory;
mod character;
mod dialogue;
mod game_menu;

pub mod skins;

use game_menu::draw_game_menu;
use dialogue::draw_dialogue_window;
use inventory::draw_inventory_window;
use character::draw_character_window;

use crate::{
    nodes::{
        GameState,
        Actor,
    }
};

pub fn draw_gui() {
    if let Some(mut game_state) = scene::find_node_by_type::<GameState>() {
        if let Some(mut player) = Actor::find_by_player_id(&game_state.local_player_id) {
            if game_state.show_character_window {
                draw_character_window(&*player);
            }
            if game_state.show_inventory_window {
                draw_inventory_window(&mut *player);
            }
            draw_dialogue_window(&mut *player);
            if game_state.show_game_menu {
                draw_game_menu(&mut game_state);
            }
        }
    }
}
