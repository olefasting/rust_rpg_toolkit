use std::ops::Sub;

use macroquad::{
    experimental::{
        scene::{
            Node,
            RefMut,
        },
    },
    prelude::*,
};

use macros::*;

pub use actor_controller::ActorController;
pub use actor_inventory::ActorInventory;
pub use physics_body::PhysicsBody;

use crate::{
    generate_string_id,
    GetStringId,
    graphics::{
        Drawable,
        SpriteAnimationPlayer,
        SpriteParams,
    },
    nodes::ItemData,
};

pub mod actor_inventory;
pub mod actor_controller;
pub mod physics_body;

#[derive(Clone, GetStringId)]
pub struct ActorData {
    pub id: String,
    pub name: String,
    pub factions: Vec<String>,
    pub position: Vec2,
    pub inventory: Vec<ItemData>,
    pub sprite_params: SpriteParams,
    pub player_control_id: Option<u32>,
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
            player_control_id: None,
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
    body: PhysicsBody,
    sprite: SpriteAnimationPlayer,
    inventory: ActorInventory,
    pub controller: ActorController,
}

impl Actor {
    const MOVE_SPEED: f32 = 25.0;
    const STOP_AT_DISTANCE: f32 = 25.0;

    pub fn new(
        data: ActorData,
    ) -> Self {
        Actor {
            id: data.id.to_string(),
            name: data.name.to_string(),
            factions: data.factions.to_vec(),
            flip_x: false,
            flip_y: false,
            body: PhysicsBody::new(data.position, 0.0, data.sprite_params.tile_size, Some(data.sprite_params.offset)),
            sprite: SpriteAnimationPlayer::new(data.sprite_params.clone()),
            inventory: ActorInventory::new(&data.inventory),
            controller: ActorController::new(data.player_control_id),
        }
    }

    #[allow(dead_code)]
    pub fn add_to_faction(&mut self, faction_id: &str) {
        self.factions.push(faction_id.to_string());
    }

    #[allow(dead_code)]
    pub fn remove_from_faction(&mut self, faction_id: &str) -> bool {
        if let Some(i) = self.factions.iter().position(|id| *id == faction_id.to_string()) {
            self.factions.remove(i);
            return true;
        };
        return false;
    }

    #[allow(dead_code)]
    pub fn to_actor_data(&self) -> ActorData {
        ActorData {
            id: self.id.to_string(),
            name: self.name.to_string(),
            factions: self.factions.clone(),
            position: self.body.position,
            inventory: self.inventory.items.clone(),
            sprite_params: self.sprite.to_sprite_params(),
            player_control_id: self.controller.player_id,
        }
    }
}

impl Node for Actor {
    fn ready(_node: RefMut<Self>) {
        // Self::apply_map_object_provider(node);
    }

    fn update(_node: RefMut<Self>) {}

    fn fixed_update(mut node: RefMut<Self>) {
        if let Some(destination) = node.controller.destination {
            if destination.distance(node.body.position) > Self::STOP_AT_DISTANCE {
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
        let (position, rotation, flip_x, flip_y)
            = (node.body.position, node.body.rotation, node.flip_x, node.flip_y);
        node.sprite.draw(position, rotation, flip_x, flip_y);
    }
}
