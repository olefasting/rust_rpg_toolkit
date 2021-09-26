use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterClass {
    pub id: String,
    pub prototype_id: String,
    pub name: String,
    pub description: String,
}

fn default_filter_mode() -> FilterMode {
    FilterMode::Nearest
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureAssetParams {
    pub id: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height_map_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_map_path: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniformAssetParams {
    pub name: String,
    #[serde(rename = "type", with = "json::UniformTypeDef")]
    pub value_type: UniformType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialAssetParams {
    pub id: String,
    pub vertex_path: String,
    pub fragment_path: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub textures: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uniforms: Vec<UniformAssetParams>,
}

impl Into<MaterialParams> for MaterialAssetParams {
    fn into(self) -> MaterialParams {
        let textures = self.textures;
        let uniforms = self.uniforms.into_iter().map(|params| (params.name, params.value_type)).collect();

        MaterialParams {
            textures,
            uniforms,
            pipeline_params: Default::default(),
        }
    }
}

pub struct Resources {
    pub actors: HashMap<String, ActorParams>,
    pub character_classes: HashMap<String, CharacterClass>,
    pub items: HashMap<String, ItemParams>,
    pub abilities: HashMap<String, AbilityParams>,
    pub missions: HashMap<String, MissionParams>,
    pub dialogue: HashMap<String, Dialogue>,
    pub chapters: Vec<Chapter>,
    pub materials: HashMap<String, MaterialSource>,
    pub textures: HashMap<String, Texture>,
    pub images: HashMap<String, Image>,
    pub font_bytes: HashMap<String, Vec<u8>>,
    pub sound_effects: HashMap<String, Sound>,
    pub music: HashMap<String, Sound>,
}

impl Resources {
    const CLASSES_FILE_NAME: &'static str = "character_classes.json";
    const ACTORS_FILE_NAME: &'static str = "actors.json";
    const ITEMS_FILE_NAME: &'static str = "items.json";
    const MISSIONS_FILE_NAME: &'static str = "missions.json";
    const DIALOGUE_FILE_NAME: &'static str = "dialogue.json";
    const ABILITIES_FILE_NAME: &'static str = "abilities.json";
    const SCENARIO_FILE_NAME: &'static str = "scenario.json";

    const MATERIALS_FILE_NAME: &'static str = "materials.json";
    const TEXTURES_FILE_NAME: &'static str = "textures.json";
    const IMAGES_FILE_NAME: &'static str = "images.json";
    const FONTS_FILE_NAME: &'static str = "fonts.json";
    const SOUND_EFFECTS_FILE_NAME: &'static str = "sound_effects.json";
    const MUSIC_FILE_NAME: &'static str = "music.json";

    pub const WHITE_TEXTURE_ID: &'static str = "__WHITE_TEXTURE__";

    pub async fn new(game_params: &GameParams) -> Result<Self> {
        let data_path = Path::new(&game_params.data_path);
        let assets_path = Path::new(&game_params.assets_path);

        let character_classes_path = data_path.join(Self::CLASSES_FILE_NAME);
        let bytes = load_file(&character_classes_path.to_string_lossy()).await?;
        let character_classes_data: Vec<CharacterClass> = serde_json::from_slice(&bytes)?;
        let character_classes = HashMap::from_iter(
            character_classes_data.into_iter().map(|class| (class.id.clone(), class)));

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
        for params in chapter_params {
            let chapter = Chapter::new(game_params, params).await?;
            chapters.push(chapter);
        }

        let materials_file_path = assets_path.join(Self::MATERIALS_FILE_NAME);
        let bytes = load_file(&materials_file_path.to_string_lossy()).await?;
        let material_assets: Vec<MaterialAssetParams> = serde_json::from_slice(&bytes)?;

        let mut materials = HashMap::new();
        for params in material_assets {
            let id = params.id.clone();
            let vertex_path = assets_path.join(&params.vertex_path);
            let fragment_path = assets_path.join(&params.fragment_path);
            let material = MaterialSource::new(vertex_path, fragment_path, params.into()).await?;
            materials.insert(id, material);
        }

        let textures_file_path = assets_path.join(Self::TEXTURES_FILE_NAME);
        let bytes = load_file(&textures_file_path.to_string_lossy()).await?;
        let texture_assets: Vec<TextureAssetParams> = serde_json::from_slice(&bytes)?;

        let mut textures = HashMap::new();
        let white_image = Image::gen_image_color(32, 32, color::WHITE);
        let white_texture = Texture2D::from_image(&white_image);
        white_texture.set_filter(FilterMode::Nearest);

        let texture = Texture::new(white_texture, None, None);
        textures.insert(Self::WHITE_TEXTURE_ID.to_string(), texture);

        for params in texture_assets {
            let path = assets_path.join(&params.path);
            let texture = load_texture(&path.to_string_lossy()).await?;
            texture.set_filter(params.filter_mode);

            let mut height_map = None;
            if let Some(path) = &params.height_map_path {
                let path = assets_path.join(path);
                let res = load_texture(&path.to_string_lossy()).await?;
                res.set_filter(params.filter_mode);
                height_map = Some(res);
            }

            let mut normal_map = None;
            if let Some(path) = &params.normal_map_path {
                let path = assets_path.join(path);
                let res = load_texture(&path.to_string_lossy()).await?;
                res.set_filter(params.filter_mode);
                normal_map = Some(res);
            }

            let texture = Texture::new(texture, height_map, normal_map);
            textures.insert(params.id.clone(), texture);
        }

        let images_file_path = assets_path.join(Self::IMAGES_FILE_NAME);
        let bytes = load_file(&images_file_path.to_string_lossy()).await?;
        let image_assets: Vec<ImageAssetParams> = serde_json::from_slice(&bytes)?;

        let mut images = HashMap::new();
        for params in image_assets {
            let path = assets_path.join(&params.path);
            let bytes = load_file(&path.to_string_lossy()).await?;
            let format = match params.format.as_ref() {
                Some(ext) => ImageFormat::from_extension(ext),
                _ => None,
            };

            let image = Image::from_file_with_format(&bytes, format);
            images.insert(params.id.clone(), image);
        }

        let fonts_file_path = assets_path.join(Self::FONTS_FILE_NAME);
        let bytes = load_file(&fonts_file_path.to_string_lossy()).await?;
        let font_assets: Vec<FontAssetParams> = serde_json::from_slice(&bytes)?;

        let mut font_bytes = HashMap::new();
        for params in font_assets {
            let path = assets_path.join(&params.path);
            let bytes = load_file(&path.to_string_lossy()).await?;
            font_bytes.insert(params.id.clone(), bytes);
        }

        let sound_effects_file_path = assets_path.join(Self::SOUND_EFFECTS_FILE_NAME);
        let bytes = load_file(&sound_effects_file_path.to_string_lossy()).await?;
        let sound_effect_assets: Vec<SoundAssetParams> = serde_json::from_slice(&bytes)?;

        let mut sound_effects = HashMap::new();
        for params in sound_effect_assets {
            let path = assets_path.join(&params.path);
            let sound = load_sound(&path.to_string_lossy()).await?;
            sound_effects.insert(params.id.clone(), sound);
        }

        let music_file_path = assets_path.join(Self::MUSIC_FILE_NAME);
        let bytes = load_file(&music_file_path.to_string_lossy()).await?;
        let music_assets: Vec<SoundAssetParams> = serde_json::from_slice(&bytes)?;

        let mut music = HashMap::new();
        for params in music_assets {
            let path = assets_path.join(&params.path);
            let track = load_sound(&path.to_string_lossy()).await?;
            music.insert(params.id.clone(), track);
        }

        let resources = Resources {
            actors,
            character_classes,
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
