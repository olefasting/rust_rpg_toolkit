mod map;
mod tiled;

pub use map::{
    Map,
    MapLayerKind,
    MapLayer,
    MapTile,
    MapTileIterator,
    MapObject,
    MapTileset,
    MapCollisionKind
};

pub use tiled::{
    TiledMap,
    TiledTileset,
    TiledMapDeclaration,
};

pub const MAP_LAYER_GROUND: &'static str = "ground";
pub const MAP_LAYER_SOLIDS: &'static str = "solids";
pub const MAP_LAYER_BARRIERS: &'static str = "barriers";
pub const MAP_LAYER_ITEMS: &'static str = "items";
pub const MAP_LAYER_SPAWN_POINTS: &'static str = "spawn_points";
