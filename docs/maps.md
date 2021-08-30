# Maps


Actor spawn points can be created in an object layer named `spawn_points`, with a property named `prototype_id` that points to a prototype in the `actors.json` file. Likewise, items can be added in a layer named `items`, with a `prototype_id` property, pointing to a prototype in `items.json`.
Both actors and items can also have an `instance_id` property that determines its unique id when spawned in-game. This can be used to identify them in quest definitions, for example.

As of writing this, only properties with `String` values are supported. This means that all properties should be either `string` or `color` (Tiled types) and converted in-code.

## Tiled Conversion

Tiled maps can be converted using the `mapconv` tool, in the `cli` folder of this repo, or you can have the engine convert for you.

Both methods require a manifest file, containing an array of definitions for each map, that specify needed information not provided by the Tiled format, like `texture_id` of tileset
textures and collision information for the map's layers.

```rust
struct TiledMapDefinition {
    pub path: String,
    pub export_path: String,
    pub collisions: Vec<TiledMapCollisionDefinition>,
    pub tilesets: Vec<TiledTilesetDefinition>,
}
```

```rust
struct TiledMapCollisionDefinition {
    pub layer_id: String,
    #[serde(rename = "kind")]
    pub collision_kind: MapCollisionKind,
}
```

```rust
enum MapCollisionKind {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "barrier")]
    Barrier,
    #[serde(rename = "solid")]
    Solid,
}
```

```rust
struct TiledTilesetDefinition {
    pub layer_id: String,
    pub texture_id: String,
}
```
