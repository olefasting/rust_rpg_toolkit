mod draw_buffer;

use serde::{Serialize, Deserialize, Serializer};

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
    nodes::{
        ActorAbility,
    },
    render::{
        Sprite,
        SpriteParams,
    },
    get_global,
    ActionFuncs,
    json,
};

pub use draw_buffer::ItemDrawBuffer;
use crate::actions::ActionParams;

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemParams {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub description: String,
    pub weight: f32,
    pub action_params: ActionParams,
    pub sprite_params: SpriteParams,
}

impl Default for ItemParams {
    fn default() -> Self {
        ItemParams {
            id: generate_id(),
            kind: Item::MISC_KIND.to_string(),
            name: "Unnamed Item".to_string(),
            description: "".to_string(),
            weight: 0.1,
            action_params: Default::default(),
            sprite_params: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Item {
    pub instance_id: String,
    pub position: Vec2,
    pub params: ItemParams,
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

    pub fn new(position: Vec2, params: ItemParams) -> Self {
        let sprite = Sprite::new(params.sprite_params.clone());
        Item {
            instance_id: generate_id(),
            position,
            params,
            sprite,
        }
    }

    pub fn add_node(position: Vec2, params: ItemParams) -> Handle<Self> {
        scene::add_node(Self::new(position, params))
    }

    pub fn draw_item(&mut self) {
        self.sprite.draw(self.position, 0.0);
    }
}

impl Node for Item {
    fn draw(node: RefMut<Self>) {
        let mut draw_queue = scene::find_node_by_type::<ItemDrawBuffer>().unwrap();
        draw_queue.add_to_buffer(node.handle());
    }
}
