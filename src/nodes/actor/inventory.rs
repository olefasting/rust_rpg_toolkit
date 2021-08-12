use crate::{ItemParams, Item};

#[derive(Clone)]
pub struct ActorInventory {
    items: Vec<Item>,
}

impl ActorInventory {
    pub fn new(items: &[ItemParams]) -> Self {
        let items = items.iter().map(|params| Item::new(params.clone())).collect();
        ActorInventory {
            items,
        }
    }

    pub fn get_all_of_kind(&self, kinds: &[&'static str]) -> Vec<Item> {
        self.items.clone().into_iter().filter(|item| {
            for kind in kinds {
                if item.kind == *kind {
                    return true;
                }
            }
            false
        }).collect()
    }

    pub fn get_total_weight(&self) -> f32 {
        let mut weight = 0.0;
        for item in &self.items {
            weight += item.weight;
        }
        weight
    }

    pub fn clone_data(&self) -> Vec<ItemParams> {
        self.items.iter().map(|item| item.to_item_params()).collect()
    }
}
