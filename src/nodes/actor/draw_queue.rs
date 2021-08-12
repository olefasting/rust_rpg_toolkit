use macroquad::{
    experimental::scene::{
        Node,
        RefMut,
        Handle,
    },
    prelude::*,
};

use crate::nodes::Actor;
use crate::get_global;
use crate::render::Viewport;

pub struct ActorDrawQueue {
    queue: Vec<Handle<Actor>>,
}

impl ActorDrawQueue {
    pub fn new() -> Self {
        ActorDrawQueue {
            queue: Vec::new(),
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }

    pub fn add_to_queue(&mut self, handle: Handle<Actor>) {
        self.queue.push(handle);
    }
}

impl Node for ActorDrawQueue {
    fn draw(mut node: RefMut<Self>) {
        node.queue.retain(|handle| {
            let viewport = get_global::<Viewport>();
            let actor = scene::get_node(*handle);
            viewport.contains(actor.body.position)
        });
        node.queue.sort_by(|a, b| {
            let actor_a = scene::get_node(*a);
            let actor_b = scene::get_node(*b);
            actor_a.body.position.y.partial_cmp(&actor_b.body.position.y).unwrap()
        });
        for handle in &node.queue {
            let mut actor = scene::get_node(*handle);
            actor.draw_actor();
        }
        node.queue = Vec::new();
    }
}
