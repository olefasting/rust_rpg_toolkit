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
        if let Some(mut collider) = self.get_offset_collider() {
            let movement = (self.velocity * 50.0) * get_frame_time();

            if movement != Vec2::ZERO {
                #[cfg(feature = "collision_between_actors")]
                    let nearby_actors = {
                    let viewport = storage::get::<Viewport>();
                    scene::find_nodes_by_type::<Actor>()
                        .into_iter()
                        .filter(|actor| actor.is_in_frustum(&viewport.get_frustum()))
                        .collect()
                };

                let increment = vec2(
                    if movement.x > 0.0 {
                        COLLISION_RESOLUTION
                    } else {
                        -COLLISION_RESOLUTION
                    },
                    if movement.y > 0.0 {
                        COLLISION_RESOLUTION
                    } else {
                        -COLLISION_RESOLUTION
                    },
                );

                let mut x_collisions = Vec::new();
                let mut y_collisions = Vec::new();

                let mut final_movement = Vec2::ZERO;

                while final_movement.x != movement.x {
                    x_collisions.clear();

                    let modified_increment = if final_movement.x.abs() < movement.x.abs() - increment.x.abs() {
                        vec2(increment.x, 0.0)
                    } else {
                        vec2(movement.x - final_movement.x, 0.0)
                    };

                    let modified_collider = collider.with_offset(modified_increment);

                    #[cfg(feature = "collision_between_actors")]
                        {
                            let mut actor_collisions = check_actor_collisions(modified_collider, &nearby_actors);
                            x_collisions.append(&mut actor_collisions);
                        }

                    let mut map_collisions = check_map_collision(modified_collider);
                    x_collisions.append(&mut map_collisions);

                    if x_collisions.len() > 0 {
                        break;
                    }

                    final_movement += modified_increment;
                    collider = modified_collider;
                }

                while final_movement.y != movement.y {
                    y_collisions.clear();

                    let modified_increment = if final_movement.y.abs() < movement.y.abs() - increment.y.abs() {
                        vec2(0.0, increment.y)
                    } else {
                        vec2(0.0, movement.y - final_movement.y)
                    };

                    let modified_collider = collider.with_offset(modified_increment);

                    #[cfg(feature = "collision_between_actors")]
                        {
                            let mut actor_collisions = check_actor_collisions(modified_collider, &nearby_actors);
                            y_collisions.append(&mut actor_collisions);
                        }

                    let mut map_collisions = check_map_collision(modified_collider);
                    y_collisions.append(&mut map_collisions);

                    if y_collisions.len() > 0 {
                        break;
                    }

                    final_movement += modified_increment;
                    collider = modified_collider;
                }

                self.last_collisions = Vec::new();
                self.last_collisions.append(&mut x_collisions);
                self.last_collisions.append(&mut y_collisions);

                self.position += final_movement;
            }
        }
    }
}

fn check_map_collision(collider: Collider) -> Vec<(Collider, CollisionKind)> {
    let map = storage::get::<Map>();
    let tile_size = map.tile_size;
    let collisions = map.get_collisions(collider);

    collisions
        .into_iter()
        .map(|(position, kind)| {
            let collider = Collider::rect(position.x, position.y, tile_size.x, tile_size.y);
            (collider, kind)
        })
        .collect()
}

#[cfg(feature = "collision_between_actors")]
fn check_actor_collisions(collider: Collider, actors: &Vec<RefMut<Actor>>) -> Vec<(Collider, CollisionKind)> {
    let mut collisions = Vec::new();
    for actor in actors {
        if let Some(other_collider) = actor.body.get_offset_collider() {
            if collider.overlaps(other_collider) {
                collisions.push((other_collider, CollisionKind::Actor));
            }
        }
    }
    collisions
}
