use crate::gui::*;

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

fn draw_module_entry(ui: &mut Ui, i: usize, name: &str, params: &ModuleParams, value: &mut bool, is_dragging: bool) -> Drag {
    let gui_skins = storage::get::<GuiSkins>();

    let module_list_entry_skin = gui_skins.custom.get("module_list_entry").unwrap();

    ui.push_skin(module_list_entry_skin);

    let size = vec2(450.0, 24.0);
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
            Checkbox::new(hash!(entry_id, "checkbox"), vec2(2.0, 0.0), &label, value)
                .with_inactive_label()
                .ui(ui);
        });

    ui.pop_skin();

    drag
}

pub(crate) async fn show_module_management_menu() {
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

                widgets::Group::new(hash!(), size).position(vec2(0.0, 48.0)).ui(ui, |ui| {
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
