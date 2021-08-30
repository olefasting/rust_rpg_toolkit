use std::{
    path::{PathBuf, Path},
    cmp::Ordering,
};

use macroquad::prelude::*;

pub fn sort_by_distance(position: Vec2, a: &Vec2, b: &Vec2) -> Ordering {
    a.distance(position)
        .partial_cmp(&b.distance(position))
        .unwrap()
}

pub fn remove_filename(path: PathBuf) -> PathBuf {
    path.ancestors()
        .collect::<Vec<&Path>>()
        .get(1)
        .cloned()
        .unwrap()
        .to_path_buf()
}
