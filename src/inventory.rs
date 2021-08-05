use crate::{
    nodes::item::{
        ItemData,
    },
};

#[derive(Clone)]
pub struct Inventory {
    pub items: Vec<ItemData>,
}

impl Inventory {
    pub fn new(items: &[ItemData]) -> Self {
        Inventory {
            items: items.to_vec(),
        }
    }
}
