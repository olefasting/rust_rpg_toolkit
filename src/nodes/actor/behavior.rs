use crate::prelude::*;

use mode::{
    Family,
    Mode,
};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorAggression {
    Passive,
    Neutral,
    Aggressive,
}

pub fn default_behavior_set() -> String {
    DEFAULT_BEHAVIOR_SET_ID.to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorBehaviorParams {
    pub aggression: ActorAggression,
    #[serde(default, with = "json::opt_vec2", skip_serializing_if = "Option::is_none")]
    pub home: Option<Vec2>,
    #[serde(default = "default_behavior_set", rename = "behavior_set")]
    pub behavior_set_id: String,
    #[serde(default)]
    pub is_stationary: bool,
    #[serde(default)]
    pub is_on_guard: bool,
    #[serde(default)]
    pub flee_at_health_factor: f32,
    #[serde(skip)]
    pub attackers: HashMap<String, Handle<Actor>>,
    #[serde(skip)]
    pub collisions: Vec<(Collider, CollisionKind)>,
}

impl Default for ActorBehaviorParams {
    fn default() -> Self {
        ActorBehaviorParams {
            aggression: ActorAggression::Neutral,
            home: None,
            behavior_set_id: DEFAULT_BEHAVIOR_SET_ID.to_string(),
            is_stationary: true,
            is_on_guard: false,
            flee_at_health_factor: 0.0,
            attackers: HashMap::new(),
            collisions: Vec::new(),
        }
    }
}

pub trait ActorBehavior: Mode<Family=ActorBehaviorFamily> {
    fn update(
        self: Box<Self>,
        params: ActorBehaviorParams,
        factions: &[String],
        stats: ActorStats,
        position: Vec2,
        controller: &mut ActorController,
        weapon_range: Option<f32>,
        selected_ability_range: Option<f32>,
        inventory: Inventory,
        equipped_items: EquippedItems,
    ) -> Box<dyn ActorBehavior>;
}

pub struct ActorBehaviorFamily;

impl Family for ActorBehaviorFamily {
    type Base = dyn ActorBehavior;

    type Mode = Box<dyn ActorBehavior>;
}
