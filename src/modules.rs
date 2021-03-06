use std::path::Path;

use crate::prelude::*;

use crate::resources::{
    FontAssetParams, ImageAssetParams, MaterialAssetParams, SoundAssetParams, TextureAssetParams,
};

use macroquad::texture::load_texture;

pub const ACTIVE_MODULES_FILE_NAME: &str = "active_modules.json";
pub const MODULE_FILE_NAME: &str = "module.json";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ModuleDataFileKind {
    Actors,
    Dialogue,
    Missions,
    Items,
    Abilities,
    Scenario,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ModuleIntegration {
    Extend,
    Replace,
}

impl Default for ModuleIntegration {
    fn default() -> Self {
        Self::Extend
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ModuleDataParams {
    pub kind: ModuleDataFileKind,
    pub path: String,
    pub integration: ModuleIntegration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ModuleDependencyParams {
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct ModuleMaterials {
    pub integration: ModuleIntegration,
    pub files: Vec<MaterialAssetParams>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct ModuleTextures {
    pub integration: ModuleIntegration,
    pub files: Vec<TextureAssetParams>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct ModuleImages {
    pub integration: ModuleIntegration,
    pub files: Vec<ImageAssetParams>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct ModuleFonts {
    pub integration: ModuleIntegration,
    pub files: Vec<FontAssetParams>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct ModuleSounds {
    pub integration: ModuleIntegration,
    pub files: Vec<SoundAssetParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ModuleAssetsParams {
    #[serde(default)]
    materials: ModuleMaterials,
    #[serde(default)]
    textures: ModuleTextures,
    #[serde(default)]
    images: ModuleImages,
    #[serde(default)]
    fonts: ModuleFonts,
    #[serde(default)]
    sound_effects: ModuleSounds,
    #[serde(default)]
    music: ModuleSounds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ModuleParams {
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

pub(crate) async fn load_modules(
    game_params: &GameParams,
    resources: &mut Resources,
) -> Result<()> {
    let modules_path = Path::new(&game_params.modules_path);

    let mut loaded_modules: Vec<(String, String)> = Vec::new();

    let active_modules_file_path = modules_path.join("active_modules.json");
    let bytes = load_file(active_modules_file_path).await?;
    let active_modules: Vec<String> = serde_json::from_slice(&bytes)?;

    'module: for module_name in active_modules {
        let module_path = modules_path.join(&module_name);
        let module_file_path = module_path.join(MODULE_FILE_NAME);

        if !module_path.exists() || !module_file_path.exists() {
            println!("WARNING: Module '{}' could not be found, even though it is listed in the active modules file!", &module_name);
            continue 'module;
        }

        let bytes = load_file(&module_file_path).await?;
        let module_params: ModuleParams = serde_json::from_slice(&bytes)?;

        if let Some(required_game_version) = &module_params.required_game_version {
            if !check_version(required_game_version, &game_params.version) {
                println!("WARNING: Module '{}' was not loaded as its game version requirement '{}' was unmet (game version is '{}')!", &module_name, &required_game_version, &game_params.version);
                continue 'module;
            }
        }

        let toolkit_version = get_toolkit_version();
        if let Some(required_toolkit_version) = &module_params.required_toolkit_version {
            if !check_version(required_toolkit_version, &toolkit_version) {
                println!("WARNING: Module '{}' was not loaded as its toolkit version requirement '{}' was unmet (toolkit version is '{}')!", &module_name, &required_toolkit_version, get_toolkit_version());
                continue 'module;
            }
        }

        for dependency in module_params.dependencies {
            if !loaded_modules.iter().any(|(name, version)| {
                if let Some(required_version) = &dependency.version {
                    return *name == dependency.name && check_version(required_version, version);
                }
                *name == dependency.name
            }) {
                println!(
                    "WARNING: Module '{}' was not loaded as its dependency on '{}' was unmet!",
                    &module_name, &dependency.name
                );
                continue 'module;
            }
        }

        for data in module_params.data {
            let data_file_path = module_path.join(&data.path);
            let bytes = load_file(data_file_path).await?;
            match data.kind {
                ModuleDataFileKind::Actors => {
                    let actors: Vec<ActorParams> = serde_json::from_slice(&bytes)?;
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
                                    .collect::<Vec<(String, ActorParams)>>(),
                            );
                            resources.actors = hash_map;
                        }
                    }
                }
                ModuleDataFileKind::Dialogue => {
                    let dialogue: Vec<Dialogue> = serde_json::from_slice(&bytes)?;
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
                                    .collect::<Vec<(String, Dialogue)>>(),
                            );
                            resources.dialogue = hash_map;
                        }
                    }
                }
                ModuleDataFileKind::Missions => {
                    let missions: Vec<MissionParams> = serde_json::from_slice(&bytes)?;
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
                                    .collect::<Vec<(String, MissionParams)>>(),
                            );
                            resources.missions = hash_map;
                        }
                    }
                }
                ModuleDataFileKind::Items => {
                    let items: Vec<ItemParams> = serde_json::from_slice(&bytes)?;
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
                                    .collect::<Vec<(String, ItemParams)>>(),
                            );
                            resources.items = hash_map;
                        }
                    }
                }
                ModuleDataFileKind::Abilities => {
                    let abilities: Vec<AbilityParams> = serde_json::from_slice(&bytes)?;
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
                                    .collect::<Vec<(String, AbilityParams)>>(),
                            );
                            resources.abilities = hash_map;
                        }
                    }
                }
                ModuleDataFileKind::Scenario => {
                    let scenario_params: Vec<ChapterParams> = serde_json::from_slice(&bytes)?;

                    if data.integration == ModuleIntegration::Replace {
                        resources.chapters = Vec::new();
                    }

                    for mut params in scenario_params {
                        params.maps.iter_mut().for_each(|map_params| {
                            let path = Path::new(&game_params.modules_path)
                                .join(&module_name)
                                .join(&map_params.path);

                            map_params.path = path.to_string_lossy().to_string();
                        });

                        let chapter = Chapter::new(game_params, params).await?;

                        resources.chapters.push(chapter);
                    }
                }
            }
        }

        if let Some(module_assets) = module_params.assets {
            {
                let mut materials = HashMap::new();
                for asset_params in module_assets.materials.files {
                    let id = asset_params.id.clone();
                    let vertex_path = module_path.join(&asset_params.vertex_path);
                    let fragment_path = module_path.join(&asset_params.fragment_path);

                    let material =
                        Material::new(vertex_path, fragment_path, asset_params.into()).await?;

                    materials.insert(id, material);
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
                for params in module_assets.textures.files {
                    let path = module_path.join(&params.path);
                    let texture = load_texture(path.to_string_lossy().as_ref()).await?;
                    texture.set_filter(params.filter_mode);

                    let mut height_map = None;
                    if let Some(path) = &params.height_map_path {
                        let path = module_path.join(path);
                        let res = load_texture(path.to_string_lossy().as_ref()).await?;
                        res.set_filter(params.filter_mode);
                        height_map = Some(res);
                    }

                    let mut normal_map = None;
                    if let Some(path) = &params.normal_map_path {
                        let path = module_path.join(path);
                        let res = load_texture(path.to_string_lossy().as_ref()).await?;
                        res.set_filter(params.filter_mode);
                        normal_map = Some(res);
                    }

                    let texture = Texture::new(texture, height_map, normal_map);

                    textures.insert(params.id.clone(), texture);
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
                let mut images = HashMap::new();
                for params in &module_assets.images.files {
                    let path = module_path.join(&params.path);
                    let bytes = load_file(path).await?;
                    let format = match params.format.as_ref() {
                        Some(ext) => ImageFormat::from_extension(ext),
                        _ => None,
                    };

                    let image = Image::from_file_with_format(&bytes, format);
                    images.insert(params.id.clone(), image);
                }

                match module_assets.images.integration {
                    ModuleIntegration::Extend => {
                        for (id, image) in images {
                            resources.images.insert(id, image);
                        }
                    }

                    ModuleIntegration::Replace => resources.images = images,
                }
            }
            {
                let mut font_bytes = HashMap::new();
                for params in &module_assets.fonts.files {
                    let path = module_path.join(&params.path);
                    let bytes = load_file(path).await?;
                    font_bytes.insert(params.id.clone(), bytes);
                }

                match module_assets.fonts.integration {
                    ModuleIntegration::Extend => {
                        for (id, font) in font_bytes {
                            resources.font_bytes.insert(id, font);
                        }
                    }

                    ModuleIntegration::Replace => resources.font_bytes = font_bytes,
                }
            }
            {
                let mut sound_effects = HashMap::new();
                for sound_params in module_assets.sound_effects.files {
                    let path = module_path.join(&sound_params.path);
                    let sound = load_sound(VolumeCategory::SoundEffect, path).await?;
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
                    let path = module_path.join(music_params.path);
                    let music_file = load_sound(VolumeCategory::Music, path).await?;
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

pub(crate) fn get_available_modules() -> Result<HashMap<String, ModuleParams>> {
    let game_params = storage::get::<GameParams>();
    let path = Path::new(&game_params.modules_path);
    let mut res = HashMap::new();
    for entry in (fs::read_dir(path)?).flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_string_lossy();

            let file_path = path.join("module.json");
            if file_path.exists() {
                let bytes = fs::read(file_path)?;
                let module = serde_json::from_slice(&bytes)?;
                res.insert(name.to_string(), module);
            }
        }
    }

    Ok(res)
}
