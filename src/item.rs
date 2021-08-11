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
    pub fn to_actor_ability(&self, actor_id: &str) -> Option<ActorAbility> {
        if let Some(action) = self.action {
            Some(ActorAbility::new(actor_id, self.cooldown.unwrap_or(0.0), action))
        } else {
            None
        }
    }
}
