use macroquad::{
    experimental::{
        scene::{
            Node,
            RefMut,
        }
    },
    prelude::*,
};

pub struct GameState {
    pub local_player_id: u32,
    pub should_quit: bool,
}

impl GameState {
    pub fn new(local_player_id: u32) -> GameState {
        // for (_, data) in &map.actors {
        //     let actor = Actor::new(data);
        //     scene::add_node(actor);
        // }
        //
        // for (_, data) in &map.items {
        //     let item = Actor::new(data);
        //     scene::add_node(item);
        // }

        GameState {
            local_player_id,
            should_quit: false,
        }
    }
}

impl Node for GameState {
    fn ready(_node: RefMut<Self>) {}

    fn update(_node: RefMut<Self>) {}

    fn fixed_update(_node: RefMut<Self>) {}
}
