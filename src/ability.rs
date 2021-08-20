use std::ops::Sub;

use macroquad::{
    color,
    prelude::*
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    Actor,
    generate_id,
    nodes::{
        Projectiles,
        ContinuousBeams,
        projectiles::ProjectileKind,
    },
    render::{
        SpriteAnimationParams,
        SpriteAnimationPlayer,
    },
    json,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct AbilityParams {
    pub effect_kind: String,
    pub action_kind: String,
    #[serde(default)]
    pub cooldown: f32,
    #[serde(default)]
    pub health_cost: f32,
    #[serde(default)]
    pub stamina_cost: f32,
    #[serde(default)]
    pub energy_cost: f32,
    pub speed: f32,
    pub spread: f32,
    pub range: f32,
    pub damage: f32,
    pub effect_size: f32,
    #[serde(with = "json::ColorDef")]
    pub effect_color: Color,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_sprite_animation: Option<SpriteAnimationParams>,
}

impl Default for AbilityParams {
    fn default() -> Self {
        AbilityParams {
            effect_kind: Ability::PROJECTILE_EFFECT.to_string(),
            action_kind: Ability::PRIMARY_ABILITY.to_string(),
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
            effect_sprite_animation: None,
        }
    }
}

#[derive(Clone)]
pub struct Ability {
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
    pub effect_sprite_animation_params: Option<SpriteAnimationParams>,
}

impl Ability {
    pub const PROJECTILE_EFFECT: &'static str = "projectile";
    pub const ENERGY_SPHERE_EFFECT: &'static str = "energy_sphere";
    pub const BEAM_EFFECT: &'static str = "beam";
    pub const CONTINUOUS_BEAM_EFFECT: &'static str = "continuous_beam";

    pub const PRIMARY_ABILITY: &'static str = "primary";
    pub const SECONDARY_ABILITY: &'static str = "secondary";

    pub fn new(params: AbilityParams) -> Self {
        Ability {
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
            effect_sprite_animation_params: params.effect_sprite_animation,
        }
    }

    pub fn activate(&mut self, actor: &mut Actor, origin: Vec2, target: Vec2) {
        if (self.health_cost == 0.0 || actor.stats.current_health >= self.health_cost)
            && (self.stamina_cost == 0.0 || actor.stats.current_stamina >= self.stamina_cost)
            && (self.energy_cost == 0.0 || actor.stats.current_energy >= self.energy_cost) {
            if self.effect_kind == Ability::CONTINUOUS_BEAM_EFFECT {
                actor.stats.current_health -= self.health_cost;
                actor.stats.current_stamina -= self.stamina_cost;
                actor.stats.current_energy -= self.energy_cost;
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
            } else if self.cooldown_timer >= self.cooldown {
                let kind = if self.effect_kind == Self::PROJECTILE_EFFECT {
                    ProjectileKind::Bullet
                } else if self.effect_kind == Self::BEAM_EFFECT {
                    ProjectileKind::Beam
                } else if self.effect_kind == Self::ENERGY_SPHERE_EFFECT {
                    ProjectileKind::EnergySphere
                } else {
                    assert!(false, "Invalid effect kind '{}'", self.effect_kind);
                    ProjectileKind::Bullet
                };
                actor.stats.current_health -= self.health_cost;
                actor.stats.current_stamina -= self.stamina_cost;
                actor.stats.current_energy -= self.energy_cost;
                self.cooldown_timer = 0.0;
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
                    self.effect_sprite_animation_params.clone(),
                );
            }
        }
    }

    pub fn update(&mut self) {
        self.cooldown_timer += get_frame_time();
    }
}