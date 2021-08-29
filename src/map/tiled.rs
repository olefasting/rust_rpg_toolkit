use crate::prelude::*;
use std::{
    ops::Deref,
    path::Path,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledMapCollisionDeclaration {
    pub layer_id: String,
    #[serde(rename = "kind")]
    pub collision_kind: MapCollisionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledTilesetDeclaration {
    pub name: String,
    pub relative_texture_path: String,
    pub texture_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledMapDeclaration {
    pub path: String,
    pub export_path: String,
    pub collisions: Vec<TiledMapCollisionDeclaration>,
    pub tilesets: Vec<TiledTilesetDeclaration>,
}
