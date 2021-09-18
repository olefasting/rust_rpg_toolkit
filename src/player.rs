use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct LocalPlayer {
    pub id: String,
    pub gamepad_id: Option<GamepadId>,
}

impl LocalPlayer {
    pub fn new(id: &str, gamepad_id: Option<GamepadId>) -> Self {
        LocalPlayer {
            id: id.to_string(),
            gamepad_id,
        }
    }
}

pub fn get_player_actor() -> Option<RefMut<Actor>> {
    let player = storage::get::<LocalPlayer>();
    Actor::find_by_player_id(&player.id)
}