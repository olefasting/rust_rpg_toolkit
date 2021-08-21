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

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub prototype_id: Option<String>,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "json::opt_vec2")]
    pub position: Option<Vec2>,
    pub kind: String,
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
            kind: Item::MISC_KIND.to_string(),
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
    pub kind: String,
    pub weight: f32,
    ability: AbilityParams,
    sprite: Sprite,
}

impl Item {
    pub const BODY_ARMOR_KIND: &'static str = "body_armor";
    pub const HEAD_ARMOR_KIND: &'static str = "body_armor";
    pub const LEG_ARMOR_KIND: &'static str = "body_armor";
    pub const ARMOR_KINDS: &'static [&'static str] = &[
        Self::BODY_ARMOR_KIND,
        Self::HEAD_ARMOR_KIND,
        Self::LEG_ARMOR_KIND,
    ];

    pub const ONE_HANDED_WEAPON_KIND: &'static str = "one_handed_weapon";
    pub const TWO_HANDED_WEAPON_KIND: &'static str = "two_handed_weapon";
    pub const WEAPON_KINDS: &'static [&'static str] = &[
        Self::ONE_HANDED_WEAPON_KIND,
        Self::TWO_HANDED_WEAPON_KIND,
    ];

    pub const MISC_KIND: &'static str = "misc";
    pub const QUEST_KIND: &'static str = "quest";

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
