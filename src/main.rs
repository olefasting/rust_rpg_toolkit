use macroquad::{
    experimental::{
        collections::storage,
        coroutines::start_coroutine,
        scene,
    },
    input::{is_key_down, KeyCode},
    prelude::*,
};

pub use resources::Resources;
pub use util::{
    Circle,
    get_mouse_position,
    draw_aligned_text,
    TextAlignment,
    generate_string_id,
    GetStringId,
    SetStringId,
    StringId,
};

mod nodes;
mod util;
mod resources;
mod graphics;

use nodes::{
    GameState,
    CameraControl,
    Input,
    Actor,
    ActorData,
};

use graphics::SpriteParams;

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
        draw_text(
            &format!("Loading resources"),
            screen_width() / 2.0 - 160.0,
            screen_height() / 2.0,
            40.,
            WHITE,
        );

        next_frame().await;
    }

    {
        let camera = CameraControl::new();
        scene::add_node(camera);

        let game_state = GameState::new(0);
        scene::add_node(game_state);

        let input = Input::new();
        scene::add_node(input);

        let actor = Actor::new(
            ActorData {
                name: "Player Actor".to_string(),
                position: vec2(100.0, 100.0),
                player_control_id: Some(0),
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
        {
            let mut game_state = scene::find_node_by_type::<GameState>().unwrap();
            if game_state.should_quit {
                break;
            }

            if is_key_down(KeyCode::Q) || is_key_down(KeyCode::Escape) {
                game_state.should_quit = true;
            }
        }

        next_frame().await;
    }

    scene::clear();
}
