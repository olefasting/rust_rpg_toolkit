use macroquad::prelude::*;

use serde::{Deserialize, Serialize};

use crate::math::Circle;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum Collider {
    Rectangle {
        #[serde(default)]
        x: f32,
        #[serde(default)]
        y: f32,
        #[serde(alias = "width")]
        w: f32,
        #[serde(alias = "height")]
        h: f32,
    },
    Circle {
        #[serde(default)]
        x: f32,
        #[serde(default)]
        y: f32,
        #[serde(alias = "radius")]
        r: f32,
    },
}

impl Collider {
    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Collider {
        Collider::Rectangle { x, y, w, h }
    }

    pub fn circle(x: f32, y: f32, r: f32) -> Collider {
        Collider::Circle { x, y, r }
    }

    pub fn with_padding(self, padding: f32) -> Collider {
        match self {
            Collider::Rectangle { x, y, w, h } => Collider::Rectangle {
                x: x - padding,
                y: y - padding,
                w: w + padding * 2.0,
                h: h + padding * 2.0,
            },
            Collider::Circle { x, y, r } => Collider::Circle {
                x,
                y,
                r: r + padding,
            },
        }
    }

    pub fn with_offset(self, offset: Vec2) -> Collider {
        match self {
            Collider::Rectangle { x, y, w, h } => Collider::Rectangle {
                x: x + offset.x,
                y: y + offset.y,
                w,
                h,
            },
            Collider::Circle { x, y, r } => Collider::Circle {
                x: x + offset.x,
                y: y + offset.y,
                r,
            },
        }
    }

    pub fn get_position(self) -> Vec2 {
        match self {
            Collider::Rectangle { x, y, w: _, h: _ } => vec2(x, y),
            Collider::Circle { x, y, r: _ } => vec2(x, y),
        }
    }

    pub fn get_center(self) -> Vec2 {
        match self {
            Collider::Rectangle { x, y, w, h } => vec2(x + w / 2.0, y + h / 2.0),
            Collider::Circle { x, y, r: _ } => vec2(x, y),
        }
    }

    pub fn overlaps(self, other: Collider) -> bool {
        match self {
            Collider::Rectangle { x, y, w, h } => {
                let rect = Rect::new(x, y, w, h);
                match other {
                    Collider::Rectangle { x, y, w, h } => rect.overlaps(&Rect::new(x, y, w, h)),
                    Collider::Circle { x, y, r } => Circle::new(x, y, r).overlaps_rect(&rect),
                }
            }
            Collider::Circle { x, y, r } => {
                let circle = Circle::new(x, y, r);
                match other {
                    Collider::Rectangle { x, y, w, h } => {
                        circle.overlaps_rect(&Rect::new(x, y, w, h))
                    }
                    Collider::Circle { x, y, r } => Circle::new(x, y, r).overlaps(&circle),
                }
            }
        }
    }

    pub fn overlaps_rect(self, rect: &Rect) -> bool {
        match self {
            Collider::Rectangle { x, y, w, h } => Rect::new(x, y, w, h).overlaps(rect),
            Collider::Circle { x, y, r } => Circle::new(x, y, r).overlaps_rect(rect),
        }
    }

    pub fn overlaps_circle(self, circle: &Circle) -> bool {
        match self {
            Collider::Rectangle { x, y, w, h } => circle.overlaps_rect(&Rect::new(x, y, w, h)),
            Collider::Circle { x, y, r } => Circle::new(x, y, r).overlaps(circle),
        }
    }

    pub fn contains(self, position: Vec2) -> bool {
        match self {
            Collider::Rectangle { x, y, w, h } => Rect::new(x, y, w, h).contains(position),
            Collider::Circle { x, y, r } => Circle::new(x, y, r).contains(&position),
        }
    }
}

impl From<Collider> for Rect {
    fn from(collider: Collider) -> Rect {
        match collider {
            Collider::Rectangle { x, y, w, h } => Rect::new(x, y, w, h),
            Collider::Circle { x, y, r } => Rect::new(x - r, y - r, r * 2.0, r * 2.0),
        }
    }
}
