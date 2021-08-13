use macroquad::{
    ui::{
        hash, root_ui,
        widgets::{self},
    },
    prelude::*,
};

use crate::{
    nodes::{GameState, Actor},
    Item,
};
use crate::actions::ActionParams;

pub fn draw_gui() {
    let game_state = scene::find_node_by_type::<GameState>().unwrap();
    let mut player = Actor::find_local_player().unwrap();
    if game_state.show_character_window {
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
    if game_state.show_inventory_window {
        widgets::Window::new(hash!(), vec2(50.0, 475.0), vec2(300.0, 300.0))
            .label("Inventory")
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, &format!("weight: {}/{}", player.inventory.get_total_weight(), player.stats.carry_capacity));
                {
                    let items = player.inventory.get_all_of_kind(Item::WEAPON_KINDS);
                    if items.len() > 0 {
                        ui.tree_node(hash!(), "Weapons", |ui| {
                            for item in &items {
                                ui.label(None, &item.params.name);
                                if ui.button(None, "Equip") {
                                    if item.params.action_params.action_kind == ActionParams::PRIMARY_ABILITY.to_string() {
                                        player.primary_ability = item.to_actor_ability();
                                    } else if item.params.action_params.action_kind == ActionParams::SECONDARY_ABILITY.to_string() {
                                        player.secondary_ability = item.to_actor_ability();
                                    }
                                }
                                if ui.button(None, "Drop") {
                                    let position = player.body.position;
                                    player.inventory.drop_item(&item.instance_id, position);
                                }
                            }
                        });
                    }
                }
                {
                    let items = player.inventory.get_all_of_kind(&[Item::MISC_KIND]);
                    if items.len() > 0 {
                        ui.tree_node(hash!(), "Miscellaneous", |ui| {
                            for item in &items {
                                ui.label(None, &item.params.name);
                                if ui.button(None, "Equip") {
                                    if item.params.action_params.action_kind == ActionParams::PRIMARY_ABILITY.to_string() {
                                        player.primary_ability = item.to_actor_ability();
                                    } else if item.params.action_params.action_kind == ActionParams::SECONDARY_ABILITY.to_string() {
                                        player.secondary_ability = item.to_actor_ability();
                                    }
                                }
                                if ui.button(None, "Drop") {
                                    let position = player.body.position;
                                    player.inventory.drop_item(&item.instance_id, position);
                                }
                            }
                        });
                    }
                }
                {
                    let items = player.inventory.get_all_of_kind(&[Item::QUEST_KIND]);
                    if items.len() > 0 {
                        ui.tree_node(hash!(), "Quest Items", |ui| {
                            for item in &items {
                                ui.label(None, &item.params.name);
                            }
                        });
                    }
                }
            });
    }
}
