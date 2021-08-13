use macroquad::{
    color,
    prelude::*
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{Actor, generate_id, json};
use crate::nodes::Projectiles;

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorAbilityParams {
    pub id: Option<String>,
    pub action_kind: String,
    pub cooldown: f32,
    pub health_cost: f32,
    pub stamina_cost: f32,
    pub energy_cost: f32,
    pub speed: f32,
    pub spread: f32,
    pub range: f32,
    pub damage: f32,
    pub projectile_size: f32,
    pub projectile_color: json::Color,
}

impl Default for ActorAbilityParams {
    fn default() -> Self {
        ActorAbilityParams {
            id: Some(generate_id()),
            action_kind: ActorAbility::PRIMARY_ABILITY.to_string(),
            cooldown: 0.0,
            health_cost: 0.0,
            stamina_cost: 0.0,
            energy_cost: 0.0,
            speed: 75.0,
            spread: 0.0,
            range: 100.0,
            damage: 0.0,
            projectile_size: 5.0,
            projectile_color: json::Color::from(color::WHITE),
        }
    }
}

#[derive(Clone)]
pub struct ActorAbility {
    pub id: String,
    pub action_kind: String,
    pub cooldown: f32,
    pub cooldown_timer: f32,
    pub health_cost: f32,
    pub stamina_cost: f32,
    pub energy_cost: f32,
    pub speed: f32,
    pub spread: f32,
    pub range: f32,
    pub damage: f32,
    pub projectile_size: f32,
    pub projectile_color: Color,
}

impl ActorAbility {
    pub const PRIMARY_ABILITY: &'static str = "primary";
    pub const SECONDARY_ABILITY: &'static str = "secondary";

    pub fn new(params: ActorAbilityParams) -> Self {
        ActorAbility {
            id: generate_id(),
            action_kind: params.action_kind,
            health_cost: params.health_cost,
            stamina_cost: params.stamina_cost,
            energy_cost: params.energy_cost,
            cooldown: params.cooldown,
            cooldown_timer: params.cooldown,
            speed: params.speed,
            spread: params.spread,
            range: params.range,
            damage: params.damage,
            projectile_size: params.projectile_size,
            projectile_color: params.projectile_color.to_macroquad(),
        }
    }

    pub fn activate(&mut self, actor: &mut Actor, origin: Vec2, target: Vec2) {
        if self.health_cost > 0.0 && actor.stats.current_health < self.health_cost {
            return;
        }
        if self.stamina_cost > 0.0 && actor.stats.current_stamina < self.stamina_cost {
            return;
        }
        if self.energy_cost > 0.0 && actor.stats.current_energy < self.energy_cost {
            return;
        }
        if self.cooldown_timer >= self.cooldown {
            actor.stats.current_health -= self.health_cost;
            actor.stats.current_stamina -= self.stamina_cost;
            actor.stats.current_energy -= self.energy_cost;
            self.cooldown_timer = 0.0;
            let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
            let ttl = self.range / self.speed;
            projectiles.spawn(
                &actor.id,
                self.damage,
                self.projectile_color,
                self.projectile_size,
                origin,
                target,
                self.speed,
                self.spread,
                ttl,
            );
        }
    }

    pub fn update(&mut self) {
        self.cooldown_timer += get_frame_time();
    }
}
