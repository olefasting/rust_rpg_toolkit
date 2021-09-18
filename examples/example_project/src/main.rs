use rust_rpg_toolkit::prelude::*;

const GAME_NAME: &'static str = "example-project";
const GAME_VERSION: &'static str = "0.1.0";

const CONFIG_PATH: &'static str = "config.json";
const DATA_PATH: &'static str = "data";
const MODULES_PATH: &'static str = "modules";
const CHARACTERS_PATH: &'static str = "characters";

pub fn window_conf() -> Conf {
    let config = Config::load(CONFIG_PATH);

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
async fn main() -> Result<()> {
    let params = GameParams {
        game_name: GAME_NAME.to_string(),
        game_version: GAME_VERSION.to_string(),
        ..Default::default()
    };

    fs::create_dir_all(&params.characters_path)?;
    storage::store(params.clone());

    init_resources().await;
    init_gui().await?;
    init_player();

    dispatch_event(Event::ShowMainMenu);

    while handle_event_queue().await? == false {
        clear_background(params.clear_bg_color);

        update_input();
        draw_gui();

        next_frame().await;
    }

    Ok(())
}
