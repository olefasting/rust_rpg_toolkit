use crate::gui::prelude::*;
use crate::prelude::*;

fn draw_entry(ui: &mut Ui, _scale: f32, player: &mut Actor, entry: &InventoryEntry) {
    //Group::new(hash!(), vec2(270.0 * scale, 50.0 * scale)).ui(ui, |ui| {
    ui.label(None, &entry.params.name);
    if entry.equipped_to == EquipmentSlot::None {
        if ui.button(None, "Equip") {
            player.equip_item(&entry.params.id);
        }
        // if item.params.is_quest_item == false {
        //     if ui.button(None, "Drop") {
        //         player.unequip_item(&item.id);
        //         let position = player.body.position;
        //         player.inventory.drop(&item.id, position);
        //     }
        // }
    } else {
        if ui.button(None, "Unequip") {
            player.unequip_item(&entry.params.id);
        }
    }
    //});
}

pub fn draw_inventory_window(player: &mut Actor) {
    let gui_skins = storage::get::<GuiSkins>();
    let scale = gui_skins.scale;
    root_ui().push_skin(&gui_skins.inventory);
    widgets::Window::new(hash!(), vec2(50.0 * scale, 375.0 * scale), vec2(300.0 * scale, 300.0 * scale))
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("credits: {}, weight: {}/{}", player.inventory.credits, player.inventory.get_total_weight(), player.stats.carry_capacity));
            {
                let items = player.inventory.get_all_of_kind(&[ItemKind::OneHandedWeapon, ItemKind::TwoHandedWeapon]);
                if items.len() > 0 {
                    ui.tree_node(hash!(), "Weapons", |ui| {
                        for item in &items {
                            draw_entry(ui, scale, player, item);
                        }
                    });
                }
            }
            {
                let items = player.inventory.get_all_of_kind(&[ItemKind::Misc]);
                if items.len() > 0 {
                    ui.tree_node(hash!(), "Miscellaneous", |ui| {
                        for item in &items {
                            draw_entry(ui, scale, player, item)
                        }
                    });
                }
            }
        });
    root_ui().pop_skin();
}
