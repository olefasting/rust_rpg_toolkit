use crate::{
    map::MapPropertyType,
    prelude::*,
};

#[derive(Debug, Clone, Deserialize)]
pub struct TiledProperty {
    pub name: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: MapPropertyType,
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
