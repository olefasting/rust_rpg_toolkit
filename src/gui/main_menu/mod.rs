mod character_creation;
mod character_selection;
//mod chapter_selection;
mod class_selection;
mod module_management;
mod settings;

use character_creation::{draw_character_creation, draw_set_character_name};
use character_selection::{draw_character_selection, CharacterSelectionResult};
use class_selection::draw_class_selection;
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
            MainMenuResult::StartGame => match draw_character_selection().await {
                CharacterSelectionResult::SelectCharacter(character) => {
                    dispatch_event(Event::StartGame { character });
                    break 'menu;
                }
                CharacterSelectionResult::CreateCharacter => match draw_class_selection().await {
                    Some(class_id) => {
                        let game_params = storage::get::<GameParams>();
                        if game_params.skip_character_creation {
                            if let Some(name) = draw_set_character_name().await {
                                let resources = storage::get::<Resources>();
                                let class = resources.character_classes.get(&class_id).unwrap();
                                let prototype =
                                    resources.actors.get(&class.prototype_id).cloned().unwrap();

                                let params = ActorParams { name, ..prototype };

                                let character: Character = params.into();

                                character.save()?;

                                dispatch_event(Event::StartGame { character });
                                break 'menu;
                            }
                        } else if let Some(character) = draw_character_creation(&class_id).await {
                            character.save()?;

                            dispatch_event(Event::StartGame { character });
                            break 'menu;
                        }
                    }
                    None => {}
                },
                CharacterSelectionResult::Cancel => {}
            },
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

        end_frame().await;
    }

    root_ui().pop_skin();

    Ok(())
}

const OPT_START_GAME: usize = 0;
const OPT_SETTINGS: usize = 1;
const OPT_MODULES: usize = 2;
const OPT_QUIT: usize = 3;

async fn draw_main_menu() -> MainMenuResult {
    let gui_skins = storage::get::<GuiSkins>();
    let params = gui_skins
        .theme
        .menu_params
        .get("main_menu")
        .cloned()
        .unwrap();
    let builder = MenuBuilder::new(hash!(), params);

    loop {
        if let MenuResult::Index(i) = builder.build(&mut *root_ui()) {
            let res = match i {
                OPT_START_GAME => Some(MainMenuResult::StartGame),
                OPT_SETTINGS => Some(MainMenuResult::Settings),
                OPT_MODULES => Some(MainMenuResult::Modules),
                OPT_QUIT => Some(MainMenuResult::Quit),
                _ => None,
            };

            if let Some(res) = res {
                return res;
            }
        }

        end_frame().await;
    }
}
