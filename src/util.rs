pub use circle::Circle;
pub use input::get_mouse_position;
pub use string_id::{assert_unique_string_ids, generate_string_id, GetStringId, has_duplicate_string_ids, SetStringId, StringId};
pub use text::{draw_aligned_text, TextAlignment};
pub use map_object::{MapObjectProvider, MapObjectCapabilities, MapObject};

mod text;
mod input;
mod string_id;
mod circle;
pub mod map_object;
