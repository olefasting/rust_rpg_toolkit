use macroquad::{
    experimental::{
        coroutines::start_coroutine,
        collections::storage,
    },
    color,
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::render::{
    draw_aligned_text,
    HorizontalAlignment,
    VerticalAlignment,
};

use crate::map::Map;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapParams {
    pub id: String,
    pub title: String,
    pub description: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterParams {
    pub title: String,
    pub description: String,
    pub initial_map_id: String,
    pub maps: Vec<MapParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioParams {
    pub chapters: Vec<ChapterParams>,
}

#[derive(Debug, Clone)]
pub struct MapData {
    pub id: String,
    pub title: String,
    pub description: String,
    pub path: String,
    pub map: Map,
}

#[derive(Debug, Clone)]
pub struct Chapter {
    pub title: String,
    pub description: String,
    pub initial_map_id: String,
    pub maps: Vec<MapData>,
}

#[derive(Debug, Clone)]
pub struct CurrentChapter {
    pub chapter_index: usize,
    pub chapter: Chapter,
    pub current_map_id: String,
}

#[derive(Debug, Clone)]
pub struct Scenario {
    pub chapters: Vec<Chapter>,
}

impl Scenario {
    const SCENARIO_FILE_PATH: &'static str = "assets/scenario.json";

    pub async fn new(params: ScenarioParams) -> Result<Self, FileError> {
        let mut chapters = Vec::new();
        for chapter_params in params.chapters.clone() {
            let mut  maps = Vec::new();
            for map_info in chapter_params.maps {
                let map = MapData {
                    id: map_info.id,
                    title: map_info.title,
                    description: map_info.description,
                    path: map_info.path.clone(),
                    map: Map::load(&format!("assets/{}", map_info.path)).await.unwrap(),
                };

                maps.push(map);
            }

            let chapter = Chapter {
                title: chapter_params.title,
                description: chapter_params.description,
                initial_map_id: chapter_params.initial_map_id,
                maps,
            };

            chapters.push(chapter);
        }

        let scenario = Scenario {
            chapters,
        };

        Ok(scenario)
    }

    pub async fn load_params() -> Result<ScenarioParams, FileError> {
        let bytes = load_file(Self::SCENARIO_FILE_PATH).await?;
        let params = serde_json::from_slice(&bytes)
            .expect(&format!("Unable to parse scenario file '{}'!", Self::SCENARIO_FILE_PATH));
        Ok(params)
    }
}
