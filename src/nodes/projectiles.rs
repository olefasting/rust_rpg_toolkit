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

use crate::{
    nodes::Actor,
    physics::Collider,
};

pub struct Projectile {
    actor_id: String,
    damage: f32,
    color: Color,
    size: f32,
    position: Vec2,
    direction: Vec2,
    speed: f32,
    lived: f32,
    ttl: f32,
}

impl Projectile {
    pub fn new(actor_id: &str, damage: f32, color: Color, size: f32, position: Vec2, direction: Vec2, speed: f32, ttl: f32) -> Self {
        Projectile {
            actor_id: actor_id.to_string(),
            damage,
            color,
            size,
            position,
            direction,
            speed,
            lived: 0.0,
            ttl,
        }
    }
}

pub struct Projectiles {
    active: Vec<Projectile>,
}

impl Projectiles {
    const SPEED_VARIANCE_MIN: f32 = 0.9;
    const SPEED_VARIANCE_MAX: f32 = 1.1;

    pub fn new() -> Self {
        Projectiles {
            active: Vec::new(),
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }

    pub fn spawn(&mut self, actor_id: &str, damage: f32, color: Color, size: f32, position: Vec2, target: Vec2, speed: f32, spread: f32, ttl: f32) {
        assert!(ttl > 0.0, "Projectile TTL must be a positive float and not 0.0");
        let spread_target = vec2(
          rand::gen_range(target.x - spread, target.x + spread),
            rand::gen_range(target.y - spread, target.y + spread),
        );
        self.active.push(Projectile::new(actor_id, damage, color, size, position, spread_target.sub(position).normalize_or_zero(), speed, ttl));
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
            for mut actor in scene::find_nodes_by_type::<Actor>() {
                if let Some(collider) = actor.body.offset_collider() {
                    if Collider::circle(0.0, 0.0, projectile.size / 2.0).offset(projectile.position).overlaps(&collider) {
                        if projectile.actor_id != actor.id {
                            actor.take_damage(projectile.damage);
                            return false;
                        }
                    }
                }
            }
            return true;
        });
    }

    fn draw(node: RefMut<Self>) {
        for projectile in &node.active {
            draw_circle(
                projectile.position.x,
                projectile.position.y,
                projectile.size / 2.0,
                projectile.color,
            )
        }
    }
}
