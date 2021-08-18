use serde::{
    Serialize,
    Deserialize,
};
use crate::MapLayerKind;
use std::collections::HashMap;
use std::iter::FromIterator;
use crate::map::MapLayerKind::TileLayer;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 {
            x,
            y,
        }
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2 {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl From<macroquad::prelude::Vec2> for Vec2 {
    fn from(other: macroquad::prelude::Vec2) -> Self {
        Vec2 {
            x: other.x,
            y: other.y,
        }
    }
}

impl From<Vec2> for macroquad::prelude::Vec2 {
    fn from(other: Vec2) -> Self {
        macroquad::prelude::vec2(
            other.x,
            other.y,
        )
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

impl UVec2 {
    pub fn new(x: u32, y: u32) -> Self {
        UVec2 {
            x,
            y,
        }
    }
}

impl Default for UVec2 {
    fn default() -> Self {
        UVec2 {
            x: 0,
            y: 0,
        }
    }
}

impl From<macroquad::prelude::UVec2> for UVec2 {
    fn from(other: macroquad::prelude::UVec2) -> Self {
        UVec2 {
            x: other.x,
            y: other.y,
        }
    }
}

impl From<UVec2> for macroquad::prelude::UVec2 {
    fn from(other: UVec2) -> Self {
        macroquad::prelude::uvec2(
            other.x,
            other.y,
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Collider {
    kind: String,
    offset: Vec2,
    radius: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
}

impl Collider {
    pub const CIRCLE_KIND: &'static str = "circle";
    pub const RECTANGLE_KIND: &'static str = "rectangle";

    pub fn from(other: crate::Collider) -> Self {
        match other {
            crate::Collider::Circle(circle) => Collider {
                kind: Self::CIRCLE_KIND.to_string(),
                offset: Vec2::new(circle.x, circle.y),
                radius: Some(circle.r),
                width: None,
                height: None,
            },
            crate::Collider::Rectangle(rect) => Collider {
                kind: Self::RECTANGLE_KIND.to_string(),
                offset: Vec2::new(rect.x, rect.y),
                radius: None,
                width: Some(rect.w),
                height: Some(rect.h),
            }
        }
    }
}

impl From<Collider> for crate::Collider {
    fn from(other: Collider) -> Self {
        match &*other.kind {
            Collider::CIRCLE_KIND => crate::Collider::circle(other.offset.x, other.offset.y, other.radius.unwrap()),
            Collider::RECTANGLE_KIND => crate::Collider::rect(other.offset.x, other.offset.y, other.width.unwrap(), other.height.unwrap()),
            _ => {
                panic!("Invalid collider kind '{}", other.kind);
                crate::Collider::circle(0.0, 0.0, 0.0)
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpriteAnimationParams {
    pub offset: Vec2,
    pub texture_id: String,
    pub tile_size: Vec2,
    pub animations: Vec<Animation>,
    pub should_play: Option<bool>,
}

impl From<crate::render::SpriteAnimationParams> for SpriteAnimationParams {
    fn from(other: crate::render::SpriteAnimationParams) -> Self {
        SpriteAnimationParams {
            offset: Vec2::from(other.offset),
            texture_id: other.texture_id,
            tile_size: Vec2::from(other.tile_size),
            animations: other.animations.into_iter().map(|anim| Animation::from(anim)).collect(),
            should_play: if other.should_play { Some(true) } else { None },
        }
    }
}

impl From<SpriteAnimationParams> for crate::render::SpriteAnimationParams {
    fn from(other: SpriteAnimationParams) -> Self {
        crate::render::SpriteAnimationParams {
            offset: macroquad::prelude::Vec2::from(other.offset),
            texture_id: other.texture_id,
            tile_size: macroquad::prelude::Vec2::from(other.tile_size),
            animations: other.animations.into_iter().map(|anim| macroquad::prelude::animation::Animation::from(anim)).collect(),
            should_play: other.should_play.unwrap_or_default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub row: u32,
    pub frames: u32,
    pub fps: u32,
}

impl From<macroquad::prelude::animation::Animation> for Animation {
    fn from(other: macroquad::prelude::animation::Animation) -> Self {
        Animation {
            name: other.name,
            row: other.row,
            frames: other.frames,
            fps: other.fps,
        }
    }
}

impl From<Animation> for macroquad::prelude::animation::Animation {
    fn from(other: Animation) -> Self {
        macroquad::prelude::animation::Animation {
            name: other.name,
            row: other.row,
            frames: other.frames,
            fps: other.fps,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub offset: Vec2,
    pub rotation: Option<f32>,
    pub flip_x: Option<bool>,
    pub flip_y: Option<bool>,
    pub tile_size: UVec2,
    pub texture_id: String,
    pub texture_coords: UVec2,
}

impl From<crate::render::Sprite> for Sprite {
    fn from(other: crate::render::Sprite) -> Self {
        Sprite {
            offset: Vec2::from(other.offset),
            rotation: if other.rotation == 0.0 { None } else { Some(other.rotation) },
            flip_x:  if other.flip_x { Some(other.flip_x) } else { None },
            flip_y:  if other.flip_y { Some(other.flip_y) } else { None },
            tile_size: UVec2::from(other.tile_size),
            texture_id: other.texture_id,
            texture_coords: UVec2::from(other.texture_coords),
        }
    }
}

impl From<Sprite> for crate::render::Sprite {
    fn from(other: Sprite) -> Self {
        crate::render::Sprite {
            offset: macroquad::prelude::Vec2::from(other.offset),
            rotation: other.rotation.unwrap_or_default(),
            flip_x: other.flip_x.unwrap_or_default(),
            flip_y: other.flip_y.unwrap_or_default(),
            tile_size: macroquad::prelude::UVec2::from(other.tile_size),
            texture_id: other.texture_id,
            texture_coords: macroquad::prelude::UVec2::from(other.texture_coords),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemPrototype {
    pub id: String,
    pub name: String,
    pub description: String,
    pub kind: String,
    pub weight: f32,
    pub ability: ActorAbilityParams,
    pub sprite: Sprite,
}

impl From<crate::nodes::item::ItemPrototype> for ItemPrototype {
    fn from(other: crate::nodes::item::ItemPrototype) -> Self {
        ItemPrototype {
            id: other.id,
            name: other.name,
            description: other.description,
            kind: other.kind,
            weight: other.weight,
            ability: ActorAbilityParams::from(other.ability),
            sprite: Sprite::from(other.sprite),
        }
    }
}

impl From<ItemPrototype> for crate::nodes::item::ItemPrototype {
    fn from(other: ItemPrototype) -> Self {
        crate::nodes::item::ItemPrototype {
            id: other.id.clone(),
            name: other.name,
            description: other.description,
            kind: other.kind,
            weight: other.weight,
            ability: crate::nodes::actor::ActorAbilityParams::from(other.ability),
            sprite: crate::render::Sprite::from(other.sprite),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemParams {
    pub name: String,
    pub description: String,
    pub position: Option<Vec2>,
    pub kind: String,
    pub weight: f32,
    pub ability: ActorAbilityParams,
    pub sprite: Sprite,
}

impl From<crate::nodes::item::ItemParams> for ItemParams {
    fn from(other: crate::nodes::item::ItemParams) -> Self {
        let position = if let Some(position) = other.position {
            Some(Vec2::from(position))
        } else {
            None
        };
        ItemParams {
            name: other.name,
            description: other.description,
            position,
            kind: other.kind,
            weight: other.weight,
            ability: ActorAbilityParams::from(other.ability),
            sprite: Sprite::from(other.sprite),
        }
    }
}

impl From<ItemParams> for crate::nodes::item::ItemParams {
    fn from(other: ItemParams) -> Self {
        let position = if let Some(position) = other.position {
            Some(macroquad::prelude::Vec2::from(position))
        } else {
            None
        };
        crate::nodes::item::ItemParams {
            name: other.name,
            description: other.description,
            position,
            kind: other.kind,
            weight: other.weight,
            ability: crate::nodes::actor::ActorAbilityParams::from(other.ability),
            sprite: crate::render::Sprite::from(other.sprite),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorAbilityParams {
    pub effect_kind: String,
    pub action_kind: String,
    pub cooldown: Option<f32>,
    pub health_cost: f32,
    pub stamina_cost: f32,
    pub energy_cost: f32,
    pub speed: f32,
    pub spread: f32,
    pub range: f32,
    pub damage: f32,
    pub effect_size: f32,
    pub effect_color: Color,
    pub effect_sprite_animation: Option<SpriteAnimationParams>,
}

impl From<crate::nodes::actor::ActorAbilityParams> for ActorAbilityParams {
    fn from(other: crate::nodes::actor::ActorAbilityParams) -> Self {
        let effect_sprite_animation =
            if let Some(params) = other.effect_sprite_animation.clone() {
                Some(SpriteAnimationParams::from(params))
            } else {
                None
            };
        ActorAbilityParams {
            effect_kind: other.effect_kind.to_string(),
            action_kind: other.action_kind.to_string(),
            cooldown: if other.cooldown == 0.0 { None } else { Some(other.cooldown) },
            health_cost: other.health_cost,
            stamina_cost: other.stamina_cost,
            energy_cost: other.energy_cost,
            speed: other.speed,
            spread: other.spread,
            range: other.range,
            damage: other.damage,
            effect_size: other.effect_size,
            effect_color: Color::from(other.effect_color),
            effect_sprite_animation,
        }
    }
}

impl From<ActorAbilityParams> for crate::nodes::actor::ActorAbilityParams {
    fn from(other: ActorAbilityParams) -> Self {
        let effect_sprite_animation =
            if let Some(params) = other.effect_sprite_animation {
                Some(crate::render::SpriteAnimationParams::from(params))
            } else {
                None
            };
        crate::nodes::actor::ActorAbilityParams {
            effect_kind: other.effect_kind.to_string(),
            action_kind: other.action_kind.to_string(),
            cooldown: other.cooldown.unwrap_or_default(),
            health_cost: other.health_cost,
            stamina_cost: other.stamina_cost,
            energy_cost: other.energy_cost,
            speed: other.speed,
            spread: other.spread,
            range: other.range,
            damage: other.damage,
            effect_size: other.effect_size,
            effect_color: macroquad::prelude::Color::from(other.effect_color),
            effect_sprite_animation,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorStats {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub willpower: u32,
    pub perception: u32,
    pub charisma: u32,
    pub current_health: Option<f32>,
    pub max_health: Option<f32>,
    pub current_stamina: Option<f32>,
    pub max_stamina: Option<f32>,
    pub current_energy: Option<f32>,
    pub max_energy: Option<f32>,
    pub health_regen: Option<f32>,
    pub stamina_regen: Option<f32>,
    pub energy_regen: Option<f32>,
    pub carry_capacity: Option<f32>,
    pub move_speed: Option<f32>,
    pub is_static: Option<bool>,
}

impl ActorStats {
    pub fn new(
        strength: u32,
        dexterity: u32,
        constitution: u32,
        intelligence: u32,
        willpower: u32,
        perception: u32,
        charisma: u32,
    ) -> Self {
        ActorStats {
            strength,
            dexterity,
            constitution,
            intelligence,
            willpower,
            perception,
            charisma,
            is_static: Some(false),
            ..Default::default()
        }
    }

    pub fn new_static(
        current_health: f32,
        max_health: f32,
        current_energy: f32,
        max_stamina: f32,
        current_stamina: f32,
        max_energy: f32,
        carry_capacity: f32,
        move_speed: f32,
    ) -> Self {
        ActorStats {
            current_health: Some(current_health),
            max_health: Some(max_health),
            current_stamina: Some(current_stamina),
            max_stamina: Some(max_stamina),
            current_energy: Some(current_energy),
            max_energy: Some(max_energy),
            carry_capacity: Some(carry_capacity),
            move_speed: Some(move_speed),
            is_static: Some(true),
            ..Default::default()
        }
    }
}

impl From<crate::ActorStats> for ActorStats {
    fn from(other: crate::ActorStats) -> Self {
        ActorStats {
            strength: other.strength,
            dexterity: other.dexterity,
            constitution: other.constitution,
            intelligence: other.intelligence,
            willpower: other.willpower,
            perception: other.perception,
            charisma: other.charisma,
            current_health: Some(other.current_health),
            max_health: Some(other.max_health),
            current_stamina: Some(other.current_stamina),
            max_stamina: Some(other.max_stamina),
            current_energy: Some(other.current_energy),
            max_energy: Some(other.max_energy),
            health_regen: Some(other.health_regen),
            stamina_regen: Some(other.stamina_regen),
            energy_regen: Some(other.energy_regen),
            carry_capacity: Some(other.carry_capacity),
            move_speed: Some(other.move_speed),
            is_static: Some(other.is_static),
        }
    }
}

impl From<ActorStats> for crate::ActorStats {
    fn from(other: ActorStats) -> Self {
        crate::ActorStats {
            strength: other.strength,
            dexterity: other.dexterity,
            constitution: other.constitution,
            intelligence: other.intelligence,
            willpower: other.willpower,
            perception: other.perception,
            charisma: other.charisma,
            current_health: other.current_health.unwrap_or_default(),
            max_health: other.max_health.unwrap_or_default(),
            current_stamina: other.current_stamina.unwrap_or_default(),
            max_stamina: other.max_stamina.unwrap_or_default(),
            current_energy: other.current_energy.unwrap_or_default(),
            max_energy: other.max_energy.unwrap_or_default(),
            health_regen: other.health_regen.unwrap_or_default(),
            stamina_regen: other.stamina_regen.unwrap_or_default(),
            energy_regen: other.energy_regen.unwrap_or_default(),
            carry_capacity: other.carry_capacity.unwrap_or_default(),
            move_speed: other.move_speed.unwrap_or_default(),
            is_static: other.is_static.unwrap_or_default(),
        }
    }
}

impl Default for ActorStats {
    fn default() -> Self {
        ActorStats {
            strength: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            willpower: 0,
            perception: 0,
            charisma: 0,
            current_health: Some(1.0),
            max_health: Some(1.0),
            current_stamina: Some(0.0),
            max_stamina: Some(0.0),
            current_energy: Some(0.0),
            max_energy: Some(0.0),
            health_regen: Some(0.0),
            stamina_regen: Some(0.0),
            energy_regen: Some(0.0),
            carry_capacity: Some(0.0),
            move_speed: Some(0.0),
            is_static: Some(true),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorPrototype {
    pub id: String,
    pub name: String,
    pub stats: ActorStats,
    pub factions: Vec<String>,
    pub collider: Option<Collider>,
    pub inventory: Vec<String>,
    pub sprite_animation: SpriteAnimationParams,
}

impl From<crate::nodes::actor::ActorPrototype> for ActorPrototype {
    fn from(other: crate::nodes::actor::ActorPrototype) -> Self {
        let collider = if let Some(collider) = other.collider {
            Some(Collider::from(collider))
        } else {
            None
        };
        ActorPrototype {
            id: other.id,
            name: other.name,
            stats: ActorStats::from(other.stats),
            factions: other.factions,
            collider,
            inventory: other.inventory,
            sprite_animation: SpriteAnimationParams::from(other.sprite_animation),
        }
    }
}

impl From<ActorPrototype> for crate::nodes::actor::ActorPrototype {
    fn from(other: ActorPrototype) -> Self {
        let collider = if let Some(collider) = other.collider {
            Some(crate::Collider::from(collider))
        } else {
            None
        };
        crate::nodes::actor::ActorPrototype {
            id: other.id,
            name: other.name,
            stats: crate::nodes::actor::ActorStats::from(other.stats),
            factions: other.factions,
            collider,
            inventory: other.inventory,
            sprite_animation: crate::render::SpriteAnimationParams::from(other.sprite_animation),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorParams {
    pub position: Vec2,
    pub name: String,
    pub stats: ActorStats,
    pub factions: Vec<String>,
    pub collider: Option<Collider>,
    pub inventory: Vec<ItemParams>,
    pub sprite_animation: SpriteAnimationParams,
}

impl From<crate::ActorParams> for ActorParams {
    fn from(other: crate::ActorParams) -> Self {
        let collider = if let Some(collider) = other.collider {
            Some(Collider::from(collider))
        } else {
            None
        };
        ActorParams {
            position: Vec2::from(other.position),
            name: other.name,
            stats: ActorStats::from(other.stats),
            factions: other.factions,
            collider,
            inventory: other.inventory.into_iter().map(|params| ItemParams::from(params)).collect(),
            sprite_animation: SpriteAnimationParams::from(other.sprite_animation),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<macroquad::prelude::Color> for Color {
    fn from(other: macroquad::color::Color) -> Self {
        Color {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }
}

impl From<Color> for macroquad::prelude::Color {
    fn from(other: Color) -> Self {
        macroquad::color::Color {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }
}

const TILE_LAYER_KIND: &'static str = "tile_layer";
const OBJECT_LAYER_KIND: &'static str = "object_layer";

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub world_offset: Option<Vec2>,
    pub grid_size: UVec2,
    pub tile_size: Vec2,
    pub layers: Vec<MapLayer>,
    pub tilesets: Vec<MapTileset>,
}

impl From<crate::Map> for Map {
    fn from(other: crate::Map) -> Self {
        let layers = other.layers.iter().map(|(_, layer)|  {
            let (kind, tiles, objects) = match layer.kind {
                crate::MapLayerKind::TileLayer => {
                    (TILE_LAYER_KIND.to_string(),
                     Some(layer.tiles.iter().map(|opt| match opt {
                         Some(tile) => {
                             let tileset = other.tilesets.get(&tile.tileset_id)
                                 .expect(&format!("Unable to find tileset with id '{}'!", tile.tileset_id));
                             tile.tile_id + tileset.first_tile_id
                         },
                         _ => 0,
                     }).collect()),
                     None)
                }
                crate::MapLayerKind::ObjectLayer => {
                    (OBJECT_LAYER_KIND.to_string(),
                     None,
                     Some(layer.objects.iter().map(|object| MapObject::from(object.clone())).collect()))
                }
            };
            MapLayer {
                id: layer.id.clone(),
                kind,
                objects,
                tiles,
            }
        }).collect();

        Map {
            world_offset: if other.world_offset != macroquad::prelude::Vec2::ZERO { Some(Vec2::from(other.world_offset)) } else { None },
            grid_size: UVec2::from(other.grid_size),
            tile_size: Vec2::from(other.tile_size),
            layers,
            tilesets: other.tilesets.into_iter().map(|(_, tileset)| MapTileset::from(tileset)).collect(),
        }
    }
}

impl From<Map> for crate::Map {
    fn from(other: Map) -> Self {
        let tilesets = HashMap::from_iter(
            other.tilesets
                .into_iter()
                .map(|tileset| (tileset.id.clone(), crate::MapTileset::from(tileset))));

        let world_offset = macroquad::prelude::Vec2::from(other.world_offset.unwrap_or_default());
        let grid_size = macroquad::prelude::UVec2::from(other.grid_size);
        let tile_size = macroquad::prelude::Vec2::from(other.tile_size);

        let layers = HashMap::from_iter(
            other.layers
                .into_iter()
                .map(|layer| {
                    let tiles = layer.tiles
                        .unwrap_or_default()
                        .into_iter()
                        .map(|tile_id| if tile_id == 0 { None } else {
                            match tilesets
                                .iter()
                                .find(|(_, tileset)| tile_id >= tileset.first_tile_id
                                    && tile_id <= tileset.first_tile_id + tileset.grid_size.x * tileset.grid_size.y) {
                                Some((_, tileset)) => Some(crate::MapTile {
                                    tile_id: tile_id - tileset.first_tile_id,
                                    tileset_id: tileset.id.clone(),
                                    texture_id: tileset.texture_id.clone(),
                                    texture_coords: tileset.get_texture_position_from_tile_id(tile_id),
                                }),
                                _ => {
                                    panic!("Unable to determine tileset from tile_id '{}'", tile_id);
                                    None
                                }
                            }
                        }).collect();

                    let layer = crate::MapLayer {
                        id: layer.id.clone(),
                        kind: MapLayerKind::from(&*layer.kind),
                        grid_size,
                        tiles,
                        objects: layer.objects
                            .unwrap_or_default()
                            .into_iter()
                            .map(|object| crate::MapObject::from(object))
                            .collect(),
                    };
                    (layer.id.clone(), layer)
                }));

        crate::Map {
            world_offset,
            grid_size,
            tile_size,
            layers,
            tilesets,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapLayer {
    pub id: String,
    pub kind: String,
    pub tiles: Option<Vec<u32>>,
    pub objects: Option<Vec<MapObject>>,
}

impl From<&str> for crate::MapLayerKind {
    fn from(other: &str) -> Self {
        match other {
            TILE_LAYER_KIND => MapLayerKind::TileLayer,
            OBJECT_LAYER_KIND => MapLayerKind::ObjectLayer,
            _ => {
                panic!("Invalid map layer kind '{}'!", other);
                MapLayerKind::TileLayer
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapObject {
    pub id: String,
    pub prototype_id: Option<String>,
    pub position: Vec2,
}

impl From<crate::MapObject> for MapObject {
    fn from(other: crate::MapObject) -> Self {
        MapObject {
            id: other.id.clone(),
            prototype_id: other.prototype_id.clone(),
            position: Vec2::from(other.position),
        }
    }
}


impl From<MapObject> for crate::MapObject {
    fn from(other: MapObject) -> Self {
        crate::MapObject {
            id: other.id.clone(),
            prototype_id: other.prototype_id.clone(),
            position: macroquad::prelude::Vec2::from(other.position),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapTileset {
    pub id: String,
    pub texture_id: String,
    pub texture_size: UVec2,
    pub tile_size: UVec2,
    pub grid_size: UVec2,
    pub first_tile_id: u32,
    pub tile_cnt: u32,
}

impl From<crate::MapTileset> for MapTileset {
    fn from(other: crate::MapTileset) -> Self {
        MapTileset {
            id: other.id.clone(),
            texture_id: other.texture_id.clone(),
            texture_size: UVec2::from(other.texture_size),
            tile_size: UVec2::from(other.tile_size),
            grid_size: UVec2::from(other.grid_size),
            first_tile_id: other.first_tile_id,
            tile_cnt: other.tile_cnt,
        }
    }
}

impl From<MapTileset> for crate::MapTileset {
    fn from(other: MapTileset) -> Self {
        crate::MapTileset {
            id: other.id.clone(),
            texture_id: other.texture_id.clone(),
            texture_size: crate::UVec2::from(other.texture_size),
            tile_size: crate::UVec2::from(other.tile_size),
            grid_size: crate::UVec2::from(other.grid_size),
            first_tile_id: other.first_tile_id,
            tile_cnt: other.tile_cnt,
        }
    }
}
