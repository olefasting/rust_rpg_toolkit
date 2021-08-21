mod physics_body;
mod collider;
mod beam;
mod raycast;

pub use physics_body::{
    PhysicsBody,
    PhysicsObject,
};

pub use collider::Collider;

pub use beam::{
    beam_collision_check,
    get_beam_end,
};

pub use raycast::raycast;

pub const ACTOR_TO_ACTOR_COLLISIONS: bool = false;
