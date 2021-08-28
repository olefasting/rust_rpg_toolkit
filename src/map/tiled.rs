use macroquad_tiled as tiled;

use crate::prelude::*;
use std::ops::Deref;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledTileset {
    pub relative_texture_path: String,
    pub texture_id: String,
}

pub struct TiledMap {
    pub tiled_map: tiled::Map,
    pub tiled_tilesets: HashMap<String, TiledTileset>,
    pub collisions: HashMap<String, MapCollisionKind>,
}

impl TiledMap {
    pub async fn load(decl: TiledMapDeclaration) -> Self {
        let tiled_tilesets = HashMap::from_iter(decl.tilesets
            .iter()
            .map(|decl| (decl.name.to_string(), TiledTileset {
                relative_texture_path: decl.relative_texture_path.to_string(),
                texture_id: decl.texture_id.to_string(),
            })));
        let resources = storage::get::<Resources>();
        let mut tileset_textures = Vec::new();
        for (_, tileset) in &tiled_tilesets {
            let texture = resources.textures
                .get(&tileset.texture_id)
                .expect(&format!("Unable to find texture with id '{}'", tileset.texture_id));
            tileset_textures.push((tileset.relative_texture_path.deref(), texture.clone()));
        }
        let game_params = storage::get::<GameParams>();
        let bytes = load_file(&format!("{}/{}", game_params.assets_path, &decl.path)).await
            .expect(&format!("Error loading tiled map file '{}'!", &decl.path));
        let tiled_map = tiled::load_map(&String::from_utf8(bytes).unwrap(), tileset_textures.deref(), &[])
            .expect(&format!("Error parsing tiled map '{}'!", &decl.path));

        let collisions = HashMap::from_iter(decl.collisions
                .into_iter()
                .map(|decl| (decl.layer_id.to_string(), decl.collision_kind.clone())).collect::<Vec<(String, MapCollisionKind)>>());

        TiledMap {
            tiled_map,
            tiled_tilesets,
            collisions,
        }
    }
}

impl Into<Map> for TiledMap {
    fn into(self) -> Map {
        let raw_tiled_map = &self.tiled_map.raw_tiled_map;
        let tile_size = vec2(raw_tiled_map.tilewidth as f32, raw_tiled_map.tileheight as f32);
        let grid_size = uvec2(raw_tiled_map.width, raw_tiled_map.height);

        let mut tileset_ids = Vec::new();
        let tilesets = HashMap::from_iter(
            raw_tiled_map.tilesets
                .iter()
                .map(|raw_tiled_tileset| {
                    let tiled_tileset = self.tiled_tilesets
                        .get(&raw_tiled_tileset.name)
                        .expect(&format!("Unable to find definition for tileset with image '{}'! Did you remember to define all the tilesets when importing the TiledMap?", raw_tiled_tileset.image));

                    let id = if tileset_ids.contains(&raw_tiled_tileset.name) {
                        format!("{}_{}", raw_tiled_tileset.name, generate_id())
                    } else {
                        tileset_ids.push(raw_tiled_tileset.name.clone());
                        raw_tiled_tileset.name.clone()
                    };

                    let tileset = MapTileset {
                        id: id.clone(),
                        texture_id: tiled_tileset.texture_id.clone(),
                        texture_size: uvec2(raw_tiled_tileset.imagewidth as u32, raw_tiled_tileset.imageheight as u32),
                        tile_size: uvec2(raw_tiled_tileset.tilewidth as u32, raw_tiled_tileset.tileheight as u32),
                        grid_size: uvec2(raw_tiled_tileset.columns as u32, raw_tiled_tileset.tilecount / raw_tiled_tileset.columns as u32),
                        first_tile_id: raw_tiled_tileset.firstgid,
                        tile_cnt: raw_tiled_tileset.tilecount,
                    };

                    (id, tileset)
                }));

        let draw_order = raw_tiled_map.layers
            .iter()
            .map(|layer| layer.name.clone())
            .collect();

        let layers = HashMap::from_iter(
            self.tiled_map.layers
                .iter()
                .map(|(layer_id, tiled_layer)| {
                    let raw_tiled_layer = raw_tiled_map.layers.iter().find(|raw_layer| raw_layer.name == layer_id.to_string())
                        .expect(&format!("Unable to find tiled layer '{}' in the raw tiled map!", layer_id));
                    let is_visible = raw_tiled_layer.visible;
                    let (kind, tiles, objects) = if tiled_layer.objects.len() > 0 {
                        let objects = tiled_layer.objects
                            .iter()
                            .cloned()
                            .map(|tiled_object| {
                                let size = if tiled_object.world_w != 0.0 || tiled_object.world_h != 0.0 {
                                    Some(vec2(tiled_object.world_w, tiled_object.world_h))
                                } else {
                                    None
                                };
                                MapObject {
                                    name: tiled_object.name.clone(),
                                    position: vec2(tiled_object.world_x, tiled_object.world_y),
                                    size,
                                    properties: tiled_object.properties,
                                }
                            }).collect();

                        (MapLayerKind::ObjectLayer, Vec::new(), objects)
                    } else {
                        let tiles = tiled_layer.data
                            .iter()
                            .map(|tiled_tile| {
                                if let Some(tiled_tile) = tiled_tile {
                                    let tileset = tilesets.get(&tiled_tile.tileset)
                                        .expect(&format!("Unable to find tiled tileset '{}'! Are you sure you defined all the tilesets when importing the map?", tiled_tile.tileset));
                                    let tile = MapTile {
                                        tile_id: tiled_tile.id,
                                        tileset_id: tileset.id.clone(),
                                        texture_id: tileset.texture_id.clone(),
                                        texture_coords: tileset.get_texture_coords(tiled_tile.id),
                                    };

                                    Some(tile)
                                } else {
                                    None
                                }
                            }).collect();

                        (MapLayerKind::TileLayer, tiles, Vec::new())
                    };

                    let collision = match self.collisions.get(layer_id) {
                        Some(collision) => collision.clone(),
                        _ => MapCollisionKind::None,
                    };

                    let layer = MapLayer {
                        id: layer_id.clone(),
                        kind,
                        collision,
                        grid_size,
                        tiles,
                        objects,
                        is_visible,
                    };

                    (layer_id.clone(), layer)
                }));

        let background_color = try_color_from_hex_string(&raw_tiled_map.backgroundcolor)
            .unwrap_or(Map::default_background_color());

        Map {
            background_color,
            world_offset: Vec2::ZERO,
            grid_size,
            tile_size,
            layers,
            tilesets,
            draw_order,
        }
    }
}
