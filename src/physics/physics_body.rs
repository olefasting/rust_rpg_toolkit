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

use crate::{physics::Collider, GameState, Map, get_global, MAP_LAYER_SOLIDS, MAP_LAYER_BARRIERS};
use crate::globals::DebugMode;
use std::ops::Deref;

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
        let debug_mode  = get_global::<DebugMode>();
        if debug_mode.is_enabled {
            if let Some(collider) = self.get_offset_collider() {
                match collider {
                    Collider::Rectangle(rect) => draw_rectangle_lines(
                        rect.x, rect.y, rect.w, rect.h, 4.0, debug_mode.color),
                    Collider::Circle(circle) => draw_circle_lines(
                        circle.x, circle.y, circle.r, 4.0, debug_mode.color)
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
            let mut game_state = scene::find_node_by_type::<GameState>().unwrap();
            let rect = game_state.map.to_map_grid(Rect::from(collider.offset(self.velocity)));
            for layer_id in &[MAP_LAYER_SOLIDS, MAP_LAYER_BARRIERS] {
                for (_, _, tile) in game_state.map.get_tiles(layer_id, Some(rect)) {
                    if tile.is_some() {
                        return;
                    }
                }
            }

            for (_, mut body_lens) in scene::find_nodes_with::<PhysicsObject>() {
                if let Some(body) = body_lens.get() {
                    if let Some(other_collider) = body.get_offset_collider() {
                        if collider.offset(self.velocity).overlaps(&other_collider) {
                            return;
                        }
                    }
                }
            }

            self.position += self.velocity;
        }
    }
}
