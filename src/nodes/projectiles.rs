use std::ops::{
    Sub,
    Mul,
};

use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
            RefMut,
        },
        collections::storage,
    },
    prelude::*,
};

use crate::{
    nodes::{
        Actor,
        GameState,
    },
    physics::Collider,
    render::{
        Viewport,
        SpriteAnimationParams,
        SpriteAnimationPlayer,
    },
};
use crate::map::MapCollisionKind;

pub enum ProjectileKind {
    Bullet,
    Beam,
    EnergySphere,
}

pub struct Projectile {
    actor_id: String,
    factions: Vec<String>,
    kind: ProjectileKind,
    damage: f32,
    color: Color,
    size: f32,
    position: Vec2,
    direction: Vec2,
    speed: f32,
    lived: f32,
    ttl: f32,
    sprite_animation: Option<SpriteAnimationPlayer>,
}

impl Projectile {
    pub fn new(
        actor_id: &str,
        factions: &[String],
        kind: ProjectileKind,
        damage: f32, color: Color,
        size: f32,
        position: Vec2,
        direction: Vec2,
        speed: f32,
        ttl: f32,
        sprite_animation_params: Option<SpriteAnimationParams>,
    ) -> Self {
        let sprite_animation = match sprite_animation_params {
            Some(params) => {
                let mut anim = SpriteAnimationPlayer::new(params);
                anim.play();
                Some(anim)
            }
            None => None,
        };
        Projectile {
            actor_id: actor_id.to_string(),
            factions: factions.to_vec(),
            kind,
            damage,
            color,
            size,
            position,
            direction,
            speed,
            lived: 0.0,
            ttl,
            sprite_animation,
        }
    }
}

pub struct Projectiles {
    active: Vec<Projectile>,
}

impl Projectiles {
    const MIN_PROJECTILE_SPEED: f32 = 1.0;
    const MAX_PROJECTILE_SPEED: f32 = 200.0;

    const SPEED_VARIANCE_MIN: f32 = 0.9;
    const SPEED_VARIANCE_MAX: f32 = 1.1;

    const SPREAD_CALCULATION_DISTANCE: f32 = 100.0;

    const PROJECTILE_LENGTH_FACTOR_MIN: f32 = 6.0;
    const PROJECTILE_LENGTH_FACTOR_MAX: f32 = 20.0;

    const BEAM_LENGTH_FACTOR_MIN: f32 = 2.0;
    const BEAM_LENGTH_FACTOR_MAX: f32 = 6.0;

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
        kind: ProjectileKind,
        damage: f32,
        color: Color,
        size: f32,
        position: Vec2,
        target: Vec2,
        speed: f32,
        spread: f32,
        ttl: f32,
        sprite_animation_params: Option<SpriteAnimationParams>,
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
            kind,
            damage,
            color,
            size,
            position,
            direction,
            speed.clamp(Self::MIN_PROJECTILE_SPEED, Self::MAX_PROJECTILE_SPEED),
            ttl,
            sprite_animation_params,
        ));
    }
}

impl Node for Projectiles {
    fn update(mut node: RefMut<Self>) {
        for projectile in &mut node.active {
            projectile.lived += get_frame_time();
            if let Some(animation) = projectile.sprite_animation.as_mut() {
                animation.update();
            }
        }
    }

    fn fixed_update(mut node: RefMut<Self>) {
        for projectile in &mut node.active {
            let speed = rand::gen_range(projectile.speed * Self::SPEED_VARIANCE_MIN, projectile.speed * Self::SPEED_VARIANCE_MAX);
            projectile.position += projectile.direction * speed;
        }

        node.active.retain(|projectile| {
            // FIXME: This will allow damage from a projectile that has already hit its ttl in last update
            if projectile.lived > projectile.ttl {
                return false;
            }
            let collider = Collider::circle(0.0, 0.0, projectile.size / 2.0).offset(projectile.position);
            'outer: for mut other_actor in scene::find_nodes_by_type::<Actor>() {
                if let Some(other_collider) = other_actor.body.get_offset_collider() {
                    if collider.overlaps(other_collider) {
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
            for (_, kind) in game_state.map.get_collisions(collider) {
                if kind == MapCollisionKind::Solid {
                    return false;
                }
            }
            return true;
        });
    }

    fn draw(mut node: RefMut<Self>) {
        let viewport = storage::get::<Viewport>();
        let frustum = viewport.get_frustum();
        for projectile in &mut node.active {
            if frustum.contains(projectile.position) {
                if let Some(animation) = projectile.sprite_animation.as_mut() {
                    let rotation = projectile.position.normalize().angle_between(projectile.direction) + 0.75; // WHY??
                    animation.draw(projectile.position, rotation);
                } else {
                    match projectile.kind {
                        ProjectileKind::Bullet => {
                            let begin = projectile
                                .position.sub(projectile.direction.mul(
                                projectile.size * rand::gen_range(
                                    Self::PROJECTILE_LENGTH_FACTOR_MIN,
                                    Self::PROJECTILE_LENGTH_FACTOR_MAX,
                                )));
                            draw_line(
                                begin.x,
                                begin.y,
                                projectile.position.x,
                                projectile.position.y,
                                projectile.size,
                                projectile.color,
                            );
                        }
                        ProjectileKind::Beam => {
                            let begin = projectile
                                .position.sub(projectile.direction.mul(
                                projectile.size * rand::gen_range(
                                    Self::BEAM_LENGTH_FACTOR_MIN,
                                    Self::BEAM_LENGTH_FACTOR_MAX,
                                )));
                            if projectile.size > 2.0 {
                                draw_circle(
                                    begin.x,
                                    begin.y,
                                    projectile.size / 2.0,
                                    projectile.color,
                                );
                                draw_circle(
                                    projectile.position.x,
                                    projectile.position.y,
                                    projectile.size / 2.0,
                                    projectile.color,
                                );
                            }
                            draw_line(
                                begin.x,
                                begin.y,
                                projectile.position.x,
                                projectile.position.y,
                                projectile.size,
                                projectile.color,
                            );
                        }
                        ProjectileKind::EnergySphere => draw_circle(
                            projectile.position.x,
                            projectile.position.y,
                            projectile.size / 2.0,
                            projectile.color,
                        ),
                    }
                }
            }
        }
    }
}
