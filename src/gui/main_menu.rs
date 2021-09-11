use crate::gui::*;

use crate::saved_character::{get_available_characters, delete_character};

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

    let gui_skins = storage::get::<GuiSkins>();

    root_ui().push_skin(&gui_skins.main_menu);

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
            return selection;
        }

        next_frame().await;
    }
}

async fn draw_delete_character_modal(name: &str, scale: f32) -> bool {
    let mut res = None;

    let size = vec2(200.0, 200.0) * scale;
    let position = get_centered_on_screen(size);

    loop {
        widgets::Window::new(hash!(name), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, &format!("Are you sure you want to delete '{}'?", name));

                if ui.button(None, "Yes") {
                    res = Some(true);
                }

                if ui.button(None, "No") {
                    res = Some(false);
                }
            });

        if let Some(res) = res {
            return res;
        }

        next_frame().await;
    }
}

async fn draw_character_select_menu() -> MainMenuSelection {
    let mut characters = {
        let game_params = storage::get::<GameParams>();
        get_available_characters(&game_params.characters_path).await.unwrap_or(Vec::new())
    };

    characters.sort_by(|a, b| a.actor.name.cmp(&b.actor.name));

    let mut delete_character_index = None;

    loop {
        let gui_skins = storage::get::<GuiSkins>();
        let scale = gui_skins.scale;

        let size = vec2(250.0, 300.0) * scale;
        let position = get_centered_on_screen(size);

        let mut result = None;

        if let Some(i) = delete_character_index {
            let character: SavedCharacter = characters.get(i).cloned().unwrap();

            let size = vec2(200.0, 150.0) * scale;
            let position = get_centered_on_screen(size);

            widgets::Window::new(hash!(), position, size)
                .titlebar(false)
                .ui(&mut *root_ui(), |ui| {
                    ui.label(None, "Do you want to delete");
                    ui.label(None, &format!("'{}'?", &character.actor.name));

                    if ui.button(vec2(12.0, 70.0) * scale, "Yes") {
                        delete_character(&character.actor.name);
                        characters.remove(i);
                        delete_character_index = None;
                    }

                    if ui.button(vec2(68.0, 70.0) * scale, "Cancel") {
                        delete_character_index = None;
                    }
                });
        } else {
            widgets::Window::new(hash!(), position, size)
                .titlebar(false)
                .ui(&mut *root_ui(), |ui| {
                    ui.label(None, "New Game");

                    ui.separator();

                    ui.push_skin(&gui_skins.default);

                    Group::new(hash!(), vec2(200.0, 22.0) * scale).position(vec2(0.0, 30.0) * scale).ui(ui, |ui| {
                        ui.push_skin(&gui_skins.label_button);
                        if ui.button(vec2(2.0, 0.0) * scale, "Create character") {
                            result = Some(MainMenuSelection::CreateCharacter);
                        }
                        ui.pop_skin();
                    });

                    Group::new(hash!(), vec2(200.0, 150.0) * scale).position(vec2(0.0, 58.0) * scale).ui(ui, |ui| {
                        for i in 0..characters.len() {
                            let character = characters.get(i).cloned().unwrap();

                            let y_offset = i as f32 * 22.0;

                            ui.push_skin(&gui_skins.label_button);
                            if ui.button(vec2(2.0, y_offset) * scale, &character.actor.name) {
                                result = Some(MainMenuSelection::SelectCharacter(character.clone()));
                            }
                            ui.pop_skin();

                            ui.push_skin(&gui_skins.condensed_button);
                            if ui.button(vec2(170.0, y_offset) * scale, "X") {
                                delete_character_index = Some(i);
                            }
                            ui.pop_skin();
                        }
                    });

                    ui.pop_skin();

                    ui.separator();

                    if ui.button(vec2(0.0, 220.0) * scale, "Cancel") {
                        result = Some(MainMenuSelection::Cancel);
                    }
                });
        }

        if let Some(selection) = result {
            return selection;
        }

        next_frame().await;
    }
}

fn draw_character_attribute(ui: &mut Ui, i: usize, name: &str, value: &mut u32, build_points: &mut u32, scale: f32) {
    let y_offset = i as f32 * 22.0 + 28.0;

    ui.label(vec2(2.0, y_offset) * scale, &format!("{}: {}", name, value));

    let gui_skins = storage::get::<GuiSkins>();
    ui.push_skin(&gui_skins.condensed_button);

    if *value > 6 {
        if ui.button(vec2(54.0, y_offset) * scale, "-") {
            *value -= 1;
            *build_points += 1;
        }
    } else {
        ui.push_skin(&gui_skins.condensed_button_inactive);
        ui.button(vec2(54.0, y_offset) * scale, "-");
        ui.pop_skin();
    }

    if *build_points > 0 {
        if ui.button(vec2(68.0, y_offset) * scale, "+") {
            *value += 1;
            *build_points -= 1;
        }
    } else {
        ui.push_skin(&gui_skins.condensed_button_inactive);
        ui.button(vec2(68.0, y_offset) * scale, "+");
        ui.pop_skin();
    }

    ui.pop_skin();
}

pub async fn draw_create_character_menu() -> Option<SavedCharacter> {
    let gui_skins = storage::get::<GuiSkins>();
    let resources = storage::get::<Resources>();
    let game_params = storage::get::<GameParams>();

    let mut build_points = game_params.new_character_build_points;
    let mut character = resources.actors.get(&game_params.new_character_prototype_id).cloned().unwrap();
    let mut is_permadeath = false;

    let mut should_show_build_points_warning = false;
    let mut should_show_name_warning = false;

    root_ui().push_skin(&gui_skins.default);

    loop {
        let gui_skins = storage::get::<GuiSkins>();
        let scale = gui_skins.scale;

        let size = vec2(320.0, 350.0) * scale;
        let position = get_centered_on_screen(size);

        let mut result = None;
        let mut should_cancel = false;

        let is_name_in_use = get_available_characters(&game_params.characters_path)
            .await
            .unwrap()
            .into_iter()
            .find(|existing| existing.actor.name == character.name)
            .is_some();

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {

                ui.label(None, "Create Character");

                ui.separator();

                ui.input_text(hash!(), "", &mut character.name);
                ui.separator();

                ui.label(None, &format!("Build points: {}", build_points));

                if should_show_build_points_warning {
                    ui.push_skin(&gui_skins.warning_text);
                    ui.label(None, "You have unspent build points!");
                    ui.pop_skin();
                } else if should_show_name_warning {
                    ui.push_skin(&gui_skins.warning_text);
                    ui.label(None, "Name is already in use!");
                    ui.pop_skin();
                }

                Group::new(hash!(), vec2(135.0, 200.0) * scale).position(vec2(0.0, 64.0) * scale).ui(ui, |ui| {
                    draw_character_attribute(ui, 0, "STR", &mut character.strength, &mut build_points, scale);
                    draw_character_attribute(ui, 1, "DEX", &mut character.dexterity, &mut build_points, scale);
                    draw_character_attribute(ui, 2, "CON", &mut character.constitution, &mut build_points, scale);
                    draw_character_attribute(ui, 3, "INT", &mut character.intelligence, &mut build_points, scale);
                    draw_character_attribute(ui, 4, "WIL", &mut character.willpower, &mut build_points, scale);
                    draw_character_attribute(ui, 5, "PER", &mut character.perception, &mut build_points, scale);
                    draw_character_attribute(ui, 6, "CHA", &mut character.charisma, &mut build_points, scale);
                });

                if is_name_in_use || build_points > 0 {
                    ui.push_skin(&gui_skins.inactive_button);
                    if ui.button(vec2(0.0, 275.0) * scale, "Done") {
                        should_show_name_warning = is_name_in_use;
                        should_show_build_points_warning = build_points > 0;
                    }
                    ui.pop_skin();
                } else {
                    if ui.button(vec2(0.0, 275.0) * scale, "Done") {
                        let mut export: SavedCharacter = character.clone().into();
                        export.is_permadeath = is_permadeath;
                        result = Some(export);
                    }
                }

                if ui.button(vec2(52.0, 275.0) * scale, "Cancel") {
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

                for i in 0..resources.chapters.len() {
                    let chapter = resources.chapters.get(i).unwrap();
                    if ui.button(None, &chapter.title.clone()) {
                        let params = SceneTransitionParams {
                            chapter_index: i,
                            map_id: chapter.initial_map_id.clone(),
                        };
                        result = Some(params);
                    }
                }

                if ui.button(None, "Cancel") {
                    should_cancel = true;
                }
            });

        if result.is_some() || should_cancel {
            if should_cancel {
                return None;
            } else {
                return result;
            }
        }

        next_frame().await;
    }
}
