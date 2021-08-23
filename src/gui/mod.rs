use macroquad::prelude::*;

mod inventory;
mod character;
mod dialogue;

use dialogue::draw_dialogue_window;

pub mod theme;

pub use inventory::draw_inventory_window;
pub use character::draw_character_window;

use crate::{
    nodes::{
        GameState,
        Actor,
    }
};

pub fn draw_gui() {
    if let Some(game_state) = scene::find_node_by_type::<GameState>() {
        if let Some(mut player) = Actor::find_by_player_id(&game_state.local_player_id) {
            if game_state.show_character_window {
                draw_character_window(&*player);
            }
            if game_state.show_inventory_window {
                draw_inventory_window(&mut *player);
            }
            draw_dialogue_window(&mut *player);
        }
    }
}
