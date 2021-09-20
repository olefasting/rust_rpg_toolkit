use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Event {
    ToMainMenu,
    StartGame { character: Character },
    ChangeMap { chapter_index: usize, map_id: String },
    SavePlayerCharacter,
    Quit,
}

static mut EVENT_QUEUE: Option<Vec<Event>> = None;

fn get_event_queue() -> &'static mut Vec<Event> {
    unsafe {
        if EVENT_QUEUE.is_none() {
            EVENT_QUEUE = Some(Vec::new());
        }

        EVENT_QUEUE.as_mut().unwrap()
    }
}

pub fn get_next_event() -> Option<Event> {
    let queue = get_event_queue();
    queue.pop()
}

pub fn dispatch_event(event: Event) {
    let queue = get_event_queue();
    queue.insert(0, event);
}

// This will perform all the internal handling of an event and return it afterwards.
// Note that `Event::Quit` will clear the scene, but it will probably also need to be
// handled manually to break the game loop and exit.
// If you have no need handle events manually, `handle_queued_events` can be used in stead.
pub async fn handle_event(event: Event) -> Result<Event> {
    match event.clone() {
        Event::ToMainMenu => {
            scene::clear();
            gui::show_main_menu().await?;
        }
        Event::StartGame { character } => {
            load_scene(character)?;
        }
        Event::ChangeMap { chapter_index, map_id} => {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            let character = game_state
                .get_player_character()
                .expect("No player character found. Use `Event::ChangeMap` to start a new game!")
                .with_current_map(chapter_index, &map_id);
            load_scene(character)?;
        }
        Event::SavePlayerCharacter => {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            let character= game_state.get_player_character().unwrap();
            save_character(character)?;
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