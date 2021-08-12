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
mod draw_buffer;
mod stats;

pub use stats::ActorStats;
pub use controller::{
    ActorControllerKind,
    ActorController,
};

pub use inventory::ActorInventory;
pub use ability::{
    ActorAbility,
    ActorAbilityFunc,
};

pub use draw_buffer::ActorDrawBuffer;

use crate::{
    get_global,
    render::{
        SpriteAnimationPlayer,
        SpriteParams,
        draw_progress_bar,
        HorizontalAlignment,
    },
    globals::LocalPlayer,
    physics::{
        PhysicsBody,
        PhysicsObject,
        Collider,
    },
    nodes::Projectiles,
    ItemParams,
    generate_id,
};
use crate::nodes::Item;

#[derive(Clone)]
pub struct ActorParams {
    pub id: String,
    pub name: String,
    pub stats: ActorStats,
    pub factions: Vec<String>,
    pub position: Vec2,
    pub collider: Option<Collider>,
    pub inventory: Vec<ItemParams>,
    pub sprite_params: SpriteParams,

    pub controller_kind: ActorControllerKind,
}

impl Default for ActorParams {
    fn default() -> Self {
        ActorParams {
            id: generate_id(),
            name: "Unnamed Actor".to_string(),
            stats: Default::default(),
            factions: Vec::new(),
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
    pub name: String,
    pub stats: ActorStats,
    pub factions: Vec<String>,
    pub body: PhysicsBody,
    sprite: SpriteAnimationPlayer,
    pub inventory: ActorInventory,
    primary_ability: Option<ActorAbility>,
    secondary_ability: Option<ActorAbility>,
    pub controller: ActorController,
}

pub fn primary_test_ability(actor_id: &str, origin: Vec2, target: Vec2) {
    let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
    projectiles.spawn(actor_id, 15.0, color::YELLOW, 2.0, origin, target, 15.0, 10.0, 1.0);
}

pub fn secondary_test_ability(actor_id: &str, origin: Vec2, target: Vec2) {
    let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
    projectiles.spawn(actor_id, 150.0, color::BLUE, 100.0, origin, target, 2.0, 0.0, 2.0);
}

impl Actor {
    const HEALTH_BAR_LENGTH: f32 = 50.0;
    const HEALTH_BAR_HEIGHT: f32 = 10.0;
    const HEALTH_BAR_OFFSET_Y: f32 = 25.0;

    const SPRINT_SPEED_FACTOR: f32 = 2.0;
    const SPRINT_STAMINA_COST: f32 = 10.0;

    const PICK_UP_RADIUS: f32 = 36.0;

    pub fn new(params: ActorParams) -> Self {
        let id = params.id.clone();
        Actor {
            id: params.id,
            name: params.name,
            stats: params.stats,
            factions: params.factions,
            body: PhysicsBody::new(params.position, 0.0, params.collider),
            sprite: SpriteAnimationPlayer::new(params.sprite_params.clone()),
            inventory: ActorInventory::new(&params.inventory),
            primary_ability: Some(ActorAbility::new(0.0, 4.0, 0.0, 0.0025, primary_test_ability)),
            secondary_ability: Some(ActorAbility::new(0.0, 4.0, 50.0, 1.25, secondary_test_ability)),
            controller: ActorController::new(params.controller_kind),
        }
    }

    pub fn add_node(params: ActorParams) -> Handle<Self> {
        scene::add_node(Self::new(params))
    }

    pub fn to_actor_params(&self) -> ActorParams {
        ActorParams {
            id: self.id.clone(),
            name: self.name.clone(),
            stats: self.stats.clone(),
            factions: self.factions.clone(),
            position: self.body.position,
            collider: self.body.collider,
            inventory: self.inventory.clone_data(),
            sprite_params: self.sprite.to_sprite_params(),
            controller_kind: self.controller.kind,
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.stats.current_health -= damage;
    }

    pub fn find_player(player_id: u32) -> Option<RefMut<Self>> {
        for actor in scene::find_nodes_by_type::<Self>() {
            match actor.controller.kind {
                ActorControllerKind::Player { id } => {
                    if player_id == id {
                        return Some(actor);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn find_local_player() -> Option<RefMut<Self>> {
        let local_player = get_global::<LocalPlayer>();
        if let Some(actor) = Self::find_player(local_player.id) {
            Some(actor)
        } else {
            None
        }
    }

    pub fn find_with_id(id: &str) -> Option<RefMut<Actor>> {
        for actor in scene::find_nodes_by_type::<Actor>() {
            if actor.id == id.to_string() {
                return Some(actor);
            }
        }
        None
    }

    pub fn set_animation(&mut self, direction: Vec2, is_stationary: bool) {
        if direction.y > 0.0 && direction.y.abs() > direction.x.abs() {
            self.sprite.start_animation(0);
        } else if direction.y < 0.0 {
            self.sprite.start_animation(1);
        } else if direction.x > 0.0 {
            self.sprite.start_animation(2);
            self.sprite.flip_x = false;
        } else if direction.x < 0.0 {
            self.sprite.start_animation(2);
            self.sprite.flip_x = true;
        } else {
            self.sprite.set_frame(1);
            self.sprite.stop();
        }
        if is_stationary {
            self.sprite.set_frame(1);
            self.sprite.stop();
        }
    }

    pub fn draw_actor(&mut self) {
        let (position, rotation) = (self.body.position, self.body.rotation);
        self.sprite.draw(position, rotation);
        // self.body.debug_draw();

        let is_local_player = self.is_local_player();
        if is_local_player || self.stats.current_health < self.stats.max_health {
            draw_progress_bar(
                self.stats.current_health,
                self.stats.max_health,
                self.body.position + vec2(0.0, Self::HEALTH_BAR_OFFSET_Y),
                Self::HEALTH_BAR_LENGTH,
                Self::HEALTH_BAR_HEIGHT,
                color::RED,
                color::GRAY,
                1.0,
                HorizontalAlignment::Center,
                None, // Some(&format!("{}/{}", self.stats.current_health.round(), self.stats.max_health.round())),
                None,
            );
        }
        if is_local_player {
            draw_progress_bar(
                self.stats.current_stamina,
                self.stats.max_stamina,
                self.body.position + vec2(0.0, Self::HEALTH_BAR_OFFSET_Y + Self::HEALTH_BAR_HEIGHT),
                Self::HEALTH_BAR_LENGTH,
                Self::HEALTH_BAR_HEIGHT,
                color::YELLOW,
                color::GRAY,
                1.0,
                HorizontalAlignment::Center,
                None, // Some(&format!("{}/{}", self.stats.current_stamina.round(), self.stats.max_stamina.round())),
                None,
            );
            draw_progress_bar(
                self.stats.current_energy,
                self.stats.max_energy,
                self.body.position + vec2(0.0, Self::HEALTH_BAR_OFFSET_Y + Self::HEALTH_BAR_HEIGHT * 2.0),
                Self::HEALTH_BAR_LENGTH,
                Self::HEALTH_BAR_HEIGHT,
                color::BLUE,
                color::GRAY,
                1.0,
                HorizontalAlignment::Center,
                None, // Some(&format!("{}/{}", self.stats.current_energy.round(), self.stats.max_energy.round())),
                None,
            );
        }
    }

    pub fn is_local_player(&self) -> bool {
        if let ActorControllerKind::Player { id } = self.controller.kind {
            let local_player = get_global::<LocalPlayer>();
            id == local_player.id
        } else {
            false
        }
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
        node.stats.update_derived(false);

        if node.stats.current_health <= 0.0 {
            let position = node.body.position;
            node.inventory.drop_all(position);
            node.delete();
            return;
        }

        if let Some(ability) = node.primary_ability.as_mut() {
            ability.update();
        }

        if let Some(ability) = node.secondary_ability.as_mut() {
            ability.update();
        }

        match node.controller.kind {
            ActorControllerKind::Player { id } => {
                let local_player = get_global::<LocalPlayer>();
                if id == local_player.id {
                    apply_local_player_input(&mut node.controller);
                } else {
                    // TODO: Remote player (?)
                }
            }
            ActorControllerKind::Computer => {
                // TODO: Computer controlled
            }
            ActorControllerKind::None => {}
        }

        let controller_direction = node.controller.direction;
        if let Some(target) = node.controller.primary_target {
            let direction = target.sub(node.body.position).normalize_or_zero();
            node.set_animation(direction, controller_direction == Vec2::ZERO);
        } else if let Some(target) = node.controller.secondary_target {
            let direction = target.sub(node.body.position).normalize_or_zero();
            node.set_animation(direction, controller_direction == Vec2::ZERO);
        } else {
            node.set_animation(controller_direction, false);
        }

        let controller = node.controller.clone();
        if let Some(target) = controller.primary_target {
            let mut primary_ability = node.primary_ability.clone();
            let position = node.body.position.clone();
            if let Some(ability) = &mut primary_ability {
                ability.activate(&mut *node, position, target);
            }
            node.primary_ability = primary_ability;
        }
        if let Some(target) = controller.secondary_target {
            let mut secondary_ability = node.secondary_ability.clone();
            let position = node.body.position.clone();
            if let Some(ability) = &mut secondary_ability {
                ability.activate(&mut *node, position, target);
            }
            node.secondary_ability = secondary_ability;
        }
    }

    fn fixed_update(mut node: RefMut<Self>) {
        let direction = node.controller.direction.normalize_or_zero();
        node.body.velocity = direction * if node.controller.is_sprinting && node.stats.current_stamina >= Self::SPRINT_STAMINA_COST {
            if direction != Vec2::ZERO {
                node.stats.current_stamina -= Self::SPRINT_STAMINA_COST;
            }
            node.stats.move_speed * Self::SPRINT_SPEED_FACTOR
        } else {
            node.stats.move_speed
        };
        node.body.integrate();

        if node.controller.pick_up_items {
            let collider = Collider::circle(0.0, 0.0, Self::PICK_UP_RADIUS).offset(node.body.position);
            for item in scene::find_node_by_type::<Item>() {
                if collider.contains(item.position) {
                    node.inventory.pick_up_item(item);
                }
            }
        }
    }

    fn draw(node: RefMut<Self>) {
        let mut draw_queue = scene::find_node_by_type::<ActorDrawBuffer>().unwrap();
        draw_queue.add_to_buffer(node.handle());
    }
}
