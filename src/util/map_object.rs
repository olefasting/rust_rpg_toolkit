use macroquad::{
    experimental::scene::{
        RefMut,
        HandleUntyped,
    },
    prelude::*,
};

#[derive(Clone)]
pub struct MapObjectCapabilities {
    pub get_id: fn(node: HandleUntyped) -> String,
    pub get_position: fn(node: HandleUntyped) -> Vec2,
}

#[allow(dead_code)]
pub type MapObjectProvider = (HandleUntyped, MapObjectCapabilities);

pub trait MapObject {
    fn map_object_capabilities() -> MapObjectCapabilities;
    fn apply_map_object_provider<T>(node: RefMut<T>);
}
