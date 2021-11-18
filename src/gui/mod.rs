use std::ops::Deref;

pub use character::draw_character_window;
pub use confirmation_modal::draw_confirmation_modal;
pub use dialogue::draw_dialogue_window;
pub use game_menu::draw_game_menu;
pub use inventory::draw_inventory_window;
pub use macroquad::ui::{hash, root_ui, widgets, Drag, Id, Skin, Ui};
pub use main_menu::show_main_menu;
pub use theme::{GuiSkins, GuiTheme};

pub use window_builder::WindowBuilder;

pub use menu_builder::{
    MenuBuilder, MenuButtonStyle, MenuOption, MenuParams, MenuPosition, MenuResult,
};

pub use button_builder::{get_button_builder, try_get_button_builder, ButtonBuilder, ButtonStyle};

use crate::prelude::*;
pub use checkbox::Checkbox;

mod button_builder;
mod character;
mod checkbox;
mod confirmation_modal;
mod dialogue;
mod game_menu;
mod inventory;
mod main_menu;
mod menu_builder;
mod theme;
mod window_builder;

#[derive(Debug, Default, Clone)]
pub struct GuiState {
    pub should_draw_character_window: bool,
    pub should_draw_inventory_window: bool,
    pub should_draw_game_menu: bool,
}

impl GuiState {
    pub fn new() -> Self {
        GuiState::default()
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
    let bounds = vec2(get_screen_width(), get_screen_height());
    get_centered(size, bounds)
}
