use macroquad::{
    experimental::scene::{
        Node,
        RefMut,
        Handle,
    },
    prelude::*,
};

use crate::nodes::Item;
use crate::get_global;
use crate::render::Viewport;

pub struct ItemDrawBuffer {
    buffer: Vec<Handle<Item>>,
}

impl ItemDrawBuffer {
    pub fn new() -> Self {
        ItemDrawBuffer {
            buffer: Vec::new(),
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }

    pub fn add_to_buffer(&mut self, handle: Handle<Item>) {
        self.buffer.push(handle);
    }
}

impl Node for ItemDrawBuffer {
    fn draw(mut node: RefMut<Self>) {
        node.buffer.retain(|handle| {
            let viewport = get_global::<Viewport>();
            if let Some(item) = scene::try_get_node(*handle) {
                viewport.contains(item.position)
            } else {
                false
            }
        });
        node.buffer.sort_by(|a, b| {
            if let Some(item_a) = scene::try_get_node(*a) {
                if let Some(item_b) = scene::try_get_node(*b) {
                    return item_a.position.y.partial_cmp(&item_b.position.y).unwrap();
                }
            }
            0.0.partial_cmp(&0.0).unwrap()
        });
        for handle in &node.buffer {
            if let Some(mut item) = scene::try_get_node(*handle) {
                item.draw_item();
            }
        }
        node.buffer = Vec::new();
    }
}
