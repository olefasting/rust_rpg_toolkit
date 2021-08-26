use macroquad::{
    experimental::{
        collections::storage,
    },
    ui::root_ui,
    prelude::*,
};

mod inventory;
mod character;
mod dialogue;
mod game_menu;

pub mod skins;
pub mod chapter_select;

use game_menu::draw_game_menu;
use dialogue::draw_dialogue_window;
use inventory::draw_inventory_window;
use character::draw_character_window;

pub use chapter_select::draw_chapter_select;

use crate::{
    nodes::{
        GameState,
        Actor,
    },
};

use super::GuiSkins;

pub fn draw_gui() {
    let gui_skins = storage::get::<GuiSkins>();
    root_ui().push_skin(&gui_skins.default);
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
    root_ui().pop_skin();
}
