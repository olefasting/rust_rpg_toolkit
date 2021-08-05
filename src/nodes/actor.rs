use macroquad::{
    experimental::{
        scene::{
            Node,
            HandleUntyped,
            RefMut,
        },
    },
    prelude::*,
};

use macros::*;

pub mod actor_inventory;

pub use actor_inventory::ActorInventory;

use crate::{
    generate_string_id,
    util::{
        GetStringId,
    },
    nodes::{
        MapObject,
        MapObjectCapabilities,
        MapObjectProvider,
    },
    SpriteAnimationPlayer,
    SpriteParams,
};
use crate::nodes::ItemData;

#[derive(Clone, GetStringId)]
pub struct ActorData {
    pub id: String,
    pub name: String,
    pub factions: Vec<String>,
    pub position: Vec2,
    pub inventory: Vec<ItemData>,
    pub sprite_params: SpriteParams,
}

impl Default for ActorData {
    fn default() -> Self {
        ActorData {
            id: generate_string_id(),
            name: "Unnamed Actor".to_string(),
            factions: Vec::new(),
            position: Vec2::ZERO,
            inventory: Vec::new(),
            sprite_params: Default::default(),
        }
    }
}

#[derive(Clone, GetStringId, MapObject)]
pub struct Actor {
    pub id: String,
    pub name: String,
    factions: Vec<String>,
    pub position: Vec2,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    should_draw: bool,
    inventory: ActorInventory,
    sprite: SpriteAnimationPlayer,
    sprite_params: SpriteParams,
}

impl Actor {
    pub fn new(
        data: ActorData,
    ) -> Self {
        Actor {
            id: data.id.to_string(),
            name: data.name.to_string(),
            factions: data.factions.to_vec(),
            position: data.position,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            should_draw: true,
            inventory: ActorInventory::new(&data.inventory),
            sprite: SpriteAnimationPlayer::new(data.sprite_params.clone()),
            sprite_params: data.sprite_params,
        }
    }

    pub fn add_to_faction(&mut self, faction_id: &str) {
        self.factions.push(faction_id.to_string());
    }

    pub fn remove_from_faction(&mut self, faction_id: &str) -> bool {
        if let Some(i) = self.factions.iter().position(|id| *id == faction_id.to_string()) {
            self.factions.remove(i);
            return true;
        };
        return false;
    }

    pub fn to_actor_data(&self) -> ActorData {
        ActorData {
            id: self.id.to_string(),
            name: self.name.to_string(),
            factions: self.factions.clone(),
            position: self.position,
            inventory: self.inventory.items.clone(),
            sprite_params: self.sprite_params.clone(),
        }
    }
}

impl Node for Actor {
    fn ready(node: RefMut<Self>) {
        Self::apply_map_object_provider(node);
    }

    fn update(_node: RefMut<Self>) {}

    fn fixed_update(_node: RefMut<Self>) {}

    fn draw(mut node: RefMut<Self>) {
        if node.should_draw {
            let (position, rotation, flip_x, flip_y)
                = (node.position, node.rotation, node.flip_x, node.flip_y);
            node.sprite.draw(position, rotation, flip_x, flip_y);
        }
    }
}
