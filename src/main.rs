#![feature(fn_traits)]
#![feature(drain_filter)]
#![feature(try_find)]

use macroquad::{
    color,
    experimental::{
        collections::storage,
        coroutines::start_coroutine,
        scene,
    },
    prelude::*,
};

use map::{
    Map,
};
use nodes::{
    Actor,
    ActorControllerKind,
    ActorParams,
    Camera,
    ContinuousBeams,
    DrawBuffer,
    GameState,
    Item,
    Projectiles,
};
use physics::Collider;
use render::{
    draw_aligned_text,
    HorizontalAlignment,
};
use resources::Resources;
use crate::render::VerticalAlignment;
use crate::nodes::ItemParams;

pub mod resources;
pub mod ability;
pub mod map;
pub mod nodes;
pub mod render;
pub mod input;
pub mod physics;
pub mod math;
pub mod gui;
pub mod json;
pub mod helpers;

pub fn generate_id() -> String {
    nanoid::nanoid!()
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Capstone".to_owned(),
        high_dpi: false,
        window_width: 1920,
        window_height: 180,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let load_resources = start_coroutine(async move {
        let resources = Resources::new().await.unwrap();
        storage::store(resources);
    });

    while load_resources.is_done() == false {
        clear_background(BLACK);
        draw_aligned_text(
            &format!("Loading resources"),
            screen_width() / 2.0,
            screen_height() / 2.0,
            HorizontalAlignment::Center,
            VerticalAlignment::Center,
            TextParams {
                font_size: 40,
                color: color::WHITE,
                ..Default::default()
            },
        );

        next_frame().await;
    }

    {
        let player_id = generate_id();

        let t = Collider::Rectangle { x: 0.0, y: 0.0, w: 10.0, h: 10.0 };
        let json = serde_json::to_string_pretty(&t).unwrap();
        std::fs::write("assets/test.json", json).unwrap();

        // let map = Map::load_tiled(
        //     "assets/maps/test_tiled_map.json",
        //     Some("assets/maps/map_01.json"),
        //     Some(&[
        //         ("barriers_2", map::MapCollisionKind::Barrier),
        //         ("barriers_1", map::MapCollisionKind::Barrier),
        //         ("solids_2", map::MapCollisionKind::Solid),
        //         ("solids_1", map::MapCollisionKind::Solid),
        //     ]),
        //     &[
        //         ("neo_zero_tiles", "../textures/neo_zero_tiles.png", "tiles"),
        //         ("neo_zero_props", "../textures/neo_zero_props.png", "props"),
        //         ("items", "../textures/items.png", "items"),
        //     ]).unwrap();

        let map = Map::load("assets/maps/map_01.json").unwrap();
        // let map = Map::load("assets/maps/test_capstone_map.json").unwrap();

        GameState::add_node(map, &player_id.clone());

        Camera::add_node();

        DrawBuffer::<Item>::add_node();

        Projectiles::add_node();
        ContinuousBeams::add_node();

        DrawBuffer::<Actor>::add_node();
        {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            let resources = storage::get::<Resources>();
            if let Some(layer) = game_state.map.layers.get("spawn_points") {
                for object in &layer.objects {
                    if object.name == "player" {
                        let params = resources.actors.get("player").cloned().unwrap();
                        let mut player = Actor::new(
                            ActorControllerKind::LocalPlayer { player_id: player_id.clone() },
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

            if let Some(layer) = game_state.map.layers.get("items") {
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
        }
    }

    loop {
        {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            if game_state.should_quit {
                break;
            }
        }

        gui::draw_gui();

        next_frame().await;
    }

    scene::clear();
}
