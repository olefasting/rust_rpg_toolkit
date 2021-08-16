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
    get_global,
    render::Sprite,
    nodes::actor::ActorAbility,
    generate_id,
    Resources,
};

#[derive(Clone)]
pub struct ActorInventoryEntry {
    pub instance_id: String,
    pub params: ItemParams,
    pub sprite: Sprite,
}

impl ActorInventoryEntry {
    pub fn new(params: ItemParams) -> Self {
        let id = generate_id();
        let sprite = Sprite::new(params.sprite_params.clone());
        ActorInventoryEntry {
            instance_id: id.clone(),
            params: ItemParams {
                id,
                ..params
            },
            sprite,
        }
    }

    pub fn to_actor_ability(&self) -> ActorAbility {
        ActorAbility::new(self.params.ability_params.clone())
    }
}

#[derive(Clone)]
pub struct ActorInventory {
    items: Vec<ActorInventoryEntry>,
}

impl ActorInventory {
    const DROP_ALL_POSITION_VARIANCE: f32 = 15.0;

    pub fn new(items: &[String]) -> Self {
        ActorInventory {
            items: items.iter().map(|item_id| {
                let resources = get_global::<Resources>();
                let params = resources.items.get(item_id).unwrap();
                ActorInventoryEntry::new(params.clone())
            }).collect(),
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

    pub fn pick_up_item(&mut self, item: RefMut<Item>) {
        self.items.push(ActorInventoryEntry::new(item.params.clone()));
        item.delete();
    }

    pub fn drop_item(&mut self, item_id: &str, position: Vec2) -> bool {
        let items: Vec<Handle<Item>> = self.items
            .drain_filter(|entry| entry.instance_id == item_id)
            .map(|entry| Item::add_node(Self::randomize_drop_position(position), entry.params))
            .collect();
        !items.is_empty()
    }

    pub fn drop_all(&mut self, position: Vec2) {
        self.items.drain_filter(|entry| {
            Item::add_node(Self::randomize_drop_position(position), entry.params.clone());
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

    pub fn clone_data(&self) -> Vec<ItemParams> {
        self.items.iter().map(|entry| entry.params.clone()).collect()
    }

    pub fn to_item_ids(&self) -> Vec<String> {
        self.items.iter().map(|entry| entry.params.id.clone()).collect()
    }

    fn randomize_drop_position(position: Vec2) -> Vec2 {
        vec2(
            rand::gen_range(position.x - Self::DROP_ALL_POSITION_VARIANCE, position.x + Self::DROP_ALL_POSITION_VARIANCE),
            rand::gen_range(position.y - Self::DROP_ALL_POSITION_VARIANCE, position.y + Self::DROP_ALL_POSITION_VARIANCE),
        )
    }
}
