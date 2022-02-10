mod maps;
pub mod shaders;
mod shapes;
mod text;

use bevy_ecs::change_detection::Mut;
pub use shapes::DebugShape2D;
pub use text::DebugText;

use bevy_app::{App, Plugin};
use bevy_math::*;

use miniquad::*;

use crate::input::{ButtonState, InputFrame, Window};

#[derive(Debug, Clone)]
pub struct SimpleMesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl SimpleMesh {
    pub fn new(ctx: &mut miniquad::Context, vertices: &[Vertex], indices: &[u16]) -> Self {
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, vertices);
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, indices);
        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn to_bindings(&self, images: impl Into<Option<Vec<Texture>>>) -> miniquad::Bindings {
        let images = images.into().unwrap_or_default();
        miniquad::Bindings {
            vertex_buffers: vec![self.vertex_buffer],
            index_buffer: self.index_buffer,
            images,
        }
    }
}

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

#[derive(Default)]
pub struct MiniquadPlugin;

impl Plugin for MiniquadPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(miniquad_runner)
            .init_resource::<DebugShape2D>()
            .init_resource::<DebugText>()
            .init_resource::<InputFrame>();
    }
}

struct Stage {
    app: App,
    start_time: f64,
    active_frame_input: InputFrame,
    last_frame_input: InputFrame,
    // TODO: Move to Bevy
    mesh: SimpleMesh,
    pipeline: miniquad::Pipeline,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Vertex {
    pos: Vec2,
}

impl Stage {
    pub fn new(mut app: App, input_frame: InputFrame) -> Self {
        let (pipeline, mesh) = {
            let mut ctx = app
                .world
                .get_resource_mut::<miniquad::Context>()
                .expect("Context MUST be in the App Resources");

            #[rustfmt::skip]
            let vertices: [Vertex; 4] = [
                Vertex { pos : Vec2::new(-0.5, -0.5 ) },
                Vertex { pos : Vec2::new( 0.5, -0.5 ) },
                Vertex { pos : Vec2::new( 0.5,  0.5 ) },
                Vertex { pos : Vec2::new(-0.5,  0.5 ) },
            ];

            let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

            let mesh = SimpleMesh::new(&mut ctx, vertices.as_slice(), indices.as_slice());

            (shaders::quad::pipeline(&mut ctx), mesh)
        };

        Self {
            app,
            start_time: miniquad::date::now(),
            active_frame_input: input_frame,
            last_frame_input: input_frame,
            pipeline,
            mesh,
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

    fn get_context(&mut self) -> Mut<Context> {
        self.app
            .world
            .get_resource_mut::<miniquad::Context>()
            .expect("Context MUST be in the App Resources")
    }
}

impl EventHandlerFree for Stage {
    fn update(&mut self) {
        self.begin_update();
        self.app.update()
    }

    fn draw(&mut self) {
        let pipeline = self.pipeline;
        let bindings = self.mesh.to_bindings(None);
        let mut ctx = self.get_context();
        // TODO: Get clear color from Resources
        ctx.begin_default_pass(Default::default());
        ctx.clear(Some((0.13, 0.137, 0.137, 1.0)), None, None);

        ctx.apply_pipeline(&pipeline);
        ctx.apply_bindings(&bindings);
        ctx.apply_uniforms(&shaders::quad::Uniform {
            color: crate::prelude::Color::WHITE.into(),
        });

        ctx.draw(0, 6, 1);

        ctx.end_render_pass();
        ctx.commit_frame();
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
        if let Some(keyboard_key) = maps::map_to_keyboard_key(keycode) {
            self.active_frame_input
                .keyboard
                .set(keyboard_key, ButtonState::JustDown);
        }
    }

    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        if let Some(keyboard_key) = maps::map_to_keyboard_key(keycode) {
            self.active_frame_input
                .keyboard
                .set(keyboard_key, ButtonState::JustUp);
        }
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
