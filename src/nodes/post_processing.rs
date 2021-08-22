use macroquad::{
    experimental::{
        scene::{
            Node,
            RefMut,
            Handle,
        },
    },
    color,
    prelude::*,
};

use super::Camera;

pub struct PostProcessing {
    pub material: Material,
}

impl PostProcessing {
    pub fn new(material: Material) -> Self {
        PostProcessing {
            material,
        }
    }

    pub fn add_node(material: Material) -> Handle<Self> {
        scene::add_node(Self::new(material))
    }
}

impl Node for PostProcessing {
    fn draw(node: RefMut<Self>) {
        let camera = scene::find_node_by_type::<Camera>().unwrap();

        set_default_camera();
        gl_use_material(node.material);
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
        gl_use_default_material();
    }
}
