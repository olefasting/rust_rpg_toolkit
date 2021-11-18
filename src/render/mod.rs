pub use texture::{draw_texture, Texture};

pub use material::{use_default_material, use_material, Material};

pub use sprite::Sprite;

pub use animation::{SpriteAnimationParams, SpriteAnimationPlayer};

pub use viewport::Viewport;

pub use helpers::{
    color_from_hex_string, draw_progress_bar, draw_text, HorizontalAlignment, VerticalAlignment,
};

pub mod animation;
pub mod helpers;
pub mod material;
pub mod sprite;
pub mod texture;
pub mod viewport;

use crate::prelude::*;

pub const COLOR_NONE: Color = Color::new(0.00, 0.00, 0.00, 0.00);
