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
    let gui_skins = storage::get::<GuiSkins>();

    root_ui().push_skin(&gui_skins.default);

    loop {
        let gui_skins = storage::get::<GuiSkins>();
        let scale = gui_skins.scale;

        let size = vec2(200.0 * scale, 300.0 * scale);
        let position = get_centered_on_screen(size);

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

        let size = vec2(200.0, 300.0) * scale;
        let position = get_centered_on_screen(size);

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

fn draw_character_attribute(ui: &mut Ui, scale: f32, name: &str, value: &mut u32, build_points: &mut u32) {
    let size = vec2(250.0, 20.0) * scale;

    Group::new(hash!(name), size).ui(ui, |ui| {
        let position = vec2(60.0, 0.0);

        ui.label(None, &format!("{}: {}", name, value));
        if ui.button(position * scale, "-") {
            if *value > 6 {
                *value -= 1;
                *build_points += 1;
            }
        }
        if ui.button(vec2(position.x + 22.0, position.y) * scale, "+") {
            if *build_points > 0 {
                *value += 1;
                *build_points -= 1;
            }
        }
    });
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

        let size = vec2(250.0, 320.0) * scale;
        let position = get_centered_on_screen(size);

        let mut result = None;
        let mut should_cancel = false;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "Create Character");
                ui.separator();

                ui.input_text(hash!(), "", &mut character.name);
                ui.separator();

                ui.label(None, &format!("Build points: {}", build_points));
                ui.separator();

                draw_character_attribute(ui, scale, "STR", &mut character.strength, &mut build_points);
                draw_character_attribute(ui, scale, "DEX", &mut character.dexterity, &mut build_points);
                draw_character_attribute(ui, scale, "CON", &mut character.constitution, &mut build_points);
                draw_character_attribute(ui, scale, "INT", &mut character.intelligence, &mut build_points);
                draw_character_attribute(ui, scale, "WIL", &mut character.willpower, &mut build_points);
                draw_character_attribute(ui, scale, "PER", &mut character.perception, &mut build_points);
                draw_character_attribute(ui, scale, "CHA", &mut character.charisma, &mut build_points);

                ui.separator();

                if ui.button(vec2(0.0, 255.0) * scale, "Done") {
                    if build_points == 0 {
                        result = Some(character.clone().into());
                    }
                }

                if ui.button(vec2(40.0, 255.0) * scale, "Cancel") {
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

    root_ui().push_skin(&gui_skins.default);

    loop {
        let gui_skins = storage::get::<GuiSkins>();
        let scale = gui_skins.scale;

        let size = vec2(200.0, 300.0) * scale;
        let position = get_centered_on_screen(size);

        let mut result = None;
        let mut should_cancel = false;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "Chapter Select");

                ui.separator();

                let resources = storage::get::<Resources>();

                let mut i = 0;
                for chapter in &resources.chapters {
                    if ui.button(None, &chapter.title.clone()) {
                        let params = SceneTransitionParams {
                            chapter_index: i,
                            map_id: chapter.initial_map_id.clone(),
                        };
                        result = Some(params);
                    }
                    i += 0;
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
