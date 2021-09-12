use crate::gui::*;

pub fn draw_game_menu(game_state: &mut RefMut<GameState>) {
    let gui_skins = storage::get::<GuiSkins>();

    let size = vec2(200.0, 200.0) ;
    let position = get_centered_on_screen(size);

    root_ui().push_skin(&gui_skins.default);

    widgets::Window::new(hash!(), position, size)
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            let btn_size = vec2(150.0, 32.0) ;

            let resume_btn = widgets::Button::new("Resume")
                .size(btn_size)
                .ui(ui);

            if resume_btn {
                game_state.should_show_game_menu = false;
            }

            let save_btn = widgets::Button::new("Save")
                .size(btn_size)
                .ui(ui);

            if save_btn {
                game_state.should_save_character = true;
                game_state.should_show_game_menu = false;
            }

            let main_menu_btn = widgets::Button::new("Main Menu")
                .size(btn_size)
                .ui(ui);

            if main_menu_btn {
                game_state.should_go_to_main_menu = true;
            }

            let quit_btn = widgets::Button::new("Quit")
                .size(btn_size)
                .position(vec2(0.0, 118.0) )
                .ui(ui);

            if quit_btn {
                game_state.should_quit = true;
            }
        });
    root_ui().pop_skin()
}
