use std::{
    path::Path,
};

use crate::prelude::*;

use crate::resources::{MaterialAssetParams, TextureAssetParams, SoundAssetParams};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleDataFileKind {
    Actors,
    Dialogue,
    Missions,
    Items,
    Abilities,
    Scenario,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleIntegration {
    Extend,
    Replace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDataParams {
    pub kind: ModuleDataFileKind,
    pub path: String,
    pub integration: ModuleIntegration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDependencyParams {
    pub name: String,
    pub version: Option<String>,
}

impl Default for ModuleDependencyParams {
    fn default() -> Self {
        ModuleDependencyParams {
            name: "".to_string(),
            version: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMaterials {
    pub integration: ModuleIntegration,
    pub files: Vec<MaterialAssetParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleTextures {
    pub integration: ModuleIntegration,
    pub files: Vec<TextureAssetParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleSounds {
    pub integration: ModuleIntegration,
    pub files: Vec<SoundAssetParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleAssetsParams {
    materials: ModuleMaterials,
    textures: ModuleTextures,
    sound_effects: ModuleSounds,
    music: ModuleSounds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleParams {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_game_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_toolkit_version: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<ModuleDependencyParams>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<ModuleDataParams>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<ModuleAssetsParams>,
}

impl Default for ModuleParams {
    fn default() -> Self {
        ModuleParams {
            title: "Unnamed Module".to_string(),
            description: "".to_string(),
            version: "not versioned".to_string(),
            required_toolkit_version: None,
            required_game_version: None,
            dependencies: Vec::new(),
            data: Vec::new(),
            assets: None,
        }
    }
}

pub async fn load_modules(game_params: &GameParams, resources: &mut Resources) -> Result<(), FileError> {
    let active_modules_file_path = &format!("{}/active_modules.json", game_params.modules_path);
    let mut loaded_modules: Vec<(String, String)> = Vec::new();

    let bytes = load_file(active_modules_file_path).await?;
    let active_modules: Vec<String> = serde_json::from_slice(&bytes).unwrap();

    'module: for module_name in active_modules {
        let module_path = format!("{}/{}", game_params.modules_path, module_name);
        let module_file_path = format!("{}/module.json", module_path);

        if Path::new(&module_path).exists() == false || Path::new(&module_file_path).exists() == false {
            println!("WARNING: Module '{}' could not be found, even though it is listed in the active modules file!", module_name);
            continue 'module;
        }

        let bytes = load_file(&module_file_path).await?;
        let module_params: ModuleParams = serde_json::from_slice(&bytes).unwrap();

        if let Some(required_game_version) = &module_params.required_game_version {
            if check_version(required_game_version, &game_params.game_version) == false {
                println!("WARNING: Module '{}' was not loaded as its game version requirement '{}' was unmet (game version is '{}')!", module_name, required_game_version, game_params.game_version);
                continue 'module;
            }
        }

        let toolkit_version = get_toolkit_version();
        if let Some(required_toolkit_version) = &module_params.required_toolkit_version {
            if check_version(required_toolkit_version, &toolkit_version) == false {
                println!("WARNING: Module '{}' was not loaded as its toolkit version requirement '{}' was unmet (toolkit version is '{}')!", module_name, required_toolkit_version, get_toolkit_version());
                continue 'module;
            }
        }

        for dependency in module_params.dependencies {
            if loaded_modules.iter().find(|(name, version)| {
                let name = name.clone();
                if let Some(required_version) = &dependency.version {
                    return name == dependency.name && check_version(required_version, version);
                }
                name == dependency.name
            }).is_none() {
                println!("WARNING: Module '{}' was not loaded as its dependency on '{}' was unmet!", module_name, dependency.name);
                continue 'module;
            }
        }

        for data in module_params.data {
            let bytes = load_file(&format!("{}/{}", module_path, data.path)).await
                .expect(&format!("Unable to find module data file '{}'!", data.path));
            match data.kind {
                ModuleDataFileKind::Actors => {
                    let actors: Vec<ActorParams> = serde_json::from_slice(&bytes)
                        .expect(&format!("Unable to parse module actor data file '{}'!", data.path));
                    match data.integration {
                        ModuleIntegration::Extend => {
                            for params in actors {
                                resources.actors.insert(params.id.clone(), params);
                            }
                        }
                        ModuleIntegration::Replace => {
                            let hash_map = HashMap::from_iter(
                                actors
                                    .into_iter()
                                    .map(|params| (params.id.clone(), params))
                                    .collect::<Vec<(String, ActorParams)>>()
                            );
                            resources.actors = hash_map;
                        }
                    }
                }
                ModuleDataFileKind::Dialogue => {
                    let dialogue: Vec<Dialogue> = serde_json::from_slice(&bytes)
                        .expect(&format!("Unable to parse module dialogue data file '{}'!", data.path));
                    match data.integration {
                        ModuleIntegration::Extend => {
                            for params in dialogue {
                                resources.dialogue.insert(params.id.clone(), params);
                            }
                        }
                        ModuleIntegration::Replace => {
                            let hash_map = HashMap::from_iter(
                                dialogue
                                    .into_iter()
                                    .map(|params| (params.id.clone(), params))
                                    .collect::<Vec<(String, Dialogue)>>()
                            );
                            resources.dialogue = hash_map;
                        }
                    }
                }
                ModuleDataFileKind::Missions => {
                    let missions: Vec<MissionParams> = serde_json::from_slice(&bytes)
                        .expect(&format!("Unable to parse module missions data file '{}'!", data.path));
                    match data.integration {
                        ModuleIntegration::Extend => {
                            for params in missions {
                                resources.missions.insert(params.id.clone(), params);
                            }
                        }
                        ModuleIntegration::Replace => {
                            let hash_map = HashMap::from_iter(
                                missions
                                    .into_iter()
                                    .map(|params| (params.id.clone(), params))
                                    .collect::<Vec<(String, MissionParams)>>()
                            );
                            resources.missions = hash_map;
                        }
                    }
                }
                ModuleDataFileKind::Items => {
                    let items: Vec<ItemParams> = serde_json::from_slice(&bytes)
                        .expect(&format!("Unable to parse module items data file '{}'!", data.path));
                    match data.integration {
                        ModuleIntegration::Extend => {
                            for params in items {
                                resources.items.insert(params.id.clone(), params);
                            }
                        }
                        ModuleIntegration::Replace => {
                            let hash_map = HashMap::from_iter(
                                items
                                    .into_iter()
                                    .map(|params| (params.id.clone(), params))
                                    .collect::<Vec<(String, ItemParams)>>()
                            );
                            resources.items = hash_map;
                        }
                    }
                }
                ModuleDataFileKind::Abilities => {
                    let abilities: Vec<AbilityParams> = serde_json::from_slice(&bytes)
                        .expect(&format!("Unable to parse module abilities data file '{}'!", data.path));
                    match data.integration {
                        ModuleIntegration::Extend => {
                            for params in abilities {
                                resources.abilities.insert(params.id.clone(), params);
                            }
                        }
                        ModuleIntegration::Replace => {
                            let hash_map = HashMap::from_iter(
                                abilities
                                    .into_iter()
                                    .map(|params| (params.id.clone(), params))
                                    .collect::<Vec<(String, AbilityParams)>>()
                            );
                            resources.abilities = hash_map;
                        }
                    }
                }
                ModuleDataFileKind::Scenario => {
                    let scenario_params: Vec<ChapterParams> = serde_json::from_slice(&bytes)
                        .expect(&format!("Unable to parse scenario file '{}'!", data.path));

                    if data.integration == ModuleIntegration::Replace {
                        resources.chapters = Vec::new();
                    }

                    for mut chapter_params in scenario_params {
                        chapter_params.maps.iter_mut().for_each(|map_params| {
                                map_params.path = format!("{}/{}/{}", game_params.modules_path, module_name, map_params.path);
                            });

                        let chapter = Chapter::new(chapter_params).await?;

                        resources.chapters.push(chapter);
                    }
                }
            }
        }

        if let Some(module_assets) = module_params.assets {
            {
                let mut materials = HashMap::new();
                for material_params in module_assets.materials.files {
                    let vertex_shader = load_file(&format!("{}/{}", module_path, material_params.vertex_shader_path)).await.unwrap();
                    let fragment_shader = load_file(&format!("{}/{}", module_path, material_params.fragment_shader_path)).await.unwrap();

                    let material = load_material(
                        &String::from_utf8(vertex_shader).unwrap(),
                        &String::from_utf8(fragment_shader).unwrap(),
                        MaterialParams {
                            ..Default::default()
                        },
                    ).unwrap();

                    materials.insert(material_params.id, material);
                }

                match module_assets.materials.integration {
                    ModuleIntegration::Extend => {
                        for (id, material) in materials {
                            resources.materials.insert(id, material);
                        }
                    }
                    ModuleIntegration::Replace => resources.materials = materials,
                }
            }
            {
                let mut textures = HashMap::new();
                for texture_params in module_assets.textures.files {
                    let texture = load_texture(&format!("{}/{}", module_path, texture_params.path)).await.unwrap();
                    texture.set_filter(texture_params.filter_mode);
                    textures.insert(texture_params.id.clone(), texture);
                }

                match module_assets.textures.integration {
                    ModuleIntegration::Extend => {
                        for (id, texture) in textures {
                            resources.textures.insert(id, texture);
                        }
                    }
                    ModuleIntegration::Replace => resources.textures = textures,
                }
            }
            {
                let mut sound_effects = HashMap::new();
                for sound_params in module_assets.sound_effects.files {
                    let sound = load_sound(&format!("{}/{}", module_path, sound_params.path)).await.unwrap();
                    sound_effects.insert(sound_params.id, sound);
                }

                match module_assets.sound_effects.integration {
                    ModuleIntegration::Extend => {
                        for (id, sound_effect) in sound_effects {
                            resources.sound_effects.insert(id, sound_effect);
                        }
                    }
                    ModuleIntegration::Replace => resources.sound_effects = sound_effects,
                }
            }
            {
                let mut music = HashMap::new();
                for music_params in module_assets.music.files {
                    let music_file = load_sound(&format!("{}/{}", module_path, music_params.path)).await.unwrap();
                    music.insert(music_params.id, music_file);
                }

                match module_assets.music.integration {
                    ModuleIntegration::Extend => {
                        for (id, music_file) in music {
                            resources.music.insert(id, music_file);
                        }
                    }
                    ModuleIntegration::Replace => resources.music = music,
                }
            }
        }

        loaded_modules.push((module_name, module_params.version));
    }

    Ok(())
}

pub fn get_available_modules(modules_path: &str) -> io::Result<HashMap<String, ModuleParams>> {
    let mut res = HashMap::new();
    for entry in fs::read_dir(modules_path)? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap().to_str().unwrap();
                let module_file_path = path.join(Path::new("module.json"));
                if module_file_path.exists() {
                    let bytes = fs::read(module_file_path)?;
                    let module = serde_json::from_slice(&bytes)?;
                    res.insert(name.to_string(), module);
                }
            }
        }
    }

    Ok(res)
}
