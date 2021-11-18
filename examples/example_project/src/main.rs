use rust_rpg_toolkit::prelude::*;

const GAME_NAME: &str = "example-project";
const GAME_VERSION: &str = "0.1.0";

// Paths must be set for examples, as they are not using their own directory as working directory.
// This is not necessary in a normal project....
const CONFIG_PATH: &str = "examples/example_project/config.json";
const DATA_PATH: &str = "examples/example_project/data";
const MODULES_PATH: &str = "examples/example_project/modules";
const CHARACTERS_PATH: &str = "examples/example_project/characters";

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
        ..Default::default()
    };

    init(params).await?;

    while handle_queued_events().await? {
        begin_frame();

        // ...

        end_frame().await;
    }

    Ok(())
}
