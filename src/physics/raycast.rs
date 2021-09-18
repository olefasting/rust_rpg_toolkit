use crate::prelude::*;

use super::COLLISION_RESOLUTION;

pub fn raycast(origin: Vec2, end: Vec2, ignore_barriers: bool, ignore_actors: bool) -> Option<Vec2> {
    if origin.distance(end) > COLLISION_RESOLUTION {
        let map = storage::get::<Map>();
        let direction = end.sub(origin).normalize_or_zero();
        let collider = Collider::circle(0.0, 0.0, 1.0);
        let change = direction * COLLISION_RESOLUTION;
        let mut current = origin;
        while current.distance(end) > COLLISION_RESOLUTION {
            let collider = collider.with_offset(current);
            for (_, kind) in map.get_collisions(collider) {
                if ignore_barriers == false || kind == CollisionKind::Solid {
                    return Some(current);
                }
            }
            if ignore_actors == false {
                for actor in  scene::find_nodes_by_type::<Actor>() {
                    if let Some(other_collider) = actor.body.get_offset_collider() {
                        if other_collider.contains(current) {
                            return Some(current);
                        }
                    }
                }
            }
            current += change;
        }
    }
    None
}
