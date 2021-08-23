use serde::{Serialize, Deserialize};

use macroquad::{
    experimental::{
        scene::{
            Node,
            RefMut,
            Handle,
        },
        collections::storage,
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
    Resources,
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
    #[serde(rename = "id")]
    pub prototype_id: String,
    pub name: String,
    pub description: String,
    #[serde(default, with = "json::opt_vec2", skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec2>,
    pub kind: ItemKind,
    pub weight: f32,
    #[serde(rename = "ability")]
    pub ability_id: String,
    pub sprite: Sprite,
    #[serde(default)]
    pub is_quest_item: bool,
}

impl From<&Item> for ItemParams {
    fn from(item: &Item) -> Self {
        ItemParams {
            prototype_id: item.prototype_id.clone(),
            name: item.name.clone(),
            description: item.description.clone(),
            position: Some(item.position),
            kind: item.kind.clone(),
            weight: item.weight,
            ability_id: item.ability.id.clone(),
            sprite: item.sprite.clone(),
            is_quest_item: item.is_quest_item,
        }
    }
}

impl Default for ItemParams {
    fn default() -> Self {
        ItemParams {
            prototype_id: "".to_string(),
            name: "Unnamed Item".to_string(),
            description: "".to_string(),
            position: Default::default(),
            kind: ItemKind::Misc,
            weight: 0.1,
            ability_id: Default::default(),
            sprite: Default::default(),
            is_quest_item: false,
        }
    }
}

#[derive(Clone)]
pub struct Item {
    pub id: String,
    pub prototype_id: String,
    pub name: String,
    pub description: String,
    pub position: Vec2,
    pub kind: ItemKind,
    pub weight: f32,
    pub is_quest_item: bool,
    ability: AbilityParams,
    sprite: Sprite,
}

impl Item {
    pub fn new(instance_id: Option<String>, params: ItemParams) -> Self {
        let resources = storage::get::<Resources>();
        let ability = resources.abilities.get(&params.ability_id).cloned().unwrap();
        Item {
            id: instance_id.unwrap_or(generate_id()).to_string(),
            prototype_id: params.prototype_id,
            position: params.position.unwrap_or_default(),
            kind: params.kind,
            name: params.name,
            description: params.description,
            weight: params.weight,
            is_quest_item: params.is_quest_item,
            ability,
            sprite: params.sprite,
        }
    }

    pub fn add_node(instance_id: Option<String>, params: ItemParams) -> Handle<Self> {
        scene::add_node(Self::new(instance_id, params))
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

#[derive(Debug, Clone)]
pub struct Credits {
    pub position: Vec2,
    pub amount: u32,
    pub sprite: Sprite,
}

impl Credits {
    pub fn new(position: Vec2, amount: u32) -> Self {
        Credits {
            position,
            amount,
            sprite: Sprite {
                texture_id: "credits".to_string(),
                tile_size: uvec2(8, 8),
                ..Default::default()
            },
        }
    }

    pub fn add_node(position: Vec2, amount: u32) -> Handle<Self> {
        scene::add_node(Self::new(position, amount))
    }
}

impl BufferedDraw for Credits {
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

impl Node for Credits {
    fn ready(node: RefMut<Self>) {
        let mut draw_buffer = scene::find_node_by_type::<DrawBuffer<Credits>>().unwrap();
        draw_buffer.buffered.push(node.handle());
    }
}
