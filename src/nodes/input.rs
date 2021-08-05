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
        PlayerControlProvider,
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

        if let Some((_handle, mut controller_lens)) = scene::find_nodes_with::<PlayerControlProvider>().last() {
            if let Some(controller) = controller_lens.get() {
                if is_mouse_button_pressed(MouseButton::Left) {
                    controller.destination = Some(camera.to_world_space(get_mouse_position()));
                }

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

                controller.direction = vec2(x, y);
            }
        };

        {
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
        }

        {
            let (_, y) = mouse_wheel();
            if y > 0.0 {
                camera.zoom_out();
            } else if y < 0.0 {
                camera.zoom_in();
            }
        }

        {
            if is_key_down(KeyCode::R) {
                camera.rotate_ccw()
            } else if is_key_down(KeyCode::T) {
                camera.rotate_cw()
            }
        }
    }
}
