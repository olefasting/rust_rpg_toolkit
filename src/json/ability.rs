use serde::{
    Serialize,
    Deserialize,
};

use crate::json::{
    Color,
    SpriteAnimationParams,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct AbilityParams {
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
    pub effect_color: Color,
    pub effect_sprite_animation: Option<SpriteAnimationParams>,
}

impl From<crate::ability::AbilityParams> for AbilityParams {
    fn from(other: crate::ability::AbilityParams) -> Self {
        let effect_sprite_animation =
            if let Some(params) = other.effect_sprite_animation.clone() {
                Some(SpriteAnimationParams::from(params))
            } else {
                None
            };
        AbilityParams {
            effect_kind: other.effect_kind.to_string(),
            action_kind: other.action_kind.to_string(),
            cooldown: if other.cooldown == 0.0 { None } else { Some(other.cooldown) },
            health_cost: other.health_cost,
            stamina_cost: other.stamina_cost,
            energy_cost: other.energy_cost,
            speed: other.speed,
            spread: other.spread,
            range: other.range,
            damage: other.damage,
            effect_size: other.effect_size,
            effect_color: Color::from(other.effect_color),
            effect_sprite_animation,
        }
    }
}

impl From<AbilityParams> for crate::ability::AbilityParams {
    fn from(other: AbilityParams) -> Self {
        let effect_sprite_animation =
            if let Some(params) = other.effect_sprite_animation {
                Some(crate::render::SpriteAnimationParams::from(params))
            } else {
                None
            };
        crate::ability::AbilityParams {
            effect_kind: other.effect_kind.to_string(),
            action_kind: other.action_kind.to_string(),
            cooldown: other.cooldown.unwrap_or_default(),
            health_cost: other.health_cost,
            stamina_cost: other.stamina_cost,
            energy_cost: other.energy_cost,
            speed: other.speed,
            spread: other.spread,
            range: other.range,
            damage: other.damage,
            effect_size: other.effect_size,
            effect_color: macroquad::prelude::Color::from(other.effect_color),
            effect_sprite_animation,
        }
    }
}
