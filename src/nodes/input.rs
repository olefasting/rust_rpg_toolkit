use macroquad::{
    experimental::scene::{
        Node,
        RefMut,
    },
    prelude::*,
};

use crate::{
    nodes::{
        CameraControl,
        GameState,
        Actor,
    },
    get_mouse_position,
};

pub struct Input {}

impl Input {
    pub fn new() -> Self {
        Input {}
    }
}

impl Node for Input {
    fn fixed_update(_: RefMut<Self>) {
        let mut camera = scene::find_node_by_type::<CameraControl>().unwrap();
        let mut game_state = scene::find_node_by_type::<GameState>().unwrap();

        for mut actor in scene::find_nodes_by_type::<Actor>() {
            if let Some(player_id) = actor.controller.player_id {
                if player_id == game_state.local_player_id {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        actor.controller.destination = Some(camera.to_world_space(get_mouse_position()));
                    }

                    {
                        let x = if is_key_down(KeyCode::A) {
                            -1.0
                        } else if is_key_down(KeyCode::D) {
                            1.0
                        } else {
                            0.0
                        };

                        let y = if is_key_down(KeyCode::W) {
                            -1.0
                        } else if is_key_down(KeyCode::S) {
                            1.0
                        } else {
                            0.0
                        };

                        actor.controller.direction = vec2(x, y);
                    }
                } else {
                    // TODO: Network input
                }
            }
        }

        if is_key_down(KeyCode::Q) || is_key_down(KeyCode::Escape) {
            game_state.should_quit = true;
        }

        let x = if is_key_down(KeyCode::Left) {
            -1.0
        } else if is_key_down(KeyCode::Right) {
            1.0
        } else {
            0.0
        };

        let y = if is_key_down(KeyCode::Up) {
            -1.0
        } else if is_key_down(KeyCode::Down) {
            1.0
        } else {
            0.0
        };

        camera.pan(vec2(x, y));

        let (_, y) = mouse_wheel();
        if y > 0.0 {
            camera.zoom_out();
        } else if y < 0.0 {
            camera.zoom_in();
        }

        if is_key_down(KeyCode::R) {
            camera.rotate_ccw()
        } else if is_key_down(KeyCode::T) {
            camera.rotate_cw()
        }
    }
}
