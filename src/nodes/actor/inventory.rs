use crate::Item;

#[derive(Clone)]
pub struct ActorInventory {
    items: Vec<Item>,
}

impl ActorInventory {
    pub fn new(items: &[Item]) -> Self {
        ActorInventory {
            items: items.to_vec(),
        }
    }

    pub fn clone_data(&self) -> Vec<Item> {
        self.items.clone()
    }
}
