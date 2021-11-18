use rust_rpg_toolkit::prelude::*;

mod example_node;

use example_node::ExampleNode;

const GAME_NAME: &str = "building-scenes";
const GAME_VERSION: &str = "0.1.0";

// Paths must be set for examples, as they are not using their own directory as working directory.
// This is not necessary in a normal project....
const CONFIG_PATH: &str = "examples/building_scenes/config.json";
const DATA_PATH: &str = "examples/building_scenes/data";
const MODULES_PATH: &str = "examples/building_scenes/modules";
const CHARACTERS_PATH: &str = "examples/building_scenes/characters";

const ASSETS_PATH: &str = "examples/shared_resources/assets";

pub fn window_conf() -> WindowConf {
    let config = Config::load(CONFIG_PATH);

    WindowConf {
        window_title: GAME_NAME.to_owned(),
        high_dpi: false,
        window_width: config.resolution.x as i32,
        window_height: config.resolution.y as i32,
        fullscreen: config.fullscreen,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let params = GameParams {
        name: GAME_NAME.to_string(),
        version: GAME_VERSION.to_string(),
        data_path: DATA_PATH.to_string(),
        modules_path: MODULES_PATH.to_string(),
        characters_path: CHARACTERS_PATH.to_string(),
        assets_path: ASSETS_PATH.to_string(),
        skip_character_creation: true,
        ..Default::default()
    };

    init(params).await?;

    SceneBuilder::new()
        .with_draw_buffer::<ExampleNode>(DrawStage::Actors)
        .with_post_build(|_, _| {
            let animation_params = SpriteAnimationParams {
                texture_id: "player".to_string(),
                tile_size: vec2(32.0, 32.0),
                animations: vec![
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
                    },
                ],
                ..Default::default()
            };

            ExampleNode::add_node(vec2(1000.0, 900.0), vec2(32.0, 32.0), animation_params);
        })
        .make_default();

    while handle_queued_events().await? {
        begin_frame();

        // ..

        end_frame().await;
    }

    Ok(())
}
