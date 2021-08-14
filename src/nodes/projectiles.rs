use std::ops::Sub;

use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
            RefMut,
        },
    },
    prelude::*,
};

use crate::{nodes::{
    Actor,
    GameState,
}, physics::Collider, get_global};
use crate::render::Viewport;

pub struct Projectile {
    actor_id: String,
    factions: Vec<String>,
    damage: f32,
    color: Color,
    size: f32,
    position: Vec2,
    direction: Vec2,
    speed: f32,
    lived: f32,
    ttl: f32,
    is_sphere: bool,
}

impl Projectile {
    pub fn new(
        actor_id: &str,
        factions: &[String],
        damage: f32, color: Color,
        size: f32,
        position: Vec2,
        direction: Vec2,
        speed: f32,
        ttl: f32,
        is_sphere: bool,
    ) -> Self {
        Projectile {
            actor_id: actor_id.to_string(),
            factions: factions.to_vec(),
            damage,
            color,
            size,
            position,
            direction,
            speed,
            lived: 0.0,
            ttl,
            is_sphere,
        }
    }
}

pub struct Projectiles {
    active: Vec<Projectile>,
}

impl Projectiles {
    const SPEED_VARIANCE_MIN: f32 = 0.9;
    const SPEED_VARIANCE_MAX: f32 = 1.1;

    const SPREAD_CALCULATION_DISTANCE: f32 = 100.0;

    pub fn new() -> Self {
        Projectiles {
            active: Vec::new(),
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }

    pub fn spawn(
        &mut self,
        actor_id: &str,
        factions: &[String],
        damage: f32,
        color: Color,
        size: f32,
        position: Vec2,
        target: Vec2,
        speed: f32,
        spread: f32,
        ttl: f32,
        is_sphere: bool,
    ) {
        assert!(ttl > 0.0, "Projectile TTL must be a positive float and not 0.0");

        let spread_target = target.sub(position).normalize_or_zero() * Self::SPREAD_CALCULATION_DISTANCE;
        let direction = vec2(
          rand::gen_range(spread_target.x - spread, spread_target.x + spread),
            rand::gen_range(spread_target.y - spread, spread_target.y + spread),
        ).normalize_or_zero();
        self.active.push(Projectile::new(
            actor_id,
            factions,
            damage,
            color,
            size,
            position,
            direction,
            speed,
            ttl,
            is_sphere,
        ));
    }
}

impl Node for Projectiles {
    fn update(mut node: RefMut<Self>) {
        for projectile in &mut node.active {
            projectile.lived += get_frame_time();
        }
    }

    fn fixed_update(mut node: RefMut<Self>) {
        for projectile in &mut node.active {
            let speed = rand::gen_range(projectile.speed * Self::SPEED_VARIANCE_MIN, projectile.speed * Self::SPEED_VARIANCE_MAX);
            projectile.position += projectile.direction * speed;
        }

        node.active.retain(|projectile| {
            // FIXME: This will allow damage from a projectile that has already hit its ttl in last update
            if projectile.lived >= projectile.ttl {
                return false;
            }
            let collider = Collider::circle(0.0, 0.0, projectile.size / 2.0).offset(projectile.position);
            'outer: for mut other_actor in scene::find_nodes_by_type::<Actor>() {
                if let Some(other_collider) = other_actor.body.get_offset_collider() {
                    if collider.overlaps(&other_collider) {
                        if projectile.actor_id != other_actor.id {
                            for faction in &projectile.factions {
                                if other_actor.factions.contains(&faction) {
                                    continue 'outer;
                                }
                            }
                            other_actor.take_damage(&projectile.actor_id, projectile.damage);
                            return false;
                        }
                    }
                }
            }
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            if game_state.map.solid_at_collider(collider, false) {
                return false;
            }
            return true;
        });
    }

    fn draw(node: RefMut<Self>) {
        let viewport = get_global::<Viewport>();
        for projectile in &node.active {
            if viewport.contains(projectile.position) {
                if projectile.is_sphere {
                    draw_circle(
                        projectile.position.x,
                        projectile.position.y,
                        projectile.size / 2.0,
                        projectile.color,
                    );
                } else {
                    let begin = projectile.position - projectile.direction * projectile.size * 2.0;
                    draw_line(
                        begin.x,
                        begin.y,
                        projectile.position.x,
                        projectile.position.y,
                        projectile.size,
                        projectile.color,
                    );
                }
            }
        }
    }
}
