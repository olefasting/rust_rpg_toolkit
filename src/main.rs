#![feature(fn_traits)]
#![feature(drain_filter)]

use macroquad::{
    color,
    experimental::{
        coroutines::start_coroutine,
        scene,
    },
    prelude::*,
};
use macroquad::prelude::animation::Animation;

pub use globals::{
    get_global,
    set_global,
    try_get_global,
};
use globals::LocalPlayer;
pub use input::get_mouse_position;
pub use map::{
    Map,
};
use nodes::{
    Actor,
    ActorControllerKind,
    ActorParams,
    Camera,
    GameState,
    Projectiles,
    Item,
    ItemParams,
};
use physics::Collider;
use render::{
    HorizontalAlignment,
    SpriteAnimationParams,
    SpriteParams,
};
pub use render::text::draw_aligned_text;
pub use resources::{
    Resources,
};

use crate::nodes::actor::{ActorDrawBuffer, ActorStats};
use crate::nodes::item::ItemDrawBuffer;

mod resources;
mod globals;
mod map;

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

fn generic_actor(name: &str, position: Vec2, skin_id: u32, factions: &[String], player_id: Option<u32>) -> Actor {
    assert!(skin_id <= 2, "invalid skin id");
    let controller_kind = match player_id {
        Some(id) => ActorControllerKind::Player { id },
        None => ActorControllerKind::Computer,
    };
    let resources = get_global::<Resources>();
    let params = resources.get_actor(&format!("generic_actor_0{}", skin_id+1));
    Actor::new(position, controller_kind, true,ActorParams {
        name: name.to_string(),
        factions: factions.to_vec(),
        ..params.clone()
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
        set_global(LocalPlayer {
            id: 0,
        });

        // TODO: Move to resources
        let map = Map::new(uvec2(16, 16), "assets/maps/map_01.json").await;
        GameState::add_node(map);

        Camera::add_node(vec2(100.0, 400.0));

        ItemDrawBuffer::add_node();

        Projectiles::add_node();

        scene::add_node(generic_actor(
            "Player Actor",
            vec2(100.0, 400.0),
            0,
            &["player_faction".to_string()],
            Some(0),
        ));

        scene::add_node(generic_actor(
            "Friendly Actor",
            vec2(200.0, 375.0),
            2,
            &["player_faction".to_string()],
            None,
        ));

        scene::add_node(generic_actor(
            "Enemy Actor",
            vec2(250.0, 350.0),
            1,
            &[],
            None,
        ));

        ActorDrawBuffer::add_node();
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
