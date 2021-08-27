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
    color,
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    nodes::{
        Actor,
        GameState,
    },
    physics::Collider,
    render::{
        Viewport,
    },
};
use crate::map::MapCollisionKind;
use crate::ability::Effect;
use crate::resources::Resources;
use macroquad::audio::play_sound_once;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProjectileKind {
    #[serde(rename = "bullet")]
    Bullet,
    #[serde(rename = "beam")]
    Beam,
}

pub struct Projectile {
    actor_id: String,
    factions: Vec<String>,
    kind: ProjectileKind,
    effects: Vec<Effect>,
    color: Color,
    size: f32,
    position: Vec2,
    direction: Vec2,
    speed: f32,
    distance_traveled: f32,
    range: f32,
    on_hit_sound_effect_id: Option<String>,
}

impl Projectile {
    pub fn new(
        actor_id: &str,
        factions: &[String],
        kind: ProjectileKind,
        effects: Vec<Effect>,
        color: Color,
        size: f32,
        position: Vec2,
        direction: Vec2,
        speed: f32,
        range: f32,
        on_hit_sound_effect_id: Option<String>,
    ) -> Self {
        Projectile {
            actor_id: actor_id.to_string(),
            factions: factions.to_vec(),
            kind,
            effects,
            color,
            size,
            position,
            direction,
            speed,
            distance_traveled: 0.0,
            range,
            on_hit_sound_effect_id,
        }
    }

    pub fn play_on_hit_sound_effect(&self) {
        if let Some(sound_effect_id) = &self.on_hit_sound_effect_id {
            let resources = storage::get::<Resources>();
            let sound_effect = resources.sound_effects.get(sound_effect_id).cloned().unwrap();
            play_sound_once(sound_effect);
        }
    }
}

pub struct Projectiles {
    active: Vec<Projectile>,
}

impl Projectiles {
    const DEFAULT_PROJECTILE_COLOR: Color = color::YELLOW;
    const DEFAULT_PROJECTILE_SIZE: f32 = 1.0;

    const DEFAULT_BEAM_COLOR: Color = color::RED;
    const DEFAULT_BEAM_SIZE: f32 = 2.0;

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
        effects: &[Effect],
        color_override: Option<Color>,
        size_override: Option<f32>,
        position: Vec2,
        target: Vec2,
        speed: f32,
        spread: f32,
        range: f32,
        on_hit_sound_effect_id: Option<String>,
    ) {
        let spread_target = target.sub(position).normalize_or_zero() * Self::SPREAD_CALCULATION_DISTANCE;
        let direction = vec2(
            rand::gen_range(spread_target.x - spread, spread_target.x + spread),
            rand::gen_range(spread_target.y - spread, spread_target.y + spread),
        ).normalize_or_zero();

        let color = if let Some(color) = color_override {
            color
        } else {
            match kind {
                ProjectileKind::Bullet => Self::DEFAULT_PROJECTILE_COLOR,
                ProjectileKind::Beam => Self::DEFAULT_BEAM_COLOR,
            }
        };
        let size = if let Some(size) = size_override {
            size
        } else {
            match kind {
                ProjectileKind::Bullet => Self::DEFAULT_PROJECTILE_SIZE,
                ProjectileKind::Beam => Self::DEFAULT_BEAM_SIZE,
            }
        };

        let speed = rand::gen_range(speed * Self::SPEED_VARIANCE_MIN, speed * Self::SPEED_VARIANCE_MAX).clamp(Self::MIN_PROJECTILE_SPEED, Self::MAX_PROJECTILE_SPEED);

        self.active.push(Projectile::new(
            actor_id,
            factions,
            kind,
            effects.to_vec(),
            color,
            size,
            position,
            direction,
            speed,
            range,
            on_hit_sound_effect_id,
        ));
    }
}

impl Node for Projectiles {
    fn update(mut node: RefMut<Self>) {
        for projectile in &mut node.active {
            let distance = (projectile.direction * projectile.speed) * 50.0 * get_frame_time();
            projectile.position += distance;
            projectile.distance_traveled += distance.length();
        }

        node.active.retain(|projectile| {
            if projectile.distance_traveled > projectile.range {
                return false;
            }
            let collider = Collider::circle(0.0, 0.0, projectile.size / 2.0).offset(projectile.position);
            'outer: for mut other_actor in scene::find_nodes_by_type::<Actor>() {
                if let Some(other_collider) = other_actor.body.get_offset_collider() {
                    if collider.overlaps(other_collider) {
                        for effect in projectile.effects.clone() {
                            if other_actor.apply_effect(&projectile.actor_id, &projectile.factions, effect) {
                                projectile.play_on_hit_sound_effect();
                                return false;
                            } else {
                                continue 'outer;
                            }
                        }
                    }
                }
            }
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            for (_, kind) in game_state.map.get_collisions(collider) {
                if kind == MapCollisionKind::Solid {
                    projectile.play_on_hit_sound_effect();
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
                }
            }
        }
    }
}
