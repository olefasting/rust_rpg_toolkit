use std::path::{Path, PathBuf};

use crate::prelude::*;
use crate::json::tiled::TiledMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledMapCollisionDeclaration {
    pub layer_id: String,
    #[serde(rename = "kind")]
    pub collision_kind: MapCollisionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledTilesetDeclaration {
    pub tileset_id: String,
    pub texture_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledMapDeclaration {
    pub path: String,
    pub export_path: String,
    pub collisions: Vec<TiledMapCollisionDeclaration>,
    pub tilesets: Vec<TiledTilesetDeclaration>,
}

pub async fn convert_tiled_maps(manifest_path: &str) -> Result<(), FileError> {
    let folder_path = remove_filename(PathBuf::from(manifest_path));

    let bytes = load_file(&manifest_path).await?;
    let tiled_maps: Vec<TiledMapDeclaration> = serde_json::from_slice(&bytes).unwrap();

    for mut decl in tiled_maps {
        decl.path = folder_path.join(Path::new(&decl.path)).to_string_lossy().to_string();
        decl.export_path = folder_path.join(Path::new(&decl.export_path)).to_string_lossy().to_string();

        Map::load_tiled(decl).await?;
    }

    Ok(())
}

pub fn convert_tiled_maps_sync(manifest_path: &str) -> io::Result<()> {
    let folder_path = remove_filename(PathBuf::from(manifest_path));

    let bytes = fs::read(&manifest_path)?;
    let tiled_maps: Vec<TiledMapDeclaration> = serde_json::from_slice(&bytes).unwrap();

    for mut decl in tiled_maps {
        decl.path = folder_path.join(Path::new(&decl.path)).to_string_lossy().to_string();
        decl.export_path = folder_path.join(Path::new(&decl.export_path)).to_string_lossy().to_string();

        Map::load_tiled_sync(decl)?;
    }

    Ok(())
}
