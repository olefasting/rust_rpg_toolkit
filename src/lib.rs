#![feature(fn_traits)]
#![feature(drain_filter)]
#![feature(try_find)]
#![feature(async_closure)]

pub mod ability;
pub mod audio;
pub mod behavior_sets;
pub mod chapter;
pub mod character;
pub mod config;
pub mod dialogue;
pub mod error;
pub mod events;
pub mod file_io;
pub mod game;
pub mod gui;
pub mod helpers;
pub mod input;
pub mod inventory;
pub mod json;
pub mod map;
pub mod math;
pub mod missions;
pub mod modules;
pub mod nodes;
pub mod noise_level;
pub mod physics;
pub mod player;
pub mod prelude;
pub mod render;
pub mod resources;
pub mod scene;
pub mod versions;

pub use macroquad;
pub use serde;
pub use serde_json;
