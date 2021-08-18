use macroquad::prelude::*;

use crate::math::URect;
use crate::nodes::GameState;

pub fn beam_collision_check(point: Vec2, origin: Vec2, end: Vec2, width: f32, tolerance: f32) -> bool {
    let va = origin - end;
    let vb = point - end;
    let area = va.x * vb.y - va.y * vb.x;
    return area.abs() < width * tolerance;
}

pub fn get_beam_end(origin: Vec2, end: Vec2, width: f32, tolerance: f32, layer_ids: &[&str]) -> Vec2 {
    let game_state = scene::find_node_by_type::<GameState>().unwrap();
    let tile_size = game_state.map.tile_size;
    let rect = {
        let (origin, end) = (game_state.map.to_map_point(origin), game_state.map.to_map_point(end));
        let ((x, w), (y, h)) = (
            if origin.x > end.x { (end.x, origin.x) } else { (origin.x, end.x) },
            if origin.y > end.y { (end.y, origin.y) } else { (origin.y, end.y) });
        URect::new(x, y, w, h)
    };

    let mut collisions = Vec::new();
    for layer_id in layer_ids {
        for (x, y, tile) in game_state.map.get_tiles(layer_id, Some(rect)) {
            if tile.is_some() {
                let position = vec2(
                    (x as f32 * tile_size.x) + tile_size.x / 2.0,
                    (y as f32 * tile_size.y) + tile_size.y / 2.0);

                if beam_collision_check(position, origin, end, width, tolerance) {
                    collisions.push(position);
                }
            }
        }
    }

    collisions
        .sort_by(|a, b| a.distance(origin)
            .partial_cmp(&b.distance(origin))
            .unwrap());

    collisions
        .first()
        .cloned()
        .unwrap_or(end)
}
