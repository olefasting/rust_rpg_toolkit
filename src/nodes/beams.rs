use macroquad::{
    experimental::scene::{
        Node,
        Handle,
        RefMut,
    },
    prelude::*,
};
use crate::nodes::Actor;

fn beam_collision_check(origin: Vec2, end: Vec2, width: f32, point: Vec2) -> bool {
    let va = origin - end;
    let vb = point - end;
    let area = va.x * vb.y - va.y * vb.x;
    return area.abs() < width * Beam::WIDTH_TOLERANCE_FACTOR;
}

pub struct Beam {
    pub actor_id: String,
    pub factions: Vec<String>,
    pub damage: f32,
    pub color: Color,
    pub width: f32,
    pub origin: Vec2,
    pub end: Vec2,
}

impl Beam {
    pub const WIDTH_TOLERANCE_FACTOR: f32 = 300.0;
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

                    if beam_collision_check(beam.origin, beam.end, beam.width,position) {
                        other_actor.take_damage(&beam.actor_id, beam.damage);
                        beam.end = position;
                    }
                }
            }
        }
    }

    fn draw(mut node: RefMut<Self>) {
        node.active.retain(|beam| {
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
                beam.width / 2.0,
                beam.color,
            );
            false
        });
    }
}
