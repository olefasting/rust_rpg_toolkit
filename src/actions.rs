use std::collections::HashMap;

use macroquad::{
    color,
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{Projectiles, generate_id};

#[derive(Clone, Serialize, Deserialize)]
pub struct ActionParams {
    pub id: String,
    pub action_kind: String,
    pub action_func_id: String,
    pub cooldown: f32,
    pub health_cost: f32,
    pub stamina_cost: f32,
    pub energy_cost: f32,
    pub speed: f32,
    pub spread: f32,
    pub range: f32,
    pub damage: f32,
}

impl ActionParams {
    pub const PRIMARY_ABILITY: &'static str = "primary";
    pub const SECONDARY_ABILITY: &'static str = "secondary";
}

impl Default for ActionParams {
    fn default() -> Self {
        ActionParams {
            id: generate_id(),
            action_kind: Self::PRIMARY_ABILITY.to_string(),
            action_func_id: ActionFuncs::PROJECTILE_ACTION_ID.to_string(),
            cooldown: 0.0,
            health_cost: 0.0,
            stamina_cost: 0.0,
            energy_cost: 0.0,
            speed: 75.0,
            spread: 0.0,
            range: 100.0,
            damage: 0.0,
        }
    }
}

pub type ActionFunc = fn (actor_id: &str, origin: Vec2, target: Vec2, speed: f32, spread: f32, range: f32, damage: f32);

fn projectile_action(actor_id: &str, origin: Vec2, target: Vec2, speed: f32, spread: f32, range: f32, damage: f32) {
    let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
    let ttl = range / speed;
    projectiles.spawn(actor_id, damage, color::YELLOW, 2.0, origin, target, speed, spread, ttl);
}

fn magic_sphere_action(actor_id: &str, origin: Vec2, target: Vec2, speed: f32, spread: f32, range: f32, damage: f32) {
    let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
    let ttl = range / speed;
    projectiles.spawn(actor_id, damage, color::BLUE, 100.0, origin, target, speed, spread, ttl);
}

pub struct ActionFuncs {
    actions: HashMap<String, ActionFunc>
}

impl ActionFuncs {
    pub const PROJECTILE_ACTION_ID: &'static str = "projectile";
    pub const MAGIC_SPHERE_ACTION_ID: &'static str = "magic_sphere";

    pub async fn new() -> Self {
        let mut actions: HashMap<String, ActionFunc> = HashMap::new();
        actions.insert(Self::PROJECTILE_ACTION_ID.to_string(), projectile_action);
        actions.insert(Self::MAGIC_SPHERE_ACTION_ID.to_string(), magic_sphere_action);
        ActionFuncs {
            actions,
        }
    }

    pub fn get(&self, id: &str) -> ActionFunc {
        *self.actions.get(id).unwrap()
    }

    pub fn try_get(&self, id: &str) -> Option<ActionFunc> {
        let func = self.actions.get(id);
        match func {
            Some(action) => Some(*action),
            None => None,
        }
    }
}
