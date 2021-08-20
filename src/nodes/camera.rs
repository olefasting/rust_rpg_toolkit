use std::ops::Sub;

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
    GameState,
    Viewport,
    Actor,
    get_mouse_position,
    draw_aligned_text,
};

use crate::render::HorizontalAlignment;

pub struct Camera {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: f32,
    pub is_following: bool,
}

impl Camera {
    const FOLLOW_THRESHOLD_FRACTION: f32 = 0.4;
    const FOLLOW_END_AT_DISTANCE: f32 = 20.0;
    const FOLLOW_LERP_FRACTION: f32 = 0.015;

    const DEFAULT_SCALE: f32 = 3.0;

    pub fn new(position: Vec2) -> Self {
        Camera {
            position,
            rotation: 0.0,
            scale: Self::DEFAULT_SCALE,
            is_following: false,
        }
    }

    pub fn add_node(position: Vec2) -> Handle<Self> {
        scene::add_node(Camera::new(position))
    }

    pub fn get_viewport(&self) -> Viewport {
        let size = vec2(screen_width() / self.scale, screen_height() / self.scale);
        let position = self.position - size / 2.0;
        Viewport {
            position,
            size,
            scale: self.scale,
        }
    }
}

impl Node for Camera {
    fn ready(node: RefMut<Self>) {
        storage::store(node.get_viewport());
    }

    fn update(node: RefMut<Self>) {
        storage::store(node.get_viewport());
    }

    fn fixed_update(mut node: RefMut<Self>) {
        let game_state = scene::find_node_by_type::<GameState>().unwrap();
        if let Some(mut player) = Actor::find_by_player_id(&game_state.local_player_id) {
            let viewport = node.get_viewport();
            let bounds = {
                let size = viewport.size * Self::FOLLOW_THRESHOLD_FRACTION;
                let center = viewport.get_center();
                Rect::new(center.x - size.x / 2.0, center.y - size.y / 2.0, size.x, size.y)
            };

            if node.is_following || bounds.contains(player.body.position) == false {
                let difference = player.body.position.sub(node.position);
                if difference.length() > Self::FOLLOW_END_AT_DISTANCE {
                    node.is_following = true;
                    node.position += difference * Self::FOLLOW_LERP_FRACTION;
                } else {
                    node.is_following = false;
                }
            }
        }

        scene::set_camera_1(Camera2D {
            offset: vec2(0.0, 0.0),
            target: vec2(node.position.x, node.position.y),
            zoom: vec2(node.scale / screen_width(), -node.scale / screen_height()) * 2.0,
            rotation: node.rotation,
            ..Camera2D::default()
        });
    }

    fn draw(_node: RefMut<Self>) where Self: Sized {}
}
