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

static mut INPUT_CONTEXT: Option<Box<InputContext>> = None;

#[derive(Debug)]
struct InputContext {
    pub gilrs: Gilrs,
    pub mappings: HashMap<String, GamepadId>,
    pub events: HashMap<GamepadId, Vec<EventType>>,
}

fn get_input_context() -> &'static mut InputContext {
    unsafe {
        if INPUT_CONTEXT.is_none() {
            let context = InputContext {
                gilrs: Gilrs::new().unwrap(),
                mappings: HashMap::new(),
                events: HashMap::new(),
            };

            INPUT_CONTEXT = Some(Box::new(context));
        }

        INPUT_CONTEXT.as_mut().unwrap()
    }
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
    context.mappings.retain(|_, gamepad_id| {
        if is_active(*gamepad_id) == false {
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

pub fn apply_input(player_id: &str, node: &mut RefMut<Actor>) {
    let mut game_state = scene::get_node(node.game_state);
    if let Some(gamepad_id) = get_gamepad_id(player_id) {
        for event in InputEventIterator::new(gamepad_id) {
            match event {
                EventType::ButtonChanged(button, value, _code) => {
                    match button {
                        Button::South => {},
                        Button::East => {
                            if value > 0.0 {
                                if game_state.gui_state.should_draw_game_menu {
                                    game_state.gui_state.should_draw_game_menu = false;
                                } else if node.current_dialogue.is_some() {
                                    node.current_dialogue = None;
                                }
                            }
                        },
                        Button::North => {},
                        Button::West => {
                            if value > 0.0 {
                                node.controller.should_start_interaction = true;
                            }
                        },
                        Button::C => {},
                        Button::Z => {},
                        Button::LeftTrigger => {},
                        Button::LeftTrigger2 => {
                            // SELECT
                            if value > 0.0 {
                                if game_state.gui_state.should_draw_character_window || game_state.gui_state.should_draw_inventory_window {
                                    game_state.gui_state.should_draw_inventory_window = false;
                                    game_state.gui_state.should_draw_character_window = false;
                                } else {
                                    game_state.gui_state.should_draw_inventory_window = true;
                                    game_state.gui_state.should_draw_character_window = true;
                                }
                            }
                        },
                        Button::RightTrigger => {},
                        Button::RightTrigger2 => {},
                        Button::Select => {},
                        Button::Start => {
                            if value > 0.0 {
                                game_state.gui_state.should_draw_game_menu = !game_state.gui_state.should_draw_game_menu;
                            }
                        },
                        Button::Mode => {},
                        Button::LeftThumb => {},
                        Button::RightThumb => {
                            if value > 0.0 {
                                node.controller.should_sprint = !node.controller.should_sprint;
                                node.controller.is_sprint_locked = node.controller.should_sprint;
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
                            node.controller.move_direction.x = value;
                        },
                        Axis::LeftStickY => {
                            node.controller.move_direction.y = value;
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
            node.controller.move_direction.x = axis_data.value();
        }
        if let Some(axis_data) = gamepad.axis_data(Axis::LeftStickY) {
            node.controller.move_direction.y = -axis_data.value();
        }

        let mut aim_changed = false;
        if let Some(axis_data) = gamepad.axis_data(Axis::LeftZ) {
            let value = axis_data.value();
            if value != 0.0 {
                node.controller.aim_direction.x = value;
                aim_changed = true;
            }
        }

        if let Some(axis_data) = gamepad.axis_data(Axis::RightZ) {
            let value = axis_data.value();
            if value != 0.0 {
                node.controller.aim_direction.y = value;
                aim_changed = true;
            }
        }

        if aim_changed == false && node.controller.move_direction != Vec2::ZERO {
            node.controller.aim_direction = node.controller.move_direction;
        }

        node.controller.should_use_weapon = gamepad.is_pressed(Button::RightTrigger);
        node.controller.should_use_selected_ability = gamepad.is_pressed(Button::LeftTrigger);

        if node.controller.is_sprint_locked == false {
            node.controller.should_sprint = gamepad.is_pressed(Button::East);
        }

        node.controller.should_pick_up_items = gamepad.is_pressed(Button::North);
    } else {
        let mouse_position = get_mouse_in_world_space();

        node.controller.should_use_weapon = is_mouse_button_down(MouseButton::Left);
        node.controller.should_use_selected_ability = is_mouse_button_down(MouseButton::Right);

        node.controller.move_direction = Vec2::ZERO;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            node.controller.move_direction.y -= 1.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            node.controller.move_direction.y += 1.0;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            node.controller.move_direction.x -= 1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            node.controller.move_direction.x += 1.0;
        }

        if is_key_pressed(KeyCode::CapsLock) {
            node.controller.is_sprint_locked = !node.controller.is_sprint_locked;
        }

        if node.controller.should_use_weapon || node.controller.should_use_selected_ability {
            node.controller.aim_direction = mouse_position.sub(node.body.position).normalize_or_zero();
        } else {
            node.controller.aim_direction = node.controller.move_direction;
        }

        if node.controller.is_sprint_locked {
            node.controller.should_sprint = true;
        } else {
            node.controller.should_sprint = is_key_down(KeyCode::LeftShift);
        }

        node.controller.should_start_interaction = is_key_released(KeyCode::F);

        node.controller.should_pick_up_items = is_key_down(KeyCode::R);

        node.controller.should_respawn = is_key_down(KeyCode::B);

        if is_key_released(KeyCode::C) {
            game_state.gui_state.should_draw_character_window = !game_state.gui_state.should_draw_character_window;
        }
        if is_key_released(KeyCode::I) {
            game_state.gui_state.should_draw_inventory_window = !game_state.gui_state.should_draw_inventory_window;
        }

        if is_key_released(KeyCode::P) {
            game_state.in_debug_mode = !game_state.in_debug_mode;
        }

        if is_key_released(KeyCode::Escape) {
            if node.current_dialogue.is_some()
                || game_state.gui_state.should_draw_inventory_window
                || game_state.gui_state.should_draw_character_window
                || game_state.gui_state.should_draw_game_menu {
                node.current_dialogue = None;
                game_state.gui_state.should_draw_inventory_window = false;
                game_state.gui_state.should_draw_character_window = false;
                game_state.gui_state.should_draw_game_menu = false;
            } else {
                game_state.gui_state.should_draw_game_menu = true;
            }
        }
    }
}

pub fn map_gamepad(player_id: &str) -> Option<GamepadId> {
    let context = get_input_context();
    if let Some(gamepad_id) = get_gamepad_id(player_id) {
        return Some(gamepad_id);
    }

    for (gamepad_id, _) in context.gilrs.gamepads() {
        if is_mapped(gamepad_id) == false {
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

pub(crate) fn get_gamepad<'a>(gamepad_id: GamepadId) -> Gamepad<'a> {
    let context = get_input_context();
    context.gilrs.gamepad(gamepad_id)
}

pub(crate) fn get_gamepad_id(player_id: &str) -> Option<GamepadId> {
    let context = get_input_context();
    context.mappings.get(player_id).cloned()
}

#[allow(dead_code)]
pub(crate) fn get_events(player_id: &str) -> InputEventIterator {
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
