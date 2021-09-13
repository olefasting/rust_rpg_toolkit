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
        let size = vec2(250.0, 250.0);
        let position = get_centered_on_screen(size);

        let mut selection = None;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                let btn_size = vec2(200.0, 32.0);

                let start_game_btn = widgets::Button::new("Start Game")
                    .size(btn_size)
                    .ui(ui);

                if start_game_btn {
                    selection = Some(MainMenuSelection::StartGame);
                }

                let settings_btn = widgets::Button::new("Settings")
                    .size(btn_size)
                    .ui(ui);

                if settings_btn {
                    selection = Some(MainMenuSelection::Settings);
                }

                let modules_btn = widgets::Button::new("Modules")
                    .size(btn_size)
                    .ui(ui);

                if modules_btn {
                    selection = Some(MainMenuSelection::Modules);
                }

                let quit_btn = widgets::Button::new("Quit")
                    .size(btn_size)
                    .position(vec2(0.0, 168.0))
                    .ui(ui);

                if quit_btn {
                    selection = Some(MainMenuSelection::Quit);
                }
            });

        if let Some(selection) = selection {
            return selection;
        }

        next_frame().await;
    }
}
