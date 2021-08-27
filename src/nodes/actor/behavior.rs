use std::{
    ops::Sub,
};

use macroquad::{
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
    #[serde(default)]
    pub is_stationary: bool,
    #[serde(default)]
    pub is_on_guard: bool,
    #[serde(default)]
    pub flee_at_health_factor: f32,
}

impl Default for ActorBehaviorParams {
    fn default() -> Self {
        ActorBehaviorParams {
            aggression: ActorAggression::Neutral,
            home: None,
            is_stationary: true,
            is_on_guard: false,
            flee_at_health_factor: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActorBehavior {
    pub aggression: ActorAggression,
    pub home: Option<Vec2>,
    pub current_action: Option<String>,
    pub is_stationary: bool,
    pub is_on_guard: bool,
    pub flee_at_health_factor: f32,
    pub last_attacked_by: Option<(f32, String)>,
    pub is_in_combat: bool,
    investigating: Option<(f32, Vec2)>,
    investigate_cooldown_timer: f32,
}

impl ActorBehavior {
    pub fn new(params: ActorBehaviorParams) -> Self {
        ActorBehavior {
            aggression: params.aggression,
            home: params.home,
            current_action: None,
            is_stationary: params.is_stationary,
            is_on_guard: params.is_on_guard,
            flee_at_health_factor: params.flee_at_health_factor.clamp(0.0, 1.0),
            last_attacked_by: None,
            is_in_combat: false,
            investigating: None,
            investigate_cooldown_timer: INVESTIGATE_COOLDOWN,
        }
    }
}

const ATTACK_FORGET_AFTER: f32 = 25.0;
const GO_TO_STOP_THRESHOLD: f32 = 8.0;
const INVESTIGATE_COOLDOWN: f32 = 15.0;
const INVESTIGATE_FORGET_AFTER: f32 = 15.0;

fn go_to(actor: &mut Actor, target: Vec2) {
    actor.behavior.current_action = Some(format!("go to {}", target.to_string()));
    if actor.body.position.distance(target) > GO_TO_STOP_THRESHOLD {
        actor.controller.direction = target.sub(actor.body.position).normalize_or_zero();
    } else {
        actor.controller.direction = Vec2::ZERO;
    }
}

fn investigate(actor: &mut Actor, target: Vec2) {
    actor.behavior.current_action = Some(format!("investigating {}", target.to_string()));
    if actor.is_target_visible(target) {
        actor.controller.direction = Vec2::ZERO;
        actor.behavior.investigate_cooldown_timer = 0.0;
        actor.behavior.investigating = None;
    } else {
        actor.controller.direction = target.sub(actor.body.position).normalize_or_zero();
    }
}

fn wander(actor: &mut Actor) {
    actor.behavior.current_action = Some("wander".to_string());
    let mut direction = actor.controller.direction;
    if direction == Vec2::ZERO {
        direction = vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize_or_zero();
    }
    if actor.body.last_collisions.len() > 0 || actor.body.raycast( actor.body.position + direction * 32.0, false).is_some() {
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
        actor.equip_item(&weapon.params.id);
    }
}

fn flee(actor: &mut Actor, target: Vec2) {
    actor.behavior.is_in_combat = true;
    actor.behavior.current_action = Some(format!("flee"));
    let mut direction = actor.controller.direction;
    if actor.body.last_collisions.len() > 0 || actor.body.raycast( actor.body.position + direction * 32.0, false).is_some() {
        direction = rotate_vector(direction, deg_to_rad(6.0));
    } else {
        let mut try_direction = actor.body.position.sub(target).normalize_or_zero();
        let deg = if rand::gen_range(-1.0, 1.0) > 0.0 {
            6.0
        } else {
            -6.0
        };
        for _ in 0..60 {
            if actor.body.raycast( actor.body.position + try_direction * 32.0, false).is_some() {
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

fn attack(actor: &mut Actor, target: Vec2) {
    actor.behavior.is_in_combat = true;
    actor.behavior.current_action = Some(format!("attack"));
    let distance = actor.body.position.distance(target);
    let mut direction = target.sub(actor.body.position).normalize_or_zero();
    if distance < actor.stats.view_distance * 0.8 {
        if let Some(ability) = actor.primary_ability.as_mut() {
            if distance < ability.range * 0.9 {
                direction = Vec2::ZERO;
                actor.controller.primary_target = Some(target);
            }
        } else if let Some(ability) = actor.secondary_ability.as_mut() {
            if distance < ability.range * 0.9 {
                direction = Vec2::ZERO;
                actor.controller.secondary_target = Some(target);
            }
        } else {
            equip_weapon(actor);
        }
    }
    actor.controller.direction = direction;
}

pub fn apply_actor_behavior(actor: &mut Actor) {
    actor.behavior.is_in_combat = false;
    actor.controller.primary_target = None;
    actor.controller.secondary_target = None;

    let dt = get_frame_time();
    actor.behavior.investigate_cooldown_timer += dt;
    if let Some((timer, target)) = actor.behavior.investigating {
        let timer = timer + dt;
        if timer >= INVESTIGATE_FORGET_AFTER {
            actor.behavior.investigating = None;
        } else {
            actor.behavior.investigating = Some((timer, target));
        }
    }
    if let Some((timer, actor_id)) = actor.behavior.last_attacked_by.clone() {
        let timer = timer + dt;
        if timer >= ATTACK_FORGET_AFTER {
            actor.behavior.last_attacked_by = None;
        } else {
            actor.behavior.last_attacked_by = Some((timer, actor_id));
        }
    }

    let mut hostiles = Vec::new();
    let mut allies = Vec::new();
    let mut unknowns = Vec::new();
    'node: for other in scene::find_nodes_by_type::<Actor>() {
        let distance = actor.body.position.distance(other.body.position);
        if let Some((_, attacker_id)) = actor.behavior.last_attacked_by.clone() {
            if other.id == attacker_id {
                hostiles.push(other);
                continue 'node;
            }
        }
        if actor.is_target_visible(other.body.position) {
            for faction in &actor.factions {
                if other.factions.contains(faction) {
                    allies.push(other);
                    continue 'node;
                }
            }
            hostiles.push(other);
        } else if distance <= other.noise_level.to_range() {
            unknowns.push(other);
        }
    }

    hostiles.sort_by(|a, b|
        sort_by_distance(actor.body.position, &a.body.position, &b.body.position));

    allies.sort_by(|a, b|
        sort_by_distance(actor.body.position, &a.body.position, &b.body.position));

    unknowns.sort_by(|a, b| a.noise_level.cmp(&b.noise_level));

    match actor.behavior.aggression {
        ActorAggression::Passive => {
            if let Some((_, actor_id)) = actor.behavior.last_attacked_by.clone() {
                if let Some(attacker) = hostiles.iter().find(|hostile| hostile.id == actor_id) {
                    return flee(actor, attacker.body.position);
                }
            }
        },
        ActorAggression::Neutral => {
            if let Some((_, actor_id)) = actor.behavior.last_attacked_by.clone() {
                if let Some(attacker) = hostiles.iter().find(|hostile| hostile.id == actor_id) {
                    if actor.stats.current_health > actor.stats.max_health * actor.behavior.flee_at_health_factor {
                        return attack(actor, attacker.body.position);
                    } else {
                        return flee(actor, attacker.body.position);
                    }
                }
            }
        },
        ActorAggression::Aggressive => {
            if let Some((_, actor_id)) = actor.behavior.last_attacked_by.clone() {
                if let Some(attacker) = hostiles.iter().find(|hostile| hostile.id == actor_id) {
                    if actor.stats.current_health > actor.stats.max_health * actor.behavior.flee_at_health_factor {
                        return attack(actor, attacker.body.position);
                    } else {
                        return flee(actor, attacker.body.position);
                    }
                }
            }
            if let Some(hostile) = hostiles.first() {
                if actor.stats.current_health > actor.stats.max_health * actor.behavior.flee_at_health_factor {
                    return attack(actor, hostile.body.position);
                }
            }
        },
    }

    if actor.behavior.is_on_guard && actor.behavior.investigate_cooldown_timer >= INVESTIGATE_COOLDOWN {
        if let Some((_, target)) = actor.behavior.investigating {
            return investigate(actor, target);
        } else if let Some(unknown) = unknowns.first() {
            actor.behavior.investigating = Some((0.0, unknown.body.position));
            return investigate(actor, unknown.body.position);
        }
    }

    if actor.behavior.is_stationary {
        if let Some(home) = actor.behavior.home {
            return go_to(actor, home);
        }
    } else {
        return wander(actor);
    }
}

impl Into<ActorBehaviorParams> for ActorBehavior {
    fn into(self) -> ActorBehaviorParams {
        ActorBehaviorParams {
            aggression: self.aggression,
            home: self.home,
            is_stationary: self.is_stationary,
            is_on_guard: self.is_on_guard,
            flee_at_health_factor: self.flee_at_health_factor,
        }
    }
}
