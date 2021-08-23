use std::ops::Sub;

use macroquad::{
    experimental::{
        collections::storage,
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
    },
    Resources,
    json,
};
use crate::nodes::actor::ActorNoiseLevel;
use macroquad::audio::{Sound, play_sound_once};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EffectKind {
    #[serde(rename = "projectile")]
    Projectile,
    #[serde(rename = "energy_sphere")]
    EnergySphere,
    #[serde(rename = "beam")]
    Beam,
    #[serde(rename = "continuous_beam")]
    ContinuousBeam,
    #[serde(rename = "heal_self")]
    HealSelf,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionKind {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AbilityParams {
    pub id: String,
    #[serde(default, rename = "sound_effect", skip_serializing_if = "Option::is_none")]
    pub sound_effect_id: Option<String>,
    #[serde(default)]
    pub noise_level: ActorNoiseLevel,
    pub effect_kind: EffectKind,
    pub action_kind: ActionKind,
    #[serde(default)]
    pub cooldown: f32,
    #[serde(default)]
    pub health_cost: f32,
    #[serde(default)]
    pub stamina_cost: f32,
    #[serde(default)]
    pub energy_cost: f32,
    #[serde(default)]
    pub speed: f32,
    #[serde(default)]
    pub spread: f32,
    pub range: f32,
    pub damage: f32,
    pub effect_size: f32,
    #[serde(with = "json::ColorDef")]
    pub effect_color: Color,
}

impl Default for AbilityParams {
    fn default() -> Self {
        AbilityParams {
            id: "".to_string(),
            sound_effect_id: None,
            noise_level: ActorNoiseLevel::Moderate,
            effect_kind: EffectKind::Projectile,
            action_kind: ActionKind::Primary,
            cooldown: 0.0,
            health_cost: 0.0,
            stamina_cost: 0.0,
            energy_cost: 0.0,
            speed: 75.0,
            spread: 0.0,
            range: 100.0,
            damage: 0.0,
            effect_size: 5.0,
            effect_color: color::WHITE,
        }
    }
}

#[derive(Clone)]
pub struct Ability {
    pub noise_level: ActorNoiseLevel,
    pub effect_kind: EffectKind,
    pub action_kind: ActionKind,
    pub sound_effect: Option<Sound>,
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
            effect_kind: params.effect_kind,
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
            effect_size: params.effect_size,
            effect_color: params.effect_color,
        }
    }

    pub fn activate(&mut self, actor: &mut Actor, origin: Vec2, target: Vec2) {
        let mut was_activated = false;
        if (self.health_cost == 0.0 || actor.stats.current_health >= self.health_cost)
            && (self.stamina_cost == 0.0 || actor.stats.current_stamina >= self.stamina_cost)
            && (self.energy_cost == 0.0 || actor.stats.current_energy >= self.energy_cost) {
            actor.set_noise_level(self.noise_level);
            actor.stats.current_health -= self.health_cost;
            actor.stats.current_stamina -= self.stamina_cost;
            actor.stats.current_energy -= self.energy_cost;
            if self.effect_kind == EffectKind::ContinuousBeam {
                self.cooldown_timer = 0.0;
                let mut beams = scene::find_node_by_type::<ContinuousBeams>().unwrap();
                let end = actor.body.position + target.sub(actor.body.position).normalize_or_zero() * self.range;
                beams.spawn(
                    &actor.id,
                    &actor.factions,
                    self.damage,
                    self.effect_color,
                    self.effect_size,
                    actor.body.position,
                    end,
                );
                was_activated = true;
            } else if self.cooldown_timer >= self.cooldown {
                self.cooldown_timer = 0.0;
                let kind = match self.effect_kind {
                    EffectKind::Beam => ProjectileKind::Beam,
                    EffectKind::EnergySphere => ProjectileKind::EnergySphere,
                    _ => ProjectileKind::Bullet,
                };
                let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
                let ttl = self.range / self.speed;
                projectiles.spawn(
                    &actor.id,
                    &actor.factions,
                    kind,
                    self.damage,
                    self.effect_color,
                    self.effect_size,
                    origin,
                    target,
                    self.speed,
                    self.spread,
                    ttl,
                );
                was_activated = true;
            }
        }
        if was_activated {
            if let Some(sound_effect) = self.sound_effect {
                play_sound_once(sound_effect);
            }
        }
    }

    pub fn update(&mut self) {
        self.cooldown_timer += get_frame_time();
    }
}
