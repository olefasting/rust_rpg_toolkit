use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Event {
    ShowMainMenu,
    ChangeMap(usize, String),
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

pub fn get_queued_event() -> Option<Event> {
    let queue = get_event_queue();
    queue.pop()
}

pub fn dispatch_event(event: Event) {
    let queue = get_event_queue();
    queue.insert(0, event);
}

// This will handle one event and return `true` if the game should quit
pub async fn handle_event(event: Event) -> Result<bool> {
    match event {
        Event::ShowMainMenu => {
            scene::clear();
            gui::show_main_menu().await?;
        }
        Event::ChangeMap(chapter_index, map_id) => {
            let character= {
                let game_state = scene::find_node_by_type::<GameState>().unwrap();
                let character = game_state.get_player_character().unwrap();

                Character {
                    current_chapter_index: chapter_index,
                    current_map_id: map_id,
                    ..character
                }
            };

            load_scene(character)?;
        }
        Event::SavePlayerCharacter => {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            let character= game_state.get_player_character().unwrap();
            save_character(character)?;
        }
        Event::Quit => {
            scene::clear();
            return Ok(true)
        }
    }

    Ok(false)
}

// This will handle all queued events and return `true` if the game should quit
pub async fn handle_event_queue() -> Result<bool> {
    while let Some(event) = get_queued_event() {
        if handle_event(event).await? == true {
            return Ok(true);
        }
    }

    Ok(false)
}