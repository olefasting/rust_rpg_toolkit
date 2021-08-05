use std::ops::Sub;

use macroquad::{
    experimental::{
        collections::storage,
        scene::{
            Node,
            RefMut,
        },
    },
    prelude::*,
};
use macros::*;

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
use crate::game_options::LocalPlayerId;
use crate::graphics::Viewport;
pub use crate::inventory::Inventory;
use crate::physics::physics_body::PhysicsBody;
use crate::physics::Collider;

#[derive(Copy, Clone)]
pub enum ActorController {
    Player { player_id: u32 },
    Computer,
}

#[derive(Clone)]
pub struct ActorControl {
    pub controller: ActorController,
    pub destination: Option<Vec2>,
    pub direction: Vec2,
}

impl ActorControl {
    pub fn new(controller: ActorController) -> Self {
        ActorControl {
            controller,
            destination: None,
            direction: Vec2::ZERO,
        }
    }
}

#[derive(Clone, GetStringId)]
pub struct ActorData {
    pub id: String,
    pub name: String,
    pub factions: Vec<String>,
    pub position: Vec2,
    pub inventory: Vec<ItemData>,
    pub sprite_params: SpriteParams,
    pub controller: ActorController,
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
            controller: ActorController::Computer,
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
    inventory: Inventory,
    pub control: ActorControl,
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
            inventory: Inventory::new(&data.inventory),
            control: ActorControl::new(data.controller),
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
            controller: self.control.controller,
        }
    }
}

impl Node for Actor {
    fn ready(_node: RefMut<Self>) {
    }

    fn update(mut node: RefMut<Self>) {
        match node.control.controller {
            ActorController::Player { player_id } => {
                let local_player_id = storage::get::<LocalPlayerId>().0;
                if player_id == local_player_id {
                    let viewport = storage::get::<Viewport>();
                    let coords = viewport.get_mouse_world_coords();
                    if is_mouse_button_down(MouseButton::Left) {
                        node.control.destination = Some(coords);
                    }
                    if is_mouse_button_down(MouseButton::Right) {}
                } else {
                    // TODO: Remote player
                }
            }
            ActorController::Computer => {
                // TODO: Computer controlled
            }
        }
    }

    fn fixed_update(mut node: RefMut<Self>) {
        if node.control.direction != Vec2::ZERO {
            node.body.velocity = node.control.direction * Self::MOVE_SPEED;
            node.control.destination = None;
            node.control.direction = Vec2::ZERO;
        } else if let Some(destination) = node.control.destination {
            if destination.distance(node.body.position) > Self::STOP_AT_DISTANCE {
                let direction = destination.sub(node.body.position).normalize();
                node.body.velocity = direction * Self::MOVE_SPEED;
            } else {
                node.control.destination = None;
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
