use crate::{
    prelude::*,
};

#[derive(Debug, Clone)]
pub enum ActorControllerKind {
    LocalPlayer { player_id: String },
    RemotePlayer { player_id: String },
    Computer,
    None,
}

impl ActorControllerKind {
    pub fn local_player(player_id: &str) -> Self {
        let player_id = player_id.to_string();
        ActorControllerKind::LocalPlayer { player_id }
    }
}

#[derive(Debug, Clone)]
pub struct ActorController {
    pub kind: ActorControllerKind,
    pub should_use_primary_ability: bool,
    pub should_use_secondary_ability: bool,
    pub move_direction: Vec2,
    pub aim_direction: Vec2,
    pub should_start_interaction: bool,
    pub should_pick_up_items: bool,
    pub should_dash: bool,
    pub should_sprint: bool,
    pub is_sprint_locked: bool,
    pub equip_weapon: Option<String>,
}

impl ActorController {
    pub fn new(kind: ActorControllerKind) -> Self {
        ActorController {
            kind,
            should_use_primary_ability: false,
            should_use_secondary_ability: false,
            move_direction: Vec2::ZERO,
            aim_direction: Vec2::ZERO,
            should_start_interaction: false,
            should_pick_up_items: false,
            should_dash: false,
            should_sprint: false,
            is_sprint_locked: false,
            equip_weapon: None,
        }
    }

    pub fn is_attacking(&self) -> bool {
        self.should_use_primary_ability
            || self.should_use_secondary_ability
    }
}
