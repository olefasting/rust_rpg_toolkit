use crate::gui::*;

pub fn draw_game_menu(game_state: &mut RefMut<GameState>) {
    let gui_skins = storage::get::<GuiSkins>();
    let scale = gui_skins.scale;
    let x = screen_width() / 2.0 - 75.0 * scale;
    let y = screen_height() / 2.0 - 100.0 * scale;
    root_ui().push_skin(&gui_skins.default);
    widgets::Window::new(hash!(), vec2(x, y), vec2(150.0 * scale, 200.0 * scale))
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            if ui.button(None, "Resume") {
                game_state.should_show_game_menu = false;
            }

            if ui.button(None, "Save") {
                game_state.should_save_character = true;
                game_state.should_show_game_menu = false;
            }

            if ui.button(None, "Quit") {
                game_state.should_quit = true;
            }
        });
    root_ui().pop_skin()
}
