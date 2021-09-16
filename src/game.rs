use crate::prelude::*;
use crate::gui::*;

fn load_map(local_player_id: &str, transition: SceneTransition) {
    scene::clear();

    let resources = storage::get::<Resources>();
    let SceneTransition { player, chapter_index, map_id } = transition;

    let transition_params = SceneTransitionParams { chapter_index, map_id: map_id.clone() };
    storage::store(transition_params);

    let chapter = resources.chapters.get(chapter_index)
        .expect(&format!("Unable to load chapter '{}'!", chapter_index));

    let game_state = {
        for key in chapter.maps.keys() {
            println!("{}", key);
        }
        let map = chapter.maps.get(&map_id)
            .cloned()
            .expect(&format!("Unable to load map '{}' of chapter '{}'!", map_id, chapter.title));

        GameState::add_node(&local_player_id, map, &player)
    };

    Camera::add_node();
    DrawBuffer::<Item>::add_node();
    DrawBuffer::<Credits>::add_node();
    Projectiles::add_node();
    ContinuousBeams::add_node();
    DrawBuffer::<Actor>::add_node();

    {
        let game_state = scene::get_node(game_state);
        let resources = storage::get::<Resources>();
        if let Some(layer) = game_state.map.layers.get("spawn_points") {
            for object in &layer.objects {
                if object.name == "player" {
                    let mut actor = Actor::from_export(
                        game_state.handle(),
                        object.position,
                        ActorControllerKind::local_player(&local_player_id),
                        player.clone(),
                    );

                    actor.stats.recalculate_derived();
                    actor.stats.restore_vitals();

                    scene::add_node(actor);
                } else if let Some(prototype_id) = object.properties.get("prototype_id") {
                    let params = resources.actors.get(&prototype_id.value).cloned()
                        .expect(&format!("Unable to find actor with prototype id '{}'", prototype_id.value));
                    let instance_id = if let Some(instance_id) = object.properties.get("instance_id").cloned() {
                        instance_id.value
                    } else {
                        generate_id()
                    };

                    let mut actor = Actor::new(
                        game_state.handle(),
                        ActorControllerKind::Computer,
                        ActorParams {
                            id: instance_id,
                            position: Some(object.position),
                            ..params
                        });

                    actor.stats.recalculate_derived();
                    actor.stats.restore_vitals();

                    scene::add_node(actor);
                }
            }
        }

        if let Some(layer) = game_state.map.layers.get("light_sources") {
            for object in &layer.objects {
                let size = if let Some(size) = object.size {
                    size
                } else {
                    LightSource::DEFAULT_SIZE
                };

                let color = if let Some(color) = object.properties.get("color") {
                    color_from_hex_string(&color.value)
                } else {
                    LightSource::DEFAULT_COLOR
                };

                let intensity = if let Some(intensity) = object.properties.get("intensity") {
                    intensity.value.parse::<f32>().unwrap()
                } else {
                    LightSource::DEFAULT_INTENSITY
                };

                LightSource::add_node(object.position, size, color, intensity);
            }
        }

        if let Some(layer) = game_state.map.layers.get("items") {
            for object in &layer.objects {
                if let Some(prototype_id) = object.properties.get("prototype_id").cloned() {
                    if prototype_id.value == "credits".to_string() {
                        let amount = object.properties.get("amount").unwrap();
                        Credits::add_node(object.position, amount.value.parse::<u32>().unwrap());
                    } else {
                        let params = resources.items.get(&prototype_id.value).cloned()
                            .expect(&format!("Unable to find item with prototype id '{}'", &prototype_id.value));
                        let instance_id = if let Some(instance_id) = object.properties.get("instance_id").cloned() {
                            instance_id.value
                        } else {
                            generate_id()
                        };

                        Item::add_node(ItemParams {
                            id: instance_id,
                            position: Some(object.position),
                            ..params
                        });
                    }
                }
            }
        }
    }

    PostProcessing::add_node();
    Hud::add_node();
}

#[derive(Debug, Clone)]
pub struct GameParams {
    pub game_name: String,
    pub game_version: String,
    pub config_path: String,
    pub data_path: String,
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
            config_path: "config.json".to_string(),
            data_path: "data".to_string(),
            modules_path: "modules".to_string(),
            characters_path: "characters".to_string(),
            new_character_prototype_id: "new_character_prototype".to_string(),
            new_character_build_points: 6,
            clear_background_color: color::BLACK,
        }
    }
}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
fn check_paths(params: &GameParams) {
    fs::create_dir_all(&params.characters_path)
        .expect(&format!("Unable to create characters directory '{}'!", params.characters_path));
}

#[cfg(target_family = "wasm")]
pub fn check_paths(_params: &GameParams) {}

#[cfg(not(any(target_family = "wasm", target_os = "android")))]
pub async fn load_resources(game_params: GameParams) {
    let bg_color = game_params.clear_background_color.clone();

    let coroutine = start_coroutine(async move {
        let mut resources = Resources::new(&game_params.data_path).await.unwrap();
        load_modules(game_params, &mut resources).await.unwrap();

        storage::store(resources);
    });

    while coroutine.is_done() == false {
        clear_background(bg_color);
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
pub async fn load_resources(game_params: GameParams) {
    let mut state = ResourceLoadingState::None;

    let mut resources = Resources::new(&game_params.data_path).await.unwrap();
    load_modules(&mut state, &game_params, &mut resources).await.unwrap();

    storage::store(resources);
}

pub fn init_local_player() -> String {
    let player_id = generate_id();
    map_gamepad(&player_id);
    player_id
}

#[derive(Debug, Clone)]
pub enum ApplicationState {
    LoadingResources,
    InMainMenu,
    LoadingScene(SceneTransition),
    InGame,
    Quitting,
}

pub async fn update() -> ApplicationState {
    let game_params = storage::get::<GameParams>();
    clear_background(game_params.clear_background_color);

    draw_gui();
    update_input();

    {
        let mut game_state = scene::find_node_by_type::<GameState>().unwrap();
        if game_state.should_save_character {
            game_state.should_save_character = false;
            game_state.save_player_character();
        }

        if game_state.should_go_to_main_menu {
            game_state.save_player_character();
            return ApplicationState::InMainMenu;
        }

        if game_state.should_quit {
            game_state.save_player_character();
            return ApplicationState::Quitting;
        }

        if let Some(transition_params) = game_state.scene_transition.clone() {
            let player = Actor::find_by_player_id(&game_state.local_player_id).unwrap();
            let scene_transition = SceneTransition::new(player.to_export(game_state.is_permadeath), transition_params);
            game_state.scene_transition = None;
            return ApplicationState::LoadingScene(scene_transition);
        }
    }

    next_frame().await;

    ApplicationState::InGame
}

// This will run the game and it can also be used as a blueprint for how to implement
// your own game loop, so that you get better access to the internals
pub async fn run_game(game_params: GameParams) {
    storage::store(game_params.clone());
    check_paths(&game_params);

    let mut state = ApplicationState::LoadingResources;

    load_resources(game_params.clone()).await;
    load_gui_theme(&game_params).await.unwrap();

    let local_player_id = init_local_player();

    state = ApplicationState::InMainMenu;

    loop {
        match &state {
            ApplicationState::InMainMenu => {
                match gui::draw_main_menu().await {
                    MainMenuResult::StartGame(transition) =>
                        state = ApplicationState::LoadingScene(transition),
                    MainMenuResult::Quit =>
                        state = ApplicationState::Quitting,
                }
            }
            ApplicationState::LoadingScene(transition) => {
                load_map(&local_player_id, transition.clone());
                state = ApplicationState::InGame;
            }
            ApplicationState::InGame => {
                state = update().await;
            }
            ApplicationState::Quitting => {
                break;
            }
            _ => {}
        }
    }

    scene::clear();
}
