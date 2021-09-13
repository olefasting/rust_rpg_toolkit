use crate::gui::*;

pub fn draw_dialogue_window(player: &mut Actor) {
    let gui_skins = storage::get::<GuiSkins>();
    let interaction = player.current_dialogue.clone();

    let size = vec2(400.0, 350.0) ;
    let position = get_centered_on_screen(size);

    if let Some(interaction) = interaction {
        root_ui().push_skin(&gui_skins.default);

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                if interaction.body.len() > 0 {
                    ui.label(None, &format!("{}:", player.name));
                }

                for line in interaction.body.clone() {
                    ui.label(None, &format!(" {}", line));
                }

                ui.separator();

                if interaction.response.len() > 0 {
                    ui.label(None, &format!("{}:", interaction.actor_name));
                }

                for line in  interaction.response.clone() {
                    ui.label(None, &format!(" {}", line));
                }

                ui.separator();

                let options = interaction.get_options(player);
                if options.len() == 0 {
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
