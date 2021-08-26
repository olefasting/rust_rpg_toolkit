use std::{
    collections::HashMap,
    iter::FromIterator,
    io,
};

use macroquad::prelude::*;

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    Resources,
    nodes::{
        ActorParams,
        ItemParams,
        actor::ActorDialogue
    },
    missions::MissionParams,
    ability::AbilityParams,
    generate_id
};

use crate::resources::{MaterialInfo, TextureInfo, SoundInfo};
use crate::render::{LINEAR_FILTER_MODE, NEAREST_FILTER_MODE};
use macroquad::audio::load_sound;

const ACTIVE_MODULES_FILE_PATH: &'static str = "modules/active_modules.json";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModuleDataFileKind {
    #[serde(rename = "actors")]
    Actors,
    #[serde(rename = "dialogue")]
    Dialogue,
    #[serde(rename = "missions")]
    Missions,
    #[serde(rename = "items")]
    Items,
    #[serde(rename = "abilities")]
    Abilities,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModuleIntegration {
    #[serde(rename = "extend")]
    Extend,
    #[serde(rename = "replace")]
    Replace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDataInfo {
    pub kind: ModuleDataFileKind,
    pub path: String,
    pub integration: ModuleIntegration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDependencyInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMaterials {
    pub integration: ModuleIntegration,
    pub files: Vec<MaterialInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleTextures {
    pub integration: ModuleIntegration,
    pub files: Vec<TextureInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleSounds {
    pub integration: ModuleIntegration,
    pub files: Vec<SoundInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleResourcesInfo {
    materials: ModuleMaterials,
    textures: ModuleTextures,
    sound_effects: ModuleSounds,
    music: ModuleSounds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDeclaration {
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub version: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<ModuleDependencyInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<ModuleDataInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ModuleResourcesInfo>,
}

impl Default for ModuleDeclaration {
    fn default() -> Self {
        ModuleDeclaration {
            display_name: "Unnamed Module".to_string(),
            description: "".to_string(),
            version: "not versioned".to_string(),
            dependencies: Vec::new(),
            data: Vec::new(),
            resources: None,
        }
    }
}

pub async fn load_modules(resources: &mut Resources) {
    let mut loaded_modules: Vec<ModuleDependencyInfo> = Vec::new();

    let bytes = load_file(ACTIVE_MODULES_FILE_PATH).await
        .expect(&format!("Unable to find active modules file '{}'!", ACTIVE_MODULES_FILE_PATH));
    let active_modules: Vec<String> = serde_json::from_slice(&bytes)
        .expect(&format!("Unable to parse active modules file '{}'!", ACTIVE_MODULES_FILE_PATH));
    'module: for module_name in active_modules {
        let module_path = format!("modules/{}", module_name);
        let module_file_path = format!("{}/{}.json", module_path, module_name);
        let bytes = load_file(&module_file_path).await
            .expect(&format!("Unable to find module file '{}'!", module_file_path));
        let module_info: ModuleDeclaration = serde_json::from_slice(&bytes)
            .expect(&format!("Unable to parse module file '{}'!", module_file_path));
        for dependency in module_info.dependencies {
            let found = loaded_modules.iter().find(|loaded|
                loaded.name.clone() == dependency.name && loaded.version.clone() == dependency.version);
            if found.is_none() {
                println!("WARNING: Dependency '{}', version '{}', unmet for module '{}'!", dependency.name, dependency.version, module_name);
                continue 'module;
            }
        }
        for data in module_info.data {
            let bytes = load_file(&format!("{}/{}", module_path, data.path)).await
                .expect(&format!("Unable to find module data file '{}'!", data.path));
            match data.kind {
                ModuleDataFileKind::Actors => {
                    let actors: Vec<ActorParams> = serde_json::from_slice(&bytes)
                        .expect(&format!("Unable to parse module actor data file '{}'!", data.path));
                    match data.integration {
                        ModuleIntegration::Extend => {
                            for params in actors {
                                resources.actors.insert(params.prototype_id.clone().unwrap_or(generate_id()), params);
                            }
                        },
                        ModuleIntegration::Replace => {
                            let hash_map = HashMap::from_iter(
                                actors
                                    .into_iter()
                                    .map(|params| (params.prototype_id.clone().unwrap_or(generate_id()), params))
                                    .collect::<Vec<(String, ActorParams)>>()
                            );
                            resources.actors = hash_map;
                        }
                    }
                },
                ModuleDataFileKind::Dialogue => {
                    let dialogue: Vec<ActorDialogue> = serde_json::from_slice(&bytes)
                        .expect(&format!("Unable to parse module dialogue data file '{}'!", data.path));
                    match data.integration {
                        ModuleIntegration::Extend => {
                            for params in dialogue {
                                resources.dialogue.insert(params.id.clone(), params);
                            }
                        },
                        ModuleIntegration::Replace => {
                            let hash_map = HashMap::from_iter(
                                dialogue
                                    .into_iter()
                                    .map(|params| (params.id.clone(), params))
                                    .collect::<Vec<(String, ActorDialogue)>>()
                            );
                            resources.dialogue = hash_map;
                        }
                    }
                },
                ModuleDataFileKind::Missions => {
                    let missions: Vec<MissionParams> = serde_json::from_slice(&bytes)
                        .expect(&format!("Unable to parse module missions data file '{}'!", data.path));
                    match data.integration {
                        ModuleIntegration::Extend => {
                            for params in missions {
                                resources.missions.insert(params.id.clone(), params);
                            }
                        },
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
                },
                ModuleDataFileKind::Items => {
                    let items: Vec<ItemParams> = serde_json::from_slice(&bytes)
                        .expect(&format!("Unable to parse module items data file '{}'!", data.path));
                    match data.integration {
                        ModuleIntegration::Extend => {
                            for params in items {
                                resources.items.insert(params.prototype_id.clone(), params);
                            }
                        },
                        ModuleIntegration::Replace => {
                            let hash_map = HashMap::from_iter(
                                items
                                    .into_iter()
                                    .map(|params| (params.prototype_id.clone(), params))
                                    .collect::<Vec<(String, ItemParams)>>()
                            );
                            resources.items = hash_map;
                        }
                    }
                },
                ModuleDataFileKind::Abilities => {
                    let abilities: Vec<AbilityParams> = serde_json::from_slice(&bytes)
                        .expect(&format!("Unable to parse module abilities data file '{}'!", data.path));
                    match data.integration {
                        ModuleIntegration::Extend => {
                            for params in abilities {
                                resources.abilities.insert(params.id.clone(), params);
                            }
                        },
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
            }
        }

        if let Some(module_resources) = module_info.resources {
            {
                let mut materials = HashMap::new();
                for material_info in module_resources.materials.files {
                    let vertex_shader = load_file(&format!("{}/{}", module_path, material_info.vertex_shader_path)).await.unwrap();
                    let fragment_shader = load_file(&format!("{}/{}", module_path, material_info.fragment_shader_path)).await.unwrap();

                    let material = load_material(
                        &String::from_utf8(vertex_shader).unwrap(),
                        &String::from_utf8(fragment_shader).unwrap(),
                        MaterialParams {
                            ..Default::default()
                        },
                    ).unwrap();

                    materials.insert(material_info.id, material);
                }
                match module_resources.materials.integration {
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
                for texture_info in module_resources.textures.files {
                    let texture = load_texture(&format!("{}/{}", module_path, texture_info.path)).await.unwrap();
                    if texture_info.filter_mode == LINEAR_FILTER_MODE.to_string() {
                        texture.set_filter(FilterMode::Linear)
                    } else if texture_info.filter_mode == NEAREST_FILTER_MODE.to_string() {
                        texture.set_filter(FilterMode::Nearest);
                    } else {
                        assert!(false, "Invalid filter mode '{}'", texture_info.filter_mode);
                    }
                    textures.insert(texture_info.id.clone(), texture);
                }
                match module_resources.textures.integration {
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
                for sound_info in module_resources.sound_effects.files {
                    let sound = load_sound(&format!("{}/{}", module_path, sound_info.path)).await.unwrap();
                    sound_effects.insert(sound_info.id, sound);
                }
                match module_resources.sound_effects.integration {
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
                for music_info in module_resources.music.files {
                    let music_file = load_sound(&format!("{}/{}", module_path, music_info.path)).await.unwrap();
                    music.insert(music_info.id, music_file);
                }
                match module_resources.music.integration {
                    ModuleIntegration::Extend => {
                        for (id, music_file) in music {
                            resources.music.insert(id, music_file);
                        }
                    }
                    ModuleIntegration::Replace => resources.music = music,
                }
            }
        }

        loaded_modules.push(ModuleDependencyInfo {
            name: module_name,
            version: module_info.version,
        })
    }
}
