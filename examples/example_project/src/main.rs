use rust_rpg_toolkit::prelude::*;

const GAME_NAME: &'static str = "example_project";
const GAME_VERSION: &'static str = "0.1.0";

const CONFIG_PATH: &'static str = "examples/example_project/config.json";
const ASSETS_PATH: &'static str = "examples/example_project/assets";
const MODULES_PATH: &'static str = "examples/example_project/modules";
const CHARACTERS_PATH: &'static str = "examples/example_project/characters";

pub fn window_conf() -> Conf {
    let config = Config::load(CONFIG_PATH);
    storage::store(config.clone());

    Conf {
        window_title: GAME_NAME.to_owned(),
        high_dpi: false,
        window_width: config.resolution.x as i32,
        window_height: config.resolution.y as i32,
        fullscreen: config.fullscreen,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let game_params = GameParams {
        game_name: GAME_NAME.to_string(),
        game_version: GAME_VERSION.to_string(),
        assets_path: ASSETS_PATH.to_string(),
        modules_path: MODULES_PATH.to_string(),
        characters_path: CHARACTERS_PATH.to_string(),
        ..Default::default()
    };

    run_game(game_params).await;

    let config = storage::get::<Config>();
    config.save(CONFIG_PATH);
}
