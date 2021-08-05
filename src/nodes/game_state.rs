use macroquad::{
    experimental::{
        scene::{
            Node,
            RefMut,
        }
    },
    prelude::*,
};

use crate::Player;

pub struct GameState {
    pub players: Vec<Player>,
    pub should_quit: bool,
}

impl GameState {
    pub fn new(/* map: MapData */) -> GameState {
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
            players: Vec::new(),
            should_quit: false,
        }
    }
}

impl Node for GameState {
    fn ready(_node: RefMut<Self>) {}

    fn update(_node: RefMut<Self>) {}

    fn fixed_update(_node: RefMut<Self>) {}
}
