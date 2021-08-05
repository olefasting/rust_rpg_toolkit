use crate::nodes::ItemData;

#[derive(Clone)]
pub struct ActorInventory {
    items: Vec<ItemData>,
}

impl ActorInventory {
    pub fn new(items: &[ItemData]) -> Self {
        ActorInventory {
            items: items.to_vec(),
        }
    }

    pub fn clone_data(&self) -> Vec<ItemData> {
        self.items.clone()
    }
}
