# Modules

Modules can be used to extend the game without having to edit the game's data files directly.
They are placed in the `modules` directory, inside the game folder, and added to the array in the `modules/active_modules.json` file. Load order is also determined by the order of this array.

Each module folder must contain a module declaration json-file, with the same name as the module, where the module data- and resource files are declared.

```rust
struct ModuleDeclaration {
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
    pub dependencies: Vec<ModuleDependencyInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<ModuleDataInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ModuleResourcesInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario: Option<ModuleScenarioParams>,
}
```

All file paths in a module's declaration file is relative to the module's root folder.

## Dependencies

A module can depend on other modules. Currently, the versions must match exactly but in the future we will implement more sophisticated version matching. We will also introduce game version dependency, at some point.

```rust
struct ModuleDependencyParams {
    pub name: String,
    pub version: String,
}
```

Name refers to a module's folder name, not the `title` from its declaration.

## Resources

The `resources` field of a module's declaration, contain information about resource files, lie textures, materials and sounds, that are included in the module.
Just as with data files, they have an `integration` field that control whether they will replace the internal game resources, as well as any resources added by previously loaded modules.

```rust
struct ModuleResourcesParams {
    materials: ModuleMaterials,
    textures: ModuleTextures,
    sound_effects: ModuleSounds,
    music: ModuleSounds,
}
```

```rust
struct ModuleMaterials {
    pub integration: ModuleIntegration,
    pub files: Vec<MaterialInfo>
}
```

```rust
struct ModuleTextures {
    pub integration: ModuleIntegration,
    pub files: Vec<TextureInfo>
}
```

```rust
struct ModuleSounds {
    pub integration: ModuleIntegration,
    pub files: Vec<SoundInfo>
}
```

## Data

A module can include data files which can either replace or extend the games corresponding data files.

```rust
struct ModuleDataParams {
    pub kind: ModuleDataFileKind,
    pub path: String,
    pub integration: ModuleIntegration,
}
```

```rust
enum ModuleDataFileKind {
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
```

```rust
enum ModuleIntegration {
    #[serde(rename = "extend")]
    Extend,
    #[serde(rename = "replace")]
    Replace,
}
```

The `integration` field is used to determine whether the module's data will be a complete replacement for the game data.
Please note that if you load several modules which has `replace` integrations, they will overwrite each other. Also, if you use `extend` integration, data entries with the same `id` will overwrite both internal data, and data from previously loaded modules.

## Scenario

A module can also contain scenario data that either extend or replace the one provided by the game. If it is set to extend, any chapters defined in the module declaration, will be appended to the game's own list of chapters.

```rust
struct ModuleScenarioParams {
    pub integration: ModuleIntegration,
    pub chapters: Vec<ChapterParams>,
}
```

See the documentation for scenarios and chapters for more details on how these are defined.
