mod character_selection;
mod character_creation;
mod chapter_selection;
mod module_management;
mod settings;

use character_selection::draw_character_select_menu;
use character_creation::draw_create_character_menu;
use chapter_selection::draw_chapter_select_menu;
use module_management::draw_module_management_menu;
use settings::draw_settings_menu;

use crate::gui::*;

pub(crate) enum MainMenuSelection {
    StartGame,
    SelectCharacter(SavedCharacter),
    CreateCharacter,
    Settings,
    Modules,
    Cancel,
    Quit,
}

pub enum MainMenuResult {
    StartGame(SceneTransition),
    Quit,
}

pub async fn draw_main_menu() -> MainMenuResult {
    let gui_skins = storage::get::<GuiSkins>();
    root_ui().push_skin(&gui_skins.default);

    let mut result = None;

    loop {
        match draw_main_menu_root().await {
            MainMenuSelection::StartGame => {
                match draw_character_select_menu().await {
                    MainMenuSelection::SelectCharacter(player) => {
                        if let Some(transition_params) = draw_chapter_select_menu().await {
                            let transition = SceneTransition::new(player, transition_params);

                            result = Some(MainMenuResult::StartGame(transition));
                        }
                    }
                    MainMenuSelection::CreateCharacter => {
                        if let Some(player) = draw_create_character_menu().await {
                            let resources = storage::get::<Resources>();
                            let chapter = resources.chapters.first().cloned().unwrap();

                            let (chapter_index, map_id) = (0, chapter.initial_map_id);
                            let transition = SceneTransition { player, chapter_index, map_id };

                            result = Some(MainMenuResult::StartGame(transition));
                        }
                    }
                    _ => {}
                }
            }
            MainMenuSelection::Settings => {
                draw_settings_menu().await;
            }
            MainMenuSelection::Modules => {
                draw_module_management_menu().await;
            }
            MainMenuSelection::Quit => {
                result = Some(MainMenuResult::Quit);
            }
            _ => {}
        }

        if let Some(result) = result {
            root_ui().pop_skin();

            return result;
        }

        next_frame().await;
    }
}

async fn draw_main_menu_root() -> MainMenuSelection {
    loop {
        let mut selection = None;

        let gui_skins = storage::get::<GuiSkins>();
        let params = gui_skins.theme.menu_params.get("main_menu").unwrap();
        if let Some(res) = WindowBuilder::new_menu(&mut *root_ui(), hash!("main_menu"), params) {
            selection = match res {
                0 => Some(MainMenuSelection::StartGame),
                1 => Some(MainMenuSelection::Settings),
                2 => Some(MainMenuSelection::Modules),
                3 => Some(MainMenuSelection::Quit),
                _ => None,
            };
        }

        if let Some(selection) = selection {
            return selection;
        }

        next_frame().await;
    }
}
