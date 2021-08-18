use std::{
    any::Any,
    ops::Deref,
};

use macroquad::{
    experimental::collections::storage,
    color,
    prelude::*,
};

#[derive(Copy, Clone)]
pub struct LocalPlayer {
    pub id: u32,
}

#[derive(Copy, Clone)]
pub struct DebugMode {
    pub color: Color,
    pub is_enabled: bool,
}

pub fn try_get_global<T: Any>() -> Option<impl Deref<Target = T>> {
    storage::try_get::<T>()
}

pub fn get_global<T: Any>() -> impl Deref<Target = T> {
    storage::get::<T>()
}

pub fn set_global<T: Any>(data: T) {
    storage::store::<T>(data);
}
