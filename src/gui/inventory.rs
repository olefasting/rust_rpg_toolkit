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
    },
    ability::ActionKind,
};
use crate::nodes::item::ItemKind;

pub fn draw_inventory_window(player: &mut Actor) {
    widgets::Window::new(hash!(), vec2(50.0, 475.0), vec2(300.0, 300.0))
        .label("Inventory")
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("weight: {}/{}", player.inventory.get_total_weight(), player.stats.carry_capacity));
            {
                let items = player.inventory.get_all_of_kind(&[ItemKind::OneHandedWeapon, ItemKind::TwoHandedWeapon]);
                if items.len() > 0 {
                    ui.tree_node(hash!(), "Weapons", |ui| {
                        for item in &items {
                            ui.label(None, &item.params.name);
                            if ui.button(None, "Equip") {
                                match item.params.ability.action_kind {
                                    ActionKind::Primary =>
                                        player.primary_ability = Some(item.to_actor_ability()),
                                    ActionKind::Secondary =>
                                        player.secondary_ability = Some(item.to_actor_ability()),
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
                let items = player.inventory.get_all_of_kind(&[ItemKind::Misc]);
                if items.len() > 0 {
                    ui.tree_node(hash!(), "Miscellaneous", |ui| {
                        for item in &items {
                            ui.label(None, &item.params.name);
                            if ui.button(None, "Equip") {
                                match item.params.ability.action_kind {
                                    ActionKind::Primary =>
                                        player.primary_ability = Some(item.to_actor_ability()),
                                    ActionKind::Secondary =>
                                        player.secondary_ability = Some(item.to_actor_ability()),
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
        });
}
