use macroquad::{
    experimental::{
        scene::{
            Node,
            RefMut,
            Handle,
        },
        collections::storage,
    },
    color,
    prelude::*,
};

use crate::{
    Resources,
    Config,
};

use super::Camera;

pub struct PostProcessing {
    pub material: Option<Material>,
}

impl PostProcessing {
    pub fn new() -> Self {
        let config = storage::get::<Config>();
        let resources = storage::get::<Resources>();
        let material = if let Some(material_id) = &config.post_processing {
            if material_id == "none" {
                None
            } else {
                Some(resources.materials.get(material_id).cloned().unwrap())
            }
        } else {
            None
        };

        PostProcessing {
            material,
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }
}

impl Node for PostProcessing {
    fn draw(node: RefMut<Self>) {
        let camera = scene::find_node_by_type::<Camera>().unwrap();

        set_default_camera();
        if let Some(material) = node.material {
            gl_use_material(material);
        }
        draw_texture_ex(
            camera.get_render_target().texture,
            0.0,
            0.0,
            color::WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        if node.material.is_some() {
            gl_use_default_material();
        }
    }
}
