use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EquipmentSlot {
    None,
    MainHand,
    OffHand,
    BothHands,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EquippedItems {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_hand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub off_hand: Option<String>,
}
