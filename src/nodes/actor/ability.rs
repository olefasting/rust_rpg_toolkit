use macroquad::{
    prelude::*
};

use crate::{Actor, ActionFunc};

#[derive(Clone)]
pub struct ActorAbility {
    pub health_cost: f32,
    pub stamina_cost: f32,
    pub energy_cost: f32,
    pub cooldown_timer: f32,
    cooldown: f32,
    damage: f32,
    func: ActionFunc,
}

impl ActorAbility {
    pub fn new(
        health_cost: f32,
        stamina_cost: f32,
        energy_cost: f32,
        cooldown: f32,
        damage: f32,
        func: ActionFunc,
    ) -> Self {
        ActorAbility {
            health_cost,
            stamina_cost,
            energy_cost,
            cooldown_timer: cooldown,
            cooldown,
            damage,
            func,
        }
    }

    pub fn activate(&mut self, actor: &mut Actor, origin: Vec2, target: Vec2) {
        if self.health_cost > 0.0 && actor.stats.current_health < self.health_cost {
            return;
        }
        if self.stamina_cost > 0.0 && actor.stats.current_stamina < self.stamina_cost {
            return;
        }
        if self.energy_cost > 0.0 && actor.stats.current_energy < self.energy_cost {
            return;
        }
        if self.cooldown_timer >= self.cooldown {
            actor.stats.current_health -= self.health_cost;
            actor.stats.current_stamina -= self.stamina_cost;
            actor.stats.current_energy -= self.energy_cost;
            (self.func)(&actor.id, origin, target, self.damage);
            self.cooldown_timer = 0.0;
        }
    }

    pub fn update(&mut self) {
        self.cooldown_timer += get_frame_time();
    }
}
