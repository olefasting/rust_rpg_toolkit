use macroquad::prelude::*;

const WIDTH_TOLERANCE_FACTOR: f32 = 300.0;

pub fn beam_collision_check(point: Vec2, origin: Vec2, end: Vec2, width: f32) -> bool {
    let va = origin - end;
    let vb = point - end;
    let area = va.x * vb.y - va.y * vb.x;
    return area.abs() < width * WIDTH_TOLERANCE_FACTOR;
}
