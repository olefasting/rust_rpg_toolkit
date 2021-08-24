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
    let x = screen_width() / 2.0 - 75.0;
    let y = screen_height() / 2.0 - 100.0;
    widgets::Window::new(hash!(), vec2(x, y), vec2(150.0, 200.0))
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            if ui.button(None, "Quit") {
                game_state.should_quit = true;
            }
        });
}
