mod equipped;

use crate::prelude::*;
pub use equipped::{EquipmentSlot, EquippedItems};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InventoryParams {
    pub items: Vec<String>,
    #[serde(default)]
    pub credits: u32,
}

#[derive(Clone)]
pub struct InventoryEntry {
    pub params: ItemParams,
    pub equipped_to: EquipmentSlot,
}

impl InventoryEntry {
    pub fn new(params: ItemParams) -> Self {
        InventoryEntry {
            params,
            equipped_to: EquipmentSlot::None,
        }
    }
}

impl InventoryEntry {
    pub fn get_actor_ability(&self) -> Option<Ability> {
        if let Some(ability_id) = self.params.ability_id.clone() {
            let resources = storage::get::<Resources>();
            let ability_params = resources.abilities.get(&ability_id).cloned().unwrap();
            Some(Ability::new(ability_params))
        } else {
            None
        }
    }
}

#[derive(Default, Clone)]
pub struct Inventory {
    pub items: Vec<InventoryEntry>,
    pub credits: u32,
}

impl Inventory {
    const DROP_ALL_POSITION_VARIANCE: f32 = 15.0;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_prototypes(params: &InventoryParams) -> Self {
        let resources = storage::get::<Resources>();
        Inventory {
            items: params
                .items
                .clone()
                .into_iter()
                .map(|id| {
                    let params = resources.items.get(&id).cloned().unwrap();
                    InventoryEntry::new(ItemParams {
                        id: generate_id(),
                        ..params
                    })
                })
                .collect(),
            credits: params.credits,
        }
    }

    pub fn from_saved(params: &InventoryParams, item_params: &[ItemParams]) -> Self {
        let mut items = Vec::new();
        for entry in params.items.clone() {
            if let Some(params) = item_params.iter().find(|item| item.id == entry) {
                items.push(InventoryEntry {
                    params: params.clone(),
                    equipped_to: EquipmentSlot::None,
                });
            }
        }
        Inventory {
            items,
            credits: params.credits,
        }
    }

    pub fn get_all_of_kind(&self, kinds: &[ItemKind]) -> Vec<InventoryEntry> {
        self.items
            .clone()
            .into_iter()
            .filter(|item| {
                if kinds.contains(&item.params.kind) {
                    return true;
                }
                false
            })
            .collect()
    }

    pub fn pick_up(&mut self, item: RefMut<Item>) {
        self.items.push(InventoryEntry::new(ItemParams {
            position: None,
            ..item.to_params()
        }));
        item.delete();
    }

    pub fn add_item(&mut self, item_params: ItemParams) {
        self.items.push(InventoryEntry::new(ItemParams {
            position: None,
            ..item_params
        }));
    }

    pub fn add_credits(&mut self, amount: u32) {
        self.credits += amount;
    }

    pub fn drop(&mut self, item_id: &str, position: Vec2) -> bool {
        self.items
            .drain_filter(|entry| {
                if entry.params.id == item_id && !entry.params.is_quest_item {
                    let params = ItemParams {
                        position: Some(Self::randomize_drop_position(position)),
                        ..entry.params.clone()
                    };
                    Item::add_node(params);
                    return true;
                }
                false
            })
            .next()
            .is_some()
    }

    pub fn drop_all(&mut self, position: Vec2, include_credits: bool) {
        self.items.drain_filter(|entry| {
            if !entry.params.is_quest_item {
                let params = ItemParams {
                    position: Some(Self::randomize_drop_position(position)),
                    ..entry.params.clone()
                };
                Item::add_node(params);
            }
            true
        });
        if include_credits {
            self.drop_all_credits(position);
        }
    }

    pub fn drop_credits(&mut self, amount: u32, position: Vec2) -> bool {
        if self.credits < amount {
            return false;
        }
        self.credits -= amount;
        Credits::add_node(Self::randomize_drop_position(position), amount);
        true
    }

    pub fn drop_all_credits(&mut self, position: Vec2) {
        self.drop_credits(self.credits, position);
    }

    pub fn get_total_weight(&self) -> f32 {
        let mut weight = 0.0;
        for item in &self.items {
            weight += item.params.weight;
        }
        weight
    }

    pub fn to_params(&self) -> InventoryParams {
        InventoryParams {
            items: self
                .items
                .iter()
                .map(|entry| entry.params.id.clone())
                .collect(),
            credits: self.credits,
        }
    }

    fn randomize_drop_position(position: Vec2) -> Vec2 {
        vec2(
            rand::gen_range(
                position.x - Self::DROP_ALL_POSITION_VARIANCE,
                position.x + Self::DROP_ALL_POSITION_VARIANCE,
            ),
            rand::gen_range(
                position.y - Self::DROP_ALL_POSITION_VARIANCE,
                position.y + Self::DROP_ALL_POSITION_VARIANCE,
            ),
        )
    }
}
