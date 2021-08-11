#![feature(fn_traits)]

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
};

pub use item::Item;
use macroquad::prelude::animation::Animation;

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
        window_title: "Capstone".to_owned(),
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

        let map = Map::new(vec2(16.0, 16.0),"assets/maps/map_01.json").await;
        GameState::add_node(map);

        Camera::add_node();

        Projectiles::add_node();

        Actor::add_node(
            ActorParams {
                current_health: 1000.0,
                max_health: 1000.0,
                position: vec2(100.0, 100.0),
                collider: Some(Collider::circle(0.0, 0.0, 8.0)),
                controller_kind: ActorControllerKind::Player { id: 0 },
                sprite_params: SpriteParams {
                    texture_id: Resources::CHARACTERS_TEXTURE_ID.to_string(),
                    tile_size: vec2(32.0, 32.0),
                    offset: vec2(-16.0, -16.0),
                    animations: vec!(
                        Animation {
                            name: "down".to_string(),
                            row: 0,
                            frames: 3,
                            fps: 8,
                        },
                        Animation {
                            name: "up".to_string(),
                            row: 1,
                            frames: 3,
                            fps: 8,
                        },
                        Animation {
                            name: "right".to_string(),
                            row: 2,
                            frames: 3,
                            fps: 8,
                        }
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        Actor::add_node(
            ActorParams {
                current_health: 1000.0,
                max_health: 1000.0,
                position: vec2(300.0, 300.0),
                collider: Some(Collider::circle(0.0, 0.0, 8.0)),
                controller_kind: ActorControllerKind::Computer,
                sprite_params: SpriteParams {
                    texture_id: Resources::CHARACTERS_TEXTURE_ID.to_string(),
                    tile_size: vec2(32.0, 32.0),
                    offset: vec2(-16.0, -16.0),
                    animations: vec!(
                        Animation {
                            name: "down".to_string(),
                            row: 3,
                            frames: 3,
                            fps: 8,
                        },
                        Animation {
                            name: "up".to_string(),
                            row: 4,
                            frames: 3,
                            fps: 8,
                        },
                        Animation {
                            name: "right".to_string(),
                            row: 4,
                            frames: 3,
                            fps: 8,
                        }
                    ),
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
