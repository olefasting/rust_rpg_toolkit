use macroquad::{
    experimental::{
        scene::{
            HandleUntyped,
            Lens,
        }
    },
    color,
    prelude::*,
};

use crate::{
    physics::Collider,
    GameState,
    Map,
    MAP_LAYER_SOLIDS,
    MAP_LAYER_BARRIERS,
};

pub type PhysicsObject = (HandleUntyped, Lens<PhysicsBody>);

#[derive(Clone)]
pub struct PhysicsBody {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub collider: Option<Collider>,
}

impl PhysicsBody {
    pub fn new(position: Vec2, rotation: f32, collider: Option<Collider>) -> Self {
        PhysicsBody {
            position,
            rotation,
            velocity: Vec2::ZERO,
            collider,
        }
    }

    pub fn debug_draw(&self) {
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
        }
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
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            if game_state.map.get_collisions(collider.offset(self.velocity)).is_empty() == false {
                // TODO: More advanced collisions
                return;
            }
            for (_, mut body_lens) in scene::find_nodes_with::<PhysicsObject>() {
                if let Some(body) = body_lens.get() {
                    if let Some(other_collider) = body.get_offset_collider() {
                        if collider.offset(self.velocity).overlaps(other_collider) {
                            return;
                        }
                    }
                }
            }

            self.position += self.velocity;
        }
    }
}
