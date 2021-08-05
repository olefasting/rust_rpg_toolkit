use macroquad::math::{Vec2, Rect};

#[derive(Copy, Clone)]
pub struct PhysicsBody {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub collider: Rect,
}

impl PhysicsBody {
    pub fn new(position: Vec2, rotation: f32, collider_size: Vec2, collider_offset: Option<Vec2>) -> Self {
        let offset = collider_offset.unwrap_or(Vec2::ZERO);
        PhysicsBody {
            position,
            rotation,
            velocity: Vec2::ZERO,
            collider: Rect::new(
                offset.x,
                offset.y,
                collider_size.x,
                collider_size.y,
            ),
        }
    }
}
