use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
            RefMut,
        },
        collections::storage,
    },
    color,
    prelude::*,
};

use super::{
    LightSource,
    Item,
    ItemParams,
    Actor,
    ActorControllerKind,
    ActorParams,
};

use crate::{
    map::Map,
    render::Viewport,
    resources::Resources,
};

#[derive(Debug, Clone)]
pub struct GameParams {
}

pub struct GameState {
    pub map: Map,
    pub local_player_id: String,
    pub show_character_window: bool,
    pub show_inventory_window: bool,
    pub in_debug_mode: bool,
    pub should_quit: bool,
}

impl GameState {
    pub fn new(map: Map, local_player_id: &str) -> GameState {
        let resources = storage::get::<Resources>();
        if let Some(layer) = map.layers.get("spawn_points") {
            for object in &layer.objects {
                if object.name == "player" {
                    let params = resources.actors.get("player").cloned().unwrap();
                    let mut player = Actor::new(
                        ActorControllerKind::LocalPlayer { player_id: local_player_id.to_string() },
                        ActorParams {
                            name: "Abraxas".to_string(),
                            position: Some(object.position),
                            ..params
                        }
                    );
                    player.stats.recalculate_derived();
                    player.stats.restore_vitals();
                    scene::add_node(player);
                } else if let Some(prototype_id) = object.properties.get("prototype_id") {
                    if let Some(params) = resources.actors.get(prototype_id).cloned() {
                        let mut actor = Actor::new(ActorControllerKind::Computer, ActorParams {
                            position: Some(object.position),
                            ..params
                        });
                        actor.stats.recalculate_derived();
                        actor.stats.restore_vitals();
                        scene::add_node(actor);
                    } else {
                        println!("actor prototype id '{}' not found!", prototype_id);
                    }
                }
            }
        }

        if let Some(layer) = map.layers.get("light_sources") {
            for object in &layer.objects {
                let size = if let Some(size) = object.size {
                    size
                } else {
                    LightSource::DEFAULT_SIZE
                };
                let color = if let Some(_color) = object.properties.get("color") {
                    // TODO: Parse hex value
                    /*Color::new()*/
                    color::WHITE
                } else {
                    LightSource::DEFAULT_COLOR
                };
                let intensity = if let Some(intensity) = object.properties.get("intensity") {
                    intensity.parse::<f32>().unwrap()
                } else {
                    LightSource::DEFAULT_INTENSITY
                };
                LightSource::add_node(object.position, size, color, intensity);
            }
        }

        if let Some(layer) = map.layers.get("items") {
            for object in &layer.objects {
                if let Some(prototype_id) = object.properties.get("prototype_id") {
                    if let Some(params) = resources.items.get(prototype_id).cloned() {
                        Item::add_node(ItemParams {
                            position: Some(object.position),
                            ..params
                        });
                    } else {
                        println!("item prototype id '{}' not found!", prototype_id);
                    }
                }
            }
        }

        GameState {
            map,
            local_player_id: local_player_id.to_string(),
            show_character_window: false,
            show_inventory_window: false,
            in_debug_mode: false,
            should_quit: false,
        }
    }

    pub fn add_node(map: Map, local_player_id: &str) -> Handle<Self> {
        scene::add_node(Self::new(map, local_player_id))
    }
}

impl Node for GameState {
    fn update(mut node: RefMut<Self>) where Self: Sized {
        if is_key_released(KeyCode::Escape) {
            if node.show_inventory_window || node.show_character_window {
                node.show_inventory_window = false;
                node.show_character_window = false;
            } else {
                node.should_quit = true;
            }
        }
    }

    fn draw(node: RefMut<Self>) {
        let viewport = storage::get::<Viewport>();
        let rect = node.map.to_grid(viewport.get_frustum());
        node.map.draw( Some(rect));
    }
}
