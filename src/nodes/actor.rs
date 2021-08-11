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
mod ability;

pub use controller::{
    ActorControllerKind,
    ActorController,
};

pub use inventory::ActorInventory;
pub use ability::{
    ActorAbility,
    ActorAbilityFunc,
};

use crate::{get_global, render::{
    SpriteAnimationPlayer,
    SpriteParams,
}, globals::LocalPlayer, physics::{
    PhysicsBody,
    PhysicsObject,
    Collider,
}, nodes::Projectiles, Item, generate_id};

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
            id: generate_id(),
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
    primary_ability: Option<ActorAbility>,
    secondary_ability: Option<ActorAbility>,
    pub controller: ActorController,
}

fn primary_test_ability(actor_id: &str, origin: Vec2, target: Vec2) {
    let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
    projectiles.spawn(actor_id, 15.0, color::YELLOW, 4.0, origin, target, 25.0, 15.5, 8.0);
}

fn secondary_test_ability(actor_id: &str, origin: Vec2, target: Vec2) {
    let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
    projectiles.spawn(actor_id, 150.0, color::BLUE, 100.0, origin, target, 2.0, 15.5, 4.0);
}

impl Actor {
    const MOVE_SPEED: f32 = 3.0;

    pub fn new(params: ActorParams) -> Self {
        let id = params.id.clone();
        Actor {
            id: params.id,
            current_health: params.current_health,
            max_health: params.max_health,
            body: PhysicsBody::new(params.position, 0.0, params.collider),
            sprite: SpriteAnimationPlayer::new(params.sprite_params.clone()),
            inventory: ActorInventory::new(&params.inventory),
            primary_ability: Some(ActorAbility::new(&id,0.005, primary_test_ability)),
            secondary_ability: Some(ActorAbility::new(&id, 1.25, secondary_test_ability)),
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

    pub fn find_player(player_id: u32) -> Option<RefMut<Actor>> {
        for actor in scene::find_nodes_by_type::<Actor>() {
            match actor.controller.kind {
                ActorControllerKind::Player { id } => {
                    if player_id == id {
                        return Some(actor);
                    }
                },
                _ => {}
            }
        }
        None
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

        if let Some(mut ability) = node.primary_ability.as_mut() {
            ability.update();
        }

        if let Some(mut ability) = node.secondary_ability.as_mut() {
            ability.update();
        }

        match node.controller.kind {
            ActorControllerKind::Player { id } => {
                let local_player = get_global::<LocalPlayer>();
                if id == local_player.id {
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

        if let Some(target) = node.controller.primary_target {
            let direction = target.sub(node.body.position).normalize_or_zero();
            if direction.y > 0.0 {
                node.sprite.set_animation(0);
            } else if direction.y < 0.0 {
                node.sprite.set_animation(1);
            } else if direction.x > 0.0 {
                node.sprite.set_animation(2);
                node.sprite.flip_x = false;
            } else if direction.x < 0.0 {
                node.sprite.set_animation(2);
                node.sprite.flip_x = true;
            }
            if node.body.velocity != Vec2::ZERO {
                node.sprite.play();
            }
        } else if let Some(target) = node.controller.secondary_target {
            let direction = target.sub(node.body.position).normalize_or_zero();
            if direction.y > 0.0 {
                node.sprite.set_animation(0);
            } else if direction.y < 0.0 {
                node.sprite.set_animation(1);
            } else if direction.x > 0.0 {
                node.sprite.set_animation(2);
                node.sprite.flip_x = false;
            } else if direction.x < 0.0 {
                node.sprite.set_animation(2);
                node.sprite.flip_x = true;
            }
            if node.body.velocity != Vec2::ZERO {
                node.sprite.play();
            }
        } else if node.controller.direction.y > 0.0 {
            node.sprite.start_animation(0);
        } else if node.controller.direction.y < 0.0 {
            node.sprite.start_animation(1);
        } else if node.controller.direction.x > 0.0 {
            node.sprite.start_animation(2);
            node.sprite.flip_x = false;
        } else if node.controller.direction.x < 0.0 {
            node.sprite.start_animation(2);
            node.sprite.flip_x = true;
        } else {
            node.sprite.stop();
        }
    }

    fn fixed_update(mut node: RefMut<Self>) {
        node.body.velocity = node.controller.direction.normalize_or_zero() * Self::MOVE_SPEED;
        node.body.integrate();

        if let Some(target) = node.controller.primary_target {
            let position = node.body.position;
            if let Some(ability) = &mut node.primary_ability {
                ability.activate(position, target);
            }
        }
        if let Some(target) = node.controller.secondary_target {
            let position = node.body.position;
            if let Some(ability) = &mut node.secondary_ability {
                ability.activate(position, target);
            }
        }
    }

    fn draw(mut node: RefMut<Self>) {
        let (position, rotation) = (node.body.position, node.body.rotation);
        node.sprite.draw(position, rotation);
        // node.body.debug_draw();
    }
}
