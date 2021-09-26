use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Texture {
    texture: Texture2D,
    height_map: Option<Texture2D>,
    normal_map: Option<Texture2D>,
}

impl Texture {
    pub fn new(texture: Texture2D, height_map: Option<Texture2D>, normal_map: Option<Texture2D>) -> Self {
        Texture {
            texture,
            height_map,
            normal_map,
        }
    }

    pub fn has_height_map(&self) -> bool {
        self.height_map.is_some()
    }

    pub fn has_normal_map(&self) -> bool {
        self.normal_map.is_some()
    }

    pub fn get(&self) -> Texture2D {
        self.texture
    }

    pub fn get_height_map(&self) -> Option<Texture2D> {
        self.normal_map
    }

    pub fn get_normal_map(&self) -> Option<Texture2D> {
        self.normal_map
    }
}