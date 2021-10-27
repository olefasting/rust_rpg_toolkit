use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapParams {
    pub id: String,
    pub title: String,
    pub description: String,
    pub path: String,
    #[serde(default)]
    pub is_tiled: bool,
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
    pub async fn new(game_params: &GameParams, params: ChapterParams) -> Result<Self> {
        let data_path = Path::new(&game_params.data_path);

        let mut  maps = HashMap::new();
        for params in params.maps {
            let path = data_path.join(&params.path);
            let map = if params.is_tiled {
                Map::load_tiled(path, None).await?
            } else {
                Map::load(path).await?
            };

            maps.insert(params.id.clone(), map);
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

pub async fn load_maps(game_params: &GameParams, params: Vec<ChapterParams>) -> Result<Vec<Chapter>> {
    let mut chapters = Vec::new();

    for params in params {
        let chapter = Chapter::new(game_params, params).await?;
        chapters.push(chapter);
    }

    Ok(chapters)
}
