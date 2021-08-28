use crate::{
    modules::load_modules,
    map::TiledMapDeclaration,
    prelude::*,
};

pub struct MapTransition {
    pub next_map_id: Option<String>,
    pub next_chapter: bool,
}

pub fn load_map(local_player_id: &str, chapter: usize, map_id: &str) {
    let scenario = storage::get::<Scenario>();
    let chapter_data = scenario.chapters.get(chapter)
        .cloned()
        .expect(&format!("Unable to load chapter '{}'!", chapter));
    let map_data = chapter_data.maps.iter().find(|map| map.id == map_id)
        .cloned()
        .expect(&format!("Unable to load map '{}' of chapter '{}'!", map_id, chapter_data.title));

    let current_chapter = CurrentChapter {
        chapter: chapter_data,
        chapter_index: chapter,
        current_map_id: map_id.to_string(),
    };

    storage::store(current_chapter);

    GameState::add_node(map_data.map, &local_player_id);
    Camera::add_node();
    DrawBuffer::<Item>::add_node();
    DrawBuffer::<Credits>::add_node();
    Projectiles::add_node();
    ContinuousBeams::add_node();
    DrawBuffer::<Actor>::add_node();
    PostProcessing::add_node();
    Hud::add_node();
}

async fn game_loop() -> Option<String> {
    loop {
        gui::draw_gui();
        update_input();
        {
            let mut game_state = scene::find_node_by_type::<GameState>().unwrap();
            if game_state.should_export_character {
                game_state.export_character();
                game_state.should_export_character = false;
            }
            if game_state.should_save_game {
                game_state.save_game();
                game_state.should_save_game = false;
            }
            if game_state.should_quit {
                break;
            }
            if let Some(map_id) = game_state.transition_to_map.clone() {
                return Some(map_id);
            }
        }

        next_frame().await;
    }

    return None;
}

pub struct GameParams {
    pub game_version: String,
    pub assets_path: String,
    pub modules_path: String,
    pub characters_path: String,
    pub saves_path: String,
}

impl Default for GameParams {
    fn default() -> Self {
        GameParams {
            game_version: "0.1.0".to_string(),
            assets_path: "assets".to_string(),
            modules_path: "modules".to_string(),
            characters_path: "characters".to_string(),
            saves_path: "save_games".to_string(),
        }
    }
}

pub async fn run_game(params: GameParams) {
    let local_player_id = generate_id();
    try_map_gamepad(&local_player_id);
    storage::store(params);
    {
        let config = storage::get::<Config>();
        storage::store(GuiSkins::new(config.gui_scale));

        let mut resources = Resources::new().await.unwrap();
        let mut scenario_params = Scenario::load_params().await.unwrap();
        load_modules(&mut resources, &mut scenario_params).await;

        storage::store(resources);

        let game_params = storage::get::<GameParams>();
        let tiled_maps_file_path = format!("{}/tiled_maps.json", game_params.assets_path);
        let bytes = load_file(&tiled_maps_file_path).await
            .expect(&format!("Unable to find tiled maps file '{}'!", tiled_maps_file_path));
        let tiled_maps: Vec<TiledMapDeclaration> = serde_json::from_slice(&bytes)
            .expect(&format!("Unable to parse tiled maps file '{}'!", tiled_maps_file_path));
        for decl in tiled_maps {
            Map::load_tiled(decl.clone()).await
                .expect(&format!("Unable to convert tiled map '{}'!", decl.path));
        }

        let scenario = Scenario::new(scenario_params).await.unwrap();
        storage::store(scenario);
    }

    let (chapter_i, map_id) = gui::draw_chapter_select().await;
    let mut next_map_id = Some(map_id);

    loop {
        load_map(&local_player_id, chapter_i, &next_map_id.unwrap());
        next_map_id = game_loop().await;

        scene::clear();

        if next_map_id.is_none() {
            break;
        }
    }
}
