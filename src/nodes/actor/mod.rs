pub use actor::{
    Actor,
    ActorParams,
};
pub use behavior::{
    ActorAggression,
    ActorBehavior,
    ActorBehaviorParams,
    apply_actor_behavior,
};
pub use controller::{
    ActorController,
    ActorControllerKind,
};

pub use stats::ActorStats;

mod controller;
mod stats;
mod actor;
mod behavior;
