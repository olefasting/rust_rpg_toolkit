
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

    pub fn get_position(&self) -> Vec2 {
        match self {
            Collider::Rectangle(rect) => vec2(rect.x, rect.y),
            Collider::Circle(circle) => vec2(circle.x, circle.y),
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

    pub fn overlaps_rect(&self, rect: &Rect) -> bool {
        match self {
            Collider::Rectangle(other_rect) => other_rect.overlaps(rect),
            Collider::Circle(circle) => circle.overlaps_rect(rect),
        }
    }

    pub fn overlaps_circle(&self, circle: &Circle) -> bool {
        match self {
            Collider::Rectangle(rect) => circle.overlaps_rect(rect),
            Collider::Circle(other_circle) => other_circle.overlaps(circle),
        }
    }

    pub fn contains(&self, position: Vec2) -> bool {
        match self {
            Collider::Rectangle(rect) => rect.contains(position),
            Collider::Circle(circle) => circle.contains(&position),
        }
    }
}

impl From<Collider> for Rect {
    fn from(collider: Collider) -> Self {
        match collider {
            Collider::Rectangle(rect) => rect,
            Collider::Circle(circle) => Rect::new(
                circle.x - circle.r,
                circle.y - circle.r,
                circle.r * 2.0,
                circle.r * 2.0,
            ),
        }
    }
}
