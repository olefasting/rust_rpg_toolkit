use std::{
    ops::Sub,
};

use macroquad::{
    experimental::scene::RefMut,
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    helpers::sort_by_distance,
    math::{
        rotate_vector,
        deg_to_rad,
    },
    json,
    ability::ActionKind,
    nodes::item::ItemKind
};

use super::Actor;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActorAggression {
    #[serde(rename = "passive")]
    Passive,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "aggressive")]
    Aggressive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorBehaviorParams {
    pub aggression: ActorAggression,
    #[serde(default, with = "json::opt_vec2", skip_serializing_if = "Option::is_none")]
    pub home: Option<Vec2>,
    pub is_stationary: bool,
}

impl Default for ActorBehaviorParams {
    fn default() -> Self {
        ActorBehaviorParams {
            aggression: ActorAggression::Neutral,
            home: None,
            is_stationary: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActorBehavior {
    pub aggression: ActorAggression,
    pub is_stationary: bool,
    pub home: Option<Vec2>,
    pub current_action: Option<String>,
}

impl ActorBehavior {
    pub fn new(params: ActorBehaviorParams) -> Self {
        ActorBehavior {
            aggression: params.aggression,
            is_stationary: params.is_stationary,
            home: params.home,
            current_action: None,
        }
    }
}

const GO_TO_STOP_THRESHOLD: f32 = 8.0;

fn go_to(actor: &mut Actor, target: Vec2) {
    actor.behavior.current_action = Some(format!("go to {}", target.to_string()));
    if actor.body.position.distance(target) > GO_TO_STOP_THRESHOLD {
        actor.controller.direction = target.sub(actor.body.position).normalize_or_zero();
    } else {
        actor.controller.direction = Vec2::ZERO;
    }
}

fn wander(actor: &mut Actor) {
    actor.behavior.current_action = Some("wander".to_string());
    let mut direction = actor.controller.direction;
    if direction == Vec2::ZERO {
        direction = vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize_or_zero();
    }
    if actor.body.last_collisions.len() > 0 || actor.body.raycast( actor.body.position + direction * 32.0).is_some() {
        let deg = if rand::gen_range(-1.0, 1.0) > 0.0 {
            45.0
        } else {
            -45.0
        };
        direction = rotate_vector(direction, deg_to_rad(deg));
    }
    actor.controller.direction = direction;
}

fn equip_weapon(actor: &mut Actor) {
    actor.behavior.current_action = Some("equip weapon".to_string());
    let weapons = actor.inventory.get_all_of_kind(&[ItemKind::OneHandedWeapon, ItemKind::TwoHandedWeapon]);
    let i = rand::gen_range(0, weapons.len() - 1);
    if let Some(weapon) = weapons.get(i) {
        match weapon.params.ability.action_kind {
            ActionKind::Primary =>
                actor.primary_ability = Some(weapon.to_actor_ability()),
            ActionKind::Secondary =>
                actor.secondary_ability = Some(weapon.to_actor_ability()),
        }
    }
}

fn flee(actor: &mut Actor, hostile: &RefMut<Actor>) {
    actor.behavior.current_action = Some(format!("flee from {}", hostile.name));
    let mut direction = actor.controller.direction;
    if actor.body.last_collisions.len() > 0 || actor.body.raycast( actor.body.position + direction * 32.0).is_some() {
        direction = rotate_vector(direction, deg_to_rad(6.0));
    } else {
        let mut try_direction = actor.body.position.sub(hostile.body.position).normalize_or_zero();
        let deg = if rand::gen_range(-1.0, 1.0) > 0.0 {
            6.0
        } else {
            -6.0
        };
        for _ in 0..60 {
            if actor.body.raycast( actor.body.position + try_direction * 32.0).is_some() {
                try_direction = rotate_vector(direction, deg_to_rad(deg));
                continue;
            }
            direction = try_direction;
            break;
        }
    }
    actor.controller.direction = direction;
    actor.controller.is_sprinting = true;
}

fn attack(actor: &mut Actor, hostile: &RefMut<Actor>) {
    actor.behavior.current_action = Some(format!("attack {}", hostile.name));
    let distance = actor.body.position.distance(hostile.body.position);
    let mut direction = hostile.body.position.sub(actor.body.position).normalize_or_zero();
    if distance < actor.stats.view_distance * 0.8 {
        if let Some(ability) = actor.primary_ability.as_mut() {
            if distance > ability.range * 0.8 {
                direction = Vec2::ZERO;
                actor.controller.primary_target = Some(hostile.body.position);
            }
        } else if let Some(ability) = actor.secondary_ability.as_mut() {
            if distance > ability.range * 0.8 {
                direction = Vec2::ZERO;
                actor.controller.secondary_target = Some(hostile.body.position);
            }
        } else {
            equip_weapon(actor);
        }
    }
    actor.controller.direction = direction;
}

pub fn apply_actor_behavior(actor: &mut Actor) {
    let mut hostiles = Vec::new();
    let mut allies = Vec::new();
    'node: for node in scene::find_nodes_by_type::<Actor>() {
        if actor.body.position.distance(node.body.position) <= actor.stats.view_distance {
            for faction in &actor.factions {
                if node.factions.contains(faction) {
                    allies.push(node);
                    continue 'node;
                }
            }
            hostiles.push(node);
        }
    }

    hostiles.sort_by(|a, b|
        sort_by_distance(actor.body.position, &a.body.position, &b.body.position));

    allies.sort_by(|a, b|
        sort_by_distance(actor.body.position, &a.body.position, &b.body.position));

    if let Some(hostile) = hostiles.first() {
        match actor.behavior.aggression {
            ActorAggression::Passive => {
                flee(actor, hostile);
            },
            ActorAggression::Neutral => {
                if actor.stats.current_health > actor.stats.max_health * 0.2 {
                    attack(actor, hostile);
                } else {
                    flee(actor, hostile);
                }
            },
            ActorAggression::Aggressive => {
                if actor.stats.current_health > actor.stats.max_health * 0.2 {
                    attack(actor, hostile);
                } else {
                    flee(actor, hostile);
                }
            },
        }
    } else {
        actor.controller.primary_target = None;
        actor.controller.secondary_target = None;
        if let Some(home) = actor.behavior.home {
            go_to(actor, home);
        } else {
            wander(actor);
        }
    }
}

impl Into<ActorBehaviorParams> for ActorBehavior {
    fn into(self) -> ActorBehaviorParams {
        ActorBehaviorParams {
            aggression: self.aggression,
            home: self.home,
            is_stationary: self.is_stationary,
        }
    }
}
