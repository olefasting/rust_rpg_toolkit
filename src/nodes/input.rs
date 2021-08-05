use macroquad::{
    experimental::scene::{
        Node,
        RefMut,
        HandleUntyped,
        Lens,
    },
    prelude::*,
};

use crate::{
    get_mouse_position,
    nodes::{
        CameraControl,
        MapObjectProvider,
    },
    PlayerProvider,
};
use crate::nodes::actor::ActorController;

pub struct Input {}

impl Input {
    pub fn new() -> Self {
        Input {}
    }
}

impl Node for Input {
    fn fixed_update(_: RefMut<Self>) {
        for (_handle, mut controller_lens) in scene::find_nodes_with::<PlayerProvider>() {
            if let controller = controller_lens.get().unwrap() {
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

                controller.set_direction(vec2(x, y));
                break;
            }
        };

        let mut camera = scene::find_node_by_type::<CameraControl>().unwrap();

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

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_coords = camera.to_world_space(get_mouse_position());
            for (handle, capabilities) in scene::find_nodes_with::<MapObjectProvider>() {
                let position = (capabilities.get_position)(handle);
                if Rect::new(position.x, position.y, 24.0, 24.0).contains(mouse_coords) {
                    println!("clicked a map object");
                }
            }
        }
    }
}
