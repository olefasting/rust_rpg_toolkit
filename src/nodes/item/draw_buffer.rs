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
            let item_a = scene::get_node(*a);
            let item_b = scene::get_node(*b);
            item_a.position.y.partial_cmp(&item_b.position.y).unwrap()
        });
        for handle in &node.buffer {
            let mut item = scene::get_node(*handle);
            item.draw_item();
        }
        node.buffer = Vec::new();
    }
}
