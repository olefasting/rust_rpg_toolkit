use crate::gui::*;

const MINIMUM_CHARACTER_NAME_LENGTH: usize = 4;

fn draw_character_attribute(ui: &mut Ui, i: usize, name: &str, value: &mut u32, build_points: &mut u32) {
    let gui_skins = storage::get::<GuiSkins>();

    let y_offset = i as f32 * 22.0;

    ui.label(vec2(2.0, y_offset - 2.0), &format!("{}: {}", name, value));

    if *value > 6 {
        ui.push_skin(&gui_skins.condensed_button);
        if ui.button(vec2(58.0, y_offset), "-") {
            *value -= 1;
            *build_points += 1;
        }
        ui.pop_skin();
    } else {
        ui.push_skin(&gui_skins.condensed_button_inactive);
        ui.button(vec2(58.0, y_offset), "-");
        ui.pop_skin();
    }

    if *build_points > 0 {
        ui.push_skin(&gui_skins.condensed_button);
        if ui.button(vec2(74.0, y_offset), "+") {
            *value += 1;
            *build_points -= 1;
        }
        ui.pop_skin();
    } else {
        ui.push_skin(&gui_skins.condensed_button_inactive);
        ui.button(vec2(74.0, y_offset), "+");
        ui.pop_skin();
    }
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
async fn is_name_in_use(name: &str) -> bool {
    let game_params = storage::get::<GameParams>();
    get_available_characters(&game_params.characters_path)
        .unwrap()
        .into_iter()
        .find(|character| character.actor.name == name)
        .is_some()
}

#[cfg(target_family = "wasm")]
async fn is_name_in_use(_name: &str) -> bool {
    false
}

pub(crate) async fn draw_character_creation(class_id: &str) -> Option<Character> {
    const WINDOW_WIDTH: f32 = 400.0;
    const WINDOW_HEIGHT: f32 = 500.0;

    let resources = storage::get::<Resources>();
    let class = resources.character_classes.get(class_id).unwrap();
    let game_params = storage::get::<GameParams>();
    let gui_skins = storage::get::<GuiSkins>();

    let mut build_points = game_params.new_character_build_points;
    let mut params = resources.actors.get(&class.prototype_id).cloned().unwrap();
    let character_class_id = params.class_id.clone().unwrap();

    assert_eq!(&character_class_id, &class_id, "Character prototype (id: {}) has different class id '{}' from the class that referenced the prototype ({})!", &params.id, &class.id, &character_class_id);

    let mut is_permadeath = false;

    let mut should_show_build_points_warning = false;
    let mut should_show_name_warning = false;

    loop {
        let size = vec2(WINDOW_WIDTH, WINDOW_HEIGHT);

        let mut res = None;
        let mut should_cancel = false;

        let name_warning = get_name_warning(&params.name).await;

        WindowBuilder::new(hash!(), size)
            .with_title("Create Character")
            .with_centered_pos(true)
            .build(&mut *root_ui(), |ui| {
                ui.push_skin(&gui_skins.big_editbox);
                ui.input_text(hash!(), "", &mut params.name);
                ui.pop_skin();

                ui.separator();

                ui.label(None, &format!("Build points: {}", build_points));

                let bottom_y = WINDOW_HEIGHT - GuiSkins::ELEMENT_MARGIN - GuiSkins::BUTTON_HEIGHT - GuiSkins::WINDOW_MARGIN_Y * 2.0;
                let columns_y = 84.0;
                let column_height = bottom_y - columns_y - 24.0 - GuiSkins::ELEMENT_MARGIN;

                widgets::Group::new(hash!(), vec2(96.0, column_height)).position(vec2(0.0, columns_y)).ui(ui, |ui| {
                    draw_character_attribute(ui, 0, "STR", &mut params.strength, &mut build_points);
                    draw_character_attribute(ui, 1, "DEX", &mut params.dexterity, &mut build_points);
                    draw_character_attribute(ui, 2, "CON", &mut params.constitution, &mut build_points);
                    draw_character_attribute(ui, 3, "INT", &mut params.intelligence, &mut build_points);
                    draw_character_attribute(ui, 4, "WIL", &mut params.willpower, &mut build_points);
                    draw_character_attribute(ui, 5, "PER", &mut params.perception, &mut build_points);
                    draw_character_attribute(ui, 6, "CHA", &mut params.charisma, &mut build_points);
                });

                widgets::Group::new(hash!(), vec2(165.0, column_height)).position(vec2(100.0, columns_y)).ui(ui, |ui| {
                    //draw_checkbox(ui, hash!(), vec2(0.0, 130.0), "Hardcore", &mut is_permadeath);

                    Checkbox::new(hash!(), vec2(0.0, 130.0), "Hardcore", &mut is_permadeath).ui(ui);
                });

                if should_show_build_points_warning {
                    let label = "You have unspent build points!";
                    let warning_y = bottom_y - GuiSkins::ELEMENT_MARGIN - ui.calc_size(label).y;
                    ui.push_skin(&gui_skins.warning_label);
                    ui.label(vec2(0.0, warning_y), label);
                    ui.pop_skin();
                } else if should_show_name_warning {
                    let label = "Name is already in use!";
                    let warning_y = bottom_y - GuiSkins::ELEMENT_MARGIN - ui.calc_size(label).y;
                    ui.push_skin(&gui_skins.warning_label);
                    ui.label(vec2(0.0, warning_y), label);
                    ui.pop_skin();
                }

                let btn_y = bottom_y + GuiSkins::ELEMENT_MARGIN;
                let btn_size = vec2((WINDOW_WIDTH  - GuiSkins::ELEMENT_MARGIN) / 2.0 - GuiSkins::WINDOW_MARGIN_X, GuiSkins::BUTTON_HEIGHT);

                if name_warning.is_some() || build_points > 0 {
                    ui.push_skin(&gui_skins.inactive_button);
                    let done_btn = widgets::Button::new("Done")
                        .size(btn_size)
                        .position(vec2(0.0, btn_y))
                        .ui(ui);

                    if done_btn {
                        should_show_name_warning = name_warning.is_some();
                        should_show_build_points_warning = build_points > 0;
                    }
                    ui.pop_skin();
                } else {
                    let done_btn = widgets::Button::new("Done")
                        .size(btn_size)
                        .position(vec2(0.0, btn_y))
                        .ui(ui);

                    if done_btn {
                        let mut export: Character = params.clone().into();
                        export.is_permadeath = is_permadeath;
                        res = Some(export);
                    }
                }

                let cancel_btn = widgets::Button::new("Cancel")
                    .size(btn_size)
                    .position(vec2(btn_size.x + GuiSkins::ELEMENT_MARGIN, btn_y))
                    .ui(ui);

                if cancel_btn {
                    res = None;
                    should_cancel = true;
                }
            });

        if res.is_some() || should_cancel {
            return res;
        }

        next_frame().await;
    }
}

pub(crate) async fn draw_set_character_name() -> Option<String> {
    const WINDOW_WIDTH: f32 = 250.0;
    const WINDOW_HEIGHT: f32 = 350.0;

    let gui_skins = storage::get::<GuiSkins>();

    let size = vec2(WINDOW_WIDTH, WINDOW_HEIGHT);
    let btn_y = size.y - gui_skins.theme.button_height - (gui_skins.theme.window_margins.top + gui_skins.theme.window_margins.bottom);
    let btn_width = (size.x - (gui_skins.theme.window_margins.left + gui_skins.theme.window_margins.right) - 2.0) / 2.0;
    let btn_size = vec2(btn_width, gui_skins.theme.button_height);

    let mut name = "".to_string();

    let mut res = None;

    let mut should_cancel = false;

    let mut should_show_warning = false;

    loop {
        let warning = get_name_warning(&name).await;

        WindowBuilder::new(hash!(), size)
            .with_centered_pos(true)
            .build(&mut *root_ui(), |ui| {
                ui.label(None, "Character Name:");

                ui.push_skin(&gui_skins.big_editbox);
                ui.input_text(hash!(), "", &mut name);
                ui.pop_skin();

                let ok_btn = widgets::Button::new("Ok")
                    .position(vec2(0.0, btn_y))
                    .size(btn_size);

                if let Some(label) = warning {
                    if should_show_warning {
                        let warning_y = btn_y - ui.calc_size(&label).y;
                        ui.push_skin(&gui_skins.warning_label);
                        ui.label(vec2(0.0, warning_y), &label);
                        ui.pop_skin();
                    }

                    ui.push_skin(&gui_skins.inactive_button);
                    if ok_btn.ui(ui) {
                        should_show_warning = true;
                    }
                    ui.pop_skin();
                } else {
                    should_show_warning = false;

                    if ok_btn.ui(ui) {
                        res = Some(name.clone());
                    }
                }

                let cancel_btn = widgets::Button::new("Cancel")
                    .position(vec2(btn_width + 2.0, btn_y))
                    .size(btn_size);

                if cancel_btn.ui(ui) {
                    res = None;
                    should_cancel = true;
                }
            });

        if should_cancel || res.is_some() {
            return res;
        }

        next_frame().await;
    }
}

async fn is_name_valid(name: &str) -> bool {
    name.len() >= MINIMUM_CHARACTER_NAME_LENGTH
}

async fn get_name_warning(name: &str) -> Option<String> {
    if is_name_in_use(&name).await {
        Some("Name is already in use!".to_string())
    } else if is_name_valid(&name).await == false {
        Some("Invalid name!".to_string())
    } else {
        None
    }
}