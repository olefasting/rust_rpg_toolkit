use crate::{
    nodes::item::{
        ItemData,
    },
};

#[derive(Clone)]
pub struct ActorInventory {
    pub items: Vec<ItemData>,
}

impl ActorInventory {
    pub fn new(items: &[ItemData]) -> Self {
        ActorInventory {
            items: items.to_vec(),
        }
    }
}
