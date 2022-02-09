use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Up,
    JustUp,
    JustDown,
    Down,
}

impl ButtonState {
    pub fn long_state(&mut self) {
        *self = match self {
            ButtonState::JustUp => ButtonState::Up,
            ButtonState::JustDown => ButtonState::Down,
            ButtonState::Up => ButtonState::Up,
            ButtonState::Down => ButtonState::Down,
        }
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::Up
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Time {
    pub frame: usize,
    pub time_in_seconds_since_start: f64,
    pub last_frame_time: f64,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Mouse {
    pub pos: Vec2,
    pub left: ButtonState,
    pub right: ButtonState,
    pub middle: ButtonState,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct KeyboardInput {
    // Modifiers
    pub left_ctrl: ButtonState,
    pub left_shift: ButtonState,
    pub left_alt: ButtonState,
    pub right_ctrl: ButtonState,
    pub right_shift: ButtonState,
    pub right_alt: ButtonState,

    // Accessories
    pub space: ButtonState,
    pub escape: ButtonState,
    pub tab: ButtonState,
}

impl KeyboardInput {
    pub fn long_state(&mut self) {
        self.left_ctrl.long_state();
        self.left_shift.long_state();
        self.left_alt.long_state();
        self.right_ctrl.long_state();
        self.right_shift.long_state();
        self.right_alt.long_state();
        self.space.long_state();
        self.escape.long_state();
        self.tab.long_state();
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Window {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct InputFrame {
    pub time: Time,
    pub window: Window,
    pub mouse: Mouse,
    pub keyboard: KeyboardInput,
}

impl InputFrame {
    pub fn long_state(&mut self) {
        self.keyboard.long_state();
        self.mouse.left.long_state();
        self.mouse.right.long_state();
        self.mouse.middle.long_state();
    }
}
