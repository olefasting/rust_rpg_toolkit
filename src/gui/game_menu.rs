use crate::gui::*;

pub fn draw_game_menu(game_state: &mut RefMut<GameState>) {
    let gui_skins = storage::get::<GuiSkins>();
    let scale = gui_skins.scale;

    let size = vec2(150.0, 200.0) * scale;
    let position = get_centered_on_screen(size);

    root_ui().push_skin(&gui_skins.main_menu);

    widgets::Window::new(hash!(), position, size)
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            if ui.button(None, "Resume") {
                game_state.should_show_game_menu = false;
            }

            if ui.button(None, "Save") {
                game_state.should_save_character = true;
                game_state.should_show_game_menu = false;
            }

            if ui.button(None, "Main Menu") {
                game_state.should_go_to_main_menu = true;
            }

            if ui.button(None, "Quit") {
                game_state.should_quit = true;
            }
        });
    root_ui().pop_skin()
}
