use std::ops::Sub;

use macroquad::{
    experimental::{
        collections::storage,
    },
    audio::{
        Sound,
        play_sound_once,
    },
    color,
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    Actor,
    nodes::{
        Projectiles,
        ContinuousBeams,
        projectiles::ProjectileKind,
        actor::ActorNoiseLevel
    },
    Resources,
    json,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Effect {
    #[serde(rename = "projectile")]
    Damage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AbilityDelivery {
    #[serde(rename = "projectile")]
    Projectile {
        projectile_kind: ProjectileKind,
        spread: f32,
        speed: f32,
    },
    #[serde(rename = "continuous_beam")]
    ContinuousBeam,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AbilityParams {
    pub id: String,
    #[serde(default, rename = "sound_effect", skip_serializing_if = "Option::is_none")]
    pub sound_effect_id: Option<String>,
    #[serde(default)]
    pub noise_level: ActorNoiseLevel,
    pub delivery: AbilityDelivery,
    #[serde(default)]
    pub cooldown: f32,
    #[serde(default)]
    pub health_cost: f32,
    #[serde(default)]
    pub stamina_cost: f32,
    #[serde(default)]
    pub energy_cost: f32,
    pub range: f32,
    pub damage: f32,
    pub effects: Vec<Effect>,
    #[serde(default, with = "json::opt_color", skip_serializing_if = "Option::is_none")]
    pub color_override: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_override: Option<f32>,
}

impl Default for AbilityParams {
    fn default() -> Self {
        AbilityParams {
            id: "".to_string(),
            sound_effect_id: None,
            noise_level: ActorNoiseLevel::Moderate,
            delivery: AbilityDelivery::Projectile {
                projectile_kind: ProjectileKind::Bullet,
                speed: 8.0,
                spread: 5.0
            },
            cooldown: 0.0,
            health_cost: 0.0,
            stamina_cost: 0.0,
            energy_cost: 0.0,
            damage: 0.0,
            range: 5.0,
            effects: Vec::new(),
            color_override: None,
            size_override: None,
        }
    }
}

#[derive(Clone)]
pub struct Ability {
    pub noise_level: ActorNoiseLevel,
    pub delivery: AbilityDelivery,
    pub sound_effect: Option<Sound>,
    pub cooldown: f32,
    pub cooldown_timer: f32,
    pub health_cost: f32,
    pub stamina_cost: f32,
    pub energy_cost: f32,
    pub damage: f32,
    pub range: f32,
    pub effects: Vec<Effect>,
    pub color_override: Option<Color>,
    pub size_override: Option<f32>,
}

impl Ability {
    pub fn new(params: AbilityParams) -> Self {
        let sound_effect = if let Some(sound_id) = params.sound_effect_id {
            let resources = storage::get::<Resources>();
            resources.sound_effects.get(&sound_id).cloned()
        } else {
            None
        };

        Ability {
            sound_effect,
            noise_level: params.noise_level,
            delivery: params.delivery,
            health_cost: params.health_cost,
            stamina_cost: params.stamina_cost,
            energy_cost: params.energy_cost,
            cooldown: params.cooldown,
            cooldown_timer: params.cooldown,
            damage: params.damage,
            range: params.range,
            effects: params.effects,
            color_override: params.color_override,
            size_override: params.size_override,
        }
    }

    pub fn activate(&mut self, actor: &mut Actor, origin: Vec2, target: Vec2) {
        if self.cooldown_timer >= self.cooldown
            && (self.health_cost == 0.0 || actor.stats.current_health >= self.health_cost)
            && (self.stamina_cost == 0.0 || actor.stats.current_stamina >= self.stamina_cost)
            && (self.energy_cost == 0.0 || actor.stats.current_energy >= self.energy_cost) {
            self.cooldown_timer = 0.0;
            actor.set_noise_level(self.noise_level);
            actor.stats.current_health -= self.health_cost;
            actor.stats.current_stamina -= self.stamina_cost;
            actor.stats.current_energy -= self.energy_cost;
            match self.delivery.clone() {
                AbilityDelivery::ContinuousBeam => {
                    let mut continuous_beams = scene::find_node_by_type::<ContinuousBeams>().unwrap();
                    let end = actor.body.position + target.sub(actor.body.position).normalize_or_zero() * self.range;
                    continuous_beams.spawn(
                        &actor.id,
                        &actor.factions,
                        self.damage,
                        self.color_override,
                        self.size_override,
                        actor.body.position,
                        end,
                    );
                },
                AbilityDelivery::Projectile {
                    projectile_kind,
                    spread,
                    speed,
                } => {
                    let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
                    let ttl = self.range / speed;
                    projectiles.spawn(
                        &actor.id,
                        &actor.factions,
                        projectile_kind,
                        self.damage,
                        self.color_override,
                        self.size_override,
                        origin,
                        target,
                        speed,
                        spread,
                        ttl,
                    );
                    if let Some(sound_effect) = self.sound_effect {
                        play_sound_once(sound_effect);
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.cooldown_timer += get_frame_time();
    }
}
