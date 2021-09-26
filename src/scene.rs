use crate::prelude::*;

static mut SCENE_BUILDER: Option<Box<SceneBuilder>> = None;

unsafe fn set_scene_builder(builder: SceneBuilder) {
    SCENE_BUILDER = Some(Box::new(builder));
}

unsafe fn get_scene_builder() -> &'static SceneBuilder {
    if SCENE_BUILDER.is_none() {
        SCENE_BUILDER = Some(Box::new(SceneBuilder::new()));
    }

    SCENE_BUILDER.as_ref().unwrap()
}

pub(crate) fn load_scene(character: Character) -> Result<()> {
    unsafe { get_scene_builder() }.build(character)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DrawStage {
    Map,
    Items,
    Projectiles,
    Actors,
    PostProcessing,
    Gui,
}

// This is a builder for scenes in the game. It is not meant to be used to build scenes directly
// (dispatch a `ChangeScene` event for that), but to create the default builder used by the event
// handler when loading scenes by calling `make_default`.
pub struct SceneBuilder {
    chapter_index: Option<usize>,
    map_id: Option<String>,
    draw_stages: HashMap<DrawStage, Vec<fn()>>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        let keys = [
            DrawStage::Map,
            DrawStage::Items,
            DrawStage::Projectiles,
            DrawStage::Actors,
            DrawStage::PostProcessing,
            DrawStage::Gui,
        ];

        let mut draw_stages = HashMap::new();
        for key in keys {
            draw_stages.insert(key, Vec::new());
        }

        SceneBuilder {
            chapter_index: None,
            map_id: None,
            draw_stages,
        }
    }

    pub fn with_map(self, chapter_index: usize, map_id: &str) -> Self {
        SceneBuilder {
            chapter_index: Some(chapter_index),
            map_id: Some(map_id.to_string()),
            ..self
        }
    }

    pub fn with_draw_buffer<T: 'static + BufferedDraw>(self, draw_stage: DrawStage) -> Self {
        let mut draw_stages = self.draw_stages;

        draw_stages
            .get_mut(&draw_stage)
            .unwrap()
            .push(|| {
                let draw_buffer = DrawBuffer::<T>::new();
                scene::add_node(draw_buffer);
            });

        SceneBuilder {
            draw_stages,
            ..self
        }
    }

    pub fn make_default(self) {
        unsafe { set_scene_builder(self) };
    }

    pub(crate) fn build(&self, character: Character) -> Result<()> {
        scene::clear();

        let (chapter_index, map_id) = if self.chapter_index.is_some() && self.map_id.is_some() {
            (self.chapter_index.unwrap(), self.map_id.as_ref().unwrap())
        } else {
            (character.chapter_index, &character.map_id)
        };

        let resources = storage::get::<Resources>();

        let chapter = resources.chapters.get(chapter_index)
            .expect(&format!("Scene could not build, due to an invalid chapter index ({})!", chapter_index));

        let map = chapter.maps.get(map_id).cloned()
            .expect(&format!("Scene could not build as no map with id '{}' was found in chapter '{}' (chapter index: {})!", map_id, &chapter.title, chapter_index));

        let player_spawn_point = map.player_spawn_point
            .expect(&format!("No player spawn point defined in map '{}' of chapter '{}' (chapter index: {})", map_id, chapter.title, chapter_index));

        let game_state = GameState::add_node(player_spawn_point, &character);

        CameraController::add_node();

        for constructor in self.draw_stages.get(&DrawStage::Map).unwrap() {
            constructor();
        }

        MapRenderer::add_node();

        for constructor in self.draw_stages.get(&DrawStage::Items).unwrap() {
            constructor();
        }

        DrawBuffer::<Item>::add_node();
        DrawBuffer::<Credits>::add_node();

        for constructor in self.draw_stages.get(&DrawStage::Projectiles).unwrap() {
            constructor();
        }

        Projectiles::add_node();
        ContinuousBeams::add_node();

        for constructor in self.draw_stages.get(&DrawStage::Actors).unwrap() {
            constructor();
        }

        DrawBuffer::<Actor>::add_node();

        for constructor in self.draw_stages.get(&DrawStage::PostProcessing).unwrap() {
            constructor();
        }

        PostProcessing::add_node()?;

        for constructor in self.draw_stages.get(&DrawStage::Gui).unwrap() {
            constructor();
        }

        Hud::add_node();

        for (_, layer) in &map.layers {
            if let MapLayerKind::ObjectLayer(kind) = layer.kind.clone() {
                match kind {
                    ObjectLayerKind::Items => {
                        for map_object in &layer.objects {
                            spawn_item(map_object);
                        }
                    }
                    ObjectLayerKind::SpawnPoints => {
                        for map_object in &layer.objects {
                            if map_object.name != Map::PLAYER_SPAWN_POINT_NAME {
                                spawn_actor(game_state, map_object);
                            }
                        }
                    }
                    ObjectLayerKind::LightSources => {
                        for map_object in &layer.objects {
                            spawn_light_source(map_object);
                        }
                    }
                    ObjectLayerKind::None => {}
                }
            }
        }

        character.spawn(game_state, player_spawn_point);

        storage::store(map);

        Ok(())
    }
}

fn spawn_item(map_object: &MapObject) {
    if let Some(prop) = map_object.properties.get("prototype_id").cloned() {
        if let MapProperty::String { value: prototype_id } = prop {
            if prototype_id == "credits" {
                if let Some(prop) = map_object.properties.get("amount") {
                    if let MapProperty::Int { value } = prop {
                        Credits::add_node(map_object.position, *value as u32);
                    }
                }
            } else {
                let resources = storage::get::<Resources>();
                let params = resources.items.get(&prototype_id).cloned().unwrap();
                let mut instance_id = None;
                if let Some(prop) = map_object.properties.get("instance_id").cloned() {
                    if let MapProperty::String { value } = prop {
                        instance_id = Some(value)
                    }
                }

                Item::add_node(ItemParams {
                    id: instance_id.unwrap_or(generate_id()),
                    position: Some(map_object.position),
                    ..params
                });
            }
        }
    }
}

fn spawn_actor(game_state: Handle<GameState>, map_object: &MapObject) {
    if let Some(prop) = map_object.properties.get("prototype_id") {
        if let MapProperty::String { value: prototype_id } = prop {
            let mut instance_id = None;
            if let Some(prop) = map_object.properties.get("instance_id").cloned() {
                if let MapProperty::String { value } = prop {
                    instance_id = Some(value);
                }
            }

            let resources = storage::get::<Resources>();
            let params = resources.actors.get(prototype_id).cloned().unwrap();
            let mut actor = Actor::new(
                game_state,
                ActorControllerKind::Computer,
                ActorParams {
                    id: instance_id.unwrap_or(generate_id()),
                    position: Some(map_object.position),
                    ..params
                });

            actor.stats.recalculate_derived();
            actor.stats.restore_vitals();

            scene::add_node(actor);
        }
    }
}

fn spawn_light_source(map_object: &MapObject) {
    let size = map_object.size.unwrap_or(LightSource::DEFAULT_SIZE);

    let mut color = LightSource::DEFAULT_COLOR;
    if let Some(prop) = map_object.properties.get("color").cloned() {
        if let MapProperty::Color { value } = prop {
            color = value;
        }
    }

    let mut intensity = LightSource::DEFAULT_INTENSITY;
    if let Some(prop) = map_object.properties.get("intensity").cloned() {
        if let MapProperty::Float { value } = prop {
            intensity = value;
        }
    }

    LightSource::add_node(map_object.position, size, color, intensity, None);
}