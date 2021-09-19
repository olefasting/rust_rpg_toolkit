use crate::{
    prelude::*,
};
use crate::map::MapProperty;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TiledProperty {
    Bool { name: String, value: bool },
    Float { name: String, value: f32 },
    Int { name: String, value: i32 },
    String { name: String, value: String },
    Color { name: String, value: String },
    Object { name: String, value: i32 },
    File { name: String, value: String },
}

impl TiledProperty {
    pub fn get_name(&self) -> String {
        match self {
            TiledProperty::Bool { name, value: _ } => name.clone(),
            TiledProperty::Float { name, value: _ } => name.clone(),
            TiledProperty::Int { name, value: _ } => name.clone(),
            TiledProperty::String { name, value: _ } => name.clone(),
            TiledProperty::Color { name, value: _ } => name.clone(),
            TiledProperty::Object { name, value: _ } => name.clone(),
            TiledProperty::File { name, value: _ } => name.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TiledObject {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
    pub visible: bool,
    pub rotation: f32,
    pub ellipse: Option<bool>,
    pub polygon: Option<Vec<TiledPolyPoint>>,
    pub properties: Option<Vec<TiledProperty>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TiledPolyPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TiledTileset {
    pub columns: i32,
    pub image: String,
    pub imagewidth: i32,
    pub imageheight: i32,
    pub margin: i32,
    pub name: String,
    #[serde(default)]
    pub properties: Option<Vec<TiledProperty>>,
    pub spacing: i32,
    pub tileheight: i32,
    pub tilewidth: i32,
    pub firstgid: u32,
    pub tilecount: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TiledLayer {
    pub name: String,
    pub visible: bool,
    #[serde(rename = "type")]
    pub layer_type: String,
    #[serde(default)]
    pub data: Vec<u32>,
    #[serde(default)]
    pub objects: Vec<TiledObject>,
    #[serde(default)]
    pub properties: Option<Vec<TiledProperty>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TiledMap {
    // Optional background color
    pub backgroundcolor: Option<String>,
    // Number of columns in the map grid
    pub width: u32,
    // Number of rows in the map grid
    pub height: u32,
    // Width of the individual tiles
    pub tilewidth: u32,
    // Height of the individual tiles
    pub tileheight: u32,
    // The JSON format version
    pub version: String,
    // The Tiled version used to create the map
    pub tiledversion: String,
    pub layers: Vec<TiledLayer>,
    pub tilesets: Vec<TiledTileset>,
    #[serde(default)]
    pub properties: Option<Vec<TiledProperty>>,
}

impl TiledMap {
    pub const OBJECT_LAYER_KIND_PROP_KEY: &'static str = "object_layer_kind";
    pub const SPAWN_POINTS_LAYER_PROP: &'static str = "spawn_points";
    pub const ITEMS_LAYER_PROP: &'static str = "items";
    pub const LIGHT_SOURCES_LAYER_PROP: &'static str = "light_sources";

    pub const BOOL_VALUE_TYPE: &'static str = "bool";
    pub const FLOAT_VALUE_TYPE: &'static str = "float";
    pub const INT_VALUE_TYPE: &'static str = "int";
    pub const STRING_VALUE_TYPE: &'static str = "string";
    pub const COLOR_VALUE_TYPE: &'static str = "color";
    pub const OBJECT_VALUE_TYPE: &'static str = "object";
    pub const FILE_VALUE_TYPE: &'static str = "file";
}

pub fn pair_from_tiled_prop(tiled_prop: TiledProperty) -> (String, MapProperty) {
    match tiled_prop {
        TiledProperty::Bool { name, value } => (name, MapProperty::Bool { value }),
        TiledProperty::Float { name, value } => (name, MapProperty::Float { value }),
        TiledProperty::Int { name, value } => (name, MapProperty::Int { value }),
        TiledProperty::String { name, value } => (name, MapProperty::String { value }),
        TiledProperty::Color { name, value } => (name, MapProperty::Color { value: color_from_hex_string(&value) }),
        TiledProperty::Object { name, value } => (name, MapProperty::Int { value }),
        TiledProperty::File { name, value } => (name, MapProperty::String { value }),
    }
}