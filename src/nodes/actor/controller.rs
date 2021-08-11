use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub enum ActorControllerKind {
    Player { id: u32 },
    Computer,
    None,
}

#[derive(Clone)]
pub struct ActorController {
    pub kind: ActorControllerKind,
    pub primary_target: Option<Vec2>,
    pub secondary_target: Option<Vec2>,
    pub direction: Vec2,
}

impl ActorController {
    pub fn new(kind: ActorControllerKind) -> Self {
        ActorController {
            kind,
            primary_target: None,
            secondary_target: None,
            direction: Vec2::ZERO,
        }
    }
}
