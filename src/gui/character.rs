use macroquad::{
    ui::{
        hash, root_ui,
        widgets::{self},
    },
    experimental::collections::storage,
    prelude::*,
};

use crate::{
    nodes::Actor,
};

use super::GuiSkins;

pub fn draw_character_window(player: &Actor) {
    let gui_skins = storage::get::<GuiSkins>();
    let scale = gui_skins.scale;
    root_ui().push_skin(&gui_skins.default);
    widgets::Window::new(hash!(), vec2(50.0, 150.0) * scale, vec2(300.0, 200.0) * scale)
        .titlebar(false)
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

            ui.separator();

            ui.label(None, &format!("XP: {}", player.experience));
        });
    root_ui().pop_skin();
}
