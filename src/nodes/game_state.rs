use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
            RefMut,
        },
        collections::storage,
    },
    prelude::*,
};

use crate::{
    map::Map,
    render::Viewport,
};
use crate::input::apply_non_player_input;

#[derive(Debug, Clone)]
pub struct GameParams {
}

pub struct GameState {
    pub map: Map,
    pub local_player_id: String,
    pub show_character_window: bool,
    pub show_inventory_window: bool,
    pub in_debug_mode: bool,
    pub should_quit: bool,
}

impl GameState {
    pub fn new(map: Map, local_player_id: &str) -> GameState {
        GameState {
            map,
            local_player_id: local_player_id.to_string(),
            show_character_window: false,
            show_inventory_window: false,
            in_debug_mode: false,
            should_quit: false,
        }
    }

    pub fn add_node(map: Map, local_player_id: &str) -> Handle<Self> {
        scene::add_node(Self::new(map, local_player_id))
    }
}

impl Node for GameState {
    fn update(node: RefMut<Self>) {
        apply_non_player_input(node);
    }

    fn draw(node: RefMut<Self>) {
        let viewport = storage::get::<Viewport>();
        let rect = node.map.to_grid(viewport.get_frustum());
        node.map.draw( Some(rect));
    }
}
