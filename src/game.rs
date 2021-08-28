use std::{
    fs,
    io,
};

use crate::{
    gui::MainMenuResult,
    modules::load_modules,
    map::TiledMapDeclaration,
    prelude::*,
};

pub fn load_map(player: ExportedCharacter, chapter: usize, map_id: &str) {
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

    GameState::add_node(player, map_data.map);
    Camera::add_node();
    DrawBuffer::<Item>::add_node();
    DrawBuffer::<Credits>::add_node();
    Projectiles::add_node();
    ContinuousBeams::add_node();
    DrawBuffer::<Actor>::add_node();
    PostProcessing::add_node();
    Hud::add_node();
}

struct SceneTransition {
    pub player: ExportedCharacter,
    pub chapter_index: usize,
    pub map_id: String,
}

async fn game_loop() -> Option<SceneTransition> {
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
                let player = Actor::find_by_player_id(&game_state.local_player_id).unwrap();
                let current_chapter = storage::get::<CurrentChapter>();
                return Some(SceneTransition {
                    player: player.to_export(),
                    chapter_index: current_chapter.chapter_index,
                    map_id,
                });
            }
        }

        next_frame().await;
    }

    return None;
}

#[derive(Debug, Clone)]
pub struct GameParams {
    pub game_version: String,
    pub assets_path: String,
    pub modules_path: String,
    pub characters_path: String,
    pub saves_path: String,
    pub new_character_prototype_id: String,
}

impl Default for GameParams {
    fn default() -> Self {
        GameParams {
            game_version: "0.1.0".to_string(),
            assets_path: "assets".to_string(),
            modules_path: "modules".to_string(),
            characters_path: "characters".to_string(),
            saves_path: "save_games".to_string(),
            new_character_prototype_id: "new_character_prototype".to_string(),
        }
    }
}

#[cfg(any(target_family = "unix", target_family = "windows"))]
fn init_environment(params: &GameParams) {
    fs::create_dir_all(&params.characters_path)
        .expect(&format!("Unable to create characters directory '{}'!", params.characters_path));
    fs::create_dir_all(&params.saves_path)
        .expect(&format!("Unable to create save games directory '{}'!", params.saves_path));
}

#[cfg(any(target_family = "wasm"))]
fn init_environment(params: &GameParams) {
    todo!("Implement for WASM");
}

pub async fn run_game(params: GameParams) {
    init_environment(&params);

    let local_player_id = generate_id();
    map_gamepad(&local_player_id);
    storage::store(params.clone());
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

    let mut scene_transition = None;
    match gui::draw_main_menu(&params).await {
        MainMenuResult::NewCharacter(character) => {
            let scenario = storage::get::<Scenario>();
            let map_id = scenario.chapters
                .first()
                .map(|chapter| chapter.initial_map_id.clone())
                .unwrap();
            scene_transition = Some(SceneTransition {
                player: character,
                chapter_index: 0,
                map_id,
            });
        }
        MainMenuResult::ImportedCharacter(character, chapter_index, map_id) =>
            scene_transition = Some(SceneTransition {
                player: character,
                chapter_index,
                map_id,
            }),
        MainMenuResult::LoadGame(save_game) =>
            scene_transition = None,
        MainMenuResult::Quit =>
            scene_transition = None,
    };

    loop {
        {
            let scene_transition = scene_transition.unwrap();
            load_map(scene_transition.player, scene_transition.chapter_index, &scene_transition.map_id);
        }

        scene_transition = game_loop().await;

        scene::clear();

        if scene_transition.is_none() {
            break;
        }
    }
}
