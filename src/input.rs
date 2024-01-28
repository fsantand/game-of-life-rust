use std::collections::HashMap;

use piston::{Button, Key, MouseButton};

#[derive(Clone, Copy)]
pub enum PlayerActions {
    Idle,
    RunSimulation,
    NextStep,
    ToggleTile,
    CountNeightbours,
}

pub struct InputHandler {
    mappings: HashMap<Key, PlayerActions>,
    click_mappings: HashMap<MouseButton, PlayerActions>,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            mappings: HashMap::new(),
            click_mappings: HashMap::new(),
        }
    }

    pub fn handle_keyboard(&self, key: Key) -> PlayerActions {
        match self.mappings.contains_key(&key) {
            true => self.mappings[&key],
            false => PlayerActions::Idle,
        }
    }

    pub fn handle_mouse(&self, button: MouseButton, _cursor: [f64;2]) -> PlayerActions {
        match self.click_mappings.contains_key(&button) {
            true =>  self.click_mappings[&button],
            false => PlayerActions::Idle,
        }
    }

    pub fn add_mapping(&mut self, key: Key, action: PlayerActions) {
        self.mappings.insert(key, action);
    }

    pub fn add_click_mapping(&mut self, button: MouseButton, action: PlayerActions) {
        self.click_mappings.insert(button, action);
    }
}

/// Handles the input
pub fn handle_input(handler: &InputHandler, event: Button, cursor: [f64;2]) ->PlayerActions {
    match event {
        Button::Keyboard(key) => handler.handle_keyboard(key),
        Button::Mouse(button) => handler.handle_mouse(button, cursor),
        _ => PlayerActions::Idle,
    }
}
