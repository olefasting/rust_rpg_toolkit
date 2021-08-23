use macroquad::{
    ui::{
        hash, root_ui,
        widgets::{self},
    },
    prelude::*,
};

use crate::nodes::Actor;

pub fn draw_dialogue_window(player: &mut Actor) {
    let interaction = player.current_dialogue.clone();
    let x = screen_width() / 2.0 - 300.0;
    let y = screen_height() / 2.0 - 225.0;
    if let Some(interaction) = interaction {
        widgets::Window::new(hash!(), vec2(x, y), vec2(600.0, 450.0))
            .label(&interaction.actor_name)
            .ui(&mut *root_ui(), |ui| {
                for line in interaction.body.clone() {
                    ui.label(None, &format!("{}", line));
                }
                if let Some(response) = interaction.response.clone() {
                    ui.label(None, &format!("{}", response));
                }
                let options = interaction.get_options(player);
                if options.len() == 0 {
                    if ui.button(None, "Bye!") {
                        player.current_dialogue = None;
                    }
                } else {
                    for option in options {
                        if ui.button(None, option.title.clone()) {
                            option.apply_action(player);
                            player.current_dialogue = Some(option);
                        }
                    }
                }
            });
    }
}
