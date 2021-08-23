use macroquad::{
    experimental::{
        collections::storage,
        scene::RefMut,
    },
    prelude::*,
};

use crate::{
    render::Viewport,
    nodes::{
        GameState,
    },
};
use crate::nodes::Actor;

pub fn get_mouse_position() -> Vec2 {
    let (x, y) = mouse_position();
    vec2(x, y)
}

pub fn get_mouse_in_world_space() -> Vec2 {
    let viewport = storage::get::<Viewport>();
    viewport.to_world_space(get_mouse_position())
}

pub fn apply_local_player_input(node: &mut RefMut<Actor>) {
    let coords = get_mouse_in_world_space();
    node.controller.primary_target = if is_mouse_button_down(MouseButton::Left) {
        Some(coords)
    } else {
        None
    };
    node.controller.secondary_target = if is_mouse_button_down(MouseButton::Right) {
        Some(coords)
    } else {
        None
    };
    node.controller.direction = Vec2::ZERO;
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        node.controller.direction.y -= 1.0;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        node.controller.direction.y += 1.0;
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        node.controller.direction.x -= 1.0;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        node.controller.direction.x += 1.0;
    }
    node.controller.is_sprinting = is_key_down(KeyCode::LeftShift);

    node.controller.is_starting_interaction = is_key_released(KeyCode::F);

    node.controller.is_picking_up_items = is_key_down(KeyCode::R);

    let mut game_state = scene::find_node_by_type::<GameState>().unwrap();
    if is_key_released(KeyCode::C) {
        game_state.show_character_window = !game_state.show_character_window;
    }
    if is_key_released(KeyCode::I) {
        game_state.show_inventory_window = !game_state.show_inventory_window;
    }

    if is_key_released(KeyCode::P) {
        game_state.in_debug_mode = !game_state.in_debug_mode;
    }

    if is_key_released(KeyCode::Escape) {
        if node.current_interaction.is_some() || game_state.show_inventory_window || game_state.show_character_window {
            node.current_interaction = None;
            game_state.show_inventory_window = false;
            game_state.show_character_window = false;
        } else {
            game_state.should_quit = true;
        }
    }
}
