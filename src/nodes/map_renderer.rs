use crate::prelude::*;

pub struct MapRenderer;

impl MapRenderer {
    pub fn new() -> Self {
        MapRenderer {}
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }
}

impl Node for MapRenderer {
    fn draw(_: RefMut<Self>) {
        let map = storage::get::<Map>();
        let viewport = storage::get::<Viewport>();
        let rect = map.to_grid(viewport.get_frustum());
        map.draw(Some(rect));
    }
}