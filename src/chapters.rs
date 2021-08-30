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

#[derive(Debug, Clone)]
pub struct Chapter {
    pub title: String,
    pub description: String,
    pub initial_map_id: String,
    pub maps: HashMap<String, Map>,
}

impl Chapter {
    pub async fn new(params: ChapterParams) -> Result<Self, FileError> {
        let mut  maps = HashMap::new();
        for map_params in params.maps {
            let map = Map::load(&map_params.path).await?;
            maps.insert(map.id.clone(), map);
        }

        let chapter = Chapter {
            title: params.title,
            description: params.description,
            initial_map_id: params.initial_map_id,
            maps,
        };

        Ok(chapter)
    }
}

pub async fn load_maps(params: Vec<ChapterParams>) -> Result<Vec<Chapter>, FileError> {
    let mut chapters = Vec::new();

    for params in params {
        let chapter = Chapter::new(params).await?;
        chapters.push(chapter);
    }

    Ok(chapters)
}

#[derive(Debug, Clone)]
pub struct SceneTransitionParams {
    pub chapter_index: usize,
    pub map_id: String,
}

impl SceneTransitionParams {
    pub fn change_map(map_id: &str) -> Self {
        let current_chapter = storage::get::<SceneTransitionParams>();
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
