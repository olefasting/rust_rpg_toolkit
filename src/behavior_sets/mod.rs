use crate::prelude::*;
use crate::behavior_sets::default_humanoid::IdleMode;

pub mod default_humanoid;

pub type ActorBehaviorConstructor = fn() -> Box<dyn ActorBehavior>;

pub const DEFAULT_BEHAVIOR_SET_ID: &'static str = "default_humanoid";

static mut DIRECTORY: Option<HashMap<String, ActorBehaviorConstructor>> = None;

fn get_directory() -> &'static mut HashMap<String, ActorBehaviorConstructor> {
    unsafe {
        if DIRECTORY.is_none() {
            DIRECTORY = Some(HashMap::new());
            DIRECTORY.as_mut().unwrap().insert(DEFAULT_BEHAVIOR_SET_ID.to_string(), IdleMode::new);
        }

        DIRECTORY.as_mut().unwrap()
    }
}

pub fn try_get_behavior_set(id: &str) -> Option<ActorBehaviorConstructor> {
    let dir = get_directory();
    dir.get(id).cloned()
}

pub fn get_behavior_set(id: &str) -> ActorBehaviorConstructor {
    try_get_behavior_set(id).unwrap()
}

pub fn get_default_behavior_set() -> ActorBehaviorConstructor {
    get_behavior_set(DEFAULT_BEHAVIOR_SET_ID)
}

pub fn register_behavior_set(id: &str, constructor: ActorBehaviorConstructor) {
    let dir = get_directory();
    dir.insert(id.to_string(), constructor);
}
