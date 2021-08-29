use crate::prelude::*;

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
    pub index: usize,
    pub title: String,
    pub description: String,
    pub initial_map_id: String,
    pub maps: Vec<MapData>,
}

#[derive(Debug, Clone)]
pub struct CurrentChapter {
    pub chapter_index: usize,
    pub chapter: Chapter,
    pub map_id: String,
}

#[derive(Debug, Clone)]
pub struct Scenario {
    pub chapters: Vec<Chapter>,
}

impl Scenario {
    pub async fn new(assets_path: &str, params: ScenarioParams) -> Result<Self, FileError> {
        let mut chapters = Vec::new();
        for i in 0..params.chapters.len() {
            let chapter_params = params.chapters
                .get(i)
                .cloned()
                .unwrap();

            let mut  maps = Vec::new();
            for map_info in chapter_params.maps {
                let map = MapData {
                    id: map_info.id,
                    title: map_info.title,
                    description: map_info.description,
                    path: map_info.path.clone(),
                    map: Map::load(&format!("{}/{}", assets_path, map_info.path)).await.unwrap(),
                };

                maps.push(map);
            }

            let chapter = Chapter {
                index: i,
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

    pub async fn load_params(assets_path: &str) -> Result<ScenarioParams, FileError> {
        let path = &format!("{}/scenario.json", assets_path.clone());
        let bytes = load_file(path).await?;
        let params = serde_json::from_slice(&bytes)
            .expect(&format!("Unable to parse scenario file '{}'!", path));
        Ok(params)
    }
}

#[derive(Debug, Clone)]
pub struct SceneTransitionParams {
    pub chapter_index: usize,
    pub map_id: String,
}

impl SceneTransitionParams {
    pub fn change_map(map_id: &str) -> Self {
        let current_chapter = storage::get::<CurrentChapter>();
        SceneTransitionParams {
            chapter_index: current_chapter.chapter_index,
            map_id: map_id.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SceneTransition {
    pub player: SavedCharacter,
    pub chapter_index: usize,
    pub map_id: String,
}

impl SceneTransition {
    pub fn new(player: SavedCharacter, params: SceneTransitionParams) -> Self {
        SceneTransition {
            player,
            chapter_index: params.chapter_index,
            map_id: params.map_id,
        }
    }
}
