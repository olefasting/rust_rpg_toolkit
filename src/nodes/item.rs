use macroquad::{
    experimental::{
        scene::{
            Node,
            RefMut,
        }
    },
    prelude::*
};

use crate::{
    generate_id,
    render::{
        SpriteAnimationPlayer,
        SpriteParams,
    },
};

#[derive(Clone)]
pub struct ItemData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub position: Vec2,
    pub sprite_params: SpriteParams,
}

impl Default for ItemData {
    fn default() -> Self {
        ItemData {
            id: generate_id(),
            name: "Unnamed Item".to_string(),
            description: "".to_string(),
            position: Vec2::ZERO,
            sprite_params: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub position: Vec2,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    should_draw: bool,
    sprite: SpriteAnimationPlayer,
}

impl Item {
    pub fn new(
        data: ItemData,
    ) -> Self {
        Item {
            id: data.id,
            name: data.name,
            description: data.description,
            position: data.position,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            should_draw: true,
            sprite: SpriteAnimationPlayer::new(data.sprite_params),
        }
    }

    pub fn to_item_data(&self) -> ItemData {
        ItemData {
            id: self.id.to_string(),
            name: self.name.to_string(),
            description: self.description.to_string(),
            position: self.position,
            sprite_params: self.sprite.to_sprite_params(),
        }
    }
}

impl Node for Item {
    fn ready(_node: RefMut<Self>) {
    }

    fn update(_node: RefMut<Self>) {
    }

    fn draw(mut node: RefMut<Self>) {
        if node.should_draw {
            let (position, rotation, flip_x, flip_y)
                = (node.position, node.rotation, node.flip_x, node.flip_y);
            node.sprite.draw(position, rotation, flip_x, flip_y);
        }
    }
}
