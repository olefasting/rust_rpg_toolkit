#![feature(fn_traits)]
#![feature(drain_filter)]
#![feature(try_find)]
#![feature(async_closure)]
#![feature(hash_drain_filter)]

pub const TOOLKIT_VERSION: &'static str = "0.1.0";

pub mod resources;
pub mod ability;
pub mod map;
pub mod nodes;
pub mod render;
pub mod input;
pub mod physics;
pub mod math;
pub mod gui;
pub mod json;
pub mod helpers;
pub mod missions;
pub mod config;
pub mod uid;
pub mod modules;
pub mod dialogue;
pub mod scenario;
pub mod save_games;
pub mod versions;
pub mod game;
pub mod prelude;
pub mod inventory;
pub mod noise_level;

mod static_wrapper;
