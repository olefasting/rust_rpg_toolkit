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
    widgets::Window::new(hash!(), vec2(50.0 * scale, 475.0 * scale), vec2(300.0 * scale, 300.0 * scale))
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("credits: {}, weight: {}/{}", player.inventory.credits, player.inventory.get_total_weight(), player.stats.carry_capacity));
            {
                let items = player.inventory.get_all_of_kind(&[ItemKind::OneHandedWeapon, ItemKind::TwoHandedWeapon]);
                if items.len() > 0 {
                    ui.tree_node(hash!(), "Weapons", |ui| {
                        for item in &items {
                            ui.label(None, &item.params.name);
                            if ui.button(None, "Equip") {
                                let ability = item.to_actor_ability();
                                match ability.action_kind {
                                    ActionKind::Primary =>
                                        player.primary_ability = Some(ability),
                                    ActionKind::Secondary =>
                                        player.secondary_ability = Some(ability),
                                }
                            }
                            if item.params.is_quest_item == false {
                                if ui.button(None, "Drop") {
                                    let position = player.body.position;
                                    player.inventory.drop(&item.id, position);
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
                            if ui.button(None, "Equip") {
                                let ability = item.to_actor_ability();
                                match ability.action_kind {
                                    ActionKind::Primary =>
                                        player.primary_ability = Some(ability),
                                    ActionKind::Secondary =>
                                        player.secondary_ability = Some(ability),
                                }
                            }
                            if item.params.is_quest_item == false {
                                if ui.button(None, "Drop") {
                                    let position = player.body.position;
                                    player.inventory.drop(&item.id, position);
                                }
                            }
                        }
                    });
                }
            }
        });
}
