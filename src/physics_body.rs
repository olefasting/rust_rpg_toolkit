use macroquad::math::Vec2;

#[derive(Copy, Clone)]
pub struct PhysicsBody {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
}

impl PhysicsBody {
    pub fn new(position: Vec2, rotation: f32) -> Self {
        PhysicsBody {
            position,
            rotation,
            velocity: Vec2::ZERO,
        }
    }
}
