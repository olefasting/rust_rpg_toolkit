use crate::{ItemParams, Item};

#[derive(Clone)]
pub struct ActorInventory {
    items: Vec<ItemParams>,
}

impl ActorInventory {
    pub fn new(items: &[ItemParams]) -> Self {
        ActorInventory {
            items: items.to_vec(),
        }
    }

    pub fn get_all_of_kind(&self, kind: &str) -> Vec<ItemParams> {
        self.items.clone().into_iter().filter(|item| item.kind == kind).collect()
    }

    pub fn get_all_of_kinds(&self, kinds: &[&'static str]) -> Vec<ItemParams> {
        self.items.clone().into_iter().filter(|item| {
            for kind in kinds {
                if item.kind == *kind {
                    return true;
                }
            }
            false
        }).collect()
    }

    pub fn clone_data(&self) -> Vec<ItemParams> {
        self.items.clone()
    }
}
