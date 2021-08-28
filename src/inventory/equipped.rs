use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EquipmentSlot {
    None,
    MainHand,
    OffHand,
    BothHands,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquippedItems {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_hand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub off_hand: Option<String>,
}

impl Default for EquippedItems {
    fn default() -> Self {
        EquippedItems {
            main_hand: None,
            off_hand: None,
        }
    }
}
