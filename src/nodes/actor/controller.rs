use macroquad::prelude::*;

#[derive(Clone)]
pub enum ActorControllerKind {
    LocalPlayer { player_id: String },
    RemotePlayer { player_id: String },
    Computer,
    None,
}

#[derive(Clone)]
pub struct ActorController {
    pub kind: ActorControllerKind,
    pub primary_target: Option<Vec2>,
    pub secondary_target: Option<Vec2>,
    pub direction: Vec2,
    pub is_interacting: bool,
    pub is_picking_up_items: bool,
    pub is_sprinting: bool,
}

impl ActorController {
    pub fn new(kind: ActorControllerKind) -> Self {
        ActorController {
            kind,
            primary_target: None,
            secondary_target: None,
            direction: Vec2::ZERO,
            is_interacting: false,
            is_picking_up_items: false,
            is_sprinting: false,
        }
    }
}
