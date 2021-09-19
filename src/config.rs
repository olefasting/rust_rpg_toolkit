use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default, with = "json::def_uvec2")]
    pub resolution: UVec2,
    #[serde(default)]
    pub fullscreen: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_processing: Option<String>,

    #[serde(default, skip)]
    path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            resolution: uvec2(1920, 1080),
            fullscreen: true,
            post_processing: None,
            path: "config.json".to_string(),
        }
    }
}

impl Config {
    pub const MIN_GUI_SCALE: f32 = 0.1;
    pub const MAX_GUI_SCALE: f32 = 5.0;
    pub const GUI_SCALE_STEP: f32 = 0.1;

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn load(path: &str) -> Self {
        let mut config: Config = if let Ok(json) = fs::read_to_string(path) {
            serde_json::from_str(&json)
                .expect(&format!("Unable to parse config file '{}'!", path))
        } else {
            Default::default()
        };

        config.path = path.to_string();
        storage::store(config.clone());

        config
    }

    #[cfg(target_family = "wasm")]
    pub fn load(key: &str) -> Self {
        let web_storage = &mut quad_storage::STORAGE.lock().unwrap();

        let mut config = if let Some(json) = web_storage.get(key) {
            let mut config: Config = serde_json::from_str(&json)
                .expect("Unable to parse config from web storage!");
            storage::store(config.clone());
            config
        } else {
            Default::default()
        };

        config.path = key.to_string();
        storage::store(config.clone());

        config
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn save(&self) {
        assert_eq!(self.path.is_empty(), false, "Config path is not set!");
        let json = serde_json::to_string_pretty(self)
            .expect("Error parsing config!");
        fs::write(&self.path, &json).expect("Error saving config to file!");
    }

    #[cfg(target_family = "wasm")]
    pub fn save(&self) {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        let json = serde_json::to_string_pretty(self)
            .expect("Error parsing config!");
        storage.set(self.path, &json)
    }
}
