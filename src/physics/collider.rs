use macroquad::prelude::*;

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

    pub fn overlaps(&self, offset: Vec2, other: &Collider, other_offset: Vec2) -> bool {
        match self {
            Collider::Rectangle(rect) => match other {
                Collider::Rectangle(other_rect) => rect.offset(offset).overlaps(&other_rect.offset(other_offset)),
                Collider::Circle(other_circle) => other_circle.offset(other_offset).overlaps_rect(&rect.offset(offset)),
            },
            Collider::Circle(circle) => match other {
                Collider::Rectangle(other_rect) => circle.offset(offset).overlaps_rect(&other_rect.offset(other_offset)),
                Collider::Circle(other_circle) => other_circle.offset(other_offset).overlaps(&circle.offset(offset)),
            },
        }
    }
}
