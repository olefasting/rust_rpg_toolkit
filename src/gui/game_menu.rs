use macroquad::{
    ui::{
        hash, root_ui,
        widgets::{self},
    },
    experimental::{
        scene::RefMut,
        collections::storage,
    },
    prelude::*,
};

use crate::{
    nodes::GameState,
    Resources,
};

pub fn draw_game_menu(game_state: &mut RefMut<GameState>) {
    let x = screen_width() / 2.0 - 50.0;
    let y = screen_height() / 2.0 - 50.0;
    let resources = storage::get::<Resources>();
    root_ui().push_skin(&resources.gui_skins.main_menu);
    widgets::Window::new(hash!(), vec2(x, y), vec2(100.0, 100.0))
        .label("Menu")
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Menu");
            if ui.button(None, "Quit") {
                game_state.should_quit = true;
            }
        });
    root_ui().pop_skin();
}
