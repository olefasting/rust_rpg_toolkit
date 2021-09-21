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
    const ACTORS_FILE_NAME: &'static str = "actors.json";
    const ITEMS_FILE_NAME: &'static str = "items.json";
    const MISSIONS_FILE_NAME: &'static str = "missions.json";
    const DIALOGUE_FILE_NAME: &'static str = "dialogue.json";
    const ABILITIES_FILE_NAME: &'static str = "abilities.json";
    const SCENARIO_FILE_NAME: &'static str = "scenario.json";
    const ASSETS_FILE_NAME: &'static str = "assets.json";

    pub const WHITE_TEXTURE_ID: &'static str = "__WHITE_TEXTURE__";

    pub async fn new<P: AsRef<Path>>(data_path: P) -> Result<Self> {
        let data_path = data_path.as_ref();

        let actors_file_path = data_path.join(Self::ACTORS_FILE_NAME);
        let bytes = load_file(&actors_file_path.to_string_lossy()).await?;
        let actor_data: Vec<ActorParams> = serde_json::from_slice(&bytes)?;
        let actors = HashMap::from_iter(
            actor_data.into_iter().map(|params| (params.id.clone(), params)));

        let items_file_path = data_path.join(Self::ITEMS_FILE_NAME);
        let bytes = load_file(&items_file_path.to_string_lossy()).await?;
        let items_data: Vec<ItemParams> = serde_json::from_slice(&bytes)?;
        let items = HashMap::from_iter(
            items_data.into_iter().map(|params| (params.id.clone(), params)));

        let missions_file_path = data_path.join(Self::MISSIONS_FILE_NAME);
        let bytes = load_file(&missions_file_path.to_string_lossy()).await?;
        let missions_data: Vec<MissionParams> = serde_json::from_slice(&bytes)?;
        let missions = HashMap::from_iter(
            missions_data.into_iter().map(|mission| (mission.id.clone(), mission)));

        let dialogue_file_path = data_path.join(Self::DIALOGUE_FILE_NAME);
        let bytes = load_file(&dialogue_file_path.to_string_lossy()).await?;
        let dialogue_data: Vec<Dialogue> = serde_json::from_slice(&bytes)?;
        let dialogue = HashMap::from_iter(
            dialogue_data.into_iter().map(|dialogue| (dialogue.id.clone(), dialogue)));

        let abilities_file_path = data_path.join(Self::ABILITIES_FILE_NAME);
        let bytes = load_file(&abilities_file_path.to_string_lossy()).await?;
        let ability_data: Vec<AbilityParams> = serde_json::from_slice(&bytes)?;
        let abilities = HashMap::from_iter(
            ability_data.into_iter().map(|ability| (ability.id.clone(), ability)));

        let scenario_path = data_path.join(Self::SCENARIO_FILE_NAME);
        let bytes = load_file(&scenario_path.to_string_lossy()).await?;
        let chapter_params: Vec<ChapterParams> = serde_json::from_slice(&bytes)?;
        let mut chapters = Vec::new();
        for chapter_params in chapter_params {
            let chapter = Chapter::new(chapter_params).await?;
            chapters.push(chapter);
        }

        let assets_file_path = data_path.join(Self::ASSETS_FILE_NAME);
        let bytes = load_file(&assets_file_path.to_string_lossy()).await?;
        let assets: AssetsParams = serde_json::from_slice(&bytes)?;


        let mut materials = HashMap::new();
        for material_params in &assets.materials {
            let vertex_shader = load_file(&material_params.vertex_shader_path).await?;
            let fragment_shader = load_file(&material_params.fragment_shader_path).await?;

            let material = load_material(
                &String::from_utf8(vertex_shader)?,
                &String::from_utf8(fragment_shader)?,
                MaterialParams {
                    ..Default::default()
                },
            )?;

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

        resources.update_sound_volume();

        Ok(resources)
    }

    pub fn get_font(&self, font_id: &str) -> Result<Font> {
        let bytes = self.font_bytes.get(font_id)
            .expect(&format!("No font with id '{}' was found!", font_id));
        let font = load_ttf_font_from_bytes(&bytes)?;
        Ok(font)
    }

    // Volume isn't working yet in macroquad, so this does nothing, for now
    pub(crate) fn update_sound_volume(&self) {
        let config = storage::get::<Config>();
        let master_volume = (config.master_volume as f32 / 100.0).clamp(0.0, 1.0);
        let sound_effects_volume = (config.sound_effects_volume as f32 / 100.0).clamp(0.0, 1.0) * master_volume;
        let music_volume = (config.music_volume as f32 / 100.0).clamp(0.0, 1.0) * master_volume;
        for (_, sound) in self.sound_effects.clone() {
            set_sound_volume(sound, sound_effects_volume);
        }
        for (_, sound) in self.music.clone() {
            set_sound_volume(sound, music_volume);
        }
    }
}
