use macroquad::prelude::*;

#[derive(Clone)]
pub struct ActorStats {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub willpower: u32,
    pub perception: u32,
    pub charisma: u32,
    pub current_health: f32,
    pub max_health: f32,
    pub current_stamina: f32,
    pub max_stamina: f32,
    pub current_energy: f32,
    pub max_energy: f32,
    pub health_regen: f32,
    pub stamina_regen: f32,
    pub energy_regen: f32,
    pub carry_capacity: f32,
    pub move_speed: f32,
    pub is_static: bool,
}

impl ActorStats {
    const VITALS_CAN_OVERFLOW: bool = false;

    pub fn new(
        strength: u32,
        dexterity: u32,
        constitution: u32,
        intelligence: u32,
        willpower: u32,
        perception: u32,
        charisma: u32,
    ) -> Self {
        let mut stats = ActorStats {
            strength,
            dexterity,
            constitution,
            intelligence,
            willpower,
            perception,
            charisma,
            is_static: false,
            ..Default::default()
        };
        stats.update_derived(true);
        stats
    }

    pub fn new_static(
        current_health: f32,
        max_health: f32,
        current_energy: f32,
        max_stamina: f32,
        current_stamina: f32,
        max_energy: f32,
        carry_capacity: f32,
        move_speed: f32,
    ) -> Self {
        ActorStats {
            current_health,
            max_health,
            current_stamina,
            max_stamina,
            current_energy,
            max_energy,
            carry_capacity,
            move_speed,
            is_static: true,
            ..Default::default()
        }
    }

    pub fn update_derived(&mut self, max_vitals: bool) {
        if !self.is_static {
            self.max_health = (self.constitution + self.strength / 4 + self.willpower / 4) as f32 * 100.0;
            self.max_stamina = (self.constitution + self.dexterity / 4 + self.willpower / 4) as f32 * 100.0;
            self.max_energy = (self.willpower + self.constitution / 2) as f32 * 100.0;
            if max_vitals {
                self.current_health = self.max_health;
                self.current_stamina = self.max_stamina;
                self.current_energy = self.max_energy;
            }
            self.health_regen = (self.constitution + self.strength / 4 + self.willpower / 4) as f32 * 0.1;
            self.stamina_regen = (self.constitution + self.dexterity / 4 + self.willpower / 4) as f32 * 8.0;
            self.energy_regen = (self.willpower + self.constitution / 2) as f32 * 0.5;
            self.move_speed = (self.dexterity + self.strength / 4 + self.willpower / 4) as f32 * 0.1;
            self.carry_capacity = (self.strength + self.constitution / 4 + self.willpower / 4) as f32 * 50.0;
        }
        let dt = get_frame_time();
        if self.current_health < self.max_health {
            self.current_health += self.health_regen * dt;
            if self.current_health > self.max_health {
                self.current_health = self.max_health;
            }
        }
        if self.current_stamina < self.max_stamina {
            self.current_stamina += self.stamina_regen * dt;
            if self.current_stamina > self.max_stamina {
                self.current_stamina = self.max_stamina;
            }
        }
        if self.current_energy < self.max_energy {
            self.current_energy += self.energy_regen * dt;
            if self.current_energy > self.max_energy {
                self.current_energy = self.max_energy;
            }
        }
        if !Self::VITALS_CAN_OVERFLOW {
            if self.current_health > self.max_health {
                self.current_health = self.max_health;
            }
            if self.current_stamina > self.max_stamina {
                self.current_stamina = self.max_stamina;
            }
            if self.current_energy > self.max_energy {
                self.current_energy = self.max_energy;
            }
        }
    }
}

impl Default for ActorStats {
    fn default() -> Self {
        ActorStats {
            strength: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            willpower: 0,
            perception: 0,
            charisma: 0,
            current_health: 1.0,
            max_health: 1.0,
            current_stamina: 0.0,
            max_stamina: 0.0,
            current_energy: 0.0,
            max_energy: 0.0,
            health_regen: 0.0,
            stamina_regen: 0.0,
            energy_regen: 0.0,
            carry_capacity: 0.0,
            move_speed: 0.0,
            is_static: true,
        }
    }
}
