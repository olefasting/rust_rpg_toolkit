use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
            RefMut,
        },
    },
    color,
    prelude::*,
};

use crate::{set_global, render::{
    get_aspect_ratio,
    to_world_space,
    to_screen_space,
    Viewport,
}, nodes::{
    Actor,
}, get_mouse_position, draw_aligned_text, get_global};
use std::ops::Sub;
use crate::render::HorizontalAlignment;

pub struct Camera {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: f32,
    pub is_following: bool,
}

impl Camera {
    const FOLLOW_THRESHOLD_FRACTION: f32 = 0.4;
    const FOLLOW_END_AT_DISTANCE: f32 = 25.0;
    const FOLLOW_LERP_FRACTION: f32 = 0.02;

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

    pub fn get_ratio(&self) -> f32 {
        get_aspect_ratio()
    }

    pub fn get_viewport(&self) -> Viewport {
        let width = screen_width() / self.scale;
        let height = screen_height() / self.scale;
        Viewport {
            x: self.position.x - (width / 2.0),
            y: self.position.y - (height / 2.0),
            width,
            height,
            scale: self.scale,
        }
    }
}

impl Node for Camera {
    fn ready(node: RefMut<Self>) {
        set_global(node.get_viewport());
    }

    fn update(node: RefMut<Self>) {
        set_global(node.get_viewport());
    }

    fn fixed_update(mut node: RefMut<Self>) {
        let actor = Actor::find_local_player_actor().unwrap();
        let viewport = node.get_viewport();
        let bounds = {
            let size = vec2(viewport.width * Self::FOLLOW_THRESHOLD_FRACTION, viewport.height * Self::FOLLOW_THRESHOLD_FRACTION);
            Rect::new(viewport.x + viewport.width / 2.0 - size.x / 2.0, viewport.y + viewport.height / 2.0 - size.y / 2.0, size.x, size.y)
        };

        if node.is_following || bounds.contains(actor.body.position) == false {
            let distance = actor.body.position.sub(node.position);
            if distance.length() <= Self::FOLLOW_END_AT_DISTANCE {
                node.is_following = false;
                return;
            }
            node.position += distance * Self::FOLLOW_LERP_FRACTION;
        }

        scene::set_camera_1(Camera2D {
            offset: vec2(0.0, 0.0),
            target: vec2(node.position.x, node.position.y),
            zoom: vec2(node.scale / screen_width(), -node.scale / screen_height()) * 2.0,
            rotation: node.rotation,
            ..Camera2D::default()
        });
    }

    fn draw(_node: RefMut<Self>) where Self: Sized {
    }
}
