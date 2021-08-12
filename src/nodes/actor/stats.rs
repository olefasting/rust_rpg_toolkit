use macroquad::prelude::*;

#[derive(Clone)]
pub struct ActorStats {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub current_health: f32,
    pub max_health: f32,
    pub move_speed: f32,
    pub is_static: bool,
}

impl ActorStats {
    const HEALTH_CAN_OVERFLOW: bool = false;

    pub fn new(strength: u32, dexterity: u32, constitution: u32) -> Self {
        let mut stats = ActorStats {
            strength,
            dexterity,
            constitution,
            is_static: false,
            ..Default::default()
        };
        stats.update(true);
        stats
    }

    pub fn new_static(current_health: f32, max_health: f32, move_speed: f32) -> Self {
        ActorStats {
            current_health,
            max_health,
            move_speed,
            is_static: true,
            ..Default::default()
        }
    }

    pub fn update(&mut self, full_health: bool) {
        if !self.is_static {
            self.max_health = (self.constitution + self.strength / 2) as f32 * 100.0;
            self.move_speed = (self.dexterity + self.strength / 2) as f32 * 0.1;
            if full_health {
                self.current_health = self.max_health;
            }
        }
        if !Self::HEALTH_CAN_OVERFLOW && self.current_health > self.max_health {
            self.current_health = self.max_health;
        }
    }
}

impl Default for ActorStats {
    fn default() -> Self {
        ActorStats {
            strength: 0,
            dexterity: 0,
            constitution: 0,
            current_health: 0.0,
            max_health: 0.0,
            move_speed: 0.0,
            is_static: true,
        }
    }
}
