use serde::{
    Serialize,
    Deserialize,
};

use crate::json::Vec2;

#[derive(Clone, Serialize, Deserialize)]
pub struct Collider {
    kind: String,
    offset: Vec2,
    radius: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
}

impl Collider {
    pub const CIRCLE_KIND: &'static str = "circle";
    pub const RECTANGLE_KIND: &'static str = "rectangle";

    pub fn from(other: crate::Collider) -> Self {
        match other {
            crate::Collider::Circle(circle) => Collider {
                kind: Self::CIRCLE_KIND.to_string(),
                offset: Vec2::new(circle.x, circle.y),
                radius: Some(circle.r),
                width: None,
                height: None,
            },
            crate::Collider::Rectangle(rect) => Collider {
                kind: Self::RECTANGLE_KIND.to_string(),
                offset: Vec2::new(rect.x, rect.y),
                radius: None,
                width: Some(rect.w),
                height: Some(rect.h),
            }
        }
    }
}

impl From<Collider> for crate::Collider {
    fn from(other: Collider) -> Self {
        match &*other.kind {
            Collider::CIRCLE_KIND => crate::Collider::circle(other.offset.x, other.offset.y, other.radius.unwrap()),
            Collider::RECTANGLE_KIND => crate::Collider::rect(other.offset.x, other.offset.y, other.width.unwrap(), other.height.unwrap()),
            _ => {
                panic!("Invalid collider kind '{}", other.kind);
                crate::Collider::circle(0.0, 0.0, 0.0)
            }
        }
    }
}
