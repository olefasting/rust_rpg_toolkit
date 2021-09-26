use crate::prelude::*;

const EVENT_QUEUE_SIZE: usize = 512;

#[derive(Debug, Clone)]
pub enum Event {
    OpenMainMenu,
    StartGame { character: Character },
    ChangeMap { chapter_index: usize, map_id: String },
    Save,
    Respawn,
    Quit,
}

impl Event {
    pub fn to_str(&self) -> &'static str {
        use Event::*;
        match self {
            OpenMainMenu => "open main menu",
            StartGame { character: _ } => "start game",
            ChangeMap { chapter_index: _, map_id: _ } => "change map",
            Save => "save",
            Respawn => "respawn",
            Quit => "quit",
        }
    }
}

static mut EVENT_QUEUE: Option<Vec<Event>> = None;

unsafe fn get_event_queue() -> &'static mut Vec<Event> {
    if EVENT_QUEUE.is_none() {
        let mut queue = Vec::new();
        queue.reserve(EVENT_QUEUE_SIZE);
        EVENT_QUEUE = Some(queue);
    }

    EVENT_QUEUE.as_mut().unwrap()
}

pub fn get_next_event() -> Option<Event> {
    let queue = unsafe { get_event_queue() };
    queue.pop()
}

pub fn dispatch_event(event: Event) {
    let queue = unsafe { get_event_queue() };
    queue.insert(0, event);
}

// This will perform all the internal handling of an event and return it afterwards.
// Note that `Event::Quit` will clear the scene, but it will probably also need to be
// handled manually to break the game loop and exit.
// If you have no need handle events manually, `handle_queued_events` can be used in stead.
pub async fn handle_event(event: Event) -> Result<Event> {
    // println!("Event: {}", event.to_str());
    match event.clone() {
        Event::OpenMainMenu => {
            scene::clear();
            gui::show_main_menu().await?;
        }
        Event::StartGame { character } => {
            load_scene(character)?;
        }
        Event::ChangeMap { chapter_index, map_id } => {
            let character = {
                let game_state = scene::find_node_by_type::<GameState>().unwrap();
                game_state
                    .get_player_character()
                    .expect("No player character found. Use `Event::StartGame` to start a new game!")
                    .with_map(chapter_index, &map_id)
            };

            load_scene(character)?;
        }
        Event::Respawn => {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            let path = character_name_to_path(&game_state.character_name);
            let character = load_character(path)?;
            character.spawn(game_state.handle(), game_state.player_spawn_point);
        }
        Event::Save => {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            let character = game_state.get_player_character().unwrap();
            character.save()?;
        }
        Event::Quit => {
            scene::clear();
        }
    }

    Ok(event)
}

// This will handle all queued events and return `true` if the game should continue running and
// `false` when it has handled an `Event::Quit`
pub async fn handle_queued_events() -> Result<bool> {
    let mut res = true;

    while let Some(event) = get_next_event() {
        if let Event::Quit = handle_event(event).await? {
            res = false;
        }
    }

    Ok(res)
}