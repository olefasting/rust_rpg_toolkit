use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
            RefMut,
        },
        collections::storage,
    },
    color,
    prelude::*,
};

use crate::{
    map::Map,
    render::Viewport,
};
use crate::render::{draw_aligned_text, HorizontalAlignment, VerticalAlignment};

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
    fn draw(node: RefMut<Self>) {
        let viewport = storage::get::<Viewport>();
        let rect = node.map.to_grid(viewport.get_frustum());
        node.map.draw( Some(rect));
        if node.in_debug_mode {
            push_camera_state();
            set_default_camera();
            draw_aligned_text(
                "DEBUG MODE",
                screen_width() / 2.0,
                50.0,
                HorizontalAlignment::Center,
                VerticalAlignment::Top,
                TextParams {
                    color: color::RED,
                    font_size: 24,
                    ..Default::default()
                },
            );
            draw_aligned_text(
                &format!("fps: {}", get_fps()),
                screen_width() - 50.0,
                50.0,
                HorizontalAlignment::Right,
                VerticalAlignment::Top,
                Default::default(),
            );
            pop_camera_state();
        }
    }
}
