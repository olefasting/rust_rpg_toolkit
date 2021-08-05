use nanoid::nanoid;

pub fn generate_string_id() -> String {
    nanoid!()
}

pub trait GetStringId {
    fn get_id(&self) -> String;
}

pub trait SetStringId {
    fn set_id(&mut self, id: &str);
}

pub trait StringId: GetStringId + SetStringId {}

pub fn has_duplicate_string_ids<T: GetStringId>(list: &[T]) -> bool {
    let ids: Vec<String> = list
        .iter()
        .map(|item| item.get_id())
        .collect();
    (1..ids.len()).any(|i| ids[i..].contains(&ids[i - 1]))
}

pub fn assert_unique_string_ids<T: GetStringId>(list: &[T], message: &str)
    where T: GetStringId {
    assert_eq!(has_duplicate_string_ids(list), false, "{}", message);
}
