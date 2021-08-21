use serde::{Serialize, Deserialize};

use macroquad::{
    experimental::{
        scene::{
            Node,
            RefMut,
            Handle,
        },
    },
    prelude::*,
};

use crate::{
    generate_id,
    render::{
        Sprite,
    },
    nodes::draw_buffer::{
        DrawBuffer,
        BufferedDraw,
        Bounds,
    },
    ability::AbilityParams,
    json,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemKind {
    #[serde(rename = "one_handed_weapon")]
    OneHandedWeapon,
    #[serde(rename = "two_handed_weapon")]
    TwoHandedWeapon,
    #[serde(rename = "misc")]
    Misc,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemParams {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub prototype_id: Option<String>,
    pub name: String,
    pub description: String,
    #[serde(with = "json::opt_vec2", skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec2>,
    pub kind: ItemKind,
    pub weight: f32,
    pub ability: AbilityParams,
    pub sprite: Sprite,
}

impl From<&Item> for ItemParams {
    fn from(item: &Item) -> Self {
        ItemParams {
            prototype_id: None,
            name: item.name.clone(),
            description: item.description.clone(),
            position: Some(item.position),
            kind: item.kind.clone(),
            weight: item.weight,
            ability: item.ability.clone(),
            sprite: item.sprite.clone(),
        }
    }
}

impl Default for ItemParams {
    fn default() -> Self {
        ItemParams {
            prototype_id: None,
            name: "Unnamed Item".to_string(),
            description: "".to_string(),
            position: Default::default(),
            kind: ItemKind::Misc,
            weight: 0.1,
            ability: Default::default(),
            sprite: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub position: Vec2,
    pub kind: ItemKind,
    pub weight: f32,
    ability: AbilityParams,
    sprite: Sprite,
}

impl Item {
    pub fn new(params: ItemParams) -> Self {
        Item {
            id: generate_id(),
            position: params.position.unwrap_or_default(),
            kind: params.kind,
            name: params.name,
            description: params.description,
            weight: params.weight,
            ability: params.ability,
            sprite: params.sprite,
        }
    }

    pub fn add_node(params: ItemParams) -> Handle<Self> {
        scene::add_node(Self::new(params))
    }
}

impl BufferedDraw for Item {
    fn buffered_draw(&mut self) {
        self.sprite.draw(self.position, 0.0);
    }

    fn get_z_index(&self) -> f32 {
        self.position.y
    }

    fn get_bounds(&self) -> Bounds {
        Bounds::Point(self.position)
    }
}

impl Node for Item {
    fn draw(node: RefMut<Self>) {
        let mut draw_buffer = scene::find_node_by_type::<DrawBuffer<Self>>().unwrap();
        draw_buffer.buffered.push(node.handle());
    }
}
