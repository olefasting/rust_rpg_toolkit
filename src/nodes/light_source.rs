use crate::prelude::*;

pub struct LightSource {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub intensity: f32,
}

impl LightSource {
    pub const DEFAULT_SIZE: Vec2 = Vec2::ZERO;
    pub const DEFAULT_COLOR: Color = color::WHITE;
    pub const DEFAULT_INTENSITY: f32 = 0.1;

    pub fn new(position: Vec2, size: Vec2, color: Color, intensity: f32, _ttl: Option<f32>) -> Self {
        let intensity = intensity.clamp(0.0, 1.0);

        LightSource {
            position,
            size,
            color,
            intensity,
        }
    }

    pub fn add_node(position: Vec2, size: Vec2, color: Color, intensity: f32, ttl: Option<f32>) -> Handle<Self> {
        scene::add_node(Self::new(position, size, color, intensity, ttl))
    }
}

impl Node for LightSource {
    fn update(_node: RefMut<Self>) {

    }
}
