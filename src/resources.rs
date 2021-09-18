use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialAssetParams {
    pub id: String,
    pub fragment_shader_path: String,
    pub vertex_shader_path: String,
}

fn default_filter_mode() -> FilterMode {
    FilterMode::Nearest
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureAssetParams {
    pub id: String,
    pub path: String,
    #[serde(default = "default_filter_mode", with = "json::FilterModeDef")]
    pub filter_mode: FilterMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAssetParams {
    pub id: String,
    pub path: String,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontAssetParams {
    pub id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundAssetParams {
    pub id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetsParams {
    materials: Vec<MaterialAssetParams>,
    textures: Vec<TextureAssetParams>,
    images: Vec<ImageAssetParams>,
    fonts: Vec<FontAssetParams>,
    sound_effects: Vec<SoundAssetParams>,
    music: Vec<SoundAssetParams>,
}

pub struct Resources {
    pub actors: HashMap<String, ActorParams>,
    pub items: HashMap<String, ItemParams>,
    pub abilities: HashMap<String, AbilityParams>,
    pub missions: HashMap<String, MissionParams>,
    pub dialogue: HashMap<String, Dialogue>,
    pub chapters: Vec<Chapter>,
    pub materials: HashMap<String, Material>,
    pub textures: HashMap<String, Texture2D>,
    pub images: HashMap<String, Image>,
    pub font_bytes: HashMap<String, Vec<u8>>,
    pub sound_effects: HashMap<String, Sound>,
    pub music: HashMap<String, Sound>,
}

impl Resources {
    pub const WHITE_TEXTURE_ID: &'static str = "__WHITE_TEXTURE__";

    pub async fn new(data_path: &str) -> Result<Self> {
        let assets_file_path = format!("{}/assets.json", data_path);
        let actors_file_path = format!("{}/actors.json", data_path);
        let items_file_path = format!("{}/items.json", data_path);
        let abilities_file_path = format!("{}/abilities.json", data_path);
        let missions_file_path = format!("{}/missions.json", data_path);
        let dialogue_file_path = format!("{}/dialogue.json", data_path);

        let bytes = load_file(&actors_file_path).await?;
        let actor_data: Vec<ActorParams> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing actors file '{}'!", &actors_file_path));
        let actors = HashMap::from_iter(
            actor_data.into_iter().map(|params| (params.id.clone(), params)));

        let bytes = load_file(&items_file_path).await?;
        let items_data: Vec<ItemParams> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing items file '{}'!", items_file_path));
        let items = HashMap::from_iter(
            items_data.into_iter().map(|params| (params.id.clone(), params)));

        let bytes = load_file(&missions_file_path).await?;
        let missions_data: Vec<MissionParams> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing missions file '{}'!", &missions_file_path));
        let missions = HashMap::from_iter(
            missions_data.into_iter().map(|mission| (mission.id.clone(), mission)));

        let bytes = load_file(&dialogue_file_path).await?;
        let dialogue_data: Vec<Dialogue> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing dialogue file '{}'!", &dialogue_file_path));
        let dialogue = HashMap::from_iter(
            dialogue_data.into_iter().map(|dialogue| (dialogue.id.clone(), dialogue)));

        let bytes = load_file(&abilities_file_path).await?;
        let ability_data: Vec<AbilityParams> = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing dialogue file '{}'!", &abilities_file_path));
        let abilities = HashMap::from_iter(
            ability_data.into_iter().map(|ability| (ability.id.clone(), ability)));

        let bytes = load_file(&format!("{}/scenario.json", data_path)).await?;
        let chapter_params: Vec<ChapterParams> = serde_json::from_slice(&bytes).unwrap();
        let mut chapters = Vec::new();
        for chapter_params in chapter_params {
            let chapter = Chapter::new(chapter_params).await?;
            chapters.push(chapter);
        }

        let bytes = load_file(&assets_file_path).await?;
        let assets: AssetsParams = serde_json::from_slice(&bytes)
            .expect(&format!("Error when parsing assets file '{}'!", assets_file_path));


        let mut materials = HashMap::new();
        for material_params in &assets.materials {
            let vertex_shader = load_file(&material_params.vertex_shader_path).await?;
            let fragment_shader = load_file(&material_params.fragment_shader_path).await?;

            let material = load_material(
                &String::from_utf8(vertex_shader).unwrap(),
                &String::from_utf8(fragment_shader).unwrap(),
                MaterialParams {
                    ..Default::default()
                },
            ).unwrap();

            materials.insert(material_params.id.clone(), material);
        }

        let mut textures = HashMap::new();
        let white_image = Image::gen_image_color(32, 32, color::WHITE);
        let white_texture = Texture2D::from_image(&white_image);
        white_texture.set_filter(FilterMode::Nearest);
        textures.insert(Self::WHITE_TEXTURE_ID.to_string(), white_texture);

        for texture_params in &assets.textures {
            let texture = load_texture(&texture_params.path).await?;
            texture.set_filter(texture_params.filter_mode);
            textures.insert(texture_params.id.clone(), texture);
        }

        let mut images = HashMap::new();
        for image_params in &assets.images {
            let bytes = load_file(&image_params.path).await?;
            let format = match image_params.format.as_ref() {
                Some(ext) => ImageFormat::from_extension(ext),
                _ => None,
            };

            let image = Image::from_file_with_format(&bytes, format);
            images.insert(image_params.id.clone(), image);
        }

        let mut font_bytes = HashMap::new();
        for font_params in &assets.fonts {
            let bytes = load_file(&font_params.path).await?;
            font_bytes.insert(font_params.id.clone(), bytes);
        }

        let mut sound_effects = HashMap::new();
        for sound_params in &assets.sound_effects {
            let sound = load_sound(&sound_params.path).await?;
            sound_effects.insert(sound_params.id.clone(), sound);
        }

        let mut music = HashMap::new();
        for music_params in &assets.music {
            let track = load_sound(&music_params.path).await?;
            music.insert(music_params.id.clone(), track);
        }

        let resources = Resources {
            actors,
            items,
            abilities,
            missions,
            dialogue,
            chapters,
            materials,
            textures,
            images,
            font_bytes,
            sound_effects,
            music,
        };

        Ok(resources)
    }

    pub fn get_font(&self, font_id: &str) -> Result<Font> {
        let bytes = self.font_bytes.get(font_id)
            .expect(&format!("No font with id '{}' was found!", font_id));
        let font = load_ttf_font_from_bytes(&bytes)?;
        Ok(font)
    }
}
