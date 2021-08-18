use macroquad::{
    experimental::{
        scene::{
            Handle,
            RefMut,
        },
    },
    prelude::*,
};

use crate::{
    ItemParams,
    Item,
    render::Sprite,
    nodes::actor::Ability,
    generate_id,
    Resources,
};

#[derive(Clone)]
pub struct ActorInventoryEntry {
    pub id: String,
    pub params: ItemParams,
}

impl ActorInventoryEntry {
    pub fn new(params: ItemParams) -> Self {
        ActorInventoryEntry {
            id: generate_id(),
            params,
        }
    }
}

impl ActorInventoryEntry {
    pub fn to_actor_ability(&self) -> Ability {
        Ability::new(self.params.ability.clone())
    }
}

#[derive(Clone)]
pub struct ActorInventory {
    items: Vec<ActorInventoryEntry>,
}

impl ActorInventory {
    const DROP_ALL_POSITION_VARIANCE: f32 = 15.0;

    pub fn new() -> Self {
        ActorInventory {
            items: Vec::new(),
        }
    }

    pub fn get_all_of_kind(&self, kinds: &[&'static str]) -> Vec<ActorInventoryEntry> {
        self.items.clone().into_iter().filter(|item| {
            for kind in kinds {
                if item.params.kind == *kind {
                    return true;
                }
            }
            false
        }).collect()
    }

    pub fn pick_up(&mut self, item: RefMut<Item>) {
        self.items.push(ActorInventoryEntry::new(ItemParams::from(&*item)));
        item.delete();
    }

    pub fn drop(&mut self, item_id: &str, position: Vec2) -> bool {
        let items: Vec<ActorInventoryEntry> = self.items
            .drain_filter(|entry| {
                if entry.id == item_id {
                    let params = ItemParams {
                        position: Some(Self::randomize_drop_position(position)),
                        ..entry.params.clone()
                    };
                    Item::add_node(params);
                    return true;
                }
                false
            })
            .collect();
        !items.is_empty()
    }

    pub fn drop_all(&mut self, position: Vec2) {
        self.items.drain_filter(|entry| {
            let params = ItemParams {
                position: Some(Self::randomize_drop_position(position)),
                ..entry.params.clone()
            };
            Item::add_node(params);
            true
        });
    }

    pub fn get_total_weight(&self) -> f32 {
        let mut weight = 0.0;
        for item in &self.items {
            weight += item.params.weight;
        }
        weight
    }

    pub fn to_params(&self) -> Vec<ItemParams> {
        self.items.iter().map(|entry| entry.params.clone()).collect()
    }

    fn randomize_drop_position(position: Vec2) -> Vec2 {
        vec2(
            rand::gen_range(position.x - Self::DROP_ALL_POSITION_VARIANCE, position.x + Self::DROP_ALL_POSITION_VARIANCE),
            rand::gen_range(position.y - Self::DROP_ALL_POSITION_VARIANCE, position.y + Self::DROP_ALL_POSITION_VARIANCE),
        )
    }
}

impl From<Vec<ItemParams>> for ActorInventory {
    fn from(params_vec: Vec<ItemParams>) -> Self {
        ActorInventory {
            items: params_vec.into_iter().map(|params| ActorInventoryEntry::new(params)).collect(),
        }
    }
}
