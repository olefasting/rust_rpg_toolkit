use macroquad_tiled as tiled;
use std::collections::HashMap;
use std::iter::FromIterator;

use crate::{get_global, Resources};
use macroquad::prelude::Texture2D;
use std::ops::Deref;

pub struct TiledMap {
    pub tiled_map: tiled::Map,
    pub tiled_tilesets: HashMap<String, TiledTileset>,
}

impl TiledMap {
    pub fn new(path: &str, tiled_tilesets: &[(&str, &str, &str)]) -> Self {
        let tiled_tilesets = HashMap::from_iter(tiled_tilesets
                .iter()
                .map(|(name, relative_path, texture_id)| (name.to_string(), TiledTileset {
                    relative_path: relative_path.to_string(),
                    texture_id: texture_id.to_string(),
                })));
        let resources = get_global::<Resources>();
        let mut tileset_textures = Vec::new();
        for (_, tileset) in &tiled_tilesets {
            let texture = resources.textures
                .get(&tileset.texture_id)
                .expect(&format!("Unable to find texture with id '{}'", tileset.texture_id));
            tileset_textures.push((tileset.relative_path.deref(), texture.clone()));
        }
        let json = std::fs::read_to_string(path).expect(&format!("Error loading tiled map file '{}'!", path));
        let tiled_map = tiled::load_map(&json, tileset_textures.deref(), &[]).expect(&format!("Error parsing tiled map '{}'!", path));

        TiledMap {
            tiled_map,
            tiled_tilesets,
        }
    }
}

pub struct TiledTileset {
    pub relative_path: String,
    pub texture_id: String,
}
