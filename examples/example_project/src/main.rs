use rust_rpg_toolkit::prelude::*;

const GAME_NAME: &'static str = "example_project";
const GAME_VERSION: &'static str = "0.1.0";

const CONFIG_PATH: &'static str = "config.json";
const ASSETS_PATH: &'static str = "assets";
const MODULES_PATH: &'static str = "modules";
const CHARACTERS_PATH: &'static str = "characters";

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
    // This is done since this example is not in crate root. For a normal project,
    // depending on the toolkit, this will not be necessary
    let params = if cfg!(wasm) {
        GameParams {
            game_name: GAME_NAME.to_string(),
            game_version: GAME_VERSION.to_string(),
            assets_path: format!("{}/{}", "examples/example_project" ,ASSETS_PATH),
            modules_path: format!("{}/{}", "examples/example_project" ,MODULES_PATH),
            characters_path: format!("{}/{}", "examples/example_project" ,CHARACTERS_PATH),
            ..Default::default()
        }
    } else {
        GameParams {
            game_name: GAME_NAME.to_string(),
            game_version: GAME_VERSION.to_string(),
            assets_path: ASSETS_PATH.to_string(),
            modules_path: MODULES_PATH.to_string(),
            characters_path: CHARACTERS_PATH.to_string(),
            ..Default::default()
        }
    };

    run_game(params).await;

    let config = storage::get::<Config>();
    config.save(CONFIG_PATH);
}
