use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
            RefMut,
        }
    },
    prelude::*,
};

use crate::Map;

pub struct GameState {
    pub map: Map,
    pub should_quit: bool,
}

impl GameState {
    pub fn new(map: Map) -> GameState {
        GameState {
            map,
            should_quit: false,
        }
    }

    pub fn add_node(map: Map) -> Handle<Self> {
        scene::add_node(Self::new(map))
    }
}

impl Node for GameState {
    fn draw(node: RefMut<Self>) {
        node.map.draw();
    }
}
