mod maps;

use bevy_app::{App, Plugin};
use bevy_math::*;

use miniquad::*;

use crate::input::{ButtonState, InputFrame, KeyboardInput, Window};

pub fn miniquad_runner(mut app: App) {
    let window_width = 1024;
    let window_height = 768;
    let config = conf::Conf {
        window_title: "FlappyBird".to_string(),
        window_width,
        window_height,
        ..Default::default()
    };
    let first_input_frame = InputFrame {
        window: Window {
            width: window_width as f32,
            height: window_height as f32,
        },
        ..Default::default()
    };
    miniquad::start(config, move |ctx| {
        app.insert_resource(ctx);
        UserData::Free(Box::new(Stage::new(app, first_input_frame)))
    });
}

pub fn debug_input(frame_input: bevy_ecs::system::Res<InputFrame>) {
    println!("{:?}", frame_input.as_ref());
}

#[derive(Default)]
pub struct MiniquadPlugin;

impl Plugin for MiniquadPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(miniquad_runner)
            .init_resource::<InputFrame>()
            .add_system(debug_input);
    }
}

struct Stage {
    app: App,
    start_time: f64,
    active_frame_input: InputFrame,
    last_frame_input: InputFrame,
}

impl Stage {
    pub fn new(app: App, input_frame: InputFrame) -> Self {
        Self {
            app,
            start_time: miniquad::date::now(),
            active_frame_input: input_frame,
            last_frame_input: input_frame,
        }
    }
}

impl Stage {
    pub fn begin_update(&mut self) {
        let mut input_frame = self.app.world.get_resource_mut::<InputFrame>().unwrap();
        self.last_frame_input = *input_frame.as_ref();
        *input_frame = self.active_frame_input;

        self.active_frame_input.time.frame += 1;
        self.active_frame_input.time.time_in_seconds_since_start =
            miniquad::date::now() - self.start_time;
        self.active_frame_input.time.last_frame_time = miniquad::date::now();
        self.active_frame_input.long_state();
    }
}

impl EventHandlerFree for Stage {
    fn update(&mut self) {
        self.begin_update();
        self.app.update()
    }

    fn draw(&mut self) {
        let ctx = self
            .app
            .world
            .get_resource_mut::<miniquad::Context>()
            .unwrap();
        ctx.clear(Some((0.13, 0.137, 0.137, 1.0)), None, None);
    }

    // Window Events
    fn resize_event(&mut self, width: f32, height: f32) {
        self.active_frame_input.window.height = height;
        self.active_frame_input.window.width = width;
    }

    fn window_minimized_event(&mut self) {}

    fn window_restored_event(&mut self) {}

    // Mouse Events

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.active_frame_input.mouse.pos = Vec2::new(x, y);
    }

    fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {}

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.active_frame_input.mouse.pos = Vec2::new(x, y);
        match button {
            MouseButton::Left => self.active_frame_input.mouse.left = ButtonState::JustDown,
            MouseButton::Right => self.active_frame_input.mouse.right = ButtonState::JustDown,
            MouseButton::Middle => self.active_frame_input.mouse.middle = ButtonState::JustDown,
            MouseButton::Unknown => {}
        };
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.active_frame_input.mouse.pos = Vec2::new(x, y);
        match button {
            MouseButton::Left => self.active_frame_input.mouse.left = ButtonState::JustUp,
            MouseButton::Right => self.active_frame_input.mouse.right = ButtonState::JustUp,
            MouseButton::Middle => self.active_frame_input.mouse.middle = ButtonState::JustUp,
            MouseButton::Unknown => {}
        };
    }

    fn raw_mouse_motion(&mut self, _dx: f32, _dy: f32) {}

    // Keyboard Events
    fn char_event(&mut self, _character: char, _keymods: KeyMods, _repeat: bool) {}

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        maps::set_keyboard(
            &mut self.active_frame_input.keyboard,
            keycode,
            ButtonState::JustDown,
        );
    }

    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        maps::set_keyboard(
            &mut self.active_frame_input.keyboard,
            keycode,
            ButtonState::JustUp,
        );
    }

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
