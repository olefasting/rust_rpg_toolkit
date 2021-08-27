use std::fs;

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

use crate::{map::Map, render::Viewport, resources::Resources, generate_id, CHARACTERS_FOLDER_PATH};
use crate::nodes::item::Credits;
use crate::save_games::SaveGame;

#[derive(Debug, Clone)]
pub struct GameParams {
}

pub struct GameState {
    pub map: Map,
    pub dead_actors: Vec<String>,
    pub local_player_id: String,
    pub show_character_window: bool,
    pub show_inventory_window: bool,
    pub show_game_menu: bool,
    pub in_debug_mode: bool,
    pub transition_to_map: Option<String>,
    pub should_quit: bool,
    pub should_save_game: bool,
    pub should_export_character: bool,
}

impl GameState {
    pub fn new(map: Map, local_player_id: &str) -> GameState {
        let resources = storage::get::<Resources>();
        if let Some(layer) = map.layers.get("spawn_points") {
            for object in &layer.objects {
                if object.name == "player" {
                    let params = resources.actors.get("player").cloned().unwrap();
                    let mut player = Actor::new(
                        true,
                        ActorControllerKind::LocalPlayer { player_id: local_player_id.to_string() },
                        ActorParams {
                            id: generate_id(),
                            name: "Abraxas".to_string(),
                            position: Some(object.position),
                            ..params
                        }
                    );
                    player.stats.recalculate_derived();
                    player.stats.restore_vitals();
                    scene::add_node(player);
                } else if let Some(prototype_id) = object.properties.get("prototype_id") {
                    let params = resources.actors.get(prototype_id).cloned()
                        .expect(&format!("Unable to find actor with prototype id '{}'", prototype_id));
                    let instance_id = object.properties.get("instance_id").cloned();
                    let mut actor = Actor::new(true, ActorControllerKind::Computer, ActorParams {
                        id: instance_id.unwrap_or(generate_id()),
                        position: Some(object.position),
                        ..params
                    });
                    actor.stats.recalculate_derived();
                    actor.stats.restore_vitals();
                    scene::add_node(actor);
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
                if let Some(prototype_id) = object.properties.get("prototype_id").cloned() {
                    if prototype_id == "credits".to_string() {
                        let amount = object.properties.get("amount").unwrap();
                        Credits::add_node(object.position, amount.parse::<u32>().unwrap());
                    } else {
                        let params = resources.items.get(&prototype_id).cloned()
                            .expect(&format!("Unable to find item with prototype id '{}'", &prototype_id));
                        let instance_id = object.properties.get("instance_id").cloned();
                        Item::add_node(ItemParams {
                            id: instance_id.unwrap_or(generate_id()),
                            position: Some(object.position),
                            ..params
                        });
                    }
                }
            }
        }

        GameState {
            map,
            dead_actors: Vec::new(),
            local_player_id: local_player_id.to_string(),
            show_character_window: false,
            show_inventory_window: false,
            show_game_menu: false,
            in_debug_mode: false,
            transition_to_map: None,
            should_quit: false,
            should_save_game: false,
            should_export_character: false,
        }
    }

    pub fn add_node(map: Map, local_player_id: &str) -> Handle<Self> {
        scene::add_node(Self::new(map, local_player_id))
    }

    pub fn save_game(&self) {
        let name = {
            let player = Actor::find_by_player_id(&self.local_player_id).unwrap();
            let time_stamp = chrono::Utc::now();
            format!("{} {}.json", player.name, time_stamp.to_rfc2822())
        };
        SaveGame::save_scene_to_file(&name, self);
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn export_character(&mut self) {
        let player = Actor::find_by_player_id(&self.local_player_id).unwrap();
        let json = serde_json::to_string_pretty(&player.to_export())
            .expect(&format!("Unable to serialize character '{}' to JSON!", player.name));
        let path = format!("{}/{}.json", CHARACTERS_FOLDER_PATH, player.name);
        fs::write(&path, json)
            .expect(&format!("Unable to write character to path '{}'!", path));
    }

    #[cfg(target_family = "wasm")]
    pub fn export_character(&self) {
        todo!("Implement wasm character export")
    }
}

impl Node for GameState {
    fn update(mut node: RefMut<Self>) where Self: Sized {
        if Actor::find_by_player_id(&node.local_player_id).is_none() {
            node.show_game_menu = is_key_released(KeyCode::Escape);
        }
    }

    fn draw(node: RefMut<Self>) {
        let viewport = storage::get::<Viewport>();
        let rect = node.map.to_grid(viewport.get_frustum());
        node.map.draw( Some(rect));
    }
}
