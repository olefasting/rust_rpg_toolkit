# Rendering

## Materials

You can load your own shader code, written in GLSL, by specifying a material's
vertex amd fragment shaders, under `materials` in the `assets.json`  file.

```rust
struct MaterialAssetParams {
    pub id: String,
    pub fragment_shader_path: String,
    pub vertex_shader_path: String,
}
```

Currently, materials are limited to post-processing. The material that will be used
is set by referencing the material's id, under `post_processing`, in a config file.
If no material has been defined, or if it has been set to `none`, there will be no
post-processing.
