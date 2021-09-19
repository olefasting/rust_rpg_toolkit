use crate::gui::*;

const GAME_MENU_OPT_RESUME: usize = 0;
const GAME_MENU_OPT_SAVE: usize = 1;
const GAME_MENU_OPT_MAIN_MENU: usize = 2;
const GAME_MENU_OPT_QUIT: usize = 3;

pub fn draw_game_menu() {
    if let Some(mut game_state) = scene::find_node_by_type::<GameState>() {
        if game_state.gui.should_draw_game_menu {
            let gui_skins = storage::get::<GuiSkins>();

            root_ui().push_skin(&gui_skins.default);

            let params = gui_skins.theme.menu_params.get("game_menu").cloned().unwrap();
            if let Some(selection) = MenuBuilder::new(hash!(), params).build(&mut *root_ui()) {
                match selection {
                    GAME_MENU_OPT_RESUME => {
                        game_state.gui.should_draw_game_menu = false;
                    }
                    GAME_MENU_OPT_SAVE => {
                        game_state.gui.should_draw_game_menu = false;
                        dispatch_event(Event::SavePlayerCharacter);
                    }
                    GAME_MENU_OPT_MAIN_MENU => {
                        dispatch_event(Event::MainMenu);
                    }
                    GAME_MENU_OPT_QUIT => {
                        dispatch_event(Event::Quit);
                    }
                    _ => {}
                }
            }

            root_ui().pop_skin();
        }
    }
}
