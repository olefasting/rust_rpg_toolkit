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
    save_games::SaveGame,
};

use super::GuiSkins;

pub fn draw_game_menu(game_state: &mut RefMut<GameState>) {
    let gui_skins = storage::get::<GuiSkins>();
    let scale = gui_skins.scale;
    let x = screen_width() / 2.0 - 75.0 * scale;
    let y = screen_height() / 2.0 - 100.0 * scale;
    widgets::Window::new(hash!(), vec2(x, y), vec2(150.0 * scale, 200.0 * scale))
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            if ui.button(None, "Save Game") {
                game_state.save_game_to = Some("save_game.json".to_string());
                game_state.show_game_menu = false;
            }

            if ui.button(None, "Quit") {
                game_state.should_quit = true;
            }
        });
}
