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

mod controller;
mod inventory;

pub use controller::{
    ActorControllerKind,
    ActorController,
};

pub use inventory::ActorInventory;

use crate::{
    get_global,
    generate_id,
    render::{
        SpriteAnimationPlayer,
        SpriteParams,
    },
    nodes::ItemData,
};

use crate::LocalPlayerId;
use crate::render::Viewport;
use crate::physics::{
    PhysicsBody,
    Collider,
};

#[derive(Clone)]
pub struct ActorData {
    pub id: String,
    pub name: String,
    pub factions: Vec<String>,
    pub position: Vec2,
    pub inventory: Vec<ItemData>,
    pub sprite_params: SpriteParams,
    pub controller_kind: ActorControllerKind,
}

impl Default for ActorData {
    fn default() -> Self {
        ActorData {
            id: generate_id(),
            name: "Unnamed Actor".to_string(),
            factions: Vec::new(),
            position: Vec2::ZERO,
            inventory: Vec::new(),
            sprite_params: Default::default(),
            controller_kind: ActorControllerKind::Computer,
        }
    }
}

#[derive(Clone)]
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
        let collider = Rect::new(
            data.sprite_params.offset.x,
            data.sprite_params.offset.y,
            data.sprite_params.tile_size.x,
            data.sprite_params.tile_size.y,
        );

        Actor {
            id: data.id.to_string(),
            name: data.name.to_string(),
            factions: data.factions.to_vec(),
            flip_x: false,
            flip_y: false,
            body: PhysicsBody::new(data.position, 0.0, Some(Collider::rect(collider))),
            sprite: SpriteAnimationPlayer::new(data.sprite_params.clone()),
            inventory: ActorInventory::new(&data.inventory),
            controller: ActorController::new(data.controller_kind),
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
            inventory: self.inventory.clone_data(),
            sprite_params: self.sprite.to_sprite_params(),
            controller_kind: self.controller.kind,
        }
    }
}

impl Node for Actor {
    fn ready(_node: RefMut<Self>) {
    }

    fn update(mut node: RefMut<Self>) {
        match node.controller.kind {
            ActorControllerKind::Player { player_id } => {
                let local_player_id = get_global::<LocalPlayerId>().0;
                if player_id == local_player_id {
                    let viewport = get_global::<Viewport>();
                    let coords = viewport.get_mouse_world_coords();
                    if is_mouse_button_down(MouseButton::Left) {
                        node.controller.destination = Some(coords);
                    }
                    if is_mouse_button_down(MouseButton::Right) {}
                } else {
                    // TODO: Remote player
                }
            }
            ActorControllerKind::Computer => {
                // TODO: Computer controlled
            }
        }
    }

    fn fixed_update(mut node: RefMut<Self>) {
        if node.controller.direction != Vec2::ZERO {
            node.body.velocity = node.controller.direction * Self::MOVE_SPEED;
            node.controller.destination = None;
            node.controller.direction = Vec2::ZERO;
        } else if let Some(destination) = node.controller.destination {
            if destination.distance(node.body.position) > Self::STOP_AT_DISTANCE {
                let direction = destination.sub(node.body.position).normalize();
                node.body.velocity = direction * Self::MOVE_SPEED;
            } else {
                node.controller.destination = None;
                node.body.velocity = Vec2::ZERO;
            }
        } else {
            node.body.velocity = Vec2::ZERO;
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
