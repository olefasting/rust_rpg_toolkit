use std::ops::Sub;

use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
            RefMut,
        },
    },
    color,
    prelude::*,
};

use crate::input::apply_local_player_input;

mod controller;
mod inventory;

pub use controller::{
    ActorControllerKind,
    ActorController,
};

pub use inventory::ActorInventory;

use crate::{
    get_global,
    render::{
        SpriteAnimationPlayer,
        SpriteParams,
    },
    globals::LocalPlayer,
    physics::{
        PhysicsBody,
        PhysicsObject,
        Collider,
    },
    nodes::Projectiles,
    Item,
};

#[derive(Clone)]
pub struct ActorParams {
    pub id: String,
    pub current_health: f32,
    pub max_health: f32,
    pub position: Vec2,
    pub collider: Option<Collider>,
    pub inventory: Vec<Item>,
    pub sprite_params: SpriteParams,
    pub controller_kind: ActorControllerKind,
}

impl Default for ActorParams {
    fn default() -> Self {
        ActorParams {
            id: "".to_string(),
            current_health: 0.0,
            max_health: 0.0,
            position: Vec2::ZERO,
            collider: None,
            inventory: Vec::new(),
            sprite_params: Default::default(),
            controller_kind: ActorControllerKind::Computer,
        }
    }
}

#[derive(Clone)]
pub struct Actor {
    pub id: String,
    current_health: f32,
    max_health: f32,
    pub body: PhysicsBody,
    sprite: SpriteAnimationPlayer,
    inventory: ActorInventory,
    pub controller: ActorController,
}

impl Actor {
    const MOVE_SPEED: f32 = 10.0;

    pub fn new(params: ActorParams) -> Self {
        Actor {
            id: params.id,
            current_health: params.current_health,
            max_health: params.max_health,
            body: PhysicsBody::new(params.position, 0.0, params.collider),
            sprite: SpriteAnimationPlayer::new(params.sprite_params.clone()),
            inventory: ActorInventory::new(&params.inventory),
            controller: ActorController::new(params.controller_kind),
        }
    }

    pub fn add_node(params: ActorParams) -> Handle<Self> {
        scene::add_node(Self::new(params))
    }

    pub fn to_actor_params(&self) -> ActorParams {
        ActorParams {
            id: self.id.clone(),
            current_health: self.current_health,
            max_health: self.max_health,
            position: self.body.position,
            collider: self.body.collider,
            inventory: self.inventory.clone_data(),
            sprite_params: self.sprite.to_sprite_params(),
            controller_kind: self.controller.kind,
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.current_health -= damage;
        println!("damage: {}, health: {}/{}", damage, self.current_health, self.max_health);
    }
}

impl Node for Actor {
    fn ready(mut node: RefMut<Self>) {
        node.provides::<PhysicsObject>((
            node.handle().untyped(),
            node.handle().lens(|actor| &mut actor.body),
        ));
    }

    fn update(mut node: RefMut<Self>) {
        if node.current_health <= 0.0 {
            node.delete();
            return;
        }

        match node.controller.kind {
            ActorControllerKind::Player { player_id } => {
                let local_player = get_global::<LocalPlayer>();
                if player_id == local_player.id {
                    apply_local_player_input(&mut node.controller);
                } else {
                    // TODO: Remote player
                }
            },
            ActorControllerKind::Computer => {
                // TODO: Computer controlled
            },
            ActorControllerKind::None => {},
        }
    }

    fn fixed_update(mut node: RefMut<Self>) {
        node.body.velocity = node.controller.direction.normalize_or_zero() * Self::MOVE_SPEED;
        node.body.integrate();

        if let Some(target) = node.controller.primary_target {
            let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
            projectiles.spawn(&node.id,15.0, color::YELLOW, 4.0, node.body.position, target, 25.0, 15.5, 0.0);
        }
        if let Some(target) = node.controller.secondary_target {
            let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
            projectiles.spawn(&node.id, 150.0, color::BLUE, 100.0, node.body.position, target, 2.0, 15.5, 4.0);
        }
    }

    fn draw(mut node: RefMut<Self>) {
        let (position, rotation) = (node.body.position, node.body.rotation);
        node.sprite.draw(position, rotation);
        node.body.debug_draw();
    }
}
