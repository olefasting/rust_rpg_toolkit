use crate::prelude::*;

use super::{
    ACTOR_TO_ACTOR_COLLISIONS,
};

#[derive(Clone)]
pub struct PhysicsBody {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub collider: Option<Collider>,
    pub last_collisions: Vec<(Vec2, MapCollisionKind)>,
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
        let game_state  = scene::find_node_by_type::<GameState>().unwrap();
        if game_state.in_debug_mode {
            if let Some(collider) = self.get_offset_collider() {
                match collider {
                    Collider::Rectangle { x, y, w, h } =>
                        draw_rectangle_lines(x, y, w, h, 2.0, color::RED),
                    Collider::Circle { x, y, r } =>
                        draw_circle_lines(x, y, r, 2.0, color::RED)
                }
            }

            for (position, kind) in self.last_collisions.clone() {
                draw_rectangle(
                    position.x,
                    position.y,
                    game_state.map.tile_size.x,
                    game_state.map.tile_size.y,
                    color::RED,
                );

                draw_aligned_text(
                    if kind == MapCollisionKind::Solid { "S" } else { "B" },
                    position.x + (game_state.map.tile_size.x / 2.0),
                    position.y + (game_state.map.tile_size.y / 2.0),
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

            draw_aligned_text(
                &format!("position: {}", self.position.to_string()),
                screen_width() - 50.0,
                150.0,
                HorizontalAlignment::Right,
                VerticalAlignment::Top,
                Default::default(),
            );
        }
    }

    pub fn raycast(&self, dest: Vec2, ignore_barriers: bool) -> Option<Vec2> {
        raycast(self.position, dest, ignore_barriers)
    }

    pub fn get_offset_collider(&self) -> Option<Collider> {
        if let Some(collider) = self.collider {
            Some(collider.offset(self.position))
        } else {
            None
        }
    }

    pub fn integrate(&mut self) {
        if let Some(collider) = self.get_offset_collider() {
            self.last_collisions = Vec::new();
            let movement = (self.velocity * 50.0) * get_frame_time();
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            let collisions = game_state.map.get_collisions(collider.offset(self.velocity));
            if collisions.is_empty() == false {
                self.last_collisions = collisions;
                return;
            }


            if ACTOR_TO_ACTOR_COLLISIONS {
                for actor in scene::find_node_by_type::<Actor>() {
                    if let Some(other_collider) = actor.body.get_offset_collider() {
                        if collider.offset(self.velocity).overlaps(other_collider) {
                            return;
                        }
                    }
                }
            }

            self.position += movement;
        }
    }
}
