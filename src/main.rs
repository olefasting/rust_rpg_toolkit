#![feature(fn_traits)]
#![feature(drain_filter)]
#![feature(try_find)]

use macroquad::{
    color,
    experimental::{
        coroutines::start_coroutine,
        scene,
        collections::storage,
    },
    prelude::*,
};

use nodes::{
    Actor,
    ActorControllerKind,
    ActorParams,
    Camera,
    GameState,
    Item,
    ItemParams,
    Projectiles,
    ContinuousBeams,
    DrawBuffer,
    ActorStats,
};

use resources::Resources;
use physics::Collider;
use map::{
    Map,
    MapCollisionKind,
};
use render::{
    HorizontalAlignment,
    draw_aligned_text,
};

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

pub fn generate_id() -> String {
    nanoid::nanoid!()
}

fn generic_actor(name: &str, position: Vec2, skin_id: u32, factions: &[String], player_id: Option<String>) -> Actor {
    assert!(skin_id <= 2, "invalid skin id");
    let controller_kind = match player_id {
        Some(player_id) => ActorControllerKind::LocalPlayer { player_id },
        None => ActorControllerKind::Computer,
    };
    let resources = storage::get::<Resources>();
    let params = resources.actors.get(&format!("generic_actor_0{}", skin_id + 1)).cloned().unwrap();
    let mut actor = Actor::new(controller_kind, ActorParams {
        position: Some(position),
        name: name.to_string(),
        factions: factions.to_vec(),
        ..params
    });
    actor.stats.recalculate_derived();
    actor.stats.restore_vitals();
    actor
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
        let player_spawn_position = vec2(32.0, 100.0);

        let t = Collider::Rectangle { x: 0.0, y: 0.0, w: 10.0, h: 10.0 };
        let json = serde_json::to_string_pretty(&t).unwrap();
        std::fs::write("assets/test.json", json).unwrap();

        // let map = Map::load_tiled(
        //     "assets/maps/test_tiled_map.json",
        //     Some("assets/maps/converted_tiled_map.json"),
        //     Some(&[
        //         ("barriers", MapCollisionKind::Barrier),
        //         ("solids", MapCollisionKind::Solid),
        //     ]),
        //     &[
        //         ("neo_zero_tiles", "../textures/neo_zero_tiles.png", "tiles"),
        //         ("neo_zero_props", "../textures/neo_zero_props.png", "props"),
        //         ("items", "../textures/items.png", "items"),
        //     ]).unwrap();

        // let map = Map::load("assets/maps/converted_tiled_map.json").unwrap();
        let map = Map::load("assets/maps/test_capstone_map.json").unwrap();

        GameState::add_node(map, &player_id, true);

        Camera::add_node(player_spawn_position);

        DrawBuffer::<Item>::add_node();

        Projectiles::add_node();
        ContinuousBeams::add_node();

        scene::add_node(generic_actor(
            "Player Actor",
            player_spawn_position,
            0,
            &["player_faction".to_string()],
            Some(player_id),
        ));

        scene::add_node(generic_actor(
            "Friendly Actor",
            vec2(225.0, 375.0),
            2,
            &["player_faction".to_string()],
            None,
        ));

        scene::add_node(generic_actor(
            "Enemy Actor",
            vec2(300.0, 350.0),
            1,
            &[],
            None,
        ));

        DrawBuffer::<Actor>::add_node();
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
