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
    pub id: String,
    pub aggression: ActorAggression,
}

impl ActorBehaviorParams {
    pub fn default_passive() -> Self {
        ActorBehaviorParams {
            id: ActorBehavior::DEFAULT_PASSIVE_ID.to_string(),
            aggression: ActorAggression::Passive,
        }
    }

    pub fn default_neutral() -> Self {
        ActorBehaviorParams {
            id: ActorBehavior::DEFAULT_NEUTRAL_ID.to_string(),
            aggression: ActorAggression::Neutral,
        }
    }

    pub fn default_aggressive() -> Self {
        ActorBehaviorParams {
            id: ActorBehavior::DEFAULT_AGGRESSIVE_ID.to_string(),
            aggression: ActorAggression::Aggressive,
        }
    }
}

impl Default for ActorBehaviorParams {
    fn default() -> Self {
        Self::default_neutral()
    }
}

#[derive(Debug, Clone)]
pub struct ActorBehavior {
    pub id: String,
    pub aggression: ActorAggression,
}

impl ActorBehavior {
    pub const DEFAULT_PASSIVE_ID: &'static str = "default_passive";
    pub const DEFAULT_NEUTRAL_ID: &'static str = "default_neutral";
    pub const DEFAULT_AGGRESSIVE_ID: &'static str = "default_aggressive";

    pub fn default_id() -> String {
        Self::DEFAULT_NEUTRAL_ID.to_string()
    }

    pub fn new(params: ActorBehaviorParams) -> Self {
        ActorBehavior {
            id: params.id,
            aggression: params.aggression,
        }
    }
}

const VIEW_DISTANCE: f32 = 1000.0;

pub fn apply_actor_behavior(actor: &mut Actor) {
    let mut hostiles = Vec::new();
    'node: for node in scene::find_nodes_by_type::<Actor>() {
        for faction in &actor.factions {
            if node.factions.contains(faction) {
                continue 'node;
            }
        }
        hostiles.push(node);
    }

    hostiles.sort_by(|a, b|
        sort_by_distance(actor.body.position, &a.body.position, &b.body.position));

    if let Some(hostile) = hostiles.first() {
        if actor.body.position.distance(hostile.body.position) < VIEW_DISTANCE {
            match actor.behavior.aggression {
                ActorAggression::Passive => {
                    let mut direction = actor.body.position.sub(hostile.body.position).normalize_or_zero();
                    let mut i = 0;
                    while i < 60 && actor.body.raycast( actor.body.position + direction * 32.0).is_some() {
                        direction = rotate_vector(direction, deg_to_rad(6.0));
                        i += 1;
                    }
                    actor.controller.direction = direction;
                    actor.controller.is_sprinting = true;
                },
                ActorAggression::Neutral => {},
                ActorAggression::Aggressive => {
                    let mut direction = hostile.body.position.sub(actor.body.position).normalize_or_zero();
                    let mut i = 0;
                    while i < 60 && actor.body.raycast( actor.body.position + direction * 32.0).is_some() {
                        direction = rotate_vector(direction, deg_to_rad(6.0));
                        i += 1;
                    }
                    actor.controller.direction = direction;
                    actor.controller.is_sprinting = true;
                    if actor.secondary_ability.is_some() {
                        actor.controller.secondary_target = Some(hostile.body.position);
                    }
                    if actor.primary_ability.is_some() {
                        actor.controller.primary_target = Some(hostile.body.position);
                    }
                },
            }
            return;
        }
    }
    actor.controller.direction = Vec2::ZERO;
}
