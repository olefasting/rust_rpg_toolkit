use std::fs;

use macroquad::{
    experimental::{
        collections::storage,
    },
    prelude::*,
};

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
    #[serde(default)]
    pub post_processing: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            resolution: uvec2(1920, 1080),
            fullscreen: true,
            gui_scale: 1.0,
            post_processing: "crt".to_string(),
        }
    }
}

impl Config {
    const CONFIG_FILE_PATH: &'static str = "config.json";

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn load() -> Self {
        let json = fs::read_to_string(Self::CONFIG_FILE_PATH)
            .expect(&format!("Unable to find config file '{}'!", Self::CONFIG_FILE_PATH));
        let mut config: Config = serde_json::from_str(&json)
            .expect(&format!("Unable to parse config file '{}'!", Self::CONFIG_FILE_PATH));
        config.gui_scale = config.gui_scale.clamp(0.25, 5.0);
        storage::store(config.clone());
        config
    }

    #[cfg(target_family = "wasm")]
    pub fn load() -> Self {
        let config = Config {
            resolution: uvec2(1080, 720),
            fullscreen: false,
            gui_scale: 1.0,
            ..Default::default()
        };
        storage::store(config.clone());
        config
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn save(&self) {
        let json = serde_json::to_string_pretty(self).expect("Error parsing config!");
        fs::write(Self::CONFIG_FILE_PATH, &json).expect("Error saving config to file!");
    }

    #[cfg(target_family = "wasm")]
    pub fn save(&self) {

    }
}
