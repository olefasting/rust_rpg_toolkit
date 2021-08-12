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

pub struct ItemDrawQueue {
    queue: Vec<Handle<Item>>,
}

impl ItemDrawQueue {
    pub fn new() -> Self {
        ItemDrawQueue {
            queue: Vec::new(),
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }

    pub fn add_to_queue(&mut self, handle: Handle<Item>) {
        self.queue.push(handle);
    }
}

impl Node for ItemDrawQueue {
    fn draw(mut node: RefMut<Self>) {
        node.queue.retain(|handle| {
            let viewport = get_global::<Viewport>();
            let item = scene::get_node(*handle);
            viewport.contains(item.position)
        });
        node.queue.sort_by(|a, b| {
            let item_a = scene::get_node(*a);
            let item_b = scene::get_node(*b);
            item_a.position.y.partial_cmp(&item_b.position.y).unwrap()
        });
        for handle in &node.queue {
            let mut item = scene::get_node(*handle);
            item.draw_item();
        }
        node.queue = Vec::new();
    }
}
