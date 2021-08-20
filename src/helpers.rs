use std::cmp::Ordering;

use macroquad::prelude::*;

pub fn sort_by_distance(position: Vec2, a: &Vec2, b: &Vec2) -> Ordering {
    a.distance(position)
        .partial_cmp(&b.distance(position))
        .unwrap()
}
