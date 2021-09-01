use std::ops::Sub;

use macroquad::{
    experimental::{
        collections::storage,
    },
    prelude::*,
};

pub use gilrs::{
    Gilrs,
    Event,
    EventType,
    Axis,
    Button,
    GamepadId,
    Gamepad,
};

use crate::prelude::*;

static mut INPUT_CONTEXT: StaticContext = StaticContext {
    context: None,
};

struct StaticContext {
    pub context: Option<InputContext>,
}

#[derive(Debug)]
struct InputContext {
    pub gilrs: Gilrs,
    pub mappings: HashMap<String, GamepadId>,
    pub events: HashMap<GamepadId, Vec<EventType>>,
}

fn get_input_context() -> &'static mut InputContext {
    let wrapper = unsafe { &mut INPUT_CONTEXT };
    if wrapper.context.is_none() {
        let context = InputContext {
            gilrs: Gilrs::new().unwrap(),
            mappings: HashMap::new(),
            events: HashMap::new(),
        };
        wrapper.context = Some(context);
    }
    wrapper.context.as_mut().unwrap()
}

fn is_mapped(gamepad_id: GamepadId) -> bool {
    let context = get_input_context();
    context.mappings
        .iter()
        .find(|(_, mapped_id)| gamepad_id == **mapped_id)
        .is_some()
}

fn is_active(gamepad_id: GamepadId) -> bool {
    let context = get_input_context();
    context.gilrs
        .gamepads()
        .find(|(available_id, _)| gamepad_id == *available_id)
        .is_some()
}

pub fn update_input() {
    let mut context = get_input_context();
    context.mappings.retain(|player_id, gamepad_id| {
        if is_active(*gamepad_id) == false {
            println!("Gamepad '{}' for player '{}' has been disconnected!", gamepad_id, player_id);
            return false;
        }
        true
    });

    context.events = HashMap::new();
    while let Some(event) = context.gilrs.next_event() {
        let gamepad_id = event.id;
        if let Some(vec) = context.events.get_mut(&gamepad_id) {
            vec.push(event.event);
        } else {
            context.events.insert(gamepad_id, vec!(event.event));
        }
    }
}

pub fn apply_input(player_id: &str, actor: &mut Actor) {
    let mut game_state = scene::find_node_by_type::<GameState>().unwrap();

    actor.controller.should_use_primary_ability = false;
    actor.controller.should_use_secondary_ability = false;
    actor.controller.move_direction = Vec2::ZERO;
    actor.controller.should_start_interaction = false;
    actor.controller.should_pick_up_items = false;
    actor.controller.should_sprint = actor.controller.is_sprint_locked && actor.controller.should_sprint;

    if let Some(gamepad_id) = get_gamepad_id(player_id) {
        for event in InputEventIterator::new(gamepad_id) {
            match event {
                EventType::ButtonChanged(button, value, _code) => {
                    match button {
                        Button::South => {},
                        Button::East => {
                            if value > 0.0 {
                                if game_state.should_show_game_menu {
                                    game_state.should_show_game_menu = false;
                                } else if actor.current_dialogue.is_some() {
                                    actor.current_dialogue = None;
                                }
                            }
                        },
                        Button::North => {},
                        Button::West => {
                            if value > 0.0 {
                                actor.controller.should_start_interaction = true;
                            }
                        },
                        Button::C => {},
                        Button::Z => {},
                        Button::LeftTrigger => {},
                        Button::LeftTrigger2 => {
                            // SELECT
                            if value > 0.0 {
                                if game_state.should_show_character_window || game_state.should_show_inventory_window {
                                    game_state.should_show_inventory_window = false;
                                    game_state.should_show_character_window = false;
                                } else {
                                    game_state.should_show_inventory_window = true;
                                    game_state.should_show_character_window = true;
                                }
                            }
                        },
                        Button::RightTrigger => {},
                        Button::RightTrigger2 => {},
                        Button::Select => {},
                        Button::Start => {
                            if value > 0.0 {
                                game_state.should_show_game_menu = !game_state.should_show_game_menu;
                            }
                        },
                        Button::Mode => {},
                        Button::LeftThumb => {},
                        Button::RightThumb => {
                            if value > 0.0 {
                                actor.controller.should_sprint = !actor.controller.should_sprint;
                                actor.controller.is_sprint_locked = actor.controller.should_sprint;
                            }
                        },
                        Button::DPadUp => {},
                        Button::DPadDown => {},
                        Button::DPadLeft => {},
                        Button::DPadRight => {},
                        Button::Unknown => {},
                    }
                }
                EventType::AxisChanged(axis, value, _code) => {
                    match axis {
                        Axis::LeftStickX => {
                            actor.controller.move_direction.x = value;
                        },
                        Axis::LeftStickY => {
                            actor.controller.move_direction.y = value;
                        },
                        Axis::LeftZ => {}
                        Axis::RightStickX => {},
                        Axis::RightStickY => {},
                        Axis::RightZ => {},
                        Axis::DPadX => {},
                        Axis::DPadY => {},
                        Axis::Unknown => {},
                    }
                }
                EventType::Disconnected => {
                    println!("Gamepad '{}' used by player '{}' was disconnected!", gamepad_id, player_id);
                    break;
                }
                EventType::Dropped => {
                    println!("Gamepad '{}' used by player '{}' was dropped!", gamepad_id, player_id);
                    break;
                }
                _ => {}
            }
        }

        let gamepad = get_gamepad(gamepad_id);

        if let Some(axis_data) = gamepad.axis_data(Axis::LeftStickX) {
            actor.controller.move_direction.x = axis_data.value();
        }
        if let Some(axis_data) = gamepad.axis_data(Axis::LeftStickY) {
            actor.controller.move_direction.y = -axis_data.value();
        }

        let mut aim_changed = false;
        if let Some(axis_data) = gamepad.axis_data(Axis::LeftZ) {
            let value = axis_data.value();
            if value != 0.0 {
                actor.controller.aim_direction.x = value;
                aim_changed = true;
            }
        }

        if let Some(axis_data) = gamepad.axis_data(Axis::RightZ) {
            let value = axis_data.value();
            if value != 0.0 {
                actor.controller.aim_direction.y = value;
                aim_changed = true;
            }
        }

        if aim_changed == false && actor.controller.move_direction != Vec2::ZERO {
            actor.controller.aim_direction = actor.controller.move_direction;
        }

        actor.controller.should_use_primary_ability = gamepad.is_pressed(Button::RightTrigger);
        actor.controller.should_use_secondary_ability = gamepad.is_pressed(Button::LeftTrigger);

        if actor.controller.is_sprint_locked == false {
            actor.controller.should_sprint = gamepad.is_pressed(Button::East);
        }

        actor.controller.should_pick_up_items = gamepad.is_pressed(Button::North);
    } else {
        let mouse_position = get_mouse_in_world_space();

        actor.controller.should_use_primary_ability = is_mouse_button_down(MouseButton::Left);
        actor.controller.should_use_secondary_ability = is_mouse_button_down(MouseButton::Right);

        actor.controller.move_direction = Vec2::ZERO;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            actor.controller.move_direction.y -= 1.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            actor.controller.move_direction.y += 1.0;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            actor.controller.move_direction.x -= 1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            actor.controller.move_direction.x += 1.0;
        }

        if is_key_pressed(KeyCode::CapsLock) {
            actor.controller.is_sprint_locked = !actor.controller.is_sprint_locked;
        }

        if actor.controller.should_use_primary_ability || actor.controller.should_use_secondary_ability {
            actor.controller.aim_direction = mouse_position.sub(actor.body.position).normalize_or_zero();
        } else {
            actor.controller.aim_direction = actor.controller.move_direction;
        }

        if actor.controller.is_sprint_locked {
            actor.controller.should_sprint = true;
        } else {
            actor.controller.should_sprint = is_key_down(KeyCode::LeftShift);
        }

        actor.controller.should_start_interaction = is_key_released(KeyCode::F);

        actor.controller.should_pick_up_items = is_key_down(KeyCode::R);

        if is_key_released(KeyCode::C) {
            game_state.should_show_character_window = !game_state.should_show_character_window;
        }
        if is_key_released(KeyCode::I) {
            game_state.should_show_inventory_window = !game_state.should_show_inventory_window;
        }

        if is_key_released(KeyCode::P) {
            game_state.in_debug_mode = !game_state.in_debug_mode;
        }

        if is_key_released(KeyCode::Escape) {
            if actor.current_dialogue.is_some()
                || game_state.should_show_inventory_window
                || game_state.should_show_character_window
                || game_state.should_show_game_menu {
                actor.current_dialogue = None;
                game_state.should_show_inventory_window = false;
                game_state.should_show_character_window = false;
                game_state.should_show_game_menu = false;
            } else {
                game_state.should_show_game_menu = true;
            }
        }
    }
}

pub fn map_gamepad(player_id: &str) -> Option<GamepadId> {
    let context = get_input_context();
    if let Some(gamepad_id) = get_gamepad_id(player_id) {
        return Some(gamepad_id);
    }
    for (gamepad_id, gamepad) in context.gilrs.gamepads() {
        if is_mapped(gamepad_id) == false {
            println!("Mapping '{}' ({:?}) to player '{}'", gamepad.name(), gamepad.power_info(), player_id);
            context.mappings.insert(player_id.to_string(), gamepad_id);
            return Some(gamepad_id);
        }
    }
    None
}

pub fn get_mapped_gamepad<'a>(player_id: &str) -> Option<Gamepad<'a>> {
    let context = get_input_context();
    if let Some(gamepad_id) = context.mappings.get(player_id) {
        return Some(context.gilrs.gamepad(*gamepad_id));
    }
    None
}

pub fn get_gamepad<'a>(gamepad_id: GamepadId) -> Gamepad<'a> {
    let context = get_input_context();
    context.gilrs.gamepad(gamepad_id)
}

pub fn get_player_id(gamepad_id: GamepadId) -> Option<String> {
    let context = get_input_context();
    context.mappings
        .iter()
        .find_map(|(player_id, found_id)|
            if gamepad_id == *found_id { Some(player_id.to_string()) } else { None })
}

pub fn get_gamepad_id(player_id: &str) -> Option<GamepadId> {
    let context = get_input_context();
    context.mappings.get(player_id).cloned()
}

pub fn get_events(player_id: &str) -> InputEventIterator {
    if let Some(gamepad_id) = get_gamepad_id(player_id) {
        return InputEventIterator::new(gamepad_id);
    }
    InputEventIterator::new_empty()
}

pub fn get_mouse_position() -> Vec2 {
    let (x, y) = mouse_position();
    vec2(x, y)
}

pub fn get_mouse_in_world_space() -> Vec2 {
    let viewport = storage::get::<Viewport>();
    viewport.to_world_space(get_mouse_position())
}


pub struct InputEventIterator<'a> {
    events_vec: Option<&'a mut Vec<EventType>>,
}

impl<'a> InputEventIterator<'a> {
    pub fn new(gamepad_id: GamepadId) -> Self {
        let context = get_input_context();
        let events_vec = context.events
            .get_mut(&gamepad_id);
        InputEventIterator {
            events_vec,
        }
    }

    pub fn new_empty() -> Self {
        InputEventIterator {
            events_vec: None,
        }
    }
}

impl<'a> Iterator for InputEventIterator<'a> {
    type Item = EventType;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(events_vec) = self.events_vec.as_mut() {
            return events_vec.pop();
        }
        None
    }
}
