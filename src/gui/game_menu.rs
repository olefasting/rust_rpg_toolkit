use crate::gui::*;

pub fn draw_game_menu(game_state: &mut RefMut<GameState>) {
    let gui_skins = storage::get::<GuiSkins>();

    let size = vec2(200.0, 200.0) ;
    let position = get_centered_on_screen(size);

    root_ui().push_skin(&gui_skins.default);

    let params = gui_skins.theme.menu_params.get("game_menu").unwrap();
    if let Some(i) = WindowBuilder::new_menu(&mut *root_ui(), hash!(), params) {
        match i {
            0 => {
                game_state.should_show_game_menu = false;
            }
            1 => {
                game_state.should_save_character = true;
                game_state.should_show_game_menu = false;
            }
            2 => {
                game_state.should_go_to_main_menu = true;
            }
            3 => {
                game_state.should_quit = true;
            }
            _ => {}
        }
    }

    root_ui().pop_skin()
}
