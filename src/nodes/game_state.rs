use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
            RefMut,
        }
    },
    color,
    prelude::*,
};

use crate::map::Map;
use crate::{MAP_LAYER_GROUND, MAP_LAYER_BARRIERS, MAP_LAYER_SOLIDS, get_global};
use crate::render::Viewport;

pub struct GameState {
    pub map: Map,
    pub show_character_window: bool,
    pub show_inventory_window: bool,
    pub should_quit: bool,
}

impl GameState {
    pub fn new(map: Map) -> GameState {
        GameState {
            map,
            show_character_window: false,
            show_inventory_window: false,
            should_quit: false,
        }
    }

    pub fn add_node(map: Map) -> Handle<Self> {
        scene::add_node(Self::new(map))
    }
}

impl Node for GameState {
    fn draw(node: RefMut<Self>) {
        let viewport = get_global::<Viewport>();
        let rect = node.map.to_grid_coords(viewport.get_frustum());
        node.map.draw(&[MAP_LAYER_GROUND, MAP_LAYER_BARRIERS, MAP_LAYER_SOLIDS], Some(rect));
    }
}
