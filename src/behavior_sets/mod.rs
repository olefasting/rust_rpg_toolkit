use crate::prelude::*;
use crate::behavior_sets::default_humanoid::IdleMode;

pub mod default_humanoid;

pub type ActorBehaviorConstructor = fn() -> Box<dyn ActorBehavior>;

pub const DEFAULT_BEHAVIOR_SET_ID: &'static str = "default_humanoid";

static mut DIRECTORY: Option<HashMap<String, ActorBehaviorConstructor>> = None;

fn init_directory() {
    unsafe {
        DIRECTORY = Some(HashMap::new());
        DIRECTORY.as_mut().unwrap().insert(DEFAULT_BEHAVIOR_SET_ID.to_string(), IdleMode::new);
    }
}

fn get_directory() -> &'static mut HashMap<String, ActorBehaviorConstructor> {
    unsafe {
        if DIRECTORY.is_none() {
            init_directory();
        }
        DIRECTORY.as_mut().unwrap()
    }
}

pub fn get_behavior_set(id: &str) -> Option<ActorBehaviorConstructor> {
    let dir = get_directory();
    dir.get(id).cloned()
}

pub fn register_behavior_set(id: &str, constructor: ActorBehaviorConstructor) {
    let dir = get_directory();
    dir.insert(id.to_string(), constructor);
}

pub fn get_default_behavior_set() -> ActorBehaviorConstructor {
    get_behavior_set(DEFAULT_BEHAVIOR_SET_ID).unwrap()
}
