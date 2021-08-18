use serde::{
    Serialize,
    Deserialize,
};

use crate::json::{
    Vec2,
    SpriteAnimationParams,
    Collider,
    ItemParams,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorStats {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub willpower: u32,
    pub perception: u32,
    pub charisma: u32,
    pub current_health: Option<f32>,
    pub max_health: Option<f32>,
    pub current_stamina: Option<f32>,
    pub max_stamina: Option<f32>,
    pub current_energy: Option<f32>,
    pub max_energy: Option<f32>,
    pub health_regen: Option<f32>,
    pub stamina_regen: Option<f32>,
    pub energy_regen: Option<f32>,
    pub carry_capacity: Option<f32>,
    pub move_speed: Option<f32>,
    pub is_static: Option<bool>,
}

impl From<crate::ActorStats> for ActorStats {
    fn from(other: crate::ActorStats) -> Self {
        ActorStats {
            strength: other.strength,
            dexterity: other.dexterity,
            constitution: other.constitution,
            intelligence: other.intelligence,
            willpower: other.willpower,
            perception: other.perception,
            charisma: other.charisma,
            current_health: Some(other.current_health),
            max_health: Some(other.max_health),
            current_stamina: Some(other.current_stamina),
            max_stamina: Some(other.max_stamina),
            current_energy: Some(other.current_energy),
            max_energy: Some(other.max_energy),
            health_regen: Some(other.health_regen),
            stamina_regen: Some(other.stamina_regen),
            energy_regen: Some(other.energy_regen),
            carry_capacity: Some(other.carry_capacity),
            move_speed: Some(other.move_speed),
            is_static: Some(other.is_static),
        }
    }
}

impl From<ActorStats> for crate::ActorStats {
    fn from(other: ActorStats) -> Self {
        crate::ActorStats {
            strength: other.strength,
            dexterity: other.dexterity,
            constitution: other.constitution,
            intelligence: other.intelligence,
            willpower: other.willpower,
            perception: other.perception,
            charisma: other.charisma,
            current_health: other.current_health.unwrap_or_default(),
            max_health: other.max_health.unwrap_or_default(),
            current_stamina: other.current_stamina.unwrap_or_default(),
            max_stamina: other.max_stamina.unwrap_or_default(),
            current_energy: other.current_energy.unwrap_or_default(),
            max_energy: other.max_energy.unwrap_or_default(),
            health_regen: other.health_regen.unwrap_or_default(),
            stamina_regen: other.stamina_regen.unwrap_or_default(),
            energy_regen: other.energy_regen.unwrap_or_default(),
            carry_capacity: other.carry_capacity.unwrap_or_default(),
            move_speed: other.move_speed.unwrap_or_default(),
            is_static: other.is_static.unwrap_or_default(),
        }
    }
}

impl Default for ActorStats {
    fn default() -> Self {
        ActorStats {
            strength: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            willpower: 0,
            perception: 0,
            charisma: 0,
            current_health: Some(1.0),
            max_health: Some(1.0),
            current_stamina: Some(0.0),
            max_stamina: Some(0.0),
            current_energy: Some(0.0),
            max_energy: Some(0.0),
            health_regen: Some(0.0),
            stamina_regen: Some(0.0),
            energy_regen: Some(0.0),
            carry_capacity: Some(0.0),
            move_speed: Some(0.0),
            is_static: Some(true),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorPrototype {
    pub id: String,
    pub name: String,
    pub stats: ActorStats,
    pub factions: Vec<String>,
    pub collider: Option<Collider>,
    pub inventory: Vec<String>,
    pub sprite_animation: SpriteAnimationParams,
}

impl From<crate::nodes::actor::ActorPrototype> for ActorPrototype {
    fn from(other: crate::nodes::actor::ActorPrototype) -> Self {
        let collider = if let Some(collider) = other.collider {
            Some(Collider::from(collider))
        } else {
            None
        };
        ActorPrototype {
            id: other.id,
            name: other.name,
            stats: ActorStats::from(other.stats),
            factions: other.factions,
            collider,
            inventory: other.inventory,
            sprite_animation: SpriteAnimationParams::from(other.sprite_animation),
        }
    }
}

impl From<ActorPrototype> for crate::nodes::actor::ActorPrototype {
    fn from(other: ActorPrototype) -> Self {
        let collider = if let Some(collider) = other.collider {
            Some(crate::Collider::from(collider))
        } else {
            None
        };
        crate::nodes::actor::ActorPrototype {
            id: other.id,
            name: other.name,
            stats: crate::nodes::actor::ActorStats::from(other.stats),
            factions: other.factions,
            collider,
            inventory: other.inventory,
            sprite_animation: crate::render::SpriteAnimationParams::from(other.sprite_animation),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorParams {
    pub position: Vec2,
    pub name: String,
    pub stats: ActorStats,
    pub factions: Vec<String>,
    pub collider: Option<Collider>,
    pub inventory: Vec<ItemParams>,
    pub sprite_animation: SpriteAnimationParams,
}

impl From<crate::ActorParams> for ActorParams {
    fn from(other: crate::ActorParams) -> Self {
        let collider = if let Some(collider) = other.collider {
            Some(Collider::from(collider))
        } else {
            None
        };
        ActorParams {
            position: Vec2::from(other.position),
            name: other.name,
            stats: ActorStats::from(other.stats),
            factions: other.factions,
            collider,
            inventory: other.inventory.into_iter().map(|params| ItemParams::from(params)).collect(),
            sprite_animation: SpriteAnimationParams::from(other.sprite_animation),
        }
    }
}
