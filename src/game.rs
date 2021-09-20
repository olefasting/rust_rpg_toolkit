use crate::prelude::*;
use crate::gui::*;

use crate::modules::load_modules;

#[derive(Debug, Clone)]
pub struct GameParams {
    pub name: String,
    pub version: String,
    pub data_path: String,
    pub modules_path: String,
    pub characters_path: String,
    pub new_character_prototype_id: String,
    pub new_character_build_points: u32,
}

impl Default for GameParams {
    fn default() -> Self {
        GameParams {
            name: "Unnamed Game".to_string(),
            version: "0.1.0".to_string(),
            data_path: "data".to_string(),
            modules_path: "modules".to_string(),
            characters_path: "characters".to_string(),
            new_character_prototype_id: "new_character_prototype".to_string(),
            new_character_build_points: 6,
        }
    }
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
async fn load_resources() {
    let game_params = storage::get::<GameParams>();
    let coroutine = {
        let game_params = game_params.clone();

        start_coroutine(async move {
            let mut resources = Resources::new(&game_params.data_path).await.unwrap();
            load_modules(&game_params, &mut resources).await.unwrap();

            storage::store(resources);
        })
    };

    while coroutine.is_done() == false {
        clear_background(color::BLACK);
        draw_aligned_text(
            "Loading game resources...",
            screen_width() / 2.0,
            screen_height() / 2.0,
            HorizontalAlignment::Center,
            VerticalAlignment::Center,
            TextParams {
                ..Default::default()
            },
        );

        next_frame().await;
    }
}

#[cfg(target_family = "wasm")]
async fn load_resources() {
    let game_params = storage::get::<GameParams>();
    let mut state = ResourceLoadingState::None;

    let mut resources = Resources::new(&game_params.data_path).await.unwrap();
    load_modules(&mut state, &game_params, &mut resources).await.unwrap();

    storage::store(resources);
}

// This will perform all the initialization necessary prior to starting a game loop
pub async fn init(params: GameParams) -> Result<()> {
    fs::create_dir_all(&params.characters_path)?;
    storage::store(params.clone());

    load_resources().await;

    let path = Path::new(&params.data_path).join("gui_theme.json");

    let bytes = load_file(path.to_str().unwrap()).await?;
    let gui_theme = serde_json::from_slice(&bytes)?;
    let gui_skins = GuiSkins::new(gui_theme);
    storage::store(gui_skins);

    let player_id = generate_id();
    let gamepad_id = map_gamepad(&player_id);
    let local_player = LocalPlayer::new(&player_id, gamepad_id);
    storage::store(local_player);

    dispatch_event(Event::MainMenu);

    Ok(())
}