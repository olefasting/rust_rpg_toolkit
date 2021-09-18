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
    prelude::*,
};

use crate::{
    render::Viewport,
    nodes::{
        GameState,
        Actor,
    },
};

pub struct Camera {
    pub position: Vec2,
    pub rotation: f32,
    pub is_following: bool,
    scale: f32,
    render_target: RenderTarget,
}

impl Camera {
    const FOLLOW_THRESHOLD_FRACTION: f32 = 0.4;
    const FOLLOW_END_AT_DISTANCE: f32 = 20.0;
    const FOLLOW_LERP_FRACTION: f32 = 0.03;

    const DEFAULT_SCALE: f32 = 2.0;

    pub fn new() -> Self {
        let scale = Self::DEFAULT_SCALE;
        let render_target = render_target(
            (screen_width() / scale) as u32,
            (screen_height() / scale) as u32,
        );
        render_target.texture.set_filter(FilterMode::Nearest);

        Camera {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale,
            is_following: false,
            render_target,
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Camera::new())
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

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: f32) {
        let render_target = render_target(
            (screen_width() / scale) as u32,
            (screen_height() / scale) as u32,
        );
        render_target.texture.set_filter(FilterMode::Nearest);
        self.render_target = render_target;
        self.scale = scale;
    }

    pub fn get_render_target(&self) -> &RenderTarget {
        &self.render_target
    }

    fn get_camera(&self) -> Camera2D {
        Camera2D {
            offset: vec2(0.0, 0.0),
            target: vec2(self.position.x.round(), self.position.y.round()),
            zoom: vec2(self.scale / screen_width(), self.scale / screen_height()) * 2.0,
            rotation: self.rotation,
            render_target: Some(self.render_target),
            ..Camera2D::default()
        }
    }
}

impl Node for Camera {
    fn ready(node: RefMut<Self>) {
        storage::store(node.get_viewport());
    }

    fn update(mut node: RefMut<Self>) {
        storage::store(node.get_viewport());

        let game_state = scene::find_node_by_type::<GameState>().unwrap();
        let actor = if let Some(handle) = game_state.player.actor_handle {
            scene::try_get_node(handle)
        } else {
            Actor::find_by_player_id(&game_state.player.id)
        };

        if let Some(actor) = actor {
            let viewport = node.get_viewport();
            let bounds = {
                let size = viewport.size * Self::FOLLOW_THRESHOLD_FRACTION;
                let center = viewport.get_center();
                Rect::new(center.x - size.x / 2.0, center.y - size.y / 2.0, size.x, size.y)
            };

            if node.is_following || bounds.contains(actor.body.position) == false {
                let distance = actor.body.position.sub(node.position);
                if distance.length() > Self::FOLLOW_END_AT_DISTANCE {
                    node.is_following = true;
                    node.position += distance * Self::FOLLOW_LERP_FRACTION;
                } else {
                    node.is_following = false;
                }
            }
        }
    }

    fn draw(node: RefMut<Self>) where Self: Sized {
        scene::set_camera(0, Some(node.get_camera()));
    }
}
