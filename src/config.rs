use macroquad::prelude::*;

use serde::{
    Serialize,
    Deserialize,
};

use crate::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default, with = "json::def_uvec2")]
    pub resolution: UVec2,
    #[serde(default)]
    pub fullscreen: bool,
    #[serde(default)]
    pub gui_scale: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            resolution: uvec2(1920, 1080),
            fullscreen: true,
            gui_scale: 1.0,
        }
    }
}
