use std::any::Any;

use macroquad::experimental::collections::storage;
use std::ops::Deref;

pub fn try_get_global<T: Any>() -> Option<impl Deref<Target = T>> {
    storage::try_get::<T>()
}

pub fn get_global<T: Any>() -> impl Deref<Target = T> {
    storage::get::<T>()
}

pub fn set_global<T: Any>(data: T) {
    storage::store::<T>(data);
}
