use crate::nodes::projectiles::ProjectileParams;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DamageType {
    Piercing,
    Slashing,
    Blunt,
    Energy,
    Heat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Effect {
    Damage {
        damage_type: DamageType,
        amount: f32,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AbilityDelivery {
    Projectile {
        projectile_kind: ProjectileKind,
        spread: f32,
        speed: f32,
    },
    Melee,
    ContinuousBeam,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AbilityParams {
    pub id: String,
    #[serde(
        default,
        rename = "sound_effect",
        skip_serializing_if = "Option::is_none"
    )]
    pub sound_effect_id: Option<String>,
    #[serde(
        default,
        rename = "on_hit_sound_effect",
        skip_serializing_if = "Option::is_none"
    )]
    pub on_hit_sound_effect_id: Option<String>,
    #[serde(default)]
    pub noise_level: NoiseLevel,
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
    pub effects: Vec<Effect>,
    #[serde(
        default,
        with = "json::opt_color",
        skip_serializing_if = "Option::is_none"
    )]
    pub color_override: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_override: Option<f32>,
}

impl Default for AbilityParams {
    fn default() -> Self {
        AbilityParams {
            id: "".to_string(),
            sound_effect_id: None,
            on_hit_sound_effect_id: None,
            noise_level: NoiseLevel::Moderate,
            delivery: AbilityDelivery::Projectile {
                projectile_kind: ProjectileKind::Bullet,
                speed: 8.0,
                spread: 5.0,
            },
            cooldown: 0.0,
            health_cost: 0.0,
            stamina_cost: 0.0,
            energy_cost: 0.0,
            range: 5.0,
            effects: Vec::new(),
            color_override: None,
            size_override: None,
        }
    }
}

#[derive(Clone)]
pub struct Ability {
    pub noise_level: NoiseLevel,
    pub delivery: AbilityDelivery,
    pub sound_effect: Option<Sound>,
    pub on_hit_sound_effect: Option<Sound>,
    pub cooldown: f32,
    pub cooldown_timer: f32,
    pub health_cost: f32,
    pub stamina_cost: f32,
    pub energy_cost: f32,
    pub range: f32,
    pub effects: Vec<Effect>,
    pub color_override: Option<Color>,
    pub size_override: Option<f32>,
}

impl Ability {
    pub fn new(params: AbilityParams) -> Self {
        let resources = storage::get::<Resources>();

        let mut sound_effect = None;
        if let Some(sound_effect_id) = params.sound_effect_id {
            let res = resources
                .sound_effects
                .get(&sound_effect_id)
                .cloned()
                .unwrap_or_else(|| {
                    panic!("Unable to find sound effect with id '{}'", &sound_effect_id)
                });
            sound_effect = Some(res);
        }

        let mut on_hit_sound_effect = None;
        if let Some(sound_effect_id) = params.on_hit_sound_effect_id {
            let res = resources
                .sound_effects
                .get(&sound_effect_id)
                .cloned()
                .unwrap_or_else(|| {
                    panic!("Unable to find sound effect with id '{}'", &sound_effect_id)
                });
            on_hit_sound_effect = Some(res);
        }

        Ability {
            sound_effect,
            on_hit_sound_effect,
            noise_level: params.noise_level,
            delivery: params.delivery,
            health_cost: params.health_cost,
            stamina_cost: params.stamina_cost,
            energy_cost: params.energy_cost,
            cooldown: params.cooldown,
            cooldown_timer: params.cooldown,
            range: params.range,
            effects: params.effects,
            color_override: params.color_override,
            size_override: params.size_override,
        }
    }

    pub fn activate(&mut self, node: &mut RefMut<Actor>, origin: Vec2, direction: Vec2) {
        if self.cooldown_timer >= self.cooldown
            && (self.health_cost == 0.0 || node.stats.current_health >= self.health_cost)
            && (self.stamina_cost == 0.0 || node.stats.current_stamina >= self.stamina_cost)
            && (self.energy_cost == 0.0 || node.stats.current_energy >= self.energy_cost)
        {
            self.cooldown_timer = 0.0;

            node.set_noise_level(self.noise_level);
            node.stats.current_health -= self.health_cost;
            node.stats.current_stamina -= self.stamina_cost;
            node.stats.current_energy -= self.energy_cost;

            match self.delivery.clone() {
                AbilityDelivery::ContinuousBeam => {
                    let end = node.body.position + direction * self.range;

                    let mut continuous_beams =
                        scene::find_node_by_type::<ContinuousBeams>().unwrap();
                    continuous_beams.spawn(
                        &node.id,
                        node.handle(),
                        &node.factions,
                        &self.effects,
                        self.color_override,
                        self.size_override,
                        origin,
                        end,
                    );
                }
                AbilityDelivery::Projectile {
                    projectile_kind,
                    spread,
                    speed,
                } => {
                    let params = ProjectileParams {
                        kind: projectile_kind,
                        effects: self.effects.clone(),
                        color: self
                            .color_override
                            .unwrap_or(Projectiles::DEFAULT_PROJECTILE_COLOR),
                        size: self
                            .size_override
                            .unwrap_or(Projectiles::DEFAULT_PROJECTILE_SIZE),
                        origin,
                        direction,
                        speed,
                        range: self.range,
                        on_hit_sound_effect: self.on_hit_sound_effect,
                    };

                    let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
                    projectiles.spawn(&node.id, node.handle(), &node.factions, spread, params);

                    if let Some(sound_effect) = self.sound_effect {
                        play_sound(sound_effect, false);
                    }
                }
                AbilityDelivery::Melee => {
                    let collider = Collider::circle(origin.x, origin.y, self.range);

                    let mut hit_success = false;

                    for mut other_actor in scene::find_nodes_by_type::<Actor>() {
                        hit_success =
                            if let Some(other_collider) = other_actor.body.get_offset_collider() {
                                collider.overlaps(other_collider)
                            } else {
                                collider.contains(other_actor.body.position)
                            };

                        if hit_success {
                            for effect in self.effects.clone() {
                                other_actor.apply_effect(
                                    &node.id,
                                    node.handle(),
                                    &node.factions,
                                    effect,
                                );
                            }
                        }
                    }

                    if hit_success {
                        if let Some(sound_effect) = self.on_hit_sound_effect {
                            play_sound(sound_effect, false);
                        } else if let Some(sound_effect) = self.sound_effect {
                            play_sound(sound_effect, false);
                        }
                    } else if let Some(sound_effect) = self.sound_effect {
                        play_sound(sound_effect, false);
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.cooldown_timer += get_frame_time();
    }
}
