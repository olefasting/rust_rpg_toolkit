#[derive(Clone)]
pub enum ItemSlot {
    MainHand,
    OffHand,
    BothHands,
}

pub type ItemAction = fn ();

#[derive(Clone)]
pub struct Item {
    name: String,
    slot: ItemSlot,
    action: Option<ItemAction>,
}
