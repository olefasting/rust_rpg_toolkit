use crate::gui::*;

pub fn draw_character_window() {
    if let Some(game_state) = scene::find_node_by_type::<GameState>() {
        if game_state.gui_state.should_draw_character_window {
            if let Some(player) = get_player_actor() {
                let gui_skins = storage::get::<GuiSkins>();

                let size = vec2(300.0, 300.0);
                let position = vec2(50.0, 150.0);

                root_ui().push_skin(&gui_skins.default);

                WindowBuilder::new(hash!(), size)
                    .with_pos(position, false)
                    .build(&mut *root_ui(), |ui| {
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
        }
    }
}
