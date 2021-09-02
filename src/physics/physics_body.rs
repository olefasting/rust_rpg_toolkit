use crate::prelude::*;

use super::{
    CollisionKind,
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
                        draw_rectangle(x, y, w, h,color::RED),
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
            self.last_collisions = Vec::new();
            let movement = (self.velocity * 50.0) * get_frame_time();
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            let collisions = game_state.map.get_collisions(collider.with_offset(movement));
            if collisions.len() > 0 {
                let tile_size = game_state.map.tile_size;
                self.last_collisions = collisions
                    .into_iter()
                    .map(|(position, kind)| {
                        let collider = Collider::rect(position.x, position.y, tile_size.x, tile_size.y);
                        (collider, kind)
                    })
                    .collect();
                return;
            }

            #[cfg(feature = "collision_between_actors")]
                {
                    let mut collisions = Vec::new();
                    for actor in scene::find_nodes_by_type::<Actor>() {
                        if let Some(other_collider) = actor.body.get_offset_collider() {
                            if collider.with_offset(self.velocity).overlaps(other_collider) {
                                collisions.push((other_collider, CollisionKind::Actor));
                            }
                        }
                    }
                    if collisions.len() > 0 {
                        self.last_collisions.append(&mut collisions);
                        return;
                    }
                }

            self.position += movement;
        }
    }
}
