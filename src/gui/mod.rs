pub use macroquad::{
    ui::{
        Ui,
        hash,
        root_ui,
        widgets::{
            self,
            Window,
            Group,
        },
        Skin,
    },
};

use crate::prelude::*;

mod inventory;
mod character;
mod dialogue;
mod game_menu;

pub mod skins;
pub mod main_menu;

use game_menu::draw_game_menu;
use dialogue::draw_dialogue_window;
use inventory::draw_inventory_window;
use character::draw_character_window;

pub use main_menu::{
    draw_main_menu,
    MainMenuResult,
};

pub fn draw_gui() {
    let _gui_skins = storage::get::<GuiSkins>();
    if let Some(mut game_state) = scene::find_node_by_type::<GameState>() {
        if let Some(mut player) = Actor::find_by_player_id(&game_state.local_player_id) {
            if game_state.should_show_character_window {
                draw_character_window(&*player);
            }
            if game_state.should_show_inventory_window {
                draw_inventory_window(&mut *player);
            }
            draw_dialogue_window(&mut *player);
            if game_state.should_show_game_menu {
                draw_game_menu(&mut game_state);
            }
        }
    }
}
