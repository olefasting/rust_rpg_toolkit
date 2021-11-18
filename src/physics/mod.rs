use crate::prelude::*;

mod beam;
mod collider;
mod physics_body;
mod raycast;

pub use physics_body::PhysicsBody;

pub use collider::Collider;

pub use beam::{beam_collision_check, get_beam_end};

pub use raycast::raycast;

pub const COLLISION_RESOLUTION: f32 = 0.25;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
        matches!(self, Self::None)
    }
}

impl Default for CollisionKind {
    fn default() -> Self {
        CollisionKind::None
    }
}
