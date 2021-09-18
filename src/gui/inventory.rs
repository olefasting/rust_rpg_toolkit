use crate::gui::*;

fn draw_entry(ui: &mut Ui, player: &mut RefMut<Actor>, entry: &InventoryEntry) {
    let gui_skins = storage::get::<GuiSkins>();

    widgets::Group::new(hash!(), vec2(250.0 , 30.0 )).ui(ui, |ui| {
        ui.label(vec2(0.0, 0.0) , &entry.params.name);
        if entry.equipped_to == EquipmentSlot::None {
            ui.push_skin(&gui_skins.condensed_button);
            if ui.button(vec2(160.0, 0.0) , "Equip") {
                player.equip_item(&entry.params.id);
            }
            ui.pop_skin();
        } else {
            ui.push_skin(&gui_skins.condensed_button);
            if ui.button(vec2(150.0, 0.0) , "Unequip") {
                player.unequip_item(&entry.params.id);
            }
            ui.pop_skin();
        }

        if entry.params.is_quest_item == false {
            ui.push_skin(&gui_skins.condensed_button_inactive);
            if ui.button(vec2(210.0, 0.0) , "Drop") {
                player.unequip_item(&entry.params.id);
                let position = player.body.position;
                player.inventory.drop(&entry.params.id, position);
            }
            ui.pop_skin();
        }
    });
}

pub fn draw_inventory_window() {
    if let Some(game_state) = scene::find_node_by_type::<GameState>() {
        if game_state.gui.should_draw_inventory_window {
            if let Some(mut player) = get_player_actor() {
                let gui_skins = storage::get::<GuiSkins>();

                let size = vec2(300.0, 400.0);
                let position = vec2(50.0, 475.0);

                root_ui().push_skin(&gui_skins.default);

                WindowBuilder::new(hash!(), size)
                    .with_pos(position, false)
                    .build(&mut *root_ui(), |ui| {
                        ui.label(None, &format!("credits: {}, weight: {}/{}", player.inventory.credits, player.inventory.get_total_weight(), player.stats.carry_capacity));
                        {
                            let items = player.inventory.get_all_of_kind(&[ItemKind::OneHandedWeapon, ItemKind::TwoHandedWeapon]);
                            if items.len() > 0 {
                                for item in &items {
                                    draw_entry(ui, &mut player, item);
                                }
                            }
                        }
                        {
                            let items = player.inventory.get_all_of_kind(&[ItemKind::Misc]);
                            if items.len() > 0 {
                                for item in &items {
                                    draw_entry(ui, &mut player, item)
                                }
                            }
                        }
                    });

                root_ui().pop_skin();
            }
        }
    }
}
