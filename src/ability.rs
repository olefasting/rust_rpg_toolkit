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
#[serde(tag = "type")]
pub enum Effect {
    #[serde(rename = "damage")]
    Damage {
        damage_type: DamageType,
        amount: f32,
    },
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
    #[serde(rename = "melee")]
    Melee,
    #[serde(rename = "continuous_beam")]
    ContinuousBeam,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AbilityParams {
    pub id: String,
    #[serde(default, rename = "sound_effect", skip_serializing_if = "Option::is_none")]
    pub sound_effect_id: Option<String>,
    #[serde(default, rename = "on_hit_sound_effect", skip_serializing_if = "Option::is_none")]
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
    pub on_hit_sound_effect_id: Option<String>,
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
        let sound_effect = if let Some(sound_id) = params.sound_effect_id {
            let resources = storage::get::<Resources>();
            resources.sound_effects.get(&sound_id).cloned()
        } else {
            None
        };

        Ability {
            sound_effect,
            on_hit_sound_effect_id: params.on_hit_sound_effect_id,
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
            && (self.energy_cost == 0.0 || node.stats.current_energy >= self.energy_cost) {
            
            self.cooldown_timer = 0.0;

            node.set_noise_level(self.noise_level);
            node.stats.current_health -= self.health_cost;
            node.stats.current_stamina -= self.stamina_cost;
            node.stats.current_energy -= self.energy_cost;

            match self.delivery.clone() {
                AbilityDelivery::ContinuousBeam => {
                    let mut continuous_beams = scene::find_node_by_type::<ContinuousBeams>().unwrap();
                    let end = node.body.position + direction * self.range;
                    continuous_beams.spawn(
                        &node.id,
                        node.handle(),
                        &node.factions,
                        &self.effects,
                        self.color_override,
                        self.size_override,
                        node.body.position,
                        end,
                    );
                },
                AbilityDelivery::Projectile {
                    projectile_kind,
                    spread,
                    speed,
                } => {
                    let mut projectiles = scene::find_node_by_type::<Projectiles>().unwrap();
                    projectiles.spawn(
                        &node.id,
                        node.handle(),
                        &node.factions,
                        projectile_kind,
                        &self.effects,
                        self.color_override,
                        self.size_override,
                        origin,
                        direction,
                        speed,
                        spread,
                        self.range,
                        self.on_hit_sound_effect_id.clone(),
                    );
                    if let Some(sound_effect) = self.sound_effect {
                        play_sound_once(sound_effect);
                    }
                },
                AbilityDelivery::Melee => {
                    let collider = Collider::circle(
                        node.body.position.x,
                        node.body.position.y,
                        self.range,
                    );
                    let mut hit_success = false;
                    for mut other_actor in scene::find_nodes_by_type::<Actor>() {
                        hit_success = if let Some(other_collider) = other_actor.body.get_offset_collider() {
                            if collider.overlaps(other_collider) {
                                true
                            } else {
                                false
                            }
                        } else if collider.contains(other_actor.body.position) {
                            true
                        } else {
                            false
                        };
                        if hit_success {
                            for effect in self.effects.clone() {
                                other_actor.apply_effect(&node.id, node.handle(), &node.factions, effect);
                            }
                        }
                    }
                    if hit_success {
                        if let Some(sound_effect_id) = &self.on_hit_sound_effect_id {
                            let resources = storage::get::<Resources>();
                            let sound_effect = resources.sound_effects.get(sound_effect_id).cloned().unwrap();
                            play_sound_once(sound_effect);
                        } else if let Some(sound_effect) = self.sound_effect {
                            play_sound_once(sound_effect);
                        }
                    } else if let Some(sound_effect) = self.sound_effect {
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
