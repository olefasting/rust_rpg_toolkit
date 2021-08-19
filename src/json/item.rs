use serde::{
    Serialize,
    Deserialize,
};

use crate::json::{
    Sprite,
    Vec2Def,
    AbilityParams,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemPrototype {
    pub id: String,
    pub name: String,
    pub description: String,
    pub kind: String,
    pub weight: f32,
    pub ability: AbilityParams,
    pub sprite: Sprite,
}
//
// impl From<crate::nodes::item::ItemPrototype> for ItemPrototype {
//     fn from(other: crate::nodes::item::ItemPrototype) -> Self {
//         ItemPrototype {
//             id: other.id,
//             name: other.name,
//             description: other.description,
//             kind: other.kind,
//             weight: other.weight,
//             ability: AbilityParams::from(other.ability),
//             sprite: Sprite::from(other.sprite),
//         }
//     }
// }
//
// impl From<ItemPrototype> for crate::nodes::item::ItemPrototype {
//     fn from(other: ItemPrototype) -> Self {
//         crate::nodes::item::ItemPrototype {
//             id: other.id.clone(),
//             name: other.name,
//             description: other.description,
//             kind: other.kind,
//             weight: other.weight,
//             ability: crate::ability::AbilityParams::from(other.ability),
//             sprite: crate::render::Sprite::from(other.sprite),
//         }
//     }
// }

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemParams {
    pub name: String,
    pub description: String,
    pub position: Option<Vec2Def>,
    pub kind: String,
    pub weight: f32,
    pub ability: AbilityParams,
    pub sprite: Sprite,
}

impl From<crate::nodes::item::ItemParams> for ItemParams {
    fn from(other: crate::nodes::item::ItemParams) -> Self {
        let position = if let Some(position) = other.position {
            Some(Vec2Def::from(position))
        } else {
            None
        };
        ItemParams {
            name: other.name,
            description: other.description,
            position,
            kind: other.kind,
            weight: other.weight,
            ability: AbilityParams::from(other.ability),
            sprite: Sprite::from(other.sprite),
        }
    }
}

impl From<ItemParams> for crate::nodes::item::ItemParams {
    fn from(other: ItemParams) -> Self {
        let position = if let Some(position) = other.position {
            Some(macroquad::prelude::Vec2::from(position))
        } else {
            None
        };
        crate::nodes::item::ItemParams {
            prototype_id: None,
            name: other.name,
            description: other.description,
            position,
            kind: other.kind,
            weight: other.weight,
            ability: crate::AbilityParams::from(other.ability),
            sprite: crate::render::Sprite::from(other.sprite),
        }
    }
}
