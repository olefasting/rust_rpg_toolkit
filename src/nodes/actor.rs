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
pub mod actor_controller;

pub use actor_inventory::ActorInventory;
pub use actor_controller::ActorController;

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
use crate::physics_body::PhysicsBody;
use macroquad::ui::Drag::No;
use std::ops::Sub;

#[derive(Clone, GetStringId)]
pub struct ActorData {
    pub id: String,
    pub name: String,
    pub factions: Vec<String>,
    pub position: Vec2,
    pub inventory: Vec<ItemData>,
    pub sprite_params: SpriteParams,
    pub is_player: bool,
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
            is_player: false,
        }
    }
}

#[derive(Clone, GetStringId)]
pub struct Actor {
    pub id: String,
    pub name: String,
    factions: Vec<String>,
    pub flip_x: bool,
    pub flip_y: bool,
    should_draw: bool,
    inventory: ActorInventory,
    controller: ActorController,
    body: PhysicsBody,
    sprite: SpriteAnimationPlayer,
    sprite_params: SpriteParams,
}

impl Actor {
    const MOVE_SPEED: f32 = 25.0;
    const VELOCITY_DAMPING_FACTOR: f32 = 0.9;
    const DESTINATION_REACHED_THRESHOLD: f32 = 25.0;

    pub fn new(
        data: ActorData,
    ) -> Self {
        Actor {
            id: data.id.to_string(),
            name: data.name.to_string(),
            factions: data.factions.to_vec(),
            flip_x: false,
            flip_y: false,
            should_draw: true,
            inventory: ActorInventory::new(&data.inventory),
            controller: ActorController::new(data.is_player),
            body: PhysicsBody::new(data.position, 0.0, data.sprite_params.tile_size, Some(data.sprite_params.offset)),
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
            position: self.body.position,
            inventory: self.inventory.items.clone(),
            sprite_params: self.sprite_params.clone(),
            is_player: self.controller.is_player,
        }
    }
}

impl Node for Actor {
    fn ready(mut node: RefMut<Self>) {
        // Self::apply_map_object_provider(node);
        if node.controller.is_player {
            node.provides((
                node.handle().untyped(),
                node.handle().lens(|node| &mut node.controller),
            ));
        }
    }

    fn update(_node: RefMut<Self>) {}

    fn fixed_update(mut node: RefMut<Self>) {
        if let Some(destination) = node.controller.destination {
            node.body.velocity *= Self::VELOCITY_DAMPING_FACTOR;

            if destination.distance(node.body.position) > Self::DESTINATION_REACHED_THRESHOLD {
                let direction = destination.sub(node.body.position).normalize();
                node.body.velocity = direction * Self::MOVE_SPEED;
            } else {
                node.controller.destination = None;
                node.body.velocity = Vec2::ZERO;
            }
        }

        if node.controller.direction != Vec2::ZERO {
            node.body.velocity = node.controller.direction * Self::MOVE_SPEED;

            node.controller.destination = None;
            node.controller.direction = Vec2::ZERO;
        }

        node.body.position.x += node.body.velocity.x;
        node.body.position.y += node.body.velocity.y;
    }

    fn draw(mut node: RefMut<Self>) {
        if node.should_draw {
            let (position, rotation, flip_x, flip_y)
                = (node.body.position, node.body.rotation, node.flip_x, node.flip_y);
            node.sprite.draw(position, rotation, flip_x, flip_y);
        }
    }
}
