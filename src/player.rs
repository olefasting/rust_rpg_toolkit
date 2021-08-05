use macroquad::experimental::scene::{HandleUntyped, Lens};

use crate::nodes::actor::ActorController;

pub type PlayerProvider = (HandleUntyped, Lens<ActorController>);
