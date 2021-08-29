use std::fs;

use crate::{
    gui::MainMenuResult,
    modules::load_modules,
    map::TiledMapDeclaration,
    prelude::*,
};

fn load_map(transition: SceneTransition) {
    let scenario = storage::get::<Scenario>();
    let SceneTransition { player, chapter_index, map_id } = transition;
    let chapter = scenario.chapters.get(chapter_index)
        .cloned()
        .expect(&format!("Unable to load chapter '{}'!", chapter_index));
    let map_data = chapter.maps.iter().find(|map| map.id == map_id)
        .cloned()
        .expect(&format!("Unable to load map '{}' of chapter '{}'!", map_id, chapter.title));

    let current_chapter = CurrentChapter {
        chapter,
        chapter_index,
        map_id,
    };

    storage::store(current_chapter);

    let map = map_data.map;
    let local_player_id = generate_id();

    let resources = storage::get::<Resources>();
    if let Some(layer) = map.layers.get("spawn_points") {
        for object in &layer.objects {
            if object.name == "player" {
                let mut actor = Actor::from_export(
                    object.position,
                    ActorControllerKind::local_player(&local_player_id),
                    player.clone(),
                );
                actor.stats.recalculate_derived();
                actor.stats.restore_vitals();
                scene::add_node(actor);
            } else if let Some(prototype_id) = object.properties.get("prototype_id") {
                let params = resources.actors.get(prototype_id).cloned()
                    .expect(&format!("Unable to find actor with prototype id '{}'", prototype_id));
                let instance_id = object.properties.get("instance_id").cloned();
                let mut actor = Actor::new(ActorControllerKind::Computer, ActorParams {
                    id: instance_id.unwrap_or(generate_id()),
                    position: Some(object.position),
                    ..params
                });
                actor.stats.recalculate_derived();
                actor.stats.restore_vitals();
                scene::add_node(actor);
            }
        }
    }

    if let Some(layer) = map.layers.get("light_sources") {
        for object in &layer.objects {
            let size = if let Some(size) = object.size {
                size
            } else {
                LightSource::DEFAULT_SIZE
            };
            let color = if let Some(_color) = object.properties.get("color") {
                // TODO: Parse hex value
                /*Color::new()*/
                color::WHITE
            } else {
                LightSource::DEFAULT_COLOR
            };
            let intensity = if let Some(intensity) = object.properties.get("intensity") {
                intensity.parse::<f32>().unwrap()
            } else {
                LightSource::DEFAULT_INTENSITY
            };
            LightSource::add_node(object.position, size, color, intensity);
        }
    }

    if let Some(layer) = map.layers.get("items") {
        for object in &layer.objects {
            if let Some(prototype_id) = object.properties.get("prototype_id").cloned() {
                if prototype_id == "credits".to_string() {
                    let amount = object.properties.get("amount").unwrap();
                    Credits::add_node(object.position, amount.parse::<u32>().unwrap());
                } else {
                    let params = resources.items.get(&prototype_id).cloned()
                        .expect(&format!("Unable to find item with prototype id '{}'", &prototype_id));
                    let instance_id = object.properties.get("instance_id").cloned();
                    Item::add_node(ItemParams {
                        id: instance_id.unwrap_or(generate_id()),
                        position: Some(object.position),
                        ..params
                    });
                }
            }
        }
    }

    GameState::add_node(&local_player_id, map);
    Camera::add_node();
    DrawBuffer::<Item>::add_node();
    DrawBuffer::<Credits>::add_node();
    Projectiles::add_node();
    ContinuousBeams::add_node();
    DrawBuffer::<Actor>::add_node();
    PostProcessing::add_node();
    Hud::add_node();
}

async fn game_loop() -> Option<SceneTransition> {
    loop {
        gui::draw_gui();
        update_input();
        {
            let mut game_state = scene::find_node_by_type::<GameState>().unwrap();
            if game_state.should_save_character {
                game_state.save_player_character();
                game_state.should_save_character = false;
            }
            if game_state.should_quit {
                break;
            }
            if let Some(transition_params) = game_state.scene_transition.clone() {
                let player = Actor::find_by_player_id(&game_state.local_player_id).unwrap();
                return Some(SceneTransition::new(player.to_export(), transition_params));
            }
        }

        next_frame().await;
    }

    return None;
}

#[derive(Debug, Clone)]
pub struct GameParams {
    pub game_name: String,
    pub game_version: String,
    pub assets_path: String,
    pub modules_path: String,
    pub characters_path: String,
    pub new_character_prototype_id: String,
    pub new_character_build_points: u32,
}

impl Default for GameParams {
    fn default() -> Self {
        GameParams {
            game_name: "Unnamed Project".to_string(),
            game_version: "0.1.0".to_string(),
            assets_path: "assets".to_string(),
            modules_path: "modules".to_string(),
            characters_path: "characters".to_string(),
            new_character_prototype_id: "new_character_prototype".to_string(),
            new_character_build_points: 6,
        }
    }
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
fn check_env(params: &GameParams) {
    fs::create_dir_all(&params.characters_path)
        .expect(&format!("Unable to create characters directory '{}'!", params.characters_path));
}

#[cfg(target_family = "wasm")]
fn check_env(_params: &GameParams) {}

pub async fn run_game(params: GameParams) {
    check_env(&params);

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
        MainMenuResult::StartGame(transition) =>
            scene_transition = Some(transition),
        MainMenuResult::Quit => return,
    };

    loop {
        load_map(scene_transition.unwrap());

        scene_transition = game_loop().await;

        if scene_transition.is_none() {
            scene::clear();
            break;
        }
    }
}
