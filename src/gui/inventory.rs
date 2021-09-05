use crate::gui::*;

fn draw_entry(ui: &mut Ui, scale: f32, player: &mut Actor, entry: &InventoryEntry) {
    Group::new(hash!(), vec2(250.0 * scale, 30.0 * scale)).ui(ui, |ui| {
        ui.label(vec2(0.0, 0.0) * scale, &entry.params.name);
        if entry.equipped_to == EquipmentSlot::None {
            if ui.button(vec2(160.0, 0.0) * scale, "Equip") {
                player.equip_item(&entry.params.id);
            }
        } else {
            if ui.button(vec2(150.0, 0.0) * scale, "Unequip") {
                player.unequip_item(&entry.params.id);
            }
        }
        if entry.params.is_quest_item == false {
            if ui.button(vec2(210.0, 0.0) * scale, "Drop") {
                player.unequip_item(&entry.params.id);
                let position = player.body.position;
                player.inventory.drop(&entry.params.id, position);
            }
        }
    });
}

pub fn draw_inventory_window(player: &mut Actor) {
    let gui_skins = storage::get::<GuiSkins>();
    let scale = gui_skins.scale;

    let size = vec2(300.0, 300.0) * scale;
    let position = vec2(50.0, 375.0) * scale;

    root_ui().push_skin(&gui_skins.inventory);

    widgets::Window::new(hash!(), position, size)
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("credits: {}, weight: {}/{}", player.inventory.credits, player.inventory.get_total_weight(), player.stats.carry_capacity));
            {
                let items = player.inventory.get_all_of_kind(&[ItemKind::OneHandedWeapon, ItemKind::TwoHandedWeapon]);
                if items.len() > 0 {
                   // ui.tree_node(hash!(), "Weapons", |ui| {
                        for item in &items {
                            draw_entry(ui, scale, player, item);
                        }
                   // });
                }
            }
            {
                let items = player.inventory.get_all_of_kind(&[ItemKind::Misc]);
                if items.len() > 0 {
                    // ui.tree_node(hash!(), "Miscellaneous", |ui| {
                        for item in &items {
                            draw_entry(ui, scale, player, item)
                        }
                    // });
                }
            }
        });

    root_ui().pop_skin();
}
