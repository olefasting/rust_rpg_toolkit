use std::ops::Deref;

use regex::Regex;

use crate::gui::*;

enum MainMenuSelection {
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
                draw_modules_menu().await;
            }
            MainMenuSelection::Quit => {
                result = Some(MainMenuResult::Quit);
            }
            _ => {}
        }

        if let Some(result) = result {
            return result;
        }

        next_frame().await;
    }
}

async fn draw_main_menu_root() -> MainMenuSelection {
    loop {
        let gui_skins = storage::get::<GuiSkins>();
        root_ui().push_skin(&gui_skins.default);

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
            root_ui().pop_skin();
            return selection;
        }

        next_frame().await;
    }
}

async fn draw_character_select_menu() -> MainMenuSelection {
    let gui_skins = storage::get::<GuiSkins>();
    root_ui().push_skin(&gui_skins.default);

    let mut characters = {
        let game_params = storage::get::<GameParams>();
        get_available_characters(&game_params.characters_path).await.unwrap_or(Vec::new())
    };

    characters.sort_by(|a, b| a.actor.name.cmp(&b.actor.name));

    let mut delete_character_index = None;

    loop {
        if let Some(i) = delete_character_index {
            let character: SavedCharacter = characters.get(i).cloned().unwrap();

            let size = vec2(200.0, 150.0);
            let position = get_centered_on_screen(size);

            widgets::Window::new(hash!(), position, size)
                .titlebar(false)
                .ui(&mut *root_ui(), |ui| {
                    ui.label(None, "Do you want to delete");
                    ui.label(None, &format!("'{}'?", &character.actor.name));

                    let yes_btn = widgets::Button::new("Yes")
                        .position(vec2(0.0, 68.0))
                        .size(vec2(72.0, 28.0))
                        .ui(ui);

                    if yes_btn {
                        delete_character(&character.actor.name);
                        characters.remove(i);
                        delete_character_index = None;
                    }

                    let cancel_btn = widgets::Button::new("Cancel")
                        .position(vec2(77.0, 68.0))
                        .size(vec2(72.0, 28.0))
                        .ui(ui);

                    if cancel_btn {
                        delete_character_index = None;
                    }
                });
        } else {
            let size = vec2(400.0, 500.0);
            let position = get_centered_on_screen(size);

            let mut result = None;

            widgets::Window::new(hash!(), position, size)
                .titlebar(false)
                .ui(&mut *root_ui(), |ui| {
                    ui.push_skin(&gui_skins.header_label);
                    ui.label(None, "Start Game");
                    ui.pop_skin();

                    ui.separator();

                    Group::new(hash!(), vec2(350.0, 372.0)).position(vec2(0.0, 45.0)).ui(ui, |ui| {
                        for i in 0..characters.len() {
                            let character = characters.get(i).cloned().unwrap();

                            let y_offset = i as f32 * 22.0;

                            ui.push_skin(&gui_skins.label_button);
                            if ui.button(vec2(2.0, y_offset), character.actor.name.deref()) {
                                result = Some(MainMenuSelection::SelectCharacter(character.clone()));
                            }
                            ui.pop_skin();

                            ui.push_skin(&gui_skins.condensed_button);
                            if ui.button(vec2(320.0, y_offset), "X") {
                                delete_character_index = Some(i);
                            }
                            ui.pop_skin();
                        }
                    });

                    ui.separator();

                    let btn_size = vec2(170.0, 28.0);

                    let new_btn = widgets::Button::new("New")
                        .size(btn_size)
                        .position(vec2(0.0, 425.0))
                        .ui(ui);

                    if new_btn {
                        result = Some(MainMenuSelection::CreateCharacter);
                    }

                    let cancel_btn = widgets::Button::new("Cancel")
                        .size(btn_size)
                        .position(vec2(175.0, 425.0))
                        .ui(ui);

                    if cancel_btn {
                        result = Some(MainMenuSelection::Cancel);
                    }
                });

            if let Some(selection) = result {
                root_ui().pop_skin();
                return selection;
            }
        }

        next_frame().await;
    }
}

fn draw_character_attribute(ui: &mut Ui, i: usize, name: &str, value: &mut u32, build_points: &mut u32) {
    let gui_skins = storage::get::<GuiSkins>();

    let y_offset = i as f32 * 22.0;

    ui.label(vec2(2.0, y_offset - 2.0), &format!("{}: {}", name, value));

    ui.push_skin(&gui_skins.condensed_button);

    if *value > 6 {
        if ui.button(vec2(58.0, y_offset), "-") {
            *value -= 1;
            *build_points += 1;
        }
    } else {
        ui.push_skin(&gui_skins.condensed_button_inactive);
        ui.button(vec2(58.0, y_offset), "-");
        ui.pop_skin();
    }

    if *build_points > 0 {
        if ui.button(vec2(74.0, y_offset), "+") {
            *value += 1;
            *build_points -= 1;
        }
    } else {
        ui.push_skin(&gui_skins.condensed_button_inactive);
        ui.button(vec2(74.0, y_offset), "+");
        ui.pop_skin();
    }

    ui.pop_skin();
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
async fn is_name_in_use(name: &str) -> bool {
    let game_params = storage::get::<GameParams>();
    get_available_characters(&game_params.characters_path)
        .await
        .unwrap()
        .into_iter()
        .find(|character| character.actor.name == name)
        .is_some()
}

#[cfg(target_family = "wasm")]
async fn is_name_in_use(_name: &str) -> bool {
    false
}

pub async fn draw_create_character_menu() -> Option<SavedCharacter> {
    let gui_skins = storage::get::<GuiSkins>();
    root_ui().push_skin(&gui_skins.default);

    let resources = storage::get::<Resources>();
    let game_params = storage::get::<GameParams>();

    let mut build_points = game_params.new_character_build_points;
    let mut character = resources.actors.get(&game_params.new_character_prototype_id).cloned().unwrap();
    let mut is_permadeath = false;

    let mut should_show_build_points_warning = false;
    let mut should_show_name_warning = false;

    loop {
        let gui_skins = storage::get::<GuiSkins>();

        let size = vec2(320.0, 350.0);
        let position = get_centered_on_screen(size);

        let mut result = None;
        let mut should_cancel = false;

        let is_name_in_use = is_name_in_use(&character.name).await;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.push_skin(&gui_skins.header_label);
                ui.label(None, "Create Character");
                ui.pop_skin();

                ui.separator();

                ui.push_skin(&gui_skins.big_editbox);
                ui.input_text(hash!(), "", &mut character.name);
                ui.pop_skin();

                ui.separator();

                ui.label(None, &format!("Build points: {}", build_points));

                Group::new(hash!(), vec2(96.0, 154.0)).position(vec2(0.0, 84.0)).ui(ui, |ui| {
                    draw_character_attribute(ui, 0, "STR", &mut character.strength, &mut build_points);
                    draw_character_attribute(ui, 1, "DEX", &mut character.dexterity, &mut build_points);
                    draw_character_attribute(ui, 2, "CON", &mut character.constitution, &mut build_points);
                    draw_character_attribute(ui, 3, "INT", &mut character.intelligence, &mut build_points);
                    draw_character_attribute(ui, 4, "WIL", &mut character.willpower, &mut build_points);
                    draw_character_attribute(ui, 5, "PER", &mut character.perception, &mut build_points);
                    draw_character_attribute(ui, 6, "CHA", &mut character.charisma, &mut build_points);
                });

                Group::new(hash!(), vec2(165.0, 154.0)).position(vec2(100.0, 84.0)).ui(ui, |ui| {
                    draw_checkbox(ui, hash!(), vec2(0.0, 130.0), "Hardcore", &mut is_permadeath);
                });

                if should_show_build_points_warning {
                    ui.push_skin(&gui_skins.warning_label);
                    ui.label(vec2(0.0, 243.0), "You have unspent build points!");
                    ui.pop_skin();
                } else if should_show_name_warning {
                    ui.push_skin(&gui_skins.warning_label);
                    ui.label(vec2(0.0, 243.0), "Name is already in use!");
                    ui.pop_skin();
                }

                let btn_size = vec2(133.0, 28.0);

                if is_name_in_use || build_points > 0 {
                    ui.push_skin(&gui_skins.inactive_button);
                    let done_btn = widgets::Button::new("Done")
                        .size(btn_size)
                        .position(vec2(0.0, 275.0))
                        .ui(ui);

                    if done_btn {
                        should_show_name_warning = is_name_in_use;
                        should_show_build_points_warning = build_points > 0;
                    }
                    ui.pop_skin();
                } else {
                    let done_btn = widgets::Button::new("Done")
                        .size(btn_size)
                        .position(vec2(0.0, 275.0))
                        .ui(ui);

                    if done_btn {
                        let mut export: SavedCharacter = character.clone().into();
                        export.is_permadeath = is_permadeath;
                        result = Some(export);
                    }
                }

                let cancel_btn = widgets::Button::new("Cancel")
                    .size(btn_size)
                    .position(vec2(138.0, 275.0))
                    .ui(ui);

                if cancel_btn {
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
        root_ui().push_skin(&gui_skins.default);

        let size = vec2(200.0, 250.0);
        let position = get_centered_on_screen(size);

        let mut result = None;
        let mut should_cancel = false;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.push_skin(&gui_skins.header_label);
                ui.label(None, "Select Chapter");
                ui.pop_skin();

                ui.separator();

                let resources = storage::get::<Resources>();

                widgets::Group::new(hash!(), vec2(150.0, 144.0)).position(vec2(0.0, 27.0)).ui(ui, |ui| {
                    let len = resources.chapters.len();

                    let btn_width = if len > 4 {
                        140.0
                    } else {
                        150.0
                    };

                    for i in 0..len {
                        let chapter = resources.chapters.get(i).unwrap();

                        let chapter_btn = widgets::Button::new(chapter.title.deref())
                            .size(vec2(btn_width, 28.0))
                            .ui(ui);

                        if chapter_btn {
                            let params = SceneTransitionParams {
                                chapter_index: i,
                                map_id: chapter.initial_map_id.clone(),
                            };
                            result = Some(params);
                        }
                    }
                });

                let cancel_btn = widgets::Button::new("Cancel")
                    .position(vec2(0.0, 175.0))
                    .size(vec2(150.0, 28.0))
                    .ui(ui);

                if cancel_btn {
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

async fn draw_settings_menu() {
    let gui_skins = storage::get::<GuiSkins>();
    root_ui().push_skin(&gui_skins.default);

    let mut config = storage::get::<Config>().deref().clone();

    let mut will_require_restart = false;

    let mut should_save = false;
    let mut should_cancel = false;

    let size = vec2(320.0, 320.0);
    let position = get_centered_on_screen(size);

    let mut resolution_x_str = config.resolution.x.to_string();
    let mut resolution_y_str = config.resolution.y.to_string();

    let mut fullscreen_cfg = config.fullscreen;

    let resolution_regex = Regex::new(r"^[0-9]*$").unwrap();

    loop {
        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.push_skin(&gui_skins.header_label);
                ui.label(None, "Settings");
                ui.pop_skin();

                ui.label(None, "Resolution");
                ui.editbox(hash!(), vec2(42.0, 18.0), &mut resolution_x_str);

                ui.same_line(48.0);
                ui.label(None, "x");

                ui.same_line(58.0);
                ui.editbox(hash!(), vec2(42.0, 18.0), &mut resolution_y_str);

                draw_checkbox(ui, hash!(), None, "Fullscreen", &mut fullscreen_cfg);

                ui.separator();

                // ui.label(None, "UI Scale");
                // ui.editbox(hash!(), vec2(32.0, 18.0), &mut gui_scale_str);
                //
                // ui.same_line(36.0);
                // if config.gui_scale > Config::MIN_GUI_SCALE {
                //     ui.push_skin(&gui_skins.condensed_button);
                //     if ui.button(None, "-") {
                //         let new_scale = ((config.gui_scale - Config::GUI_SCALE_STEP) * 100.0).round() / 100.0;
                //         config.gui_scale = new_scale.clamp(Config::MIN_GUI_SCALE, Config::MAX_GUI_SCALE);
                //         gui_scale_str = config.gui_scale.to_string();
                //     }
                //     ui.pop_skin();
                // } else {
                //     ui.push_skin(&gui_skins.condensed_button_inactive);
                //     ui.button(None, "-");
                //     ui.pop_skin();
                // }
                //
                //
                // ui.same_line(52.0);
                // if config.gui_scale < Config::MAX_GUI_SCALE {
                //     ui.push_skin(&gui_skins.condensed_button);
                //     if ui.button(None, "+") {
                //         let new_scale = ((config.gui_scale + Config::GUI_SCALE_STEP) * 100.0).round() / 100.0;
                //         config.gui_scale = new_scale.clamp(Config::MIN_GUI_SCALE, Config::MAX_GUI_SCALE);
                //         gui_scale_str = config.gui_scale.to_string();
                //     }
                //     ui.pop_skin();
                // } else {
                //     ui.push_skin(&gui_skins.condensed_button_inactive);
                //     ui.button(None, "+");
                //     ui.pop_skin();
                // }

                if will_require_restart {
                    ui.push_skin(&gui_skins.warning_label);
                    ui.label(vec2(0.0, 213.0), "Changes require a restart!");
                    ui.pop_skin();
                }

                let btn_size = vec2(132.0, 28.0);

                let save_btn = widgets::Button::new("Save")
                    .position(vec2(0.0, 245.0))
                    .size(btn_size)
                    .ui(ui);

                let cancel_btn = widgets::Button::new("Cancel")
                    .position(vec2(137.0, 245.0))
                    .size(btn_size)
                    .ui(ui);

                should_save = save_btn;
                should_cancel = cancel_btn;
            });


        if resolution_regex.is_match(&resolution_x_str) == false {
            resolution_x_str = config.resolution.x.to_string();
        }

        if resolution_regex.is_match(&resolution_y_str) == false {
            resolution_y_str = config.resolution.y.to_string();
        }

        let resolution = uvec2(
            resolution_x_str.parse().unwrap(),
            resolution_y_str.parse().unwrap(),
        );

        will_require_restart = resolution != config.resolution || fullscreen_cfg != config.fullscreen;

        if should_save || should_cancel {
            root_ui().pop_skin();

            if should_save {
                config.resolution = uvec2(
                    resolution_x_str.parse().unwrap(),
                    resolution_y_str.parse().unwrap(),
                );

                config.fullscreen = fullscreen_cfg;

                storage::store(config.clone());
            }

            return;
        }

        next_frame().await;
    }
}

fn draw_module_entry(ui: &mut Ui, i: usize, name: &str, params: &ModuleParams, value: &mut bool, is_dragging: bool) -> Drag {
    let gui_skins = storage::get::<GuiSkins>();

    ui.push_skin(&gui_skins.module_list_entry);

    let size = vec2(550.0, 24.0);
    let position = vec2(0.0, i as f32 * 28.0);

    let (entry_id, drop_before_id) = module_index_to_id(i);

    widgets::Group::new(drop_before_id, vec2(size.x, 4.0))
        .position(position)
        .draggable(false)
        .hoverable(is_dragging && *value)
        .ui(ui, |_| {});

    let label = format!("{} ({}) [{}]", params.title, name, params.version);

    let drag = widgets::Group::new(entry_id, size)
        .position(position + vec2(0.0, 4.0))
        .draggable(*value)
        .hoverable(is_dragging && *value)
        .ui(ui, |ui| {
            draw_checkbox(ui, hash!(entry_id, "checkbox"), vec2(2.0, 0.0), &label, value);
        });

    ui.pop_skin();

    drag
}

#[derive(Debug, Copy, Clone)]
enum LoadOrderChange {
    LoadBefore { i: usize, target_i: usize },
    LoadAfter { i: usize, target_i: usize },
}

// Returns a modules index in the active_modules vector, calculated from either the entry
// id, or the id of the drop-zone before the entry, as well as a bool that will be true
// if the id was for the modules entry in the module list
fn id_to_module_index(id: u64) -> (usize, bool) {
    if id % 2 == 0 {
        ((id as usize / 2) - 1, true)
    } else {
        (((id as usize - 1) / 2) - 1, false)
    }
}

// Returns two ids, the first for the group holding the module entry in the list and the
// second for the drop-zone before the entry, letting you drop a module before another in
// the load order
fn module_index_to_id(i: usize) -> (u64, u64) {
    let id = (i as u64 + 1) * 2;
    (id, id + 1)
}

async fn draw_modules_menu() {
    let gui_skins = storage::get::<GuiSkins>();
    root_ui().push_skin(&gui_skins.default);

    let mut will_require_restart = false;

    let mut should_save = false;
    let mut should_cancel = false;

    let size = vec2(500.0, 600.0);
    let position = get_centered_on_screen(size);

    let game_params = storage::get::<GameParams>();

    let available_modules = get_available_modules(&game_params.modules_path).unwrap();

    let active_modules_file_path = format!("{}/active_modules.json", &game_params.modules_path);
    let bytes = load_file(&active_modules_file_path).await.unwrap();
    let mut active_modules = serde_json::from_slice::<Vec<String>>(&bytes)
        .unwrap()
        .into_iter()
        .filter(|module| available_modules.contains_key(module))
        .collect::<Vec<String>>();

    let mut module_state: HashMap<String, bool> = HashMap::from_iter(
        available_modules.iter().map(|(name, _)| (name.clone(), active_modules.contains(name))));

    let mut is_dragging = false;
    let mut load_order_change = None;

    loop {
        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.push_skin(&gui_skins.header_label);
                ui.label(None, "Modules");
                ui.pop_skin();

                let size = vec2(450.0, 550.0);

                widgets::Group::new(hash!(), size).position(vec2(0.0, 26.0)).ui(ui, |ui| {
                    let mut i = 0;
                    for name in &active_modules {
                        if let Some(module) = available_modules.get(name) {
                            let value = module_state.get_mut(name).unwrap();

                            match draw_module_entry(ui, i, &name, &module, value, is_dragging) {
                                Drag::Dropped(_, Some(id)) => {
                                    is_dragging = false;

                                    let (target_i, is_entry) = id_to_module_index(id);
                                    load_order_change = if is_entry {
                                        Some(LoadOrderChange::LoadAfter { i, target_i })
                                    } else {
                                        Some(LoadOrderChange::LoadBefore { i, target_i })
                                    };
                                }
                                Drag::Dropped(_, _) => {
                                    is_dragging = false;
                                }
                                Drag::Dragging(_pos, _id) => {
                                    is_dragging = true;
                                }
                                _ => {}
                            }

                            i += 1;
                        }
                    }

                    for (name, module) in &available_modules {
                        if active_modules.contains(name) == false {
                            let value = module_state.get_mut(name).unwrap();
                            draw_module_entry(ui, i, &name, &module, value, false);

                            i += 1;
                        }
                    }
                });

                if will_require_restart {
                    ui.push_skin(&gui_skins.warning_label);
                    ui.label(vec2(0.0, 493.0), "Changes require a restart!");
                    ui.pop_skin();
                }

                let btn_size = vec2(222.0, 32.0);

                let save_btn = widgets::Button::new("Save")
                    .position(vec2(0.0, 520.0))
                    .size(btn_size)
                    .ui(ui);

                let cancel_btn = widgets::Button::new("Cancel")
                    .position(vec2(226.0, 520.0))
                    .size(btn_size)
                    .ui(ui);

                should_save = save_btn;
                should_cancel = cancel_btn;
            });

        if let Some(load_order_change) = load_order_change {
            match load_order_change {
                LoadOrderChange::LoadBefore { i, target_i } => {
                    let entry = active_modules.remove(i);

                    let target_i = if i < target_i {
                        target_i - 1
                    } else {
                        target_i
                    };

                    active_modules.insert(target_i, entry);
                }
                LoadOrderChange::LoadAfter { i, target_i } => {
                    let entry = active_modules.remove(i);

                    let target_i = if i < target_i {
                        target_i
                    } else {
                        target_i + 1
                    };

                    active_modules.insert(target_i, entry);
                }
            };

            will_require_restart = true;
        }

        load_order_change = None;

        active_modules.retain(|module| *module_state.get(module).unwrap_or(&false));

        for (name, state) in &module_state {
            if *state && active_modules.contains(name) == false {
                active_modules.push(name.clone());
                will_require_restart = true;
            }
        }

        if should_save || should_cancel {
            root_ui().pop_skin();

            #[cfg(not(any(target_os = "android", target_family = "wasm")))]
            if should_save {
                let json = serde_json::to_string_pretty(&active_modules).unwrap();
                fs::write(active_modules_file_path, &json).unwrap();
            }

            return;
        }

        next_frame().await;
    }
}
