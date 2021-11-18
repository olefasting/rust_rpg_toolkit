use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct LocalPlayer {
    pub id: String,
}

impl LocalPlayer {
    pub fn new(id: &str) -> Self {
        LocalPlayer { id: id.to_string() }
    }
}

pub fn get_player_actor() -> Option<RefMut<Actor>> {
    let player = storage::get::<LocalPlayer>();
    Actor::find_by_player_id(&player.id)
}
