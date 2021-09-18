mod character_selection;
mod character_creation;
//mod chapter_selection;
mod module_management;
mod settings;

use character_selection::{CharacterSelectionResult, draw_character_selection};
use character_creation::draw_character_creation;
use module_management::show_module_management;
use settings::show_settings;

use crate::gui::*;

enum MainMenuResult {
    StartGame,
    Settings,
    Modules,
    Quit,
}

pub async fn show_main_menu() -> Result<()> {
    let gui_skins = storage::get::<GuiSkins>();
    root_ui().push_skin(&gui_skins.default);

    'menu: loop {
        match draw_main_menu().await {
            MainMenuResult::StartGame => {
                match draw_character_selection().await {
                    CharacterSelectionResult::SelectCharacter(character) => {
                        load_scene(character)?;
                        break 'menu;
                    }
                    CharacterSelectionResult::CreateCharacter => {
                        if let Some(character) = draw_character_creation().await {
                            load_scene(character)?;
                            break 'menu;
                        }
                    }
                    _ => {}
                }
            }
            MainMenuResult::Settings => {
                show_settings().await;
            }
            MainMenuResult::Modules => {
                show_module_management().await;
            }
            MainMenuResult::Quit => {
                dispatch_event(Event::Quit);
                break 'menu;
            }
        }

        next_frame().await;
    }

    root_ui().pop_skin();

    Ok(())
}

const OPT_START_GAME: usize = 0;
const OPT_SETTINGS: usize = 1;
const OPT_MODULES: usize = 2;
const OPT_QUIT: usize = 3;

async fn draw_main_menu() -> MainMenuResult {
    loop {
        let mut result = None;

        let gui_skins = storage::get::<GuiSkins>();
        let params = gui_skins.theme.menu_params.get("main_menu").cloned().unwrap();
        if let Some(selection) = MenuBuilder::new(hash!(), params).build(&mut *root_ui()) {
            result = match selection {
                OPT_START_GAME => Some(MainMenuResult::StartGame),
                OPT_SETTINGS => Some(MainMenuResult::Settings),
                OPT_MODULES => Some(MainMenuResult::Modules),
                OPT_QUIT => Some(MainMenuResult::Quit),
                _ => None,
            };
        }

        if let Some(result) = result {
            return result;
        }

        next_frame().await;
    }
}
