use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemKind {
    OneHandedWeapon,
    TwoHandedWeapon,
    Misc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemParams {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default, with = "json::opt_vec2", skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec2>,
    pub kind: ItemKind,
    pub weight: f32,
    #[serde(default, rename = "ability", skip_serializing_if = "Option::is_none")]
    pub ability_id: Option<String>,
    pub sprite: Sprite,
    #[serde(default)]
    pub is_quest_item: bool,
}

impl Default for ItemParams {
    fn default() -> Self {
        ItemParams {
            id: "".to_string(),
            name: "Unnamed Item".to_string(),
            description: "".to_string(),
            position: Default::default(),
            kind: ItemKind::Misc,
            weight: 0.1,
            ability_id: None,
            sprite: Default::default(),
            is_quest_item: false,
        }
    }
}

#[derive(Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub position: Vec2,
    pub kind: ItemKind,
    pub weight: f32,
    pub is_quest_item: bool,
    ability: Option<AbilityParams>,
    sprite: Sprite,
}

impl Item {
    pub fn new(params: ItemParams) -> Self {
        let resources = get_resources();
        let ability = if let Some(ability_id) = params.ability_id {
            Some(resources.abilities.get(&ability_id).cloned().unwrap())
        } else {
            None
        };

        Item {
            id: params.id,
            position: params.position.unwrap_or_default(),
            kind: params.kind,
            name: params.name,
            description: params.description,
            weight: params.weight,
            is_quest_item: params.is_quest_item,
            ability,
            sprite: params.sprite,
        }
    }

    pub fn add_node(params: ItemParams) -> Handle<Self> {
        scene::add_node(Self::new(params))
    }

    pub fn to_params(&self) -> ItemParams {
        let ability_id = if let Some(ability) = &self.ability {
            Some(ability.id.clone())
        } else {
            None
        };

        ItemParams {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            position: Some(self.position),
            kind: self.kind.clone(),
            weight: self.weight,
            ability_id,
            sprite: self.sprite.clone(),
            is_quest_item: self.is_quest_item
        }
    }
}

impl BufferedDraw for Item {
    fn buffered_draw(&mut self) {
        self.sprite.draw(self.position, 0.0);
    }

    fn get_z_index(&self) -> f32 {
        self.position.y
    }

    fn get_bounds(&self) -> Bounds {
        Bounds::Point(self.position)
    }
}

impl Node for Item {
    fn draw(node: RefMut<Self>) {
        let mut draw_buffer = scene::find_node_by_type::<DrawBuffer<Self>>().unwrap();
        draw_buffer.buffered.push(node.handle());
    }
}

#[derive(Debug, Clone)]
pub struct Credits {
    pub position: Vec2,
    pub amount: u32,
    pub sprite: Sprite,
}

impl Credits {
    pub fn new(position: Vec2, amount: u32) -> Self {
        Credits {
            position,
            amount,
            sprite: Sprite {
                texture_id: "credits".to_string(),
                tile_size: uvec2(8, 8),
                ..Default::default()
            },
        }
    }

    pub fn add_node(position: Vec2, amount: u32) -> Handle<Self> {
        scene::add_node(Self::new(position, amount))
    }
}

impl BufferedDraw for Credits {
    fn buffered_draw(&mut self) {
        self.sprite.draw(self.position, 0.0);
    }

    fn get_z_index(&self) -> f32 {
        self.position.y
    }

    fn get_bounds(&self) -> Bounds {
        Bounds::Point(self.position)
    }
}

impl Node for Credits {
    fn ready(node: RefMut<Self>) {
        let mut draw_buffer = scene::find_node_by_type::<DrawBuffer<Credits>>().unwrap();
        draw_buffer.buffered.push(node.handle());
    }
}
