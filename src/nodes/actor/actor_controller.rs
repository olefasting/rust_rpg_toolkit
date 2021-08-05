use macroquad::math::Vec2;

use macroquad::experimental::scene::{HandleUntyped, Lens};

pub type PlayerControlProvider = (HandleUntyped, Lens<ActorController>);

pub type ComputerControlProvider = (HandleUntyped, Lens<ActorController>);


#[derive(Clone)]
pub struct ActorController {
    pub destination: Option<Vec2>,
    pub direction: Vec2,
    pub is_player_controlled: bool,
}

impl ActorController {
    pub fn new(is_player_controlled: bool) -> Self {
        ActorController {
            destination: None,
            direction: Vec2::ZERO,
            is_player_controlled,
        }
    }

    #[allow(dead_code)]
    pub fn clear_all(&mut self) {
        self.direction = Vec2::ZERO;
        self.destination = None;
    }
}
