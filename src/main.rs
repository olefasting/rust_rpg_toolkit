use macroquad::{
    color,
    experimental::{
        collections::storage,
        coroutines::start_coroutine,
        scene,
    },
    prelude::*,
};

pub use input::get_mouse_position;

use physics::Collider;

use nodes::{
    Camera,
    Actor,
    ActorControllerKind,
    ActorParams,
    GameState,
    Projectiles,
};

use render::{
    SpriteParams,
    text::{
        draw_aligned_text,
        TextAlignment,
    }
};

pub use resources::Resources;
pub use globals::{
    try_get_global,
    get_global,
    set_global,
};

use globals::LocalPlayer;

pub use map::{
    Map,
    MapTile,
};

pub use item::Item;

mod resources;
mod globals;
mod map;
mod item;

pub mod nodes;
pub mod render;
pub mod input;
pub mod physics;
pub mod math;

pub fn generate_id() -> String {
    nanoid::nanoid!()
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Armada".to_owned(),
        high_dpi: true,
        window_width: 1080,
        window_height: 720,
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
            TextAlignment::Center,
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

        Projectiles::add_node();

        Camera::add_node();

        let map = Map::new(uvec2(10, 10));
        GameState::add_node(map);

        Actor::add_node(
            ActorParams {
                position: vec2(100.0, 100.0),
                collider: Some(Collider::circle(0.0, 0.0, 16.0)),
                controller_kind: ActorControllerKind::Player { player_id: 0 },
                sprite_params: SpriteParams {
                    tile_size: vec2(32.0, 32.0),
                    offset: vec2(-16.0, -16.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        Actor::add_node(
            ActorParams {
                position: vec2(300.0, 300.0),
                collider: Some(Collider::circle(0.0, 0.0, 16.0)),
                controller_kind: ActorControllerKind::Computer,
                sprite_params: SpriteParams {
                    tile_size: vec2(32.0, 32.0),
                    offset: vec2(-16.0, -16.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        );
    }

    loop {
        {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            if game_state.should_quit {
                break;
            }
        }

        next_frame().await;
    }

    scene::clear();
}
