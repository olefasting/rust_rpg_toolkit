use macroquad::{
    experimental::{
        scene::{
            Node,
            RefMut,
        }
    },
    prelude::*,
};

pub struct GameState {
    pub should_quit: bool,
}

impl GameState {
    pub fn new() -> GameState {
        // for (_, data) in &map.actors {
        //     let actor = Actor::new(data);
        //     scene::add_node(actor);
        // }
        //
        // for (_, data) in &map.items {
        //     let item = Actor::new(data);
        //     scene::add_node(item);
        // }

        GameState {
            should_quit: false,
        }
    }
}

impl Node for GameState {
    fn ready(_node: RefMut<Self>) {}

    fn update(mut node: RefMut<Self>) {
        node.should_quit = is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q);
    }

    fn fixed_update(_node: RefMut<Self>) {}
}
