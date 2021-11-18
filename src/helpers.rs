use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
};

use macroquad::prelude::*;

// Used in serde attributes to skip serialization of bools that are false
pub fn is_false(value: &bool) -> bool {
    *value
}

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

pub fn get_timestamp() -> String {
    chrono::Utc::now().to_string()
}

const UID_CHARS_LEN: usize = 36;
const UID_LENGTH: usize = 16;
const UID_CHARS: [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

pub fn generate_id() -> String {
    let mut chars = Vec::with_capacity(UID_LENGTH);

    for _ in 0..UID_LENGTH {
        let i: usize = rand::gen_range(0, UID_CHARS_LEN);
        chars.push(UID_CHARS[i]);
    }

    chars.iter().collect::<String>()
}
