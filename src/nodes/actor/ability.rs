use macroquad::prelude::*;

pub type ActorAbilityFunc = fn (actor_id: &str, origin: Vec2, target: Vec2);

#[derive(Clone)]
pub struct ActorAbility {
    actor_id: String,
    pub cooldown_timer: f32,
    cooldown: f32,
    func: ActorAbilityFunc,
}

impl ActorAbility {
    pub fn new(actor_id: &str, cooldown: f32, func: ActorAbilityFunc) -> Self {
        ActorAbility {
            actor_id: actor_id.to_string(),
            cooldown_timer: cooldown,
            cooldown,
            func,
        }
    }

    pub fn activate(&mut self, origin: Vec2, target: Vec2) {
        if self.cooldown_timer >= self.cooldown {
            (self.func)(&self.actor_id, origin, target);
            self.cooldown_timer = 0.0;
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.cooldown_timer += dt;
    }
}
