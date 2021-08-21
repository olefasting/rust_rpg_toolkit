use macroquad::prelude::*;

pub fn rotate_vector(vec: Vec2, rad: f32) -> Vec2 {
    let sa = rad.sin();
    let ca = rad.cos();
    vec2(ca * vec.x - sa * vec.y, sa * vec.x + ca * vec.y)
}

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

pub fn rad_to_deg(rad: f32) -> f32 {
    (rad * 180.0) / std::f32::consts::PI
}
