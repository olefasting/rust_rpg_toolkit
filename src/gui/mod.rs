use std::ops::Deref;

pub use macroquad::{
    ui::{
        Drag,
        hash,
        root_ui,
        Skin,
        Ui,
        widgets,
        Id,
    },
};
pub use character::draw_character_window;
pub use confirmation_modal::draw_confirmation_modal;
pub use dialogue::draw_dialogue_window;
pub use game_menu::draw_game_menu;
pub use inventory::draw_inventory_window;
pub use main_menu::show_main_menu;
pub use theme::{
    GuiSkins,
    GuiTheme,
    MenuPosition,
    MenuParams,
    MenuOption,
    MenuButtonStyle,
};

pub use window_builder::{
    WindowBuilder,
    MenuBuilder,
};

pub use checkbox::Checkbox;
use crate::prelude::*;

mod inventory;
mod character;
mod dialogue;
mod game_menu;
mod theme;
mod main_menu;
mod confirmation_modal;
mod checkbox;
mod window_builder;

#[derive(Debug, Clone)]
pub struct GuiState {
    pub should_draw_character_window: bool,
    pub should_draw_inventory_window: bool,
    pub should_draw_game_menu: bool,
}

impl GuiState {
    pub fn new() -> Self {
        GuiState {
            should_draw_character_window: false,
            should_draw_inventory_window: false,
            should_draw_game_menu: false,
        }
    }
}

pub(crate) fn draw_gui() {
    draw_character_window();
    draw_inventory_window();
    draw_dialogue_window();
    draw_game_menu();
}

pub fn get_centered(size: Vec2, bounds: Vec2) -> Vec2 {
    (bounds - size) / 2.0
}

pub fn get_centered_on_screen(size: Vec2) -> Vec2 {
    let bounds = vec2(screen_width(), screen_height());
    get_centered(size, bounds)
}
