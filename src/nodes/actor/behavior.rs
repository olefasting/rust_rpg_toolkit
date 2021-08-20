use std::ops::Sub;

use macroquad::prelude::*;

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    helpers::sort_by_distance,
};

use super::Actor;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub enum ActorAggression {
    Passive,
    Neutral,
    Aggressive,
}

impl ActorAggression {
    const PASSIVE_STRING: &'static str = "passive";
    const NEUTRAL_STRING: &'static str = "neutral";
    const AGGRESSIVE_STRING: &'static str = "aggressive";
}

impl Default for ActorAggression {
    fn default() -> Self {
        Self::Neutral
    }
}

impl Into<String> for ActorAggression {
    fn into(self) -> String {
        match self {
            Self::Passive => Self::PASSIVE_STRING.to_string(),
            Self::Neutral => Self::NEUTRAL_STRING.to_string(),
            Self::Aggressive => Self::AGGRESSIVE_STRING.to_string(),
        }
    }
}

impl From<String> for ActorAggression {
    fn from(string: String) -> Self {
        if string == Self::PASSIVE_STRING {
            Self::Neutral
        } else if string == Self::AGGRESSIVE_STRING {
            Self::Aggressive
        } else {
            Self::default()
        }
    }
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
        match actor.behavior.aggression {
            ActorAggression::Passive => {
                let direction = actor.body.position.sub(hostile.body.position).normalize_or_zero();
                actor.controller.direction = direction;
                actor.controller.is_sprinting = true;
            },
            ActorAggression::Neutral => {},
            ActorAggression::Aggressive => {
                let direction = hostile.body.position.sub(actor.body.position).normalize_or_zero();
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
    }
}
