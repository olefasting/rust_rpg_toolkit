use macroquad::*;

const UID_CHARS: [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0',
    '1', '2', '3', '4', '5', '6', '7', '8', '9',
];
const UID_CHARS_LEN: usize = 36;
const UID_LENGTH: usize = 16;

pub fn generate_id() -> String {
    let mut chars = Vec::with_capacity(UID_LENGTH);
    for _ in 0..UID_LENGTH {
        let i: usize = rand::gen_range(0, UID_CHARS_LEN);
        chars.push(UID_CHARS[i]);
    }
    chars.iter().collect::<String>()
}
