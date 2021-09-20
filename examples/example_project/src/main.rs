use rust_rpg_toolkit::prelude::*;

const GAME_NAME: &'static str = "example-project";
const GAME_VERSION: &'static str = "0.1.0";
const CONFIG_PATH: &'static str = "config.json";

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
        ..Default::default()
    };

    init(params).await?;

    while handle_queued_events().await? == false {
        begin_frame();

        // ...

        end_frame().await;
    }

    Ok(())
}
