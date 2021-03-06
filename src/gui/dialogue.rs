use crate::gui::*;

pub fn draw_dialogue_window() {
    if let Some(mut player) = get_player_actor() {
        if let Some(dialogue) = player.current_dialogue.clone() {
            let gui_skins = storage::get::<GuiSkins>();

            let size = vec2(400.0, 350.0);

            root_ui().push_skin(&gui_skins.default);

            WindowBuilder::new(hash!(), size)
                .with_centered_pos(true)
                .build(&mut *root_ui(), |ui| {
                    if !dialogue.body.is_empty() {
                        ui.label(None, &format!("{}:", player.name));
                    }

                    for line in dialogue.body.clone() {
                        ui.label(None, &format!(" {}", line));
                    }

                    ui.separator();

                    if !dialogue.response.is_empty() {
                        ui.label(None, &format!("{}:", dialogue.actor_name));
                    }

                    for line in dialogue.response.clone() {
                        ui.label(None, &format!(" {}", line));
                    }

                    ui.separator();

                    let options = dialogue.get_options(&player);
                    if options.is_empty() {
                        if ui.button(None, "Continue") {
                            player.current_dialogue = None;
                        }
                    } else {
                        for mut option in options {
                            if ui.button(None, &*option.title.clone()) {
                                option.should_apply = true;
                                player.current_dialogue = Some(option);
                            }
                        }
                    }
                });

            root_ui().pop_skin();
        }
    }
}
