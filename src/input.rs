use macroquad::{
    experimental::{
        scene::{
            RefMut,
        },
    },
    prelude::*,
};

use crate::{
    globals::LocalPlayer,
    get_global,
    render::Viewport,
    nodes::{
        Actor,
        GameState,
    },
};
use crate::nodes::ActorController;

pub fn get_mouse_position() -> Vec2 {
    let (x, y) = mouse_position();
    vec2(x, y)
}

pub fn apply_local_player_input(controller: &mut ActorController) {
    let viewport = get_global::<Viewport>();
    let coords = viewport.get_mouse_world_coords();
    if is_mouse_button_down(MouseButton::Left) {
        controller.primary_target = Some(coords);
    } else {
        controller.primary_target = None;
    }
    if is_mouse_button_down(MouseButton::Right) {
        controller.secondary_target = Some(coords);
    } else {
        controller.secondary_target = None;
    }
    let mut direction = Vec2::ZERO;
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        direction.y -= 1.0;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        direction.y += 1.0;
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        direction.x -= 1.0;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        direction.x += 1.0;
    }
    controller.direction = direction;

    controller.is_sprinting = is_key_down(KeyCode::LeftShift);

    controller.pick_up_items = is_key_down(KeyCode::E);

    let mut game_state = scene::find_node_by_type::<GameState>().unwrap();
    if is_key_released(KeyCode::C) {
        game_state.show_character_window = !game_state.show_character_window;
    }
    if is_key_released(KeyCode::I) {
        game_state.show_inventory_window = !game_state.show_inventory_window;
    }
    game_state.should_quit = is_key_released(KeyCode::Escape) || is_key_pressed(KeyCode::Q);
}
