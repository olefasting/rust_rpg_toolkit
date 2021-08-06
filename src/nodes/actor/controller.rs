use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub enum ActorControllerKind {
    Player { player_id: u32 },
    Computer,
    None,
}

#[derive(Clone)]
pub struct ActorController {
    pub kind: ActorControllerKind,
    pub destination: Option<Vec2>,
    pub direction: Vec2,
}

impl ActorController {
    pub fn new(kind: ActorControllerKind) -> Self {
        ActorController {
            kind,
            destination: None,
            direction: Vec2::ZERO,
        }
    }
}
