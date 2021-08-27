use macroquad::{
    experimental::collections::storage,
    prelude::*
};

use rust_rpg_toolkit::{Config, GameParams};

// Used when determining whether module dependencies on game version are met
const GAME_VERSION: &'static str = "0.1.0";

// These would be in the project root, if this was a project depending on the crate,
// not an example in a sub directory of the crate...
const CONFIG_PATH: &'static str = "examples/example_project/config.json";
const ASSETS_PATH: &'static str = "examples/example_project/assets";
const MODULES_PATH: &'static str = "examples/example_project/modules";
const CHARACTERS_PATH: &'static str = "examples/example_project/characters";
const SAVES_PATH: &'static str = "examples/example_project/save_games";

fn window_conf() -> Conf {
    let config = Config::load(CONFIG_PATH);
    storage::store(config.clone());

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
    let params = GameParams {
        game_version: GAME_VERSION.to_string(),
        assets_path: ASSETS_PATH.to_string(),
        modules_path: MODULES_PATH.to_string(),
        characters_path: CHARACTERS_PATH.to_string(),
        saves_path: SAVES_PATH.to_string()
    };

    rust_rpg_toolkit::run_game(params).await;

    let config = storage::get::<Config>();
    config.save(CONFIG_PATH);
}
