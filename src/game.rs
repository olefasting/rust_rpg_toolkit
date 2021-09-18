use crate::prelude::*;
use crate::gui::*;

// This will clear the current scene and create a new one, based on the specified `SceneTransition`
pub fn load_scene(character: Character) -> Result<()> {
    scene::clear();

    let resources = storage::get::<Resources>();
    let chapter = resources.chapters.get(character.current_chapter_index).unwrap();
    let map = chapter.maps.get(&character.current_map_id).cloned().unwrap();
    storage::store(map);

    Camera::add_node();
    GameState::add_node(&character);
    DrawBuffer::<Item>::add_node();
    DrawBuffer::<Credits>::add_node();
    Projectiles::add_node();
    ContinuousBeams::add_node();
    DrawBuffer::<Actor>::add_node();

    let map = storage::get::<Map>();

    let mut player_spawn = None;

    if let Some(layer) = map.layers.get("spawn_points") {
        for object in &layer.objects {
            if object.name == "player" {
                player_spawn = Some(object.position);
            } else if let Some(prototype_id) = object.properties.get("prototype_id") {
                let instance_id = if let Some(instance_id) = object.properties.get("instance_id").cloned() {
                    instance_id.value
                } else {
                    generate_id()
                };

                let params = resources.actors.get(&prototype_id.value).cloned().unwrap();
                let mut actor = Actor::new(
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

    let player_spawn = player_spawn
        .expect(&format!("No player spawn point in map '{}'!", character.current_map_id));

    let player = storage::get::<LocalPlayer>();
    let mut actor = Actor::from_saved(
        player_spawn,
        ActorControllerKind::local_player(&player.id),
        character,
    );

    actor.stats.recalculate_derived();
    actor.stats.restore_vitals();

    scene::add_node(actor);

    if let Some(layer) = map.layers.get("light_sources") {
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

    if let Some(layer) = map.layers.get("items") {
        for object in &layer.objects {
            if let Some(prototype_id) = object.properties.get("prototype_id").cloned() {
                if prototype_id.value == "credits".to_string() {
                    let amount = object.properties.get("amount").unwrap();
                    Credits::add_node(object.position, amount.value.parse::<u32>().unwrap());
                } else {
                    let params = resources.items.get(&prototype_id.value).cloned().unwrap();
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

    PostProcessing::add_node();
    Hud::add_node();

    Ok(())
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
    pub clear_bg_color: Color,
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
            clear_bg_color: color::BLACK,
        }
    }
}

// This will load all resources to memory. This means both assets, such as textures and sound, as
// well as all data files. It will also apply modules.
// The assets can later be accessed by getting the `Resources` struct from storage
#[cfg(not(any(target_family = "wasm", target_os = "android")))]
pub async fn init_resources() {
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
        clear_background(game_params.clear_bg_color);
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
pub async fn init_resources() {
    let game_params = storage::get::<GameParams>();
    let mut state = ResourceLoadingState::None;

    let mut resources = Resources::new(&game_params.data_path).await.unwrap();
    load_modules(&mut state, &game_params, &mut resources).await.unwrap();

    storage::store(resources);
}

// This will create the GUI skins, based on the `gui_theme.json` file, in the data directory.
// It must be called after resources has completed loading, as it relies on images imported to
// `Resources`. The `GuiSkins` struct can later be accessed by retrieving it from storage.
pub async fn init_gui() -> Result<()> {
    let game_params = storage::get::<GameParams>();
    let path = format!("{}/gui_theme.json", &game_params.data_path);
    let bytes = load_file(&path).await?;
    let gui_theme = serde_json::from_slice(&bytes)?;
    let gui_skins = GuiSkins::new(gui_theme);
    storage::store(gui_skins);
    Ok(())
}

// This will initialize the local player and map the first available gamepad, if  any are available
pub fn init_local_player() {
    let player_id = generate_id();
    let gamepad_id = map_gamepad(&player_id);
    let player = LocalPlayer::new(&player_id, gamepad_id);
    storage::store(player);
}