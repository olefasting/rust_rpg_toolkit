use macroquad::{
    experimental::{
        scene::{
            Handle,
            RefMut,
        },
    },
    prelude::*,
};

use crate::{ItemParams, Item};

#[derive(Clone)]
pub struct ActorInventory {
    items: Vec<ItemParams>,
}

impl ActorInventory {
    const DROP_ALL_POSITION_VARIANCE: f32 = 15.0;

    pub fn new(items: &[ItemParams]) -> Self {
        ActorInventory {
            items: items.to_vec(),
        }
    }

    pub fn get_all_of_kind(&self, kinds: &[&'static str]) -> Vec<ItemParams> {
        self.items.clone().into_iter().filter(|item| {
            for kind in kinds {
                if item.kind == *kind {
                    return true;
                }
            }
            false
        }).collect()
    }

    pub fn pick_up_item(&mut self, item: RefMut<Item>) {
        self.items.push(item.to_item_params());
        item.delete();
    }

    pub fn drop_item(&mut self, item_id: &str, position: Vec2) -> Option<Handle<Item>> {
        let mut item = None;
        self.items.retain(|params| {
           if params.id == item_id {
               item = Some(Item::add_node(ItemParams {
                   position: Self::randomize_drop_position(position),
                   ..params.clone()
               }));
               false
           } else {
               true
           }
        });
        item
    }

    pub fn drop_all(&mut self, position: Vec2) {
        self.items.retain(|params| {
            Item::add_node(ItemParams {
                position: Self::randomize_drop_position(position),
                ..params.clone()
            });
           false
        });
    }

    pub fn get_total_weight(&self) -> f32 {
        let mut weight = 0.0;
        for item in &self.items {
            weight += item.weight;
        }
        weight
    }

    pub fn clone_data(&self) -> Vec<ItemParams> {
        self.items.clone()
    }

    fn randomize_drop_position(position: Vec2) -> Vec2 {
        vec2(
            rand::gen_range(position.x - Self::DROP_ALL_POSITION_VARIANCE, position.x + Self::DROP_ALL_POSITION_VARIANCE),
            rand::gen_range(position.y - Self::DROP_ALL_POSITION_VARIANCE, position.y + Self::DROP_ALL_POSITION_VARIANCE),
        )
    }
}
