use macroquad::prelude::*;

use crate::math::Circle;

#[derive(Copy, Clone)]
pub enum Collider {
    Rectangle(Rect),
    Circle(Circle),
}

impl Collider {
    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Collider {
        Collider::Rectangle(Rect::new(x, y, w, h))
    }

    pub fn circle(x: f32, y: f32, r: f32) -> Collider {
        Collider::Circle(Circle::new(x, y, r))
    }

    pub fn offset(self, offset: Vec2) -> Collider {
        match self {
            Collider::Rectangle(rect) => Collider::Rectangle(rect.offset(offset)),
            Collider::Circle(circle) => Collider::Circle(circle.offset(offset)),
        }
    }

    pub fn overlaps(&self, other: &Collider) -> bool {
        match self {
            Collider::Rectangle(rect) => match other {
                Collider::Rectangle(other_rect) => rect.overlaps(&other_rect),
                Collider::Circle(other_circle) => other_circle.overlaps_rect(&rect),
            },
            Collider::Circle(circle) => match other {
                Collider::Rectangle(other_rect) => circle.overlaps_rect(&other_rect),
                Collider::Circle(other_circle) => other_circle.overlaps(&circle),
            },
        }
    }
}
