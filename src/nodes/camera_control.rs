use macroquad::{
    experimental::scene::{
        Node,
        RefMut,
    },
    prelude::*,
};

use crate::graphics::{
    get_aspect_ratio,
    to_world_space,
    to_screen_space,
};

pub struct CameraControl {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: f32,

    zoom_speed: f32,
    pan_speed: f32,
    rotation_speed: f32,
}

impl CameraControl {
    #[allow(dead_code)]
    const FRUSTUM_PADDING: f32 = 100.0;

    const DEFAULT_PAN_SPEED: f32 = 300.0;
    const DEFAULT_ROTATION_SPEED: f32 = 300.0;
    const DEFAULT_ZOOM_SPEED: f32 = 3.0;

    const ZOOM_MIN: f32 = 0.5;
    const ZOOM_MAX: f32 = 3.0;

    pub fn new() -> Self {
        CameraControl {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: 1.0,
            zoom_speed: Self::DEFAULT_ZOOM_SPEED,
            pan_speed: Self::DEFAULT_PAN_SPEED,
            rotation_speed: Self::DEFAULT_ROTATION_SPEED,
        }
    }

    #[allow(dead_code)]
    pub fn get_aspect_ratio(&self) -> f32 {
        get_aspect_ratio()
    }

    pub fn get_viewport(&self) -> Rect {
        let width = screen_width() / self.scale;
        let height = screen_height() / self.scale;
        Rect::new(
            self.position.x - (width / 2.0),
            -self.position.y - (height / 2.0),
            width,
            height,
        )
    }

    #[allow(dead_code)]
    pub fn is_in_view(&self, rect: &Rect) -> bool {
        let padding = Self::FRUSTUM_PADDING / self.scale;
        let mut view_rect = self.get_viewport();
        view_rect.x -= padding;
        view_rect.y -= padding;
        view_rect.w += padding * 2.0;
        view_rect.h += padding * 2.0;
        view_rect.overlaps(rect)
    }

    #[allow(dead_code)]
    pub fn to_screen_space(&self, coords: Vec2) -> Vec2 {
        to_screen_space(coords, self.get_viewport().point(), self.scale)
    }

    #[allow(dead_code)]
    pub fn to_world_space(&self, coords: Vec2) -> Vec2 {
        to_world_space(coords, self.get_viewport().point(), self.scale)
    }

    pub fn pan(&mut self, direction: Vec2) {
        let dt = get_frame_time();
        self.position.x += direction.x * (self.pan_speed * dt);
        self.position.y -= direction.y * (self.pan_speed * dt);
    }

    pub fn rotate_cw(&mut self) {
        self.rotation += self.rotation_speed * get_frame_time();
    }

    pub fn rotate_ccw(&mut self) {
        self.rotation -= self.rotation_speed * get_frame_time();
    }

    pub fn zoom_in(&mut self) {
        let zoom = self.scale - self.zoom_speed * get_frame_time();
        self.scale = zoom.clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);
    }

    pub fn zoom_out(&mut self) {
        let zoom = self.scale + self.zoom_speed * get_frame_time();
        self.scale = zoom.clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);
    }
}

impl Node for CameraControl {
    fn update(node: RefMut<Self>) {
        scene::set_camera_1(Camera2D {
            offset: vec2(0.0, 0.0),
            target: vec2(node.position.x, -node.position.y),
            zoom: vec2(node.scale / screen_width(), -node.scale / screen_height()) * 2.0,
            rotation: node.rotation,
            ..Camera2D::default()
        });
    }
}
