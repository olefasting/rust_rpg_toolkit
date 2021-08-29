use crate::{
    prelude::*,
};

#[derive(Debug, Clone, Deserialize)]
pub enum RawTiledPropertyType {
    #[serde(rename = "bool")]
    BoolType,
    #[serde(rename = "float")]
    FloatType,
    #[serde(rename = "integer")]
    IntType,
    #[serde(rename = "string")]
    StringType,
    #[serde(rename = "color")]
    ColorType,
    #[serde(rename = "object")]
    ObjectType,
    #[serde(rename = "file")]
    FileType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawTiledProperty {
    pub name: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: RawTiledPropertyType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawTiledObject {
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
    pub polygon: Option<Vec<RawTiledPolyPoint>>,
    pub properties: Option<Vec<RawTiledProperty>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawTiledPolyPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawTiledTileset {
    pub columns: i32,
    pub image: String,
    pub imagewidth: i32,
    pub imageheight: i32,
    pub margin: i32,
    pub name: String,
    pub properties: Option<Vec<RawTiledProperty>>,
    pub spacing: i32,
    pub tileheight: i32,
    pub tilewidth: i32,
    pub firstgid: u32,
    pub tilecount: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawTiledLayer {
    pub name: String,
    pub visible: bool,
    #[serde(rename = "type")]
    pub layer_type: String,
    #[serde(default)]
    pub data: Vec<u32>,
    #[serde(default)]
    pub objects: Vec<RawTiledObject>,
    #[serde(default)]
    pub properties: Option<HashMap<String, RawTiledProperty>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawTiledMap {
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
    pub layers: Vec<RawTiledLayer>,
    pub tilesets: Vec<RawTiledTileset>,
    #[serde(default)]
    pub properties: Option<HashMap<String, RawTiledProperty>>,
}
