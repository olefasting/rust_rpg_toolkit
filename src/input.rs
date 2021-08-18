use macroquad::{
    prelude::*,
};

use crate::{
    get_global,
    render::Viewport,
    nodes::{
        GameState,
    },
};
use crate::nodes::{
    ActorController,
    Camera,
};

pub fn get_mouse_position() -> Vec2 {
    let (x, y) = mouse_position();
    vec2(x, y)
}

pub fn get_mouse_in_world_space() -> Vec2 {
    let viewport = get_global::<Viewport>();
    viewport.to_world_space(get_mouse_position())
}

pub fn apply_local_input(controller: &mut ActorController) {
    let coords = get_mouse_in_world_space();
    controller.primary_target = if is_mouse_button_down(MouseButton::Left) {
        Some(coords)
    } else {
        None
    };
    controller.secondary_target = if is_mouse_button_down(MouseButton::Right) {
        Some(coords)
    } else {
        None
    };
    controller.direction = Vec2::ZERO;
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        controller.direction.y -= 1.0;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        controller.direction.y += 1.0;
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        controller.direction.x -= 1.0;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        controller.direction.x += 1.0;
    }
    controller.is_sprinting = is_key_down(KeyCode::LeftShift);

    controller.is_interacting = is_key_released(KeyCode::E);

    controller.is_picking_up_items = is_key_down(KeyCode::R);

    let mut game_state = scene::find_node_by_type::<GameState>().unwrap();
    if is_key_released(KeyCode::C) {
        game_state.show_character_window = !game_state.show_character_window;
    }
    if is_key_released(KeyCode::I) {
        game_state.show_inventory_window = !game_state.show_inventory_window;
    }
    game_state.should_quit = is_key_released(KeyCode::Escape) || is_key_pressed(KeyCode::Q);
}
