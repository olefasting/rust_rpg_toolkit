use macroquad::{
    ui::{
        hash, root_ui,
        widgets::{self},
    },
    prelude::*,
};

use crate::{
    nodes::{
        Actor,
        Item,
    },
    ability::Ability,
};

pub fn draw_inventory_window(player: &mut Actor) {
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
                                if item.params.ability.action_kind == Ability::PRIMARY_ABILITY.to_string() {
                                    player.primary_ability = Some(item.to_actor_ability());
                                } else if item.params.ability.action_kind == Ability::SECONDARY_ABILITY.to_string() {
                                    player.secondary_ability = Some(item.to_actor_ability());
                                }
                            }
                            if ui.button(None, "Drop") {
                                let position = player.body.position;
                                player.inventory.drop(&item.id, position);
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
                                if item.params.ability.action_kind == Ability::PRIMARY_ABILITY.to_string() {
                                    player.primary_ability = Some(item.to_actor_ability());
                                } else if item.params.ability.action_kind == Ability::SECONDARY_ABILITY.to_string() {
                                    player.secondary_ability = Some(item.to_actor_ability());
                                }
                            }
                            if ui.button(None, "Drop") {
                                let position = player.body.position;
                                player.inventory.drop(&item.id, position);
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
