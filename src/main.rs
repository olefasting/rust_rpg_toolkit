use macroquad::{
    color,
    experimental::{
        collections::storage,
        coroutines::start_coroutine,
        scene,
    },
    prelude::*,
};

pub use circle::Circle;
pub use input::get_mouse_position;
use nodes::{
    Actor,
    ActorControllerKind,
    ActorData,
    Camera,
    GameState,
};
use render::{
    SpriteParams,
    text::{
        draw_aligned_text,
        TextAlignment,
    }
};
pub use resources::Resources;
pub use global_value::{
    try_get_global,
    get_global,
    set_global,
};

mod resources;
mod circle;
mod global_value;

pub mod nodes;
pub mod render;
pub mod input;
pub mod physics;

pub fn generate_id() -> String {
    nanoid::nanoid!()
}

#[derive(Copy, Clone)]
pub struct LocalPlayerId(pub u32);

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
        storage::store(resources);
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
        set_global(LocalPlayerId(0));

        let camera = Camera::new();
        scene::add_node(camera);

        let game_state = GameState::new();
        scene::add_node(game_state);

        let actor = Actor::new(
            ActorData {
                name: "Player Actor".to_string(),
                position: vec2(100.0, 100.0),
                controller_kind: ActorControllerKind::Player { player_id: 0 },
                sprite_params: SpriteParams {
                    tile_size: vec2(64.0, 64.0),
                    offset: vec2(-32.0, -32.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        scene::add_node(actor);
    }

    loop {
        // Do not remove this scope! It will cause weird bugs with GameState node
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
