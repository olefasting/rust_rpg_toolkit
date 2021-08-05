use macroquad::math::Rect;

use crate::Circle;

#[derive(Clone)]
pub enum Collider {
    Rectangle(Rect),
    Circle(Circle),
}

impl Collider {
    pub fn from_rect(rect: Rect) -> Collider {
        Collider::Rectangle(rect)
    }

    pub fn from_circle(circle: Circle) -> Collider {
        Collider::Circle(circle)
    }

    pub fn overlaps(&self, other: &Collider) -> bool {
        match self {
            Collider::Rectangle(rect) => match other {
                Collider::Rectangle(other_rect) => rect.overlaps(&other_rect),
                Collider::Circle(circle) => circle.overlaps_rect(&rect),
            },
            Collider::Circle(circle) => match other {
                Collider::Rectangle(rect) => circle.overlaps_rect(&rect),
                Collider::Circle(other_circle) => circle.overlaps(&other_circle),
            },
        }
    }
}
