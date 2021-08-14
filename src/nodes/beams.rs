use macroquad::{
    experimental::scene::{
        Node,
        Handle,
        RefMut,
    },
    color,
    prelude::*,
};

use crate::{
    nodes::Actor,
    physics::beam_collision_check,
};
use std::ops::Sub;
use crate::nodes::GameState;

pub struct Beam {
    pub actor_id: String,
    pub factions: Vec<String>,
    pub damage: f32,
    pub color: Color,
    pub width: f32,
    pub origin: Vec2,
    pub end: Vec2,
}

pub struct Beams {
    active: Vec<Beam>,
}

impl Beams {
    pub fn new() -> Self {
        Beams {
            active: Vec::new(),
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }

    pub fn spawn(&mut self, actor_id: &str, factions: &[String], damage: f32, color: Color, width: f32, origin: Vec2, end: Vec2) {
        let beam = Beam {
            actor_id: actor_id.to_string(),
            factions: factions.to_vec(),
            damage,
            color,
            width,
            origin,
            end,
        };
        self.active.push(beam);
    }
}

impl Node for Beams {
    fn fixed_update(mut node: RefMut<Self>) {
        for mut beam in &mut node.active {
            'outer: for mut other_actor in scene::find_nodes_by_type::<Actor>() {
                if other_actor.id != beam.actor_id {
                    for faction in &beam.factions {
                        if other_actor.factions.contains(&faction) {
                            continue 'outer;
                        }
                    }
                    let position = match other_actor.body.get_offset_collider() {
                        Some(collider) => collider.get_position(),
                        None => other_actor.body.position,
                    };
                    let game_state = scene::find_node_by_type::<GameState>().unwrap();
                    beam.end = game_state.map.get_beam_collision_point(beam.origin, beam.end, beam.width, false);
                    if beam_collision_check(position, beam.origin, beam.end, beam.width) {
                        other_actor.take_damage(&beam.actor_id, beam.damage);
                        beam.end -= beam.end.sub(position);
                    }
                }
            }
        }
    }

    fn draw(mut node: RefMut<Self>) {
        node.active.retain(|beam| {
            let mut highlight = color::WHITE;
            highlight.a = 0.5;
            draw_circle(
                beam.end.x,
                beam.end.y,
                beam.width / 2.0,
                beam.color,
            );
            draw_line(
                beam.origin.x,
                beam.origin.y,
                beam.end.x,
                beam.end.y,
                beam.width,
                beam.color,
            );
            draw_circle(
                beam.end.x,
                beam.end.y,
                ((beam.width / 2.0) * 0.8) / 2.0,
                highlight,
            );
            draw_line(
                beam.origin.x,
                beam.origin.y,
                beam.end.x,
                beam.end.y,
                (beam.width - 4.0) * 0.8,
                highlight,
            );
            false
        });
    }
}
