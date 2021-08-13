use serde::{
    Serialize,
    Deserialize,
};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 {
            x,
            y,
        }
    }

    pub fn from(other: macroquad::prelude::Vec2) -> Self {
        Vec2 {
            x: other.x,
            y: other.y,
        }
    }

    pub fn to_macroquad(&self) -> macroquad::prelude::Vec2 {
        macroquad::prelude::vec2(
            self.x,
            self.y,
        )
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

impl UVec2 {
    pub fn new(x: u32, y: u32) -> Self {
        UVec2 {
            x,
            y,
        }
    }

    pub fn from(other: macroquad::prelude::UVec2) -> Self {
        UVec2 {
            x: other.x,
            y: other.y,
        }
    }

    pub fn to_macroquad(&self) -> macroquad::prelude::UVec2 {
        macroquad::prelude::uvec2(
            self.x,
            self.y,
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Collider {
    kind: String,
    offset: Vec2,
    radius: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
}

impl Collider {
    pub const CIRCLE_KIND: &'static str = "circle";
    pub const RECTANGLE_KIND: &'static str = "rectangle";

    pub fn from(other: crate::Collider) -> Self {
        match other {
            crate::Collider::Circle(circle) => Collider {
                kind: Self::CIRCLE_KIND.to_string(),
                offset: Vec2::new(circle.x, circle.y),
                radius: Some(circle.r),
                width: None,
                height: None,
            },
            crate::Collider::Rectangle(rect) => Collider {
                kind: Self::RECTANGLE_KIND.to_string(),
                offset: Vec2::new(rect.x, rect.y),
                radius: None,
                width: Some(rect.w),
                height: Some(rect.h),
            }
        }
    }

    pub fn to_collider(&self) -> crate::Collider {
        if self.kind == Self::CIRCLE_KIND {
            crate::Collider::circle(self.offset.x, self.offset.y, self.radius.unwrap())
        } else if self.kind == Self::RECTANGLE_KIND {
            crate::Collider::rect(self.offset.x, self.offset.y, self.width.unwrap(), self.height.unwrap())
        } else {
            assert!(false, "Invalid collider kind '{}", self.kind);
            crate::Collider::circle(0.0,0.0,0.0)
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub row: u32,
    pub frames: u32,
    pub fps: u32,
}

impl Animation {
    pub fn to_macroquad(&self) -> macroquad::prelude::animation::Animation {
        macroquad::prelude::animation::Animation {
            name: self.name.clone(),
            row: self.row,
            frames: self.frames,
            fps: self.fps,
        }
    }

    pub fn from(other: macroquad::prelude::animation::Animation) -> Self {
        Animation {
            name: other.name.clone(),
            row: other.row,
            frames: other.frames,
            fps: other.fps,
        }
    }
}

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

impl ActorStats {
    pub fn new(
        strength: u32,
        dexterity: u32,
        constitution: u32,
        intelligence: u32,
        willpower: u32,
        perception: u32,
        charisma: u32,
    ) -> Self {
        ActorStats {
            strength,
            dexterity,
            constitution,
            intelligence,
            willpower,
            perception,
            charisma,
            is_static: Some(false),
            ..Default::default()
        }
    }

    pub fn new_static(
        current_health: f32,
        max_health: f32,
        current_energy: f32,
        max_stamina: f32,
        current_stamina: f32,
        max_energy: f32,
        carry_capacity: f32,
        move_speed: f32,
    ) -> Self {
        ActorStats {
            current_health: Some(current_health),
            max_health: Some(max_health),
            current_stamina: Some(current_stamina),
            max_stamina: Some(max_stamina),
            current_energy: Some(current_energy),
            max_energy: Some(max_energy),
            carry_capacity: Some(carry_capacity),
            move_speed: Some(move_speed),
            is_static: Some(true),
            ..Default::default()
        }
    }

    pub fn from(other: crate::ActorStats) -> Self {
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

    pub fn to_actor_stats(&self, max_vitals: bool) -> crate::ActorStats {
        let mut stats = crate::ActorStats {
            strength: self.strength,
            dexterity: self.dexterity,
            constitution: self.constitution,
            intelligence: self.intelligence,
            willpower: self.willpower,
            perception: self.perception,
            charisma: self.charisma,
            current_health: self.current_health.unwrap_or_default(),
            max_health: self.max_health.unwrap_or_default(),
            current_stamina: self.current_stamina.unwrap_or_default(),
            max_stamina: self.max_stamina.unwrap_or_default(),
            current_energy: self.current_energy.unwrap_or_default(),
            max_energy: self.max_energy.unwrap_or_default(),
            health_regen: self.health_regen.unwrap_or_default(),
            stamina_regen: self.stamina_regen.unwrap_or_default(),
            energy_regen: self.energy_regen.unwrap_or_default(),
            carry_capacity: self.carry_capacity.unwrap_or_default(),
            move_speed: self.move_speed.unwrap_or_default(),
            is_static: self.is_static.unwrap_or_default(),
        };
        stats.update_derived(max_vitals);
        stats
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
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn from(other: macroquad::color::Color) -> Self {
        Color {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }

    pub fn to_macroquad(&self) -> macroquad::color::Color {
        macroquad::color::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}
