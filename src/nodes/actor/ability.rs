use macroquad::{
    color,
    prelude::*
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{Actor, generate_id, json};
use crate::nodes::{Projectiles, Beams};
use std::ops::Sub;

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorAbilityParams {
    pub id: Option<String>,
    pub effect_kind: String,
    pub action_kind: String,
    pub cooldown: Option<f32>,
    pub health_cost: f32,
    pub stamina_cost: f32,
    pub energy_cost: f32,
    pub speed: f32,
    pub spread: f32,
    pub range: f32,
    pub damage: f32,
    pub effect_size: f32,
    pub effect_color: json::Color,
}

impl Default for ActorAbilityParams {
    fn default() -> Self {
        ActorAbilityParams {
            id: Some(generate_id()),
            effect_kind: ActorAbility::PROJECTILE_EFFECT.to_string(),
            action_kind: ActorAbility::PRIMARY_ABILITY.to_string(),
            cooldown: None,
            health_cost: 0.0,
            stamina_cost: 0.0,
            energy_cost: 0.0,
            speed: 75.0,
            spread: 0.0,
            range: 100.0,
            damage: 0.0,
            effect_size: 5.0,
            effect_color: json::Color::from(color::WHITE),
        }
    }
}

#[derive(Clone)]
pub struct ActorAbility {
    pub id: String,
    pub effect_kind: String,
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
    pub effect_size: f32,
    pub effect_color: Color,
}

impl ActorAbility {
    pub const PROJECTILE_EFFECT: &'static str = "projectile";
    pub const ENERGY_SPHERE: &'static str = "energy_sphere";
    pub const BEAM_EFFECT: &'static str = "beam";

    pub const PRIMARY_ABILITY: &'static str = "primary";
    pub const SECONDARY_ABILITY: &'static str = "secondary";

    pub fn new(params: ActorAbilityParams) -> Self {
        ActorAbility {
            id: generate_id(),
            effect_kind: params.effect_kind,
            action_kind: params.action_kind,
            health_cost: params.health_cost,
            stamina_cost: params.stamina_cost,
            energy_cost: params.energy_cost,
            cooldown: params.cooldown.unwrap_or_default(),
            cooldown_timer: params.cooldown.unwrap_or_default(),
            speed: params.speed,
            spread: params.spread,
            range: params.range,
            damage: params.damage,
            effect_size: params.effect_size,
            effect_color: params.effect_color.to_macroquad(),
        }
    }

    pub fn activate(&mut self, actor: &mut Actor, origin: Vec2, target: Vec2) {
        if (self.health_cost == 0.0 || actor.stats.current_health >= self.health_cost)
            && (self.stamina_cost == 0.0 || actor.stats.current_stamina >= self.stamina_cost)
            && (self.energy_cost == 0.0 || actor.stats.current_energy >= self.energy_cost) {
            if self.effect_kind == ActorAbility::BEAM_EFFECT {
                actor.stats.current_health -= self.health_cost;
                actor.stats.current_stamina -= self.stamina_cost;
                actor.stats.current_energy -= self.energy_cost;
                let mut beams = scene::find_node_by_type::<Beams>().unwrap();
                let end = actor.body.position + target.sub(actor.body.position).normalize_or_zero() * self.range;
                beams.spawn(
                    &actor.id,
                    &actor.factions,
                    self.damage,
                    self.effect_color,
                    self.effect_size,
                    actor.body.position,
                    end,
                )
            } else if self.cooldown_timer >= self.cooldown {
                actor.stats.current_health -= self.health_cost;
                actor.stats.current_stamina -= self.stamina_cost;
                actor.stats.current_energy -= self.energy_cost;
                self.cooldown_timer = 0.0;
                let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
                let ttl = self.range / self.speed;
                let is_sphere = [
                    ActorAbility::ENERGY_SPHERE.to_string(),
                ].contains(&self.effect_kind);
                projectiles.spawn(
                    &actor.id,
                    &actor.factions,
                    self.damage,
                    self.effect_color,
                    self.effect_size,
                    origin,
                    target,
                    self.speed,
                    self.spread,
                    ttl,
                    is_sphere,
                );
            }
        }
    }

    pub fn update(&mut self) {
        self.cooldown_timer += get_frame_time();
    }
}
