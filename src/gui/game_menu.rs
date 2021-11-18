use crate::gui::*;

const GAME_MENU_OPT_RESUME: usize = 0;
const GAME_MENU_OPT_SAVE: usize = 1;
const GAME_MENU_OPT_MAIN_MENU: usize = 2;
const GAME_MENU_OPT_QUIT: usize = 3;

pub fn draw_game_menu() {
    if let Some(mut game_state) = scene::find_node_by_type::<GameState>() {
        if game_state.gui_state.should_draw_game_menu {
            let gui_skins = storage::get::<GuiSkins>();
            let params = gui_skins
                .theme
                .menu_params
                .get("game_menu")
                .cloned()
                .unwrap();
            let builder = MenuBuilder::new(hash!(), params);

            root_ui().push_skin(&gui_skins.default);

            match builder.build(&mut *root_ui()) {
                MenuResult::Index(i) => match i {
                    GAME_MENU_OPT_RESUME => {
                        game_state.gui_state.should_draw_game_menu = false;
                    }
                    GAME_MENU_OPT_SAVE => {
                        game_state.gui_state.should_draw_game_menu = false;
                        dispatch_event(Event::Save);
                    }
                    GAME_MENU_OPT_MAIN_MENU => {
                        dispatch_event(Event::OpenMainMenu);
                    }
                    GAME_MENU_OPT_QUIT => {
                        dispatch_event(Event::Quit);
                    }
                    _ => {}
                },
                _ => {}
            }

            root_ui().pop_skin();
        }
    }
}
