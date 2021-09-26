pub use texture::Texture;

pub use material::{
    MaterialSource,
    use_default_material,
};

pub use sprite::{
    Sprite,
};

pub use animation::{
    SpriteAnimationPlayer,
    SpriteAnimationParams,
};

pub use viewport::{
    Viewport,
};

pub use helpers::{
    VerticalAlignment,
    HorizontalAlignment,
    draw_progress_bar,
    draw_aligned_text,
    color_from_hex_string,
};

pub mod sprite;
pub mod animation;
pub mod viewport;
pub mod helpers;
pub mod texture;
pub mod material;

use crate::prelude::*;

pub const COLOR_NONE: Color = Color::new(0.00, 0.00, 0.00, 0.00);