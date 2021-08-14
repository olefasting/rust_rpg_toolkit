mod inventory;
mod character;

use macroquad::prelude::*;

use inventory::draw_inventory_window;
use character::draw_character_window;

use crate::nodes::{
    GameState,
    Actor,
};

pub fn draw_gui() {
    let game_state = scene::find_node_by_type::<GameState>().unwrap();
    let mut player = Actor::find_local_player().unwrap();
    if game_state.show_character_window {
        draw_character_window(&*player);
    }
    if game_state.show_inventory_window {
        draw_inventory_window(&mut *player);
    }
}
