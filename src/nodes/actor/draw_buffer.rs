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

pub struct ActorDrawBuffer {
    buffer: Vec<Handle<Actor>>,
}

impl ActorDrawBuffer {
    pub fn new() -> Self {
        ActorDrawBuffer {
            buffer: Vec::new(),
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }

    pub fn add_to_buffer(&mut self, handle: Handle<Actor>) {
        self.buffer.push(handle);
    }
}

impl Node for ActorDrawBuffer {
    fn draw(mut node: RefMut<Self>) {
        node.buffer.retain(|handle| {
            let viewport = get_global::<Viewport>();
            if let Some(actor) = scene::try_get_node(*handle) {
                viewport.contains(actor.body.position)
            } else {
                false
            }
        });
        node.buffer.sort_by(|a, b| {
            let actor_a = scene::get_node(*a);
            let actor_b = scene::get_node(*b);
            actor_a.body.position.y.partial_cmp(&actor_b.body.position.y).unwrap()
        });
        for handle in &node.buffer {
            let mut actor = scene::get_node(*handle);
            actor.draw_actor();
        }
        node.buffer = Vec::new();
    }
}
