use macroquad::math::Rect;

use crate::Circle;

#[derive(Clone)]
pub enum Collider {
    Rectangle { shape: Rect },
    Circle { shape: Circle },
}

impl Collider {
    pub fn rect(shape: Rect) -> Collider {
        Collider::Rectangle { shape }
    }

    pub fn circle(shape: Circle) -> Collider {
        Collider::Circle { shape }
    }
}
