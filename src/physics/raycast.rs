use std::ops::Sub;

use macroquad::prelude::*;

use crate::{
    nodes::GameState,
    map::MapCollisionKind,
};

use super::{
    ACTOR_TO_ACTOR_COLLISIONS,
    Collider,
    PhysicsObject,
};

const RESOLUTION: f32 = 2.5;

pub fn raycast(origin: Vec2, end: Vec2, ignore_barriers: bool) -> Option<Vec2> {
    if origin.distance(end) > RESOLUTION {
        let direction = end.sub(origin).normalize_or_zero();
        let game_state = scene::find_node_by_type::<GameState>().unwrap();
        let collider = Collider::circle(0.0, 0.0, 1.0);
        let change = direction * RESOLUTION;
        let mut current = origin;
        while current.distance(end) > RESOLUTION {
            let collider = collider.offset(current);
            for (_, kind) in game_state.map.get_collisions(collider) {
                if ignore_barriers == false || kind == MapCollisionKind::Solid {
                    return Some(current);
                }
            }
            if ACTOR_TO_ACTOR_COLLISIONS {
                for (_, mut body) in scene::find_nodes_with::<PhysicsObject>() {
                    if let Some(body) = body.get() {
                        if let Some(other_collider) = body.get_offset_collider() {
                            if other_collider.contains(current) {
                                return Some(current);
                            }
                        }
                    }
                }
            }
            current += change;
        }
    }
    None
}
