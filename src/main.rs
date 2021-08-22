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
    Camera,
    ContinuousBeams,
    DrawBuffer,
    GameState,
    Item,
    Projectiles,
    PostProcessing,
    Hud,
};
use render::{
    draw_aligned_text,
    HorizontalAlignment,
};
use resources::Resources;
use render::VerticalAlignment;

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

        GameState::add_node(map, &player_id.clone());
        Camera::add_node();
        DrawBuffer::<Item>::add_node();
        Projectiles::add_node();
        ContinuousBeams::add_node();
        DrawBuffer::<Actor>::add_node();
        PostProcessing::add_node();
        Hud::add_node();
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
