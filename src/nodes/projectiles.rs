use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
        },
    },
    prelude::*,
};
use macroquad::experimental::scene::RefMut;
use std::ops::Sub;

pub struct Projectile {
    color: Color,
    size: f32,
    position: Vec2,
    target: Vec2,
    speed: f32,
    lived: f32,
    ttl: f32,
}

impl Projectile {
    pub fn new(color: Color, size: f32, position: Vec2, target: Vec2, speed: f32, ttl: f32) -> Self {
        Projectile {
            color,
            size,
            position,
            target,
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
    pub fn new() -> Self {
        Projectiles {
            active: Vec::new(),
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }

    pub fn spawn(&mut self, color: Color, size: f32, position: Vec2, target: Vec2, speed: f32, ttl: f32) {
        self.active.push(Projectile::new(color, size, position, target, speed, ttl));
    }
}

impl Node for Projectiles {
    fn update(mut node: RefMut<Self>) {
        for projectile in &mut node.active {
            projectile.lived += get_frame_time();
        }
    }

    fn fixed_update(mut node: RefMut<Self>) {
        node.active.retain(|projectile| (projectile.ttl == 0.0 || projectile.lived < projectile.ttl)
                && projectile.position.distance(projectile.target) > projectile.speed);

        for projectile in &mut node.active {
            let direction = projectile.target.sub(projectile.position).normalize_or_zero();
            projectile.position += direction * projectile.speed;
        }
    }

    fn draw(node: RefMut<Self>) {
        for projectile in &node.active {
            draw_circle(
                projectile.position.x - projectile.size,
                projectile.position.y - projectile.size,
                projectile.size / 2.0,
                projectile.color,
            )
        }
    }
}
