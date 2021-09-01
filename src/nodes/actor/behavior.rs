use std::{
    ops::Sub,
};

use crate::prelude::*;

use mode::{
    Family,
    Automaton,
    Mode,
};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorAggression {
    Passive,
    Neutral,
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
    #[serde(skip)]
    pub last_attacked_by: Option<String>,
    #[serde(skip)]
    pub current_path: Option<NavigationPath>,
}

impl Default for ActorBehaviorParams {
    fn default() -> Self {
        ActorBehaviorParams {
            aggression: ActorAggression::Neutral,
            home: None,
            is_stationary: true,
            is_on_guard: false,
            flee_at_health_factor: 0.0,
            last_attacked_by: None,
            current_path: None,
        }
    }
}

pub trait ActorBehavior : Mode<Family = ActorBehaviorFamily> {
    fn update(self : Box<Self>) -> Box<dyn ActorBehavior>;
}

pub struct ActorBehaviorFamily;

impl Family for ActorBehaviorFamily {
    type Base = dyn ActorBehavior;

    type Mode = Box<dyn ActorBehavior>;
}

pub struct TestMode {}

impl Mode for TestMode {
    type Family = ActorBehaviorFamily;
}

impl ActorBehavior for TestMode {
    fn update(mut self : Box<Self>) -> Box<dyn ActorBehavior> {
        println!("test behavior");
        self
    }
}
