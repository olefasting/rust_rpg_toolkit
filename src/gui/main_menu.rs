use crate::gui::*;

use crate::saved_character::{
    get_available_characters,
};

enum MainMenuSelection {
    StartGame,
    SelectCharacter(SavedCharacter),
    CreateCharacter,
    Cancel,
    Quit,
}

pub enum MainMenuResult {
    StartGame(SceneTransition),
    Quit,
}

pub async fn draw_main_menu(params: &GameParams) -> MainMenuResult {
    let mut result = None;

    loop {
        match draw_main_menu_root().await {
            MainMenuSelection::StartGame => {
                let characters = get_available_characters(&params.characters_path).await.unwrap_or(Vec::new());
                match draw_character_select_menu(&characters).await {
                    MainMenuSelection::SelectCharacter(player) => {
                        if let Some(transition_params) = draw_chapter_select_menu().await {
                            let transition = SceneTransition::new(player, transition_params);
                            result = Some(MainMenuResult::StartGame(transition));
                        }
                    },
                    MainMenuSelection::CreateCharacter => {
                        if let Some(player) = draw_create_character_menu().await {
                            let chapter = storage::get::<Scenario>().chapters.first().cloned().unwrap();
                            let (chapter_index, map_id) = (chapter.index, chapter.initial_map_id);
                            let transition = SceneTransition { player, chapter_index, map_id };
                            result = Some(MainMenuResult::StartGame(transition));
                        }
                    },
                    _ => {},
                }
            },
            MainMenuSelection::Quit => {
                result = Some(MainMenuResult::Quit);
            },
            _ => {},
        }

        if let Some(result) = result {
            root_ui().pop_skin();
            return result;
        }

        next_frame().await;
    }
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
                if ui.button(None, "Start Game") {
                    selection = Some(MainMenuSelection::StartGame);
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

async fn draw_character_select_menu(available_characters: &[SavedCharacter]) -> MainMenuSelection {
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
                        result = Some(MainMenuSelection::SelectCharacter(character.clone()));
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

pub async fn draw_create_character_menu() -> Option<SavedCharacter> {
    let gui_skins = storage::get::<GuiSkins>();
    let resources = storage::get::<Resources>();
    let game_params = storage::get::<GameParams>();
    let mut build_points = game_params.new_character_build_points;
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

                ui.input_text(hash!(), "", &mut character.name);

                ui.separator();

                ui.label(None, &format!("Build points: {}", build_points));

                ui.separator();

                //root_ui().push_skin(&gui_skins.character);

                ui.label(None , &format!("STR: {}", character.strength));
                if ui.button(vec2(180.0, 50.0) * scale, "-") {
                    if character.strength > 6 {
                        character.strength -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 50.0) * scale, "+") {
                    if build_points > 0 {
                        character.strength += 1;
                        build_points -= 1;
                    }
                }

                ui.label(None, &format!("DEX: {}", character.dexterity));
                if ui.button(vec2(180.0, 65.0) * scale, "-") {
                    if character.dexterity > 6 {
                        character.dexterity -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 65.0) * scale, "+") {
                    if build_points > 0 {
                        character.dexterity += 1;
                        build_points -= 1;
                    }
                }

                ui.label(None, &format!("CON: {}", character.constitution));
                if ui.button(vec2(180.0, 80.0) * scale, "-") {
                    if character.constitution > 6 {
                        character.constitution -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 80.0) * scale, "+") {
                    if build_points > 0 {
                        character.constitution += 1;
                        build_points -= 1;
                    }
                }

                ui.label(None, &format!("INT: {}", character.intelligence));
                if ui.button(vec2(180.0, 95.0) * scale, "-") {
                    if character.intelligence > 6 {
                        character.intelligence -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 95.0) * scale, "+") {
                    if build_points > 0 {
                        character.intelligence += 1;
                        build_points -= 1;
                    }
                }

                ui.label(None, &format!("WIL: {}", character.willpower));
                if ui.button(vec2(180.0, 110.0) * scale, "-") {
                    if character.willpower > 6 {
                        character.willpower -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 110.0) * scale, "+") {
                    if build_points > 0 {
                        character.willpower += 1;
                        build_points -= 1;
                    }
                }

                ui.label(None, &format!("PER: {}", character.perception));
                if ui.button(vec2(180.0, 125.0) * scale, "-") {
                    if character.perception > 6 {
                        character.perception -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 125.0) * scale, "+") {
                    if build_points > 0 {
                        character.perception += 1;
                        build_points -= 1;
                    }
                }

                ui.label(None, &format!("CHA: {}", character.charisma));
                if ui.button(vec2(180.0, 140.0) * scale, "-") {
                    if character.charisma > 6 {
                        character.charisma -= 1;
                        build_points += 1;
                    }
                }
                if ui.button(vec2(202.0, 140.0) * scale, "+") {
                    if build_points > 0 {
                        character.charisma += 1;
                        build_points -= 1;
                    }
                }

               // root_ui().pop_skin();

                ui.separator();

                if ui.button(vec2(0.0, 260.0) * scale, "Done") {
                    if build_points == 0 {
                        result = Some(character.clone().into());
                    }
                }

                if ui.button(vec2(48.0, 260.0) * scale, "Cancel") {
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

async fn draw_chapter_select_menu() -> Option<SceneTransitionParams> {
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

                for chapter in scenario.chapters.clone() {
                    if ui.button(None, &chapter.title.clone()) {
                        let (chapter_index, map_id) = (chapter.index, chapter.initial_map_id);
                        let params = SceneTransitionParams { chapter_index, map_id };
                        result = Some(params);
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
