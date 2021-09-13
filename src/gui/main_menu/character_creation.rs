use crate::gui::*;

const WINDOW_WIDTH: f32 = 400.0;
const WINDOW_HEIGHT: f32 = 500.0;

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

pub(crate) async fn draw_create_character_menu() -> Option<SavedCharacter> {
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

                Group::new(hash!(), vec2(96.0, 160.0)).position(vec2(0.0, 84.0)).ui(ui, |ui| {
                    draw_character_attribute(ui, 0, "STR", &mut character.strength, &mut build_points);
                    draw_character_attribute(ui, 1, "DEX", &mut character.dexterity, &mut build_points);
                    draw_character_attribute(ui, 2, "CON", &mut character.constitution, &mut build_points);
                    draw_character_attribute(ui, 3, "INT", &mut character.intelligence, &mut build_points);
                    draw_character_attribute(ui, 4, "WIL", &mut character.willpower, &mut build_points);
                    draw_character_attribute(ui, 5, "PER", &mut character.perception, &mut build_points);
                    draw_character_attribute(ui, 6, "CHA", &mut character.charisma, &mut build_points);
                });

                Group::new(hash!(), vec2(165.0, 160.0)).position(vec2(100.0, 84.0)).ui(ui, |ui| {
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
            if should_cancel {
                return None;
            }

            return result;
        }

        next_frame().await;
    }
}
