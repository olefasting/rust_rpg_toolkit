use macroquad::{
    ui::{
        hash, root_ui,
        widgets::{self},
    },
    experimental::{
        scene::RefMut,
    },
    prelude::*,
};

use crate::{
    nodes::GameState,
};

pub fn draw_game_menu(scale: f32, game_state: &mut RefMut<GameState>) {
    let x = screen_width() / 2.0 - 75.0 * scale;
    let y = screen_height() / 2.0 - 100.0 * scale;
    widgets::Window::new(hash!(), vec2(x, y), vec2(150.0 * scale, 200.0 * scale))
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            if ui.button(None, "Quit") {
                game_state.should_quit = true;
            }
        });
}
