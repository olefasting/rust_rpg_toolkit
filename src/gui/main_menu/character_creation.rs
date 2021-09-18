use crate::gui::*;

const WINDOW_WIDTH: f32 = 400.0;
const WINDOW_HEIGHT: f32 = 500.0;

fn draw_character_attribute(ui: &mut Ui, i: usize, name: &str, value: &mut u32, build_points: &mut u32) {
    let gui_skins = storage::get::<GuiSkins>();

    let y_offset = i as f32 * 22.0;

    ui.label(vec2(2.0, y_offset - 2.0), &format!("{}: {}", name, value));

    let condensed_button_skin = gui_skins.custom.get("condensed_button").unwrap();
    let condensed_button_inactive_skin = gui_skins.custom.get("condensed_button_inactive").unwrap();

    if *value > 6 {
        ui.push_skin(condensed_button_skin);
        if ui.button(vec2(58.0, y_offset), "-") {
            *value -= 1;
            *build_points += 1;
        }
        ui.pop_skin();
    } else {
        ui.push_skin(condensed_button_inactive_skin);
        ui.button(vec2(58.0, y_offset), "-");
        ui.pop_skin();
    }

    if *build_points > 0 {
        ui.push_skin(condensed_button_skin);
        if ui.button(vec2(74.0, y_offset), "+") {
            *value += 1;
            *build_points -= 1;
        }
        ui.pop_skin();
    } else {
        ui.push_skin(condensed_button_inactive_skin);
        ui.button(vec2(74.0, y_offset), "+");
        ui.pop_skin();
    }
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

pub(crate) async fn draw_character_creation_menu() -> Option<CharacterExport> {
    let resources = storage::get::<Resources>();
    let game_params = storage::get::<GameParams>();
    let gui_skins = storage::get::<GuiSkins>();

    let mut build_points = game_params.new_character_build_points;
    let mut character = resources.actors.get(&game_params.new_character_prototype_id).cloned().unwrap();
    let mut is_permadeath = false;

    let mut should_show_build_points_warning = false;
    let mut should_show_name_warning = false;

    loop {
        let size = vec2(WINDOW_WIDTH, WINDOW_HEIGHT);
        let position = get_centered_on_screen(size);

        let mut res = None;
        let mut should_cancel = false;

        let is_name_in_use = is_name_in_use(&character.name).await;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.push_skin(&gui_skins.header_label);
                ui.label(None, "Create Character");
                ui.pop_skin();

                ui.separator();

                let big_editbox_skin = gui_skins.custom.get("big_editbox").unwrap();

                ui.push_skin(big_editbox_skin);
                ui.input_text(hash!(), "", &mut character.name);
                ui.pop_skin();

                ui.separator();

                ui.label(None, &format!("Build points: {}", build_points));

                let bottom_y = WINDOW_HEIGHT - GuiSkins::ELEMENT_MARGIN - GuiSkins::BUTTON_HEIGHT - GuiSkins::WINDOW_MARGIN_Y * 2.0;
                let columns_y = 84.0;
                let column_height = bottom_y - columns_y - 24.0 - GuiSkins::ELEMENT_MARGIN;

                widgets::Group::new(hash!(), vec2(96.0, column_height)).position(vec2(0.0, columns_y)).ui(ui, |ui| {
                    draw_character_attribute(ui, 0, "STR", &mut character.strength, &mut build_points);
                    draw_character_attribute(ui, 1, "DEX", &mut character.dexterity, &mut build_points);
                    draw_character_attribute(ui, 2, "CON", &mut character.constitution, &mut build_points);
                    draw_character_attribute(ui, 3, "INT", &mut character.intelligence, &mut build_points);
                    draw_character_attribute(ui, 4, "WIL", &mut character.willpower, &mut build_points);
                    draw_character_attribute(ui, 5, "PER", &mut character.perception, &mut build_points);
                    draw_character_attribute(ui, 6, "CHA", &mut character.charisma, &mut build_points);
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

                if is_name_in_use || build_points > 0 {
                    ui.push_skin(&gui_skins.inactive_button);
                    let done_btn = widgets::Button::new("Done")
                        .size(btn_size)
                        .position(vec2(0.0, btn_y))
                        .ui(ui);

                    if done_btn {
                        should_show_name_warning = is_name_in_use;
                        should_show_build_points_warning = build_points > 0;
                    }
                    ui.pop_skin();
                } else {
                    let done_btn = widgets::Button::new("Done")
                        .size(btn_size)
                        .position(vec2(0.0, btn_y))
                        .ui(ui);

                    if done_btn {
                        let mut export: CharacterExport = character.clone().into();
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
            if should_cancel {
                return None;
            }

            return res;
        }

        next_frame().await;
    }
}
