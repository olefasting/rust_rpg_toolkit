use std::ops::Sub;

use macroquad::{experimental::collections::storage, prelude::*};

/*
pub use fishsticks::{
    GamepadContext,
    Axis,
    Button,
    GamepadId,
    Gamepad,
};
*/

use crate::prelude::*;

pub fn apply_input(_player_id: &str, node: &mut RefMut<Actor>) {
    let mut game_state = scene::find_node_by_type::<GameState>().unwrap();

    let mouse_position = get_mouse_in_world_space();

    node.controller.should_use_weapon = is_mouse_button_down(MouseButton::Left);
    node.controller.should_use_selected_ability = is_mouse_button_down(MouseButton::Right);

    node.controller.move_direction = Vec2::ZERO;
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        node.controller.move_direction.y -= 1.0;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        node.controller.move_direction.y += 1.0;
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        node.controller.move_direction.x -= 1.0;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        node.controller.move_direction.x += 1.0;
    }

    if is_key_pressed(KeyCode::CapsLock) {
        node.controller.is_sprint_locked = !node.controller.is_sprint_locked;
    }

    if node.controller.should_use_weapon || node.controller.should_use_selected_ability {
        node.controller.aim_direction = mouse_position.sub(node.body.position).normalize_or_zero();
    } else {
        node.controller.aim_direction = node.controller.move_direction;
    }

    if node.controller.is_sprint_locked {
        node.controller.should_sprint = true;
    } else {
        node.controller.should_sprint = is_key_down(KeyCode::LeftShift);
    }

    node.controller.should_start_interaction = is_key_released(KeyCode::F);

    node.controller.should_pick_up_items = is_key_down(KeyCode::R);

    node.controller.should_respawn = is_key_down(KeyCode::B);

    if is_key_released(KeyCode::C) {
        game_state.gui_state.should_draw_character_window =
            !game_state.gui_state.should_draw_character_window;
    }
    if is_key_released(KeyCode::I) {
        game_state.gui_state.should_draw_inventory_window =
            !game_state.gui_state.should_draw_inventory_window;
    }

    if is_key_released(KeyCode::P) {
        game_state.in_debug_mode = !game_state.in_debug_mode;
    }

    if is_key_released(KeyCode::Escape) {
        if node.current_dialogue.is_some()
            || game_state.gui_state.should_draw_inventory_window
            || game_state.gui_state.should_draw_character_window
            || game_state.gui_state.should_draw_game_menu
        {
            node.current_dialogue = None;
            game_state.gui_state.should_draw_inventory_window = false;
            game_state.gui_state.should_draw_character_window = false;
            game_state.gui_state.should_draw_game_menu = false;
        } else {
            game_state.gui_state.should_draw_game_menu = true;
        }
    }
}

pub fn get_mouse_position() -> Vec2 {
    let (x, y) = mouse_position();
    vec2(x, y)
}

pub fn get_mouse_in_world_space() -> Vec2 {
    let viewport = storage::get::<Viewport>();
    viewport.to_world_space(get_mouse_position())
}
