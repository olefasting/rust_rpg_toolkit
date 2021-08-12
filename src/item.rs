use crate::nodes::actor::{
    ActorAbility,
    ActorAbilityFunc,
};
use crate::generate_id;

#[derive(Clone)]
pub struct ItemParams {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub description: String,
    pub weight: f32,
    pub cooldown: f32,
    pub action: Option<ActorAbilityFunc>,
}

impl Default for ItemParams {
    fn default() -> Self {
        ItemParams {
            id: generate_id(),
            kind: Item::TRINKET_KIND.to_string(),
            name: "Unnamed Item".to_string(),
            description: "".to_string(),
            weight: 0.1,
            cooldown: 0.0,
            action: None,
        }
    }
}

#[derive(Clone)]
pub struct Item {
    id: String,
    kind: String,
    name: String,
    description: String,
    weight: f32,
    cooldown: f32,
    action: Option<ActorAbilityFunc>,
}

impl Item {
    pub const TRINKET_KIND: &'static str = "trinket";
    pub const ONE_HANDED_WEAPON_KIND: &'static str = "one_handed_weapon";
    pub const TWO_HANDED_WEAPON_KIND: &'static str = "two_handed_weapon";

    pub fn new(params: ItemParams) -> Self {
        Item {
            id: params.id,
            kind: params.kind,
            name: params.name,
            description: params.description,
            weight: params.weight,
            cooldown: params.cooldown,
            action: params.action,
        }
    }

    pub fn to_item_params(&self) -> ItemParams {
        ItemParams {
            id: self.id.clone(),
            kind: self.kind.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            weight: self.weight,
            cooldown: self.cooldown,
            action: self.action,
        }
    }

    pub fn to_actor_ability(&self, health_cost: f32, stamina_cost: f32, energy_cost: f32) -> Option<ActorAbility> {
        if let Some(action) = self.action {
            Some(ActorAbility::new(health_cost, stamina_cost, energy_cost, self.cooldown, action))
        } else {
            None
        }
    }
}
