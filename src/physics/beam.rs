use crate::prelude::*;

pub fn beam_collision_check(
    point: Vec2,
    origin: Vec2,
    end: Vec2,
    width: f32,
    tolerance: f32,
) -> bool {
    let va = origin - end;
    let vb = point - end;
    let area = va.x * vb.y - va.y * vb.x;
    return area.abs() < width * tolerance;
}

pub fn get_beam_end(origin: Vec2, end: Vec2, width: f32, tolerance: f32) -> Vec2 {
    let map = storage::get::<Map>();
    let tile_size = map.tile_size;
    let collider = {
        let ((x, w), (y, h)) = (
            if origin.x > end.x {
                (end.x, origin.x)
            } else {
                (origin.x, end.x)
            },
            if origin.y > end.y {
                (end.y, origin.y)
            } else {
                (origin.y, end.y)
            },
        );
        Collider::rect(x, y, w, h)
    };

    let mut collisions: Vec<Vec2> = map
        .get_collisions(collider)
        .into_iter()
        .filter_map(|(position, kind)| {
            let position = position + tile_size / 2.0;
            if kind == CollisionKind::Solid
                && beam_collision_check(position, origin, end, width, tolerance)
            {
                Some(position)
            } else {
                None
            }
        })
        .collect();

    collisions.sort_by(|a, b| sort_by_distance(origin, a, b));
    collisions.first().cloned().unwrap_or(end)
}
