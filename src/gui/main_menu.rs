use crate::gui::*;

use crate::save_games::{
    get_available_characters,
    get_available_save_games,
};

enum MainMenuSelection {
    NewGame,
    LoadGame,
    SelectedCharacter(ExportedCharacter),
    CreateCharacter,
    Cancel,
    Quit,
}

pub enum MainMenuResult {
    NewCharacter(ExportedCharacter),
    ImportedCharacter(ExportedCharacter, usize, String),
    LoadGame(SaveGame),
    Quit,
}

pub async fn draw_main_menu(params: &GameParams) -> MainMenuResult {
    let mut result = None;

    'menu: loop {
        match draw_main_menu_root().await {
            MainMenuSelection::NewGame => {
                let characters = get_available_characters(&params.characters_path).await.unwrap_or(Vec::new());
                match draw_character_select_menu(&characters).await {
                    MainMenuSelection::SelectedCharacter(character) => {
                        if let Some((chapter_i, map_id)) = draw_chapter_select_menu().await {
                            result = Some(MainMenuResult::ImportedCharacter(character, chapter_i, map_id));
                            break 'menu;
                        }
                    },
                    MainMenuSelection::CreateCharacter => {
                        if let Some(character) = draw_create_character_menu().await {
                            result = Some(MainMenuResult::NewCharacter(character));
                            break 'menu;
                        }
                    },
                    _ => {},
                }
            },
            MainMenuSelection::LoadGame => {
                let save_games = get_available_save_games(&params.saves_path).await.unwrap_or(Vec::new());
                if let Some(save_game) = draw_load_game_menu(&save_games).await {
                    result = Some(MainMenuResult::LoadGame(save_game));
                    break 'menu;
                }
                {};
            },
            MainMenuSelection::Quit => {
                result = Some(MainMenuResult::Quit);
                break 'menu;
            },
            _ => {},
        }

        next_frame().await;
    }

    root_ui().pop_skin();
    result.unwrap()
}

async fn draw_main_menu_root() -> MainMenuSelection {
    let gui_skins = storage::get::<GuiSkins>();

    root_ui().push_skin(&gui_skins.default);
    loop {
        let gui_skins = storage::get::<GuiSkins>();
        let scale = gui_skins.scale;

        let size = vec2(200.0 * scale, 300.0 * scale);
        let position = vec2((screen_width() - size.x)  / 2.0, (screen_height() - size.y) / 2.0);

        let mut selection = None;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                if ui.button(None, "New Game") {
                    selection = Some(MainMenuSelection::NewGame);
                }

                if ui.button(None, "Load Game") {
                    selection = Some(MainMenuSelection::LoadGame);
                }

                if ui.button(None, "Quit") {
                    selection = Some(MainMenuSelection::Quit);
                }
            });

        if let Some(selection) = selection {
            root_ui().pop_skin();
            return selection;
        }

        next_frame().await;
    }
}

async fn draw_character_select_menu(available_characters: &[ExportedCharacter]) -> MainMenuSelection {
    let gui_skins = storage::get::<GuiSkins>();

    root_ui().push_skin(&gui_skins.default);
    loop {
        let gui_skins = storage::get::<GuiSkins>();
        let scale = gui_skins.scale;

        let size = vec2(200.0 * scale, 300.0 * scale);
        let position = vec2((screen_width() - size.x)  / 2.0, (screen_height() - size.y) / 2.0);

        let mut result = None;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "New Game");

                ui.separator();

                for character in available_characters {
                    if ui.button(None, &character.actor.name) {
                        result = Some(MainMenuSelection::SelectedCharacter(character.clone()));
                    }
                }

                if ui.button(None, "Create Character") {
                    result = Some(MainMenuSelection::CreateCharacter);
                }

                if ui.button(None, "Cancel") {
                    result = Some(MainMenuSelection::Cancel);
                }
            });

        if let Some(selection) = result {
            root_ui().pop_skin();
            return selection;
        }

        next_frame().await;
    }
}

async fn draw_load_game_menu(available_saves: &[SaveGame]) -> Option<SaveGame> {
    let gui_skins = storage::get::<GuiSkins>();

    root_ui().push_skin(&gui_skins.default);
    loop {
        let gui_skins = storage::get::<GuiSkins>();
        let scale = gui_skins.scale;

        let size = vec2(200.0 * scale, 300.0 * scale);
        let position = vec2((screen_width() - size.x)  / 2.0, (screen_height() - size.y) / 2.0);

        let mut result = None;
        let mut should_cancel = false;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "Load Game");

                ui.separator();

                for save_game in available_saves {
                    if ui.button(None, &save_game.filename) {
                        result = Some(save_game.clone());
                    }
                }

                if ui.button(None, "Cancel") {
                    result = None;
                    should_cancel = true
                }
            });

        if result.is_some() || should_cancel {
            root_ui().pop_skin();
            if should_cancel {
                return None;
            }
            return result;
        }

        next_frame().await;
    }
}

pub async fn draw_create_character_menu() -> Option<ExportedCharacter> {
    let gui_skins = storage::get::<GuiSkins>();

    let resources = storage::get::<Resources>();
    let game_params = storage::get::<GameParams>();
    let mut build_points = game_params.character_build_points;
    let mut character = resources.actors.get(&game_params.new_character_prototype_id).cloned().unwrap();

    root_ui().push_skin(&gui_skins.character);
    loop {
        let gui_skins = storage::get::<GuiSkins>();
        let scale = gui_skins.scale;

        let size = vec2(250.0 * scale, 300.0 * scale);
        let position = vec2((screen_width() - size.x)  / 2.0, (screen_height() - size.y) / 2.0);

        let mut result = None;
        let mut should_cancel = false;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "Create Character");

                ui.separator();

                ui.label(None, &format!("Build points: {}", build_points));

                ui.separator();

                //root_ui().push_skin(&gui_skins.character);

                ui.label(None , &format!("STR: {}", character.strength));
                if ui.button(vec2(180.0, 35.0) * scale, "-") {
                    if character.strength > 6 {
                        character.strength -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 35.0) * scale, "+") {
                    if build_points > 0 {
                        character.strength += 1;
                        build_points -= 1;
                    }
                }

                ui.label(None, &format!("DEX: {}", character.dexterity));
                if ui.button(vec2(180.0, 50.0) * scale, "-") {
                    if character.dexterity > 6 {
                        character.dexterity -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 50.0) * scale, "+") {
                    if build_points > 0 {
                        character.dexterity += 1;
                        build_points -= 1;
                    }
                }

                ui.label(None, &format!("CON: {}", character.constitution));
                if ui.button(vec2(180.0, 65.0) * scale, "-") {
                    if character.constitution > 6 {
                        character.constitution -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 65.0) * scale, "+") {
                    if build_points > 0 {
                        character.constitution += 1;
                        build_points -= 1;
                    }
                }

                ui.label(None, &format!("INT: {}", character.intelligence));
                if ui.button(vec2(180.0, 80.0) * scale, "-") {
                    if character.intelligence > 6 {
                        character.intelligence -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 80.0) * scale, "+") {
                    if build_points > 0 {
                        character.intelligence += 1;
                        build_points -= 1;
                    }
                }

                ui.label(None, &format!("WIL: {}", character.willpower));
                if ui.button(vec2(180.0, 95.0) * scale, "-") {
                    if character.willpower > 6 {
                        character.willpower -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 95.0) * scale, "+") {
                    if build_points > 0 {
                        character.willpower += 1;
                        build_points -= 1;
                    }
                }

                ui.label(None, &format!("PER: {}", character.perception));
                if ui.button(vec2(180.0, 110.0) * scale, "-") {
                    if character.perception > 6 {
                        character.perception -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 110.0) * scale, "+") {
                    if build_points > 0 {
                        character.perception += 1;
                        build_points -= 1;
                    }
                }

                ui.label(None, &format!("CHA: {}", character.charisma));
                if ui.button(vec2(180.0, 125.0) * scale, "-") {
                    if character.charisma > 6 {
                        character.charisma -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 125.0) * scale, "+") {
                    if build_points > 0 {
                        character.charisma += 1;
                        build_points -= 1;
                    }
                }

               // root_ui().pop_skin();

                ui.separator();

                if ui.button(None, "Done") {
                    if build_points == 0 {
                        result = Some(character.clone().into());
                    }
                }

                if ui.button(None, "Cancel") {
                    result = None;
                    should_cancel = true;
                }
            });

        if result.is_some() || should_cancel {
            root_ui().pop_skin();
            if should_cancel {
                return None;
            }
            return result;
        }

        next_frame().await;
    }
}

async fn draw_chapter_select_menu() -> Option<(usize, String)> {
    let gui_skins = storage::get::<GuiSkins>();
    let scenario = storage::get::<Scenario>();

    root_ui().push_skin(&gui_skins.default);
    loop {
        let gui_skins = storage::get::<GuiSkins>();
        let scale = gui_skins.scale;

        let size = vec2(200.0 * scale, 300.0 * scale);
        let position = vec2((screen_width() - size.x)  / 2.0, (screen_height() - size.y) / 2.0);

        let mut result = None;
        let mut should_cancel = false;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "Chapter Select");

                ui.separator();

                for i in 0..scenario.chapters.len() {
                    let chapter = scenario.chapters.get(i).unwrap();
                    if ui.button(None, &chapter.title.clone()) {
                        result = Some((i, chapter.initial_map_id.clone()));
                    }
                }

                if ui.button(None, "Cancel") {
                    should_cancel = true;
                }
            });

        if result.is_some() || should_cancel {
            root_ui().pop_skin();
            if should_cancel {
                return None;
            } else {
                return result;
            }
        }

        next_frame().await;
    }
}
