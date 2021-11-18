use std::ops::{Mul, Sub};

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectileKind {
    Bullet,
    Beam,
}

pub struct ProjectileParams {
    pub kind: ProjectileKind,
    pub effects: Vec<Effect>,
    pub color: Color,
    pub size: f32,
    pub origin: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub range: f32,
    pub on_hit_sound_effect: Option<Sound>,
}

pub struct Projectile {
    actor_id: String,
    actor: Handle<Actor>,
    factions: Vec<String>,
    kind: ProjectileKind,
    effects: Vec<Effect>,
    color: Color,
    size: f32,
    origin: Vec2,
    position: Vec2,
    direction: Vec2,
    speed: f32,
    distance_traveled: f32,
    range: f32,
    on_hit_sound_effect: Option<Sound>,
}

impl Projectile {
    pub fn new(
        actor_id: &str,
        actor: Handle<Actor>,
        factions: &[String],
        params: ProjectileParams,
    ) -> Self {
        Projectile {
            actor_id: actor_id.to_string(),
            actor,
            factions: factions.to_vec(),
            kind: params.kind,
            effects: params.effects,
            color: params.color,
            size: params.size,
            origin: params.origin,
            position: params.origin,
            direction: params.direction,
            speed: params.speed,
            distance_traveled: 0.0,
            range: params.range,
            on_hit_sound_effect: params.on_hit_sound_effect,
        }
    }
}

#[derive(Default)]
pub struct Projectiles {
    active: Vec<Projectile>,
}

impl Projectiles {
    pub const DEFAULT_PROJECTILE_COLOR: Color = color::YELLOW;
    pub const DEFAULT_PROJECTILE_SIZE: f32 = 1.0;

    pub const DEFAULT_BEAM_COLOR: Color = color::RED;
    pub const DEFAULT_BEAM_SIZE: f32 = 2.0;

    const MIN_PROJECTILE_SPEED: f32 = 1.0;
    const MAX_PROJECTILE_SPEED: f32 = 200.0;

    const SPEED_VARIANCE_MIN: f32 = 0.9;
    const SPEED_VARIANCE_MAX: f32 = 1.1;

    const SPREAD_CALCULATION_DISTANCE: f32 = 100.0;

    const PROJECTILE_LENGTH_FACTOR_MIN: f32 = 3.0;
    const PROJECTILE_LENGTH_FACTOR_MAX: f32 = 15.0;

    const BEAM_LENGTH_FACTOR_MIN: f32 = 2.0;
    const BEAM_LENGTH_FACTOR_MAX: f32 = 6.0;

    pub fn new() -> Self {
        Projectiles { active: Vec::new() }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }

    pub fn spawn(
        &mut self,
        actor_id: &str,
        actor: Handle<Actor>,
        factions: &[String],
        spread: f32,
        mut params: ProjectileParams,
    ) {
        let spread_target = params.direction * Self::SPREAD_CALCULATION_DISTANCE;
        params.direction = vec2(
            rand::gen_range(spread_target.x - spread, spread_target.x + spread),
            rand::gen_range(spread_target.y - spread, spread_target.y + spread),
        )
        .normalize_or_zero();

        params.speed = rand::gen_range(
            params.speed * Self::SPEED_VARIANCE_MIN,
            params.speed * Self::SPEED_VARIANCE_MAX,
        )
        .clamp(Self::MIN_PROJECTILE_SPEED, Self::MAX_PROJECTILE_SPEED);

        self.active
            .push(Projectile::new(actor_id, actor, factions, params));
    }
}

impl Node for Projectiles {
    fn fixed_update(mut node: RefMut<Self>) {
        for projectile in &mut node.active {
            let distance = (projectile.direction * projectile.speed) * 50.0 * get_frame_time();
            projectile.position += distance;
            projectile.distance_traveled += distance.length();
        }

        node.active.retain(|projectile| {
            if projectile.distance_traveled > projectile.range {
                return false;
            }

            let collider =
                Collider::circle(0.0, 0.0, projectile.size / 2.0).with_offset(projectile.position);
            'outer: for mut other_actor in scene::find_nodes_by_type::<Actor>() {
                if let Some(other_collider) = other_actor.body.get_offset_collider() {
                    if collider.overlaps(other_collider) {
                        let mut is_hit = false;

                        for effect in &projectile.effects {
                            if other_actor.apply_effect(
                                &projectile.actor_id,
                                projectile.actor,
                                &projectile.factions,
                                effect.clone(),
                            ) {
                                is_hit = true;
                            } else {
                                continue 'outer;
                            }
                        }

                        if is_hit {
                            if let Some(sound_effect) = projectile.on_hit_sound_effect {
                                play_sound(sound_effect, false);
                            }

                            return false;
                        }
                    }
                }
            }

            let map = storage::get::<Map>();
            for (_, kind) in map.get_collisions(collider) {
                if kind == CollisionKind::Solid {
                    if let Some(sound_effect) = projectile.on_hit_sound_effect {
                        play_sound(sound_effect, false);
                    }
                    return false;
                }
            }
            true
        });
    }

    fn draw(mut node: RefMut<Self>) {
        let viewport = storage::get::<Viewport>();
        let frustum = viewport.get_frustum();
        for projectile in &mut node.active {
            if frustum.contains(projectile.position) {
                match projectile.kind {
                    ProjectileKind::Bullet => {
                        let mut begin = projectile.position.sub(projectile.direction.mul(
                            projectile.size
                                * rand::gen_range(
                                    Self::PROJECTILE_LENGTH_FACTOR_MIN,
                                    Self::PROJECTILE_LENGTH_FACTOR_MAX,
                                ),
                        ));

                        if begin.distance(projectile.position) > projectile.distance_traveled {
                            begin = projectile.origin;
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
                    ProjectileKind::Beam => {
                        let mut begin = projectile.position.sub(projectile.direction.mul(
                            projectile.size
                                * rand::gen_range(
                                    Self::BEAM_LENGTH_FACTOR_MIN,
                                    Self::BEAM_LENGTH_FACTOR_MAX,
                                ),
                        ));

                        if begin.distance(projectile.position) > projectile.distance_traveled {
                            begin = projectile.origin;
                        }

                        if projectile.size > 2.0 {
                            draw_circle(begin.x, begin.y, projectile.size / 2.0, projectile.color);
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
                }
            }
        }
    }
}
