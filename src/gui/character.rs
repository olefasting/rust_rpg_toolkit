use macroquad::{
    ui::{
        hash, root_ui,
        widgets::{self},
    },
    prelude::*,
};

use crate::{
    nodes::Actor,
};

pub fn draw_character_window(player: &Actor) {
    widgets::Window::new(hash!(), vec2(50.0, 150.0), vec2(300.0, 300.0))
        .label(&player.name)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("STR: {}", player.stats.strength));
            ui.label(None, &format!("DEX: {}", player.stats.dexterity));
            ui.label(None, &format!("CON: {}", player.stats.constitution));
            ui.label(None, &format!("INT: {}", player.stats.intelligence));
            ui.label(None, &format!("WIL: {}", player.stats.willpower));
            ui.label(None, &format!("PER: {}", player.stats.perception));
            ui.label(None, &format!("CHA: {}", player.stats.charisma));

            ui.separator();

            ui.tree_node(hash!(), "Regeneration", |ui| {
                ui.label(None, &format!("Health:  {}", player.stats.health_regen));
                ui.label(None, &format!("Stamina: {}", player.stats.stamina_regen));
                ui.label(None, &format!("Energy:  {}", player.stats.energy_regen));
            });
        });
}
