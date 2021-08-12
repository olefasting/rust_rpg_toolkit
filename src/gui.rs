use macroquad::{
    ui::{
        hash, root_ui,
        widgets::{self, Group},
        Drag, Ui,
    },
    prelude::*,
};

use crate::nodes::{GameState, Actor};

pub fn draw_gui() {
    let game_state = scene::find_node_by_type::<GameState>().unwrap();
    let player = Actor::find_local_player().unwrap();
    if game_state.show_character_window {
        widgets::Window::new(hash!(), vec2(50.0, 50.0), vec2(300.0, 300.0))
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
            });
    }
    if game_state.show_inventory_window {
        widgets::Window::new(hash!(), vec2(50.0, 375.0), vec2(300.0, 300.0))
            .label("Inventory")
            .ui(&mut *root_ui(), |ui| {
            });
    }
}
