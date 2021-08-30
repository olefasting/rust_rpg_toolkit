use crate::prelude::*;

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

pub async fn convert_tiled_maps(assets_path: &str) -> Result<(), FileError> {
    let tiled_maps_file_path = format!("{}/tiled_maps.json", assets_path);
    let bytes = load_file(&tiled_maps_file_path).await?;
    let tiled_maps: Vec<TiledMapDeclaration> = serde_json::from_slice(&bytes).unwrap();
    for decl in tiled_maps {
        Map::load_tiled(&assets_path, decl.clone()).await?;
    }
    Ok(())
}
