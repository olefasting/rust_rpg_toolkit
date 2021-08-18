#![feature(fn_traits)]
#![feature(drain_filter)]
#![feature(try_find)]

use macroquad::{
    color,
    experimental::{
        coroutines::start_coroutine,
        scene,
    },
    prelude::*,
};

pub use input::get_mouse_position;
pub use map::{
    Map,
    MapLayer,
    MapLayerKind,
    MapObject,
    MapTile,
    MapTileset,
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

use physics::Collider;
use render::HorizontalAlignment;

pub use render::{
    Viewport,
    text::draw_aligned_text,
};
pub use resources::Resources;
pub use ability::{
    AbilityParams,
    Ability,
};
pub use global::{
    try_get_global,
    get_global,
    set_global,
};

mod resources;
mod map;
mod ability;

pub mod global;
pub mod nodes;
pub mod render;
pub mod input;
pub mod physics;
pub mod math;
pub mod gui;
pub mod json;

pub const MAP_LAYER_GROUND: &'static str = "ground";
pub const MAP_LAYER_SOLIDS: &'static str = "solids";
pub const MAP_LAYER_BARRIERS: &'static str = "barriers";
pub const MAP_LAYER_ITEMS: &'static str = "items";
pub const MAP_LAYER_SPAWN_POINTS: &'static str = "spawn_points";

pub fn generate_id() -> String {
    nanoid::nanoid!()
}

fn generic_actor(name: &str, position: Vec2, skin_id: u32, factions: &[String], player_id: Option<String>) -> Actor {
    assert!(skin_id <= 2, "invalid skin id");
    let controller_kind = match player_id {
        Some(player_id) => ActorControllerKind::LocalPlayer { player_id },
        None => ActorControllerKind::Computer,
    };
    let resources = get_global::<Resources>();
    let prototype = resources.actors.get(&format!("generic_actor_0{}", skin_id+1)).unwrap();
    let params = ActorParams::from_prototype(position, prototype.clone());
    Actor::new(controller_kind, ActorParams {
        name: name.to_string(),
        factions: factions.to_vec(),
        ..params
    })
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
        set_global(resources);
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
            }
        );

        next_frame().await;
    }

    {
        let player_id = generate_id();
        let player_spawn_position = vec2(64.0, 100.0);

        let map = Map::from(map::TiledMap::new("assets/maps/test_tiled_map.json", &[
            ("neo_zero_tiles", "../textures/neo_zero_tiles.png", "tiles"),
            ("neo_zero_props", "../textures/neo_zero_props.png", "props"),
            ("items", "../textures/items.png", "items"),
        ]));
        // let map = Map::new("assets/maps/capstone_map.json");

        GameState::add_node(map, &player_id);

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
