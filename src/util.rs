mod string_id;
mod map_object;
mod global_value;

pub use map_object::{
    MapObject,
    MapObjectCapabilities,
    MapObjectProvider,
};

pub use string_id::{
    assert_unique_string_ids,
    generate_string_id,
    GetStringId,
    has_duplicate_string_ids,
    SetStringId,
    StringId,
};

pub use global_value::{
    GlobalValue,
    try_get_global,
    get_global,
    set_global,
};
