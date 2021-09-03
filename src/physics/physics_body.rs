use crate::prelude::*;

use super::{
    COLLISION_RESOLUTION,
};

#[derive(Clone)]
pub struct PhysicsBody {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub collider: Option<Collider>,
    pub last_collisions: Vec<(Collider, CollisionKind)>,
}

impl PhysicsBody {
    pub fn new(position: Vec2, rotation: f32, collider: Option<Collider>) -> Self {
        PhysicsBody {
            position,
            rotation,
            velocity: Vec2::ZERO,
            collider,
            last_collisions: Vec::new(),
        }
    }

    pub fn debug_draw(&mut self) {
        let game_state = scene::find_node_by_type::<GameState>().unwrap();
        if game_state.in_debug_mode {
            if let Some(collider) = self.get_offset_collider() {
                match collider {
                    Collider::Rectangle { x, y, w, h } =>
                        draw_rectangle_lines(x, y, w, h, 2.0, color::RED),
                    Collider::Circle { x, y, r } =>
                        draw_circle_lines(x, y, r, 2.0, color::RED)
                }
            }

            for (collider, kind) in self.last_collisions.clone() {
                match collider {
                    Collider::Rectangle { x, y, w, h } =>
                        draw_rectangle(x, y, w, h, color::RED),
                    Collider::Circle { x, y, r } =>
                        draw_circle(x, y, r, color::RED),
                }

                let position = collider.get_center();
                draw_aligned_text(
                    match kind {
                        CollisionKind::Actor => "A",
                        CollisionKind::Barrier => "B",
                        CollisionKind::Solid => "S",
                        CollisionKind::None => "?",
                    },
                    position.x,
                    position.y,
                    HorizontalAlignment::Center,
                    VerticalAlignment::Center,
                    TextParams {
                        ..Default::default()
                    },
                );
            }

            let begin = if let Some(collider) = self.get_offset_collider() {
                collider.get_center()
            } else {
                self.position
            };

            let end = begin + self.velocity * 10.0;
            draw_line(
                begin.x,
                begin.y,
                end.x,
                end.y,
                2.0,
                color::RED,
            );
        }
    }

    pub fn raycast(&self, dest: Vec2, ignore_barriers: bool, ignore_actors: bool) -> Option<Vec2> {
        raycast(self.position, dest, ignore_barriers, ignore_actors)
    }

    pub fn get_offset_collider(&self) -> Option<Collider> {
        if let Some(collider) = self.collider {
            Some(collider.with_offset(self.position))
        } else {
            None
        }
    }

    pub fn integrate(&mut self) {
        if let Some(collider) = self.get_offset_collider() {
            let mut movement = (self.velocity * 50.0) * get_frame_time();

            if movement != Vec2::ZERO {
                let game_state = scene::find_node_by_type::<GameState>().unwrap();
                let viewport = storage::get::<Viewport>();
                let nearby_actors = scene::find_nodes_by_type::<Actor>()
                    .into_iter()
                    .filter(|actor| actor.is_in_frustum(&viewport.get_frustum()))
                    .collect();

                let mut x_collisions = Vec::new();
                let resolution = if movement.x > 0.0 {
                    COLLISION_RESOLUTION
                } else {
                    -COLLISION_RESOLUTION
                };

                let x_movement = check_axis(collider, &mut x_collisions, vec2(movement.x, 0.0), vec2(resolution, 0.0), true, &nearby_actors, &game_state);

                let mut y_collisions = Vec::new();
                let resolution = if movement.y > 0.0 {
                    COLLISION_RESOLUTION
                } else {
                    -COLLISION_RESOLUTION
                };

                movement = check_axis(collider, &mut y_collisions, vec2(x_movement.x, movement.y), vec2(0.0, resolution), false, &nearby_actors, &game_state);

                self.last_collisions = Vec::new();
                self.last_collisions.append(&mut x_collisions);
                self.last_collisions.append(&mut y_collisions);

                self.position += movement;
            }
        }
    }
}

fn check_axis(collider: Collider, collisions: &mut Vec<(Collider, CollisionKind)>, movement: Vec2, increment: Vec2, is_x: bool, nearby_actors: &Vec<RefMut<Actor>>, game_state: &RefMut<GameState>) -> Vec2 {
    let mut final_movement = if is_x {
        vec2(0.0, 0.0)
    } else {
        vec2(movement.x, 0.0)
    };

    while final_movement != movement {
        collisions.clear();

        let mut modified_movement =
            if final_movement.length() < (movement - increment).length() {
                final_movement + increment
            } else {
                movement
            };

        let map_collisions = game_state.map.get_collisions(collider.with_offset(modified_movement));

        if map_collisions.len() > 0 {
            let tile_size = game_state.map.tile_size;
            let mut map_collisions = map_collisions
                .into_iter()
                .map(|(position, kind)| {
                    let collider = Collider::rect(position.x, position.y, tile_size.x, tile_size.y);
                    (collider, kind)
                })
                .collect();
            collisions.append(&mut map_collisions);
        }

        #[cfg(feature = "collision_between_actors")]
            {
                for actor in nearby_actors {
                    if let Some(other_collider) = actor.body.get_offset_collider() {
                        if collider.with_offset(modified_movement).overlaps(other_collider) {
                            collisions.push((other_collider, CollisionKind::Actor));
                        }
                    }
                }
            }

        if collisions.len() > 0 {
            break;
        }

        final_movement = modified_movement;
    }

    final_movement
}
