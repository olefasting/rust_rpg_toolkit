use std::ops::Deref;

use regex::Regex;

use crate::gui::*;

pub(crate) async fn draw_settings_menu() {
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
                config.save();
            }

            return;
        }

        next_frame().await;
    }
}
