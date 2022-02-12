mod maps;
pub mod shaders;
mod shapes;
mod text;

use bevy_app::{App, CoreStage, Plugin};
use bevy_ecs::prelude::*;
use bevy_math::*;
use miniquad::*;

use crate::input::{ButtonState, FrameInput, Window};

pub use shapes::DebugShape2D;
pub use text::DebugText;

#[derive(Debug, PartialEq, Eq, Hash, Clone, StageLabel)]
pub struct RenderStage;

#[derive(Debug, PartialEq, Eq, Hash, Clone, StageLabel)]
pub struct InputProcessing;

pub mod components {
    use crate::prelude::*;
    use miniquad::*;

    use super::shaders::Vertex;

    #[derive(Debug, Clone, Copy, Component)]
    pub struct Projection {
        pub aspect_ratio: f32,
        pub field_of_view: f32,
    }

    impl Default for Projection {
        fn default() -> Self {
            Self {
                aspect_ratio: 1.0,
                field_of_view: 45.0,
            }
        }
    }

    #[derive(Debug, Clone, Component)]
    pub struct SimpleMesh {
        vertex_buffer: Buffer,
        index_buffer: Buffer,
    }

    #[derive(Debug, Clone, Component)]
    pub struct SimpleMeshTexture(pub miniquad::Texture);

    impl SimpleMeshTexture {
        pub fn from_data(
            context: &mut miniquad::Context,
            bytes: &[u8],
            params: TextureParams,
        ) -> Self {
            Self(miniquad::Texture::from_data_and_format(
                context, bytes, params,
            ))
        }
    }

    #[derive(Debug, Clone, Copy, Component)]
    pub struct MeshColor(pub Color);

    impl From<Color> for MeshColor {
        fn from(v: Color) -> Self {
            Self(v)
        }
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
}

mod systems {
    use crate::prelude::*;

    use super::{
        components::{MeshColor, Projection, SimpleMesh, SimpleMeshTexture},
        shaders::{
            quad::{QuadPipeline, Uniform},
            Vertex,
        },
    };

    fn get_projection(camera: Query<&Projection>) -> Mat4 {
        let Projection {
            aspect_ratio,
            field_of_view,
        } = camera.get_single().ok().cloned().unwrap_or_default();
        let top = field_of_view / 2.0;
        let right = top * aspect_ratio;
        Mat4::orthographic_rh_gl(-right, right, -top, top, -1., Z_FAR)
    }

    pub fn quad_render_pass(
        mut ctx: ResMut<miniquad::Context>,
        mesh: Query<(&SimpleMesh, Option<&MeshColor>, Option<&SimpleMeshTexture>)>,
        camera: Query<&Projection>,
        pipeline: Res<QuadPipeline>,
    ) {
        let projection = get_projection(camera);

        ctx.begin_default_pass(Default::default());
        ctx.clear(Some((0.13, 0.137, 0.137, 1.0)), None, None);

        ctx.apply_pipeline(pipeline.as_ref());

        for (mesh, color, texture) in mesh.iter() {
            let texture = texture.map(|m| vec![m.0]);
            let bindings = mesh.to_bindings(texture);
            let color: Vec4 = color.map(|color| color.0).unwrap_or(Color::WHITE).into();
            ctx.apply_bindings(&bindings);
            ctx.apply_uniforms(&Uniform {
                color,
                projection,
                ..Default::default()
            });

            ctx.draw(0, 6, 1);
        }

        ctx.end_render_pass();
    }

    pub fn gather_aspect_ratio(frame_input: Res<FrameInput>, mut query: Query<&mut Projection>) {
        let window = frame_input.window;
        query.iter_mut().for_each(|mut projection| {
            projection.aspect_ratio = window.width / window.height;
        });
    }

    pub fn load_square(mut commands: Commands, mut ctx: ResMut<miniquad::Context>) {
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { position: Vec3::new(-0.5, -0.5, 0.0 ), uv: Vec2::new(0., 0.) },
            Vertex { position: Vec3::new( 0.5, -0.5, 0.0 ), uv: Vec2::new(1., 0.) },
            Vertex { position: Vec3::new( 0.5,  0.5, 0.0 ), uv: Vec2::new(1., 1.) },
            Vertex { position: Vec3::new(-0.5,  0.5, 0.0 ), uv: Vec2::new(0., 1.) },
        ];

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let pixels: [u8; 4 * 4 * 4] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];

        let mesh = SimpleMesh::new(&mut ctx, &vertices, &indices);
        let color: MeshColor = Color::hsl(301., 0.58, 0.25).into();
        let tex: SimpleMeshTexture = SimpleMeshTexture::from_data(
            &mut ctx,
            &pixels,
            miniquad::TextureParams {
                width: 4,
                height: 4,
                filter: miniquad::FilterMode::Nearest,
                ..Default::default()
            },
        );
        commands.spawn().insert_bundle((mesh, color, tex));
        commands.spawn_bundle((Transform::identity(), Projection::default()));
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
    let first_frame_input = FrameInput {
        window: Window {
            width: window_width as f32,
            height: window_height as f32,
        },
        ..Default::default()
    };
    miniquad::start(config, move |ctx| {
        app.insert_resource(ctx);
        UserData::Free(Box::new(Stage::new(app, first_frame_input)))
    });
}

#[derive(Default)]
pub struct MiniquadPlugin;

impl Plugin for MiniquadPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(miniquad_runner)
            .init_resource::<DebugShape2D>()
            .init_resource::<DebugText>()
            .init_resource::<FrameInput>()
            .add_startup_system(systems::load_square) // TODO: Remove me
            .add_stage_before(
                CoreStage::PreUpdate,
                InputProcessing,
                SystemStage::parallel(),
            )
            .add_stage_after(
                CoreStage::PostUpdate,
                RenderStage,
                SystemStage::single_threaded(),
            )
            .add_system_to_stage(InputProcessing, systems::gather_aspect_ratio)
            .add_system_to_stage(RenderStage, systems::quad_render_pass);
    }
}

struct Stage {
    app: App,
    start_time: f64,
    active_frame_input: FrameInput,
    last_frame_input: FrameInput,
}

impl Stage {
    pub fn new(mut app: App, frame_input: FrameInput) -> Self {
        let pipeline = {
            let mut ctx = app
                .world
                .get_resource_mut::<miniquad::Context>()
                .expect("Context MUST be in the App Resources");
            shaders::quad::build(&mut ctx)
        };
        app.insert_resource(pipeline);

        Self {
            app,
            start_time: miniquad::date::now(),
            active_frame_input: frame_input,
            last_frame_input: frame_input,
        }
    }
}

impl Stage {
    pub fn begin_update(&mut self) {
        let mut frame_input = self.app.world.get_resource_mut::<FrameInput>().unwrap();
        self.last_frame_input = *frame_input.as_ref();
        *frame_input = self.active_frame_input;

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
    }

    fn draw(&mut self) {
        self.app.update();
        let mut ctx = self
            .app
            .world
            .get_resource_mut::<miniquad::Context>()
            .expect("Miniquad Context is required");
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
