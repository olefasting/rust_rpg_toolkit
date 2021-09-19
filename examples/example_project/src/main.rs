use rust_rpg_toolkit::prelude::*;

const GAME_NAME: &'static str = "example-project";
const GAME_VERSION: &'static str = "0.1.0";
const CONFIG_PATH: &'static str = "config.json";

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

    init(params).await?;

    while handle_event_queue().await? == false {
        clear_background(color::BLACK);

        update_input();
        draw_gui();

        next_frame().await;
    }

    Ok(())
}
