use std::{
    fs,
};

use crate::{
    gui::MainMenuResult,
    modules::load_modules,
    map::TiledMapDeclaration,
    prelude::*,
};

use crate::prelude::coroutines::start_coroutine;

fn load_map(local_player_id: &str, transition: SceneTransition) {
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

#[derive(Debug, Clone)]
pub struct GameParams {
    pub game_name: String,
    pub game_version: String,
    pub assets_path: String,
    pub modules_path: String,
    pub characters_path: String,
    pub new_character_prototype_id: String,
    pub new_character_build_points: u32,
    pub clear_background_color: Color,
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
            clear_background_color: color::BLACK,
        }
    }
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
fn check_env(params: &GameParams) {
    fs::create_dir_all(&params.characters_path)
        .expect(&format!("Unable to create characters directory '{}'!", params.characters_path));
}

#[cfg(target_family = "wasm")]
pub fn check_env(_params: &GameParams) {}

pub fn setup_local_player() -> String {
    let local_player_id = generate_id();
    map_gamepad(&local_player_id);
    local_player_id
}

pub async fn load_resources(game_params: GameParams) -> Result<(), FileError> {
    let assets_path = game_params.assets_path.clone();
    let mut resources = Resources::new(&assets_path).await.unwrap();
    let mut scenario_params = Scenario::load_params(&assets_path).await.unwrap();
    load_modules(&game_params, &mut resources, &mut scenario_params).await;
    storage::store(resources);

    let tiled_maps_file_path = format!("{}/tiled_maps.json", assets_path);
    let bytes = load_file(&tiled_maps_file_path).await?;
    let tiled_maps: Vec<TiledMapDeclaration> = serde_json::from_slice(&bytes).unwrap();
    for decl in tiled_maps {
        Map::load_tiled(&assets_path, decl.clone()).await?;
    }

    let scenario = Scenario::new(&assets_path, scenario_params).await?;
    storage::store(scenario);
    Ok(())
}

pub async fn run_game(game_params: GameParams) {
    storage::store(game_params.clone());
    check_env(&game_params);

    let player_id = setup_local_player();

    let config = storage::get::<Config>();
    storage::store(GuiSkins::new(config.gui_scale));

    {
        let game_params = game_params.clone();
        let clear_background_color = game_params.clear_background_color.clone();
        let coroutine = start_coroutine({
            async move {
                load_resources(game_params).await.unwrap();
            }
        });

        while coroutine.is_done() == false {
            clear_background(clear_background_color);
            draw_aligned_text(
                "Loading game data...",
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

    #[allow(unused_assignments)]
    let mut scene_transition = None;
    match gui::draw_main_menu(&game_params).await {
        MainMenuResult::StartGame(transition) =>
            scene_transition = Some(transition),
        MainMenuResult::Quit => return,
    };

    'outer: loop {
        load_map(&player_id, scene_transition.unwrap());

        'inner: loop {
            gui::draw_gui();
            update_input();

            {
                let mut game_state = scene::find_node_by_type::<GameState>().unwrap();
                if game_state.should_save_character {
                    game_state.save_player_character();
                    game_state.should_save_character = false;
                }

                if game_state.should_quit {
                    break 'outer;
                }

                if let Some(transition_params) = game_state.scene_transition.clone() {
                    let player = Actor::find_by_player_id(&game_state.local_player_id).unwrap();
                    scene_transition = Some(SceneTransition::new(player.to_export(), transition_params));
                    break 'inner;
                }
            }

            next_frame().await;
        }

        if scene_transition.is_none() {
            scene::clear();
            break 'outer;
        }
    }
}
