use std::{
    ops::Sub,
};

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

pub trait ActorBehavior : Mode<Family = ActorBehaviorFamily> {
    fn update(self : Box<Self>, params: ActorBehaviorParams, position: Vec2, controller: &mut ActorController) -> Box<dyn ActorBehavior>;
}

pub struct ActorBehaviorFamily;

impl Family for ActorBehaviorFamily {
    type Base = dyn ActorBehavior;

    type Mode = Box<dyn ActorBehavior>;
}

pub struct IdleMode {
}

impl Mode for IdleMode {
    type Family = ActorBehaviorFamily;
}

impl ActorBehavior for IdleMode {
    fn update(self : Box<Self>, params: ActorBehaviorParams, position: Vec2, _: &mut ActorController) -> Box<dyn ActorBehavior> {
        if params.is_stationary {
            if let Some(home) = params.home {
                if position.distance(home) > 2.0 {
                    return Box::new(GoToMode::new(home));
                }
            }
        } else {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            let dist_x = 5.0 * game_state.map.tile_size.x;
            let dist_y = 5.0 * game_state.map.tile_size.y;
            let x = rand::gen_range(position.x - dist_x, position.x + dist_x);
            let y = rand::gen_range(position.y - dist_y, position.y + dist_y);
            return Box::new(GoToMode::new(vec2(x, y)));
        }
        self
    }
}

pub struct GoToMode {
    pub destination: Vec2,
    path: Option<NavigationPath>,
}

impl GoToMode {
    pub fn new(destination: Vec2) -> Self {
        GoToMode {
            destination,
            path: None,
        }
    }
}

impl Mode for GoToMode {
    type Family = ActorBehaviorFamily;
}

impl ActorBehavior for GoToMode {
    fn update(mut self: Box<Self>, _params: ActorBehaviorParams, position: Vec2, controller: &mut ActorController) -> Box<dyn ActorBehavior> {
        if self.path.is_none() {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            self.path = game_state.map.get_path(position,self.destination);
            controller.current_path = self.path.clone();
        }
        if let Some(path) = self.path.clone() {
            self.path = process_path(position, controller, path);
        }
        if self.path.is_none() {
            return Box::new(IdleMode{});
        }
        self
    }
}

fn process_path(position: Vec2, controller: &mut ActorController, mut path: NavigationPath) -> Option<NavigationPath> {
    if let Some(mut node) = path.nodes.first().cloned() {
        if position.distance(node) <= 2.0 {
            path.nodes.remove(0);
            if  let Some(next) = path.nodes.first().cloned() {
                node = next;
            } else {
                return None;
            }
        }
        controller.move_direction = node.sub(position).normalize_or_zero();
        return Some(path);
    }
    None
}
