use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ButtonState {
    Up,
    JustUp,
    JustDown,
    Down,
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
pub struct Keyboard {
    // Modifiers
    pub left_ctrl: ButtonState,
    pub left_shift: ButtonState,
    pub left_alt: ButtonState,

    // Accessories
    pub space: ButtonState,
    pub escape: ButtonState,
    pub tab: ButtonState,
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
}
