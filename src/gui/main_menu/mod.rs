mod character_selection;
mod character_creation;
mod chapter_selection;
mod module_management;
mod settings;

use character_selection::{CharacterSelectionResult, draw_character_selection_menu};
use character_creation::draw_character_creation_menu;
use chapter_selection::draw_chapter_selection_menu;
use module_management::show_module_management_menu;
use settings::show_settings_menu;

use crate::gui::*;

enum MainMenuSelection {
    StartGame,
    Settings,
    Modules,
    Quit,
}

pub async fn show_main_menu() {
    let gui_skins = storage::get::<GuiSkins>();
    root_ui().push_skin(&gui_skins.default);

    'menu: loop {
        match draw_main_menu_root().await {
            MainMenuSelection::StartGame => {
                match draw_character_selection_menu().await {
                    CharacterSelectionResult::SelectCharacter(character) => {
                        if let Some(params) = draw_chapter_selection_menu().await {
                            dispatch_event(Event::CreateGame(character, params.chapter_index, params.map_id));
                            break 'menu;
                        }
                    }
                    CharacterSelectionResult::CreateCharacter => {
                        if let Some(character) = draw_character_creation_menu().await {
                            let resources = storage::get::<Resources>();
                            let chapter = resources.chapters.first().cloned().unwrap();
                            dispatch_event(Event::CreateGame(character, 0, chapter.initial_map_id));
                            break 'menu;
                        }
                    }
                    _ => {}
                }
            }
            MainMenuSelection::Settings => {
                show_settings_menu().await;
            }
            MainMenuSelection::Modules => {
                show_module_management_menu().await;
            }
            MainMenuSelection::Quit => {
                dispatch_event(Event::Quit);
                break 'menu;
            }
            _ => {}
        }

        next_frame().await;
    }

    root_ui().pop_skin();
}

const MAIN_MENU_OPT_START_GAME: usize = 0;
const MAIN_MENU_OPT_SETTINGS: usize = 1;
const MAIN_MENU_OPT_MODULES: usize = 2;
const MAIN_MENU_OPT_QUIT: usize = 3;

async fn draw_main_menu_root() -> MainMenuSelection {
    loop {
        let mut result = None;

        let gui_skins = storage::get::<GuiSkins>();
        let params = gui_skins.theme.menu_params.get("main_menu").cloned().unwrap();
        if let Some(selection) = MenuBuilder::new(hash!(), params).build(&mut *root_ui()) {
            result = match selection {
                MAIN_MENU_OPT_START_GAME => Some(MainMenuSelection::StartGame),
                MAIN_MENU_OPT_SETTINGS => Some(MainMenuSelection::Settings),
                MAIN_MENU_OPT_MODULES => Some(MainMenuSelection::Modules),
                MAIN_MENU_OPT_QUIT => Some(MainMenuSelection::Quit),
                _ => None,
            };
        }

        if let Some(result) = result {
            return result;
        }

        next_frame().await;
    }
}
