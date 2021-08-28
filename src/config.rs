use std::fs;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default, with = "json::def_uvec2")]
    pub resolution: UVec2,
    #[serde(default)]
    pub fullscreen: bool,
    #[serde(default)]
    pub gui_scale: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_processing: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            resolution: uvec2(1920, 1080),
            fullscreen: true,
            gui_scale: 1.0,
            post_processing: None,
        }
    }
}

impl Config {
    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn load(path: &str) -> Self {
        let json = fs::read_to_string(path)
            .expect(&format!("Unable to find config file '{}'!", path));
        let mut config: Config = serde_json::from_str(&json)
            .expect(&format!("Unable to parse config file '{}'!", path));
        config.gui_scale = config.gui_scale.clamp(0.25, 5.0);
        config
    }

    #[cfg(target_family = "wasm")]
    pub fn load() -> Self {
        let config = Config {
            resolution: uvec2(1080, 720),
            fullscreen: false,
            gui_scale: 1.0,
            post_processing: Some("crt".to_string()),
        };
        storage::store(config.clone());
        config
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn save(&self, path: &str) {
        let json = serde_json::to_string_pretty(self).expect("Error parsing config!");
        fs::write(path, &json).expect("Error saving config to file!");
    }

    #[cfg(target_family = "wasm")]
    pub fn save(&self) {

    }
}
