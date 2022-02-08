use bevy_app::{App, Plugin};
use bevy_math::*;

use miniquad::*;

use crate::input::InputFrame;

pub fn miniquad_runner(mut app: App) {
    let config = conf::Conf {
        window_title: "FlappyBird".to_string(),
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    };
    miniquad::start(config, |ctx| {
        app.insert_resource(ctx);
        UserData::Free(Box::new(Stage::new(app)))
    });
}

#[derive(Default)]
pub struct MiniquadPlugin;

impl Plugin for MiniquadPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(miniquad_runner);
    }
}

struct Stage {
    app: App,
    start_time: f64,
    last_frame_time: f64,
    current_frame_input: InputFrame,
    last_frame_input: InputFrame,
}

impl Stage {
    pub fn new(app: App) -> Self {
        Self {
            app,
            start_time: miniquad::date::now(),
            last_frame_time: miniquad::date::now(),
            current_frame_input: Default::default(),
            last_frame_input: Default::default(),
        }
    }
}

impl Stage {}

impl EventHandlerFree for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        let ctx = self
            .app
            .world
            .get_resource_mut::<miniquad::Context>()
            .unwrap();
        ctx.clear(Some((0.13, 0.137, 0.137, 1.0)), None, None);
    }

    // Window Events
    fn resize_event(&mut self, _width: f32, _heightt: f32) {}

    fn window_minimized_event(&mut self) {}

    fn window_restored_event(&mut self) {}

    // Mouse Events

    fn mouse_motion_event(&mut self, _x: f32, _y: f32) {}

    fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {}

    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {}

    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {}

    fn raw_mouse_motion(&mut self, _dx: f32, _dy: f32) {}

    // Keyboard Events
    fn char_event(&mut self, _character: char, _keymods: KeyMods, _repeat: bool) {}

    fn key_down_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {}

    fn key_up_event(&mut self, _keycode: KeyCode, _keymods: KeyMods) {}

    // Touch Events
    fn touch_event(&mut self, phase: TouchPhase, _id: u64, x: f32, y: f32) {
        if phase == TouchPhase::Started {
            self.mouse_button_down_event(MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Ended {
            self.mouse_button_up_event(MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Moved {
            self.mouse_motion_event(x, y);
        }
    }
}
