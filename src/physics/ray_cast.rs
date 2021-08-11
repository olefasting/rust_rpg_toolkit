use std::ops::Sub;

use macroquad::prelude::*;

use crate::physics::Collider;

pub const RAY_CAST_RESOLUTION: f32 = 1.0;

pub fn ray_cast(from: Vec2, to: Vec2) -> Option<f32> {
    // let step = to.sub(from).normalize() * RAY_CAST_RESOLUTION;
    // let collider = Collider::rect(0.0, 0.0, 1.0, 1.0).offset(from - step);
    // while collider.offset(step).overlaps(other) {
    //
    // }
    None
}
