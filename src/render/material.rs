use crate::prelude::*;

use macroquad::material::{
    self,
    load_material,
};

#[derive(Debug, Clone)]
pub struct Material {
    compiled: Option<material::Material>,
    vertex_src: String,
    fragment_src: String,
    textures: Vec<String>,
    uniforms: HashMap<String, UniformType>,
    pipeline_params: PipelineParams,
}

impl Material {
    pub async fn new<P: AsRef<Path>>(vertex_path: P, fragment_path: P, params: MaterialParams) -> Result<Self> {
        let vertex_path = vertex_path.as_ref();
        let fragment_path = fragment_path.as_ref();

        let vertex_bytes = load_file(vertex_path).await?;
        let vertex_src = String::from_utf8(vertex_bytes)?;

        let fragment_bytes = load_file(fragment_path).await?;
        let fragment_src = String::from_utf8(fragment_bytes)?;

        let textures = params.textures;
        let uniforms = HashMap::from_iter(params.uniforms.into_iter());

        let res = Material {
            compiled: None,
            vertex_src,
            fragment_src,
            textures,
            uniforms,
            pipeline_params: PipelineParams {
                ..Default::default()
            },
        };

        Ok(res)
    }

    pub fn is_compiled(&self) -> bool {
        self.compiled.is_some()
    }

    pub fn compile(&mut self) -> Result<()> {
        if let Some(compiled) = &mut self.compiled {
            compiled.delete();
        }

        let textures = self.textures.clone();
        let uniforms = self.uniforms
            .iter()
            .map(|(key, value)| (key.clone(), *value))
            .collect();

        let pipeline_params = self.pipeline_params.clone();

        let res = load_material(
            &self.vertex_src,
            &self.fragment_src,
            MaterialParams {
                textures,
                uniforms,
                pipeline_params,
            },
        )?;

        self.compiled = Some(res);

        Ok(())
    }

    pub fn compile_and_use(&mut self) -> Result<()> {
        self.compile()?;
        self.use_material()?;

        Ok(())
    }

    pub fn delete_compiled(&mut self) -> Result<()> {
        if let Some(compiled) = &mut self.compiled {
            compiled.delete();
        } else {
            let err = Error::from_str(ErrorKind::Material, &"Attempting to delete a material that has not been compiled");
            return Err(err);
        }

        Ok(())
    }

    pub fn use_material(&self) -> Result<()> {
        if let Some(material) = self.compiled.clone() {
            gl_use_material(material);
        } else {
            let err = Error::from_str(ErrorKind::Material, &"Attempting to use a material that has not been compiled");
            return Err(err);
        }

        Ok(())
    }
}

pub fn use_default_material() {
    gl_use_default_material()
}

pub fn use_material(material: &Material) -> Result<()> {
    material.use_material()
}