use crate::prelude::*;

pub struct PostProcessing {
    lighting_material: Option<MaterialSource>,
    effect_material: Option<MaterialSource>,
    current_effect_material_id: Option<String>,
}

impl PostProcessing {
    const LIGHTING_MATERIAL_ID: &'static str = "dynamic_lighting";

    pub fn new() -> Result<Self> {
        let mut res = PostProcessing {
            lighting_material: None,
            effect_material: None,
            current_effect_material_id: None,
        };

        res.apply_config()?;

        Ok(res)
    }

    pub fn add_node() -> Result<Handle<Self>> {
        let node = Self::new()?;
        let res = scene::add_node(node);

        Ok(res)
    }

    // Apply current configuration from Config in storage.
    // This is called on construction but should also be called if config changes.
    // It does its own cleanup with deletion of compiled materials, where necessary.
    pub fn apply_config(&mut self) -> Result<()> {
        let resources = storage::get::<Resources>();
        let config = storage::get::<Config>();

        if config.dynamic_lighting {
            if self.lighting_material.is_none() {
                let mut material = resources.materials.get(Self::LIGHTING_MATERIAL_ID).cloned().unwrap();
                material.compile()?;
                self.lighting_material = Some(material);
            }
        } else if let Some(lighting_material) = &mut self.lighting_material {
            lighting_material.delete_compiled()?;
            self.lighting_material = None;
        }

        let mut should_load_effect_material = false;
        if let Some(material_id) = &config.post_processing {
            if material_id != "none" {
                should_load_effect_material = true;
            }
        }

        if should_load_effect_material {
            let material_id = config.post_processing.clone().unwrap();
            if let Some(current_id) = self.current_effect_material_id.clone() {
                if current_id != material_id {
                    let material = self.effect_material.as_mut().unwrap();
                    material.delete_compiled()?;
                    self.effect_material = None;
                    self.current_effect_material_id = None;
                }
            }

            if self.effect_material.is_none() {
                let mut material = resources.materials.get(&material_id).cloned().unwrap();
                material.compile()?;
                self.effect_material = Some(material);
                self.current_effect_material_id = Some(material_id);
            }
        } else if let Some(material) = &mut self.effect_material {
            material.delete_compiled()?;
            self.effect_material = None;
            self.current_effect_material_id = None;
        }

        Ok(())
    }
}

impl Node for PostProcessing {
    fn draw(node: RefMut<Self>) {
        let camera = scene::find_node_by_type::<CameraController>().unwrap();

        if let Some(material) = &node.lighting_material {
            material.use_compiled().unwrap();

            for light in scene::find_nodes_by_type::<LightSource>() {}
        }

        set_default_camera();

        if let Some(material) = &node.effect_material {
            material.use_compiled().unwrap();
        } else {
            use_default_material();
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

        use_default_material();
    }
}
