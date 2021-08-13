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
        if let Some(collider) = self.get_offset_collider() {
            match collider {
                Collider::Rectangle(rect) => draw_rectangle_lines(
                    rect.x - (rect.w / 2.0),rect.y - (rect.h / 2.0), rect.w, rect.h, 4.0, color::RED),
                Collider::Circle(circle) => draw_circle_lines(
                    circle.x, circle.y, circle.r, 4.0, color::RED)
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
            let mut movement = self.velocity;
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            if game_state.map.solid_at_collider(collider.offset(movement)) {
                return;
            }
            for (_, mut body_lens) in scene::find_nodes_with::<PhysicsObject>() {
                if let Some(body) = body_lens.get() {
                    if let Some(other_collider) = body.get_offset_collider() {
                        if collider.offset(movement).overlaps(&other_collider) {
                            return;
                        }
                    }
                }
            }
            self.position += movement;
        }
    }
}
