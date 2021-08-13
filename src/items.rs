use crate::nodes::ItemParams;
use std::collections::HashMap;
use macroquad::file::FileError;

pub struct Items {
    items: HashMap<String, ItemParams>,
}

impl Items {
    const ITEMS_FILE_PATH: &'static str = "assets/items.json";

    pub async fn new() -> Result<Items, FileError> {
        let mut items= HashMap::new();

        let json = std::fs::read_to_string(Self::ITEMS_FILE_PATH)
            .expect(&format!("Unable to find resources file '{}'", Self::ITEMS_FILE_PATH));
        let items_data: Vec<ItemParams> = serde_json::from_str(&json)
            .expect(&format!("Error when parsing items file '{}'", Self::ITEMS_FILE_PATH));

        for item in items_data {
            items.insert(item.id.clone(), item);
        }

        Ok(Items {
            items,
        })
    }

    pub fn get(&self, id: &str) -> &ItemParams {
        self.items.get(id).unwrap()
    }

    pub fn try_get(&self, id: &str) -> Option<&ItemParams> {
        self.items.get(id)
    }
}
