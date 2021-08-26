#![feature(fn_traits)]
#![feature(drain_filter)]
#![feature(try_find)]

use macroquad::{
    experimental::{
        collections::storage,
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
use resources::Resources;
use nodes::item::Credits;

use config::Config;
use gui::{
    skins::GuiSkins,
};

pub use uid::generate_id;

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
pub mod missions;
pub mod config;
pub mod uid;

fn window_conf() -> Conf {
    let config = Config::load();

    Conf {
        window_title: "Capstone".to_owned(),
        high_dpi: true,
        window_width: config.resolution.x as i32,
        window_height: config.resolution.y as i32,
        fullscreen: config.fullscreen,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    Resources::load().await;

    {
        let config = storage::get::<Config>();
        storage::store(GuiSkins::new(config.gui_scale));
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
        //     ]).await.unwrap();

        let map = Map::load("assets/maps/map_01.json").await.unwrap();

        GameState::add_node(map, &player_id.clone());
        Camera::add_node();
        DrawBuffer::<Item>::add_node();
        DrawBuffer::<Credits>::add_node();
        Projectiles::add_node();
        ContinuousBeams::add_node();
        DrawBuffer::<Actor>::add_node();
        PostProcessing::add_node("crt");
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

    let config = storage::get::<Config>();
    config.save();
}
