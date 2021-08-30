use std::path::{Path, PathBuf};

use crate::prelude::*;
use crate::json::tiled::TiledMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileCollisionDefinition {
    pub layer_id: String,
    #[serde(rename = "kind")]
    pub collision_kind: MapCollisionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledTilesetDefinition {
    pub tileset_id: String,
    pub texture_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledMapDefinition {
    pub path: String,
    pub export_path: String,
    pub collisions: Vec<TileCollisionDefinition>,
    pub tilesets: Vec<TiledTilesetDefinition>,
}

pub async fn convert_tiled_maps(manifest_path: &str) -> Result<(), FileError> {
    let folder_path = remove_filename(PathBuf::from(manifest_path));

    let bytes = load_file(&manifest_path).await?;
    let tiled_maps: Vec<TiledMapDefinition> = serde_json::from_slice(&bytes).unwrap();

    for mut def in tiled_maps {
        def.path = folder_path.join(Path::new(&def.path)).to_string_lossy().to_string();
        def.export_path = folder_path.join(Path::new(&def.export_path)).to_string_lossy().to_string();

        Map::load_tiled(def).await?;
    }

    Ok(())
}

pub fn convert_tiled_maps_sync(manifest_path: &str) -> io::Result<()> {
    let folder_path = remove_filename(PathBuf::from(manifest_path));

    let bytes = fs::read(&manifest_path)?;
    let tiled_maps: Vec<TiledMapDefinition> = serde_json::from_slice(&bytes).unwrap();

    for mut def in tiled_maps {
        def.path = folder_path.join(Path::new(&def.path)).to_string_lossy().to_string();
        def.export_path = folder_path.join(Path::new(&def.export_path)).to_string_lossy().to_string();

        Map::load_tiled_sync(def)?;
    }

    Ok(())
}
