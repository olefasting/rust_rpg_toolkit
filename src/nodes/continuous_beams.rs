use std::ops::Sub;

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
use crate::physics::get_beam_end;
use crate::ability::Effect;

pub struct ContinuousBeam {
    pub actor_id: String,
    pub actor: Handle<Actor>,
    pub factions: Vec<String>,
    pub effects: Vec<Effect>,
    pub color: Color,
    pub width: f32,
    pub origin: Vec2,
    pub end: Vec2,
}

pub struct ContinuousBeams {
    active: Vec<ContinuousBeam>,
}

impl ContinuousBeams {
    const DEFAULT_COLOR: Color = color::RED;
    const DEFAULT_WIDTH: f32 = 4.0;

    const WIDTH_TOLERANCE_FACTOR: f32 = 350.0;

    pub fn new() -> Self {
        ContinuousBeams {
            active: Vec::new(),
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }

    pub fn spawn(
        &mut self,
        actor_id: &str,
        actor: Handle<Actor>,
        factions: &[String],
        effects: &[Effect],
        color_override: Option<Color>,
        width_override: Option<f32>,
        origin: Vec2,
        end: Vec2,
    ) {
        let beam = ContinuousBeam {
            actor_id: actor_id.to_string(),
            actor,
            factions: factions.to_vec(),
            effects: effects.to_vec(),
            color: color_override.unwrap_or(Self::DEFAULT_COLOR),
            width: width_override.unwrap_or(Self::DEFAULT_WIDTH),
            origin,
            end,
        };
        self.active.push(beam);
    }
}

impl Node for ContinuousBeams {
    fn update(mut node: RefMut<Self>) {
        for mut beam in &mut node.active {
            let mut cutoff = get_beam_end(
                beam.origin,
                beam.end,
                beam.width,
                Self::WIDTH_TOLERANCE_FACTOR,
            );
            'outer: for mut other_actor in scene::find_nodes_by_type::<Actor>() {
                let position = match other_actor.body.get_offset_collider() {
                    Some(collider) => collider.get_position(),
                    None => other_actor.body.position,
                };
                if beam_collision_check(position, beam.origin, beam.end, beam.width,Self::WIDTH_TOLERANCE_FACTOR) {
                    for effect in beam.effects.clone() {
                        if other_actor.apply_effect(&beam.actor_id, beam.actor, &beam.factions, effect) {
                            if beam.origin.distance(position) < beam.origin.distance(cutoff) {
                                cutoff = position;
                            }
                        } else {
                            continue 'outer;
                        }
                    }
                }
            }
            beam.end = beam.origin + beam.end.sub(beam.origin).clamp_length(0.0, beam.origin.distance(cutoff));
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
