use crate::prelude::*;
use crate::gui::*;

// This will clear the current scene and create a new one, based on the specified `SceneTransition`
pub fn load_scene(character: PlayerCharacter) -> Result<()> {
    scene::clear();

    let player = &*storage::get::<Player>();
    let resources = storage::get::<Resources>();

    let chapter_index = character.current_chapter_index;

    let chapter = resources.chapters.get(chapter_index)
        .expect(&format!("Unable to load chapter '{}'!", chapter_index));

    let map_id = character.current_map_id.clone();

    let map = chapter.maps.get(&map_id)
        .cloned()
        .expect(&format!("Unable to load map '{}' of chapter '{}' ({})!", map_id, chapter.title, chapter_index));

    let game_state = GameState::add_node(player.clone(), character.clone(), map);

    Camera::add_node();
    DrawBuffer::<Item>::add_node();
    DrawBuffer::<Credits>::add_node();
    Projectiles::add_node();
    ContinuousBeams::add_node();
    DrawBuffer::<Actor>::add_node();

    {
        let mut game_state = scene::get_node(game_state);
        let resources = storage::get::<Resources>();
        if let Some(layer) = game_state.map.layers.get("spawn_points").cloned() {
            for object in &layer.objects {
                if object.name == "player" {
                    let mut actor = Actor::from_export(
                        game_state.handle(),
                        object.position,
                        ActorControllerKind::local_player(&game_state.player.id),
                        character.clone(),
                    );

                    actor.stats.recalculate_derived();
                    actor.stats.restore_vitals();

                    let actor_id = actor.id.clone();
                    let actor_name = actor.name.clone();

                    let handle = scene::add_node(actor);

                    game_state.player.set_actor(&actor_id, &actor_name, handle, character.is_permadeath);
                    storage::store(game_state.player.clone());

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
pub async fn load_resources() {
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
    let gui_theme = serde_json::from_slice(&bytes)
        .expect(&format!("Error when parsing gui theme '{}'", path));
    let gui_skins = GuiSkins::new(gui_theme);
    storage::store(gui_skins);
    Ok(())
}

// This will initialize the player
pub fn init_player() {
    let player_id = generate_id();
    let gamepad_id = map_gamepad(&player_id);
    let player = Player::new(&player_id, gamepad_id);
    storage::store(player);
}