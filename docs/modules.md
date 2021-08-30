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

Module dependencies can be declared with or without a version requirement. If no version is specified, the dependency is considered met if a module by the specified name exist earlier in the load order.

Version requirements can be specified for individual module dependencies, as well as for the toolkit and game, in thew module's declaration. All version requirements can be without a patch to match only against major and minor version. By default a version requirement is only considered met on an exact match. All requirements can be prefixed with a `^`, to match on anything below and including the version requirement. `

```rust
struct ModuleDependencyParams {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
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
#[serde(rename_all = "snake_case")]
enum ModuleDataFileKind {
    Actors,
    Dialogue,
    Missions,
    Items,
    Abilities,
}
```

```rust
#[serde(rename_all = "snake_case")]
enum ModuleIntegration {
    Extend,
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
