use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DrawStage {
    First,
    Items,
    Projectiles,
    Actors,
    PostProcessing,
    Last,
}

pub struct SceneBuilder {
    chapter_index: Option<usize>,
    map_id: Option<String>,
    draw_stages: HashMap<DrawStage, Vec<Box<dyn Node>>>,
}

impl SceneBuilder {
    const SPAWN_POINTS_LAYER_ID: &'static str = "spawn_points";
    const ITEMS_LAYER_ID: &'static str = "items";

    pub fn new() -> Self {
        let keys = [
            DrawStage::First,
            DrawStage::Items,
            DrawStage::Projectiles,
            DrawStage::Actors,
            DrawStage::PostProcessing,
            DrawStage::Last,
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
            draw_stages: self.draw_stages,
        }
    }

    pub fn with_nodes(self, draw_stage: DrawStage, should_append: bool, mut nodes: Vec<Box<dyn Node>>) -> Self {
        let mut draw_stages = self.draw_stages;
        if should_append {
            draw_stages
                .get_mut(&draw_stage)
                .unwrap()
                .append(&mut nodes);
        } else {
            draw_stages
                .insert(draw_stage, nodes);
        }

        SceneBuilder {
            chapter_index: self.chapter_index,
            map_id: self.map_id,
            draw_stages,
        }
    }

    pub fn build(self, character: Character) -> Result<()> {
        scene::clear();

        let resources = storage::get::<Resources>();

        let (chapter_index, map_id) = if self.chapter_index.is_some() && self.map_id.is_some() {
            (self.chapter_index.unwrap(), self.map_id.as_ref().unwrap())
        } else {
            (character.current_chapter_index, &character.current_map_id)
        };

        let chapter = resources.chapters.get(chapter_index)
            .expect(&format!("Scene could not build, due to an invalid chapter index ({})!", chapter_index));
        let map = chapter.maps.get(map_id).cloned()
            .expect(&format!("Scene could not build as no map with id '{}' was found in chapter '{}' (chapter index: {})!", map_id, &chapter.title, chapter_index));

        storage::store(map.clone());

        Camera::add_node();
        GameState::add_node(&character);
        DrawBuffer::<Item>::add_node();
        DrawBuffer::<Credits>::add_node();
        Projectiles::add_node();
        ContinuousBeams::add_node();
        DrawBuffer::<Actor>::add_node();
        PostProcessing::add_node();
        Hud::add_node();

        let mut player_spawn = None;

        let layer = map.layers.get(Self::SPAWN_POINTS_LAYER_ID)
            .expect(&format!("No spawn points layer in map '{}' of chapter '{}' (chapter index: {})!", map_id, chapter.title, chapter_index));

        for object in &layer.objects {
            if object.name == "player" {
                player_spawn = Some(object.position);
            } else if let Some(prop) = object.properties.get("prototype_id") {
                if let MapProperty::String { value: prototype_id } = prop {
                    let mut instance_id = None;
                    if let Some(prop) = object.properties.get("instance_id").cloned() {
                        if let MapProperty::String { value } = prop {
                            instance_id = Some(value);
                        }
                    }

                    let params = resources.actors.get(prototype_id).cloned().unwrap();
                    let mut actor = Actor::new(
                        ActorControllerKind::Computer,
                        ActorParams {
                            id: instance_id.unwrap_or(generate_id()),
                            position: Some(object.position),
                            ..params
                        });

                    actor.stats.recalculate_derived();
                    actor.stats.restore_vitals();

                    scene::add_node(actor);
                }
            }
        }

        let player_spawn = player_spawn
            .expect(&format!("No player spawn point in map '{}' of chapter '{}' (chapter index: {})!", map_id, chapter.title, chapter_index));

        let player = storage::get::<LocalPlayer>();
        let mut actor = Actor::from_saved(
            player_spawn,
            ActorControllerKind::local_player(&player.id),
            &character,
        );

        actor.stats.recalculate_derived();
        actor.stats.restore_vitals();

        scene::add_node(actor);

        if let Some(layer) = map.layers.get("light_sources") {
            for object in &layer.objects {
                let size = if let Some(size) = object.size {
                    size
                } else {
                    LightSource::DEFAULT_SIZE
                };

                let mut color = LightSource::DEFAULT_COLOR;
                if let Some(prop) = object.properties.get("color").cloned() {
                    if let MapProperty::Color { value } = prop {
                        color = value;
                    }
                }

                let mut intensity = LightSource::DEFAULT_INTENSITY;
                if let Some(prop) = object.properties.get("intensity").cloned() {
                    if let MapProperty::Float { value } = prop {
                        intensity = value;
                    }
                }

                LightSource::add_node(object.position, size, color, intensity);
            }
        }

        if let Some(layer) = map.layers.get("items") {
            for object in &layer.objects {
                if let Some(prop) = object.properties.get("prototype_id").cloned() {
                    if let MapProperty::String { value: prototype_id } = prop {
                        if prototype_id == "credits" {
                            if let Some(prop) = object.properties.get("amount") {
                                if let MapProperty::Int { value } = prop {
                                    Credits::add_node(object.position, *value as u32);
                                }
                            }
                        } else {
                            let params = resources.items.get(&prototype_id).cloned().unwrap();
                            let mut instance_id = None;
                            if let Some(prop) = object.properties.get("instance_id").cloned() {
                                if let MapProperty::String { value } = prop {
                                    instance_id = Some(value)
                                }
                            }

                            Item::add_node(ItemParams {
                                id: instance_id.unwrap_or(generate_id()),
                                position: Some(object.position),
                                ..params
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }
}