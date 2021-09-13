pub use macroquad::{
    ui::{
        Drag,
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
mod skins;
mod main_menu;
mod checkbox;
mod confirmation_modal;

pub use game_menu::draw_game_menu;
pub use dialogue::draw_dialogue_window;
pub use inventory::draw_inventory_window;
pub use character::draw_character_window;
pub use skins::GuiSkins;
pub use checkbox::draw_checkbox;
pub use confirmation_modal::draw_confirmation_modal;

pub use main_menu::{
    draw_main_menu,
    MainMenuResult,
};

pub fn draw_gui() {
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

pub fn get_centered(size: Vec2, bounds: Vec2) -> Vec2 {
    (bounds - size) / 2.0
}

pub fn get_centered_on_screen(size: Vec2) -> Vec2 {
    let bounds = vec2(screen_width(), screen_height());
    get_centered(size, bounds)
}
