use crate::nodes::actor::ActorAbility;

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
    action: Option<ActorAbility>,
}
