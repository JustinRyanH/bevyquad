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

#[derive(Debug, Clone, Copy)]
pub struct KeyboardInput {
    pub keys: [ButtonState; 128],
}

impl Default for KeyboardInput {
    fn default() -> Self {
        Self {
            keys: [ButtonState::Up; 128],
        }
    }
}

impl KeyboardInput {
    pub fn long_state(&mut self) {
        self.keys.iter_mut().for_each(|state| state.long_state());
    }

    pub fn get(&self, key: KeyboardKey) -> ButtonState {
        let index: usize = key.into();
        self.keys[index]
    }

    pub fn set(&mut self, key: KeyboardKey, state: ButtonState) -> ButtonState {
        let index: usize = key.into();
        let old_state = self.keys[index];
        self.keys[index] = state;
        old_state
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Window {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct FrameInput {
    pub time: Time,
    pub window: Window,
    pub mouse: Mouse,
    pub keyboard: KeyboardInput,
}

impl FrameInput {
    pub fn long_state(&mut self) {
        self.keyboard.long_state();
        self.mouse.left.long_state();
        self.mouse.right.long_state();
        self.mouse.middle.long_state();
    }
}

pub enum KeyboardKey {
    Space = 0,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Semicolon,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,
    Escape,
    Enter,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDecimal,
    KpDivide,
    KpMultiply,
    KpSubtract,
    KpAdd,
    KpEnter,
    KpEqual,
    Tab,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    Menu,
}

impl From<KeyboardKey> for usize {
    fn from(val: KeyboardKey) -> Self {
        val as usize
    }
}
