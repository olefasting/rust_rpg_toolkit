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
        item::ItemKind,
    },
    ability::ActionKind,
};

pub fn draw_inventory_window(scale: f32, player: &mut Actor) {
    widgets::Window::new(hash!(), vec2(50.0 * scale, 375.0 * scale), vec2(300.0 * scale, 300.0 * scale))
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("credits: {}, weight: {}/{}", player.inventory.credits, player.inventory.get_total_weight(), player.stats.carry_capacity));
            {
                let items = player.inventory.get_all_of_kind(&[ItemKind::OneHandedWeapon, ItemKind::TwoHandedWeapon]);
                if items.len() > 0 {
                    ui.tree_node(hash!(), "Weapons", |ui| {
                        for item in &items {
                            ui.label(None, &item.params.name);
                            if item.is_equipped {
                                if ui.button(None, "Unequip") {
                                    let slot = item.get_item_slot();
                                    player.unequip_item(slot);
                                }
                            } else {
                                if ui.button(None, "Equip") {
                                    let slot = item.get_item_slot();
                                    player.equip_item(&item.id, slot);
                                }
                                if item.params.is_quest_item == false {
                                    if ui.button(None, "Drop") {
                                        let slot = item.get_item_slot();
                                        player.unequip_item(slot);
                                        let position = player.body.position;
                                        player.inventory.drop(&item.id, position);
                                    }
                                }
                            }
                        }
                    });
                }
            }
            {
                let items = player.inventory.get_all_of_kind(&[ItemKind::Misc]);
                if items.len() > 0 {
                    ui.tree_node(hash!(), "Miscellaneous", |ui| {
                        for item in &items {
                            ui.label(None, &item.params.name);
                            if item.is_equipped {
                                if ui.button(None, "Unequip") {
                                    let slot = item.get_item_slot();
                                    player.unequip_item(slot);
                                }
                            } else {
                                if ui.button(None, "Equip") {
                                    let slot = item.get_item_slot();
                                    player.equip_item(&item.id, slot);
                                }
                                if item.params.is_quest_item == false {
                                    if ui.button(None, "Drop") {
                                        let slot = item.get_item_slot();
                                        player.unequip_item(slot);
                                        let position = player.body.position;
                                        player.inventory.drop(&item.id, position);
                                    }
                                }
                            }
                        }
                    });
                }
            }
        });
}
