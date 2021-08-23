use macroquad::{
    ui::{
        hash, root_ui,
        widgets::{self},
    },
    prelude::*,
};

use crate::nodes::Actor;

pub fn draw_interaction_window(player: &mut Actor) {
    let interaction = player.current_interaction.clone();
    if let Some(interaction) = interaction {
        widgets::Window::new(hash!(), vec2(50.0, 150.0), vec2(300.0, 300.0))
            //.label(&player.name)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None,&interaction.body);
                let options = interaction.get_options(player);
                for option in options {
                    if ui.button(None, option.title.clone()) {
                        option.apply_action(player);
                        player.current_interaction = Some(option);
                    }
                }
            });
    }
}
