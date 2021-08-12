use crate::nodes::actor::{
    ActorAbility,
    ActorAbilityFunc,
};

#[derive(Clone)]
pub enum ItemSlot {
    MainHand,
    OffHand,
    BothHands,
}

#[derive(Clone)]
pub struct Item {
    name: String,
    slot: ItemSlot,
    cooldown: Option<f32>,
    action: Option<ActorAbilityFunc>,
}

impl Item {
    pub fn to_actor_ability(&self, health_cost: f32, stamina_cost: f32, energy_cost: f32) -> Option<ActorAbility> {
        if let Some(action) = self.action {
            Some(ActorAbility::new(health_cost, stamina_cost, energy_cost, self.cooldown.unwrap_or(0.0), action))
        } else {
            None
        }
    }
}
