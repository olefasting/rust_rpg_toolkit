use std::ops::Sub;

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

use crate::physics::Collider;

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
        if let Some(collider) = self.offset_collider() {
            match collider {
                Collider::Rectangle(rect) => draw_rectangle_lines(
                    rect.x, rect.y, rect.w, rect.h, 4.0, color::RED),
                Collider::Circle(circle) => draw_circle_lines(
                    circle.x, circle.y, circle.r, 4.0, color::RED)
            }
        }
    }

    pub fn offset_collider(&self) -> Option<Collider> {
        if let Some(collider) = self.collider {
            Some(collider.offset(self.position))
        } else {
            None
        }
    }

    pub fn is_colliding(&self, other: &PhysicsBody) -> bool {
        if let (Some(collider), Some(other_collider)) = (self.offset_collider(), other.offset_collider()) {
            collider.overlaps(&other_collider)
        } else {
            false
        }
    }

    pub fn integrate(&mut self) {
        let mut new_position = vec2(self.position.x + self.velocity.x, self.position.y + self.velocity.y);
        let direction = new_position.sub(self.position).normalize();

        for (_, mut body_lens) in scene::find_nodes_with::<PhysicsObject>() {
            if let Some(body) = body_lens.get() {
                if let (Some(collider), Some(other_collider)) = (self.collider, body.offset_collider()) {
                    while collider.offset(new_position).overlaps(&other_collider) {
                        new_position -= direction * 5.0;
                    }
                }
            }
        }

        self.position = new_position;
    }
}
