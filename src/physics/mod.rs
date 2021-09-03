use crate::prelude::*;

mod physics_body;
mod collider;
mod beam;
mod raycast;

pub use physics_body::{
    PhysicsBody,
};

pub use collider::Collider;

pub use beam::{
    beam_collision_check,
    get_beam_end,
};

pub use raycast::raycast;

pub const COLLISION_RESOLUTION: f32 = 0.25;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollisionKind {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "barrier")]
    Barrier,
    #[serde(rename = "solid")]
    Solid,
    #[serde(rename = "actor")]
    Actor,
}

impl CollisionKind {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
}

impl Default for CollisionKind {
    fn default() -> Self {
        CollisionKind::None
    }
}
