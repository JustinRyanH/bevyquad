mod color;
mod input;

pub mod components;
pub mod mq;

mod prelude {
    pub const Z_NEAR: f32 = 0.01;
    pub const Z_FAR: f32 = 10000.0;

    pub(crate) use bevy_app::prelude::*;
    pub(crate) use bevy_ecs::prelude::*;
    pub(crate) use bevy_math::prelude::*;
    pub(crate) use bevy_transform::prelude::*;

    pub use crate::color::*;
    pub use crate::components::*;
    pub use crate::input::*;
    pub use crate::mq::{DebugShape2D, DebugText, MiniquadPlugin};
}

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Component)]
pub struct WaveQuad;

pub fn load_square(mut commands: Commands, mut ctx: ResMut<miniquad::Context>) {
    use mq::shaders::Vertex;
    #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { position: Vec3::new(-0.5, -0.5, 0.0 ), uv: Vec2::new(0., 0.) },
            Vertex { position: Vec3::new( 0.5, -0.5, 0.0 ), uv: Vec2::new(1., 0.) },
            Vertex { position: Vec3::new( 0.5,  0.5, 0.0 ), uv: Vec2::new(1., 1.) },
            Vertex { position: Vec3::new(-0.5,  0.5, 0.0 ), uv: Vec2::new(0., 1.) },
        ];

    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
    let pixels: [u8; 4 * 4 * 4] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
        0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF,
    ];

    let mesh = SimpleMesh::new(&mut ctx, &vertices, &indices);
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

    for i in 0..10 {
        let t = i as f32 * 0.3;
        let t_sin = t.sin() as f32;

        let color: MeshColor = Color::hsl(t_sin.abs() * 360., 0.58, 0.25).into();
        let translation = Vec3::new(t_sin * 3.0, (t * 3.0).cos() as f32 * 3.0, 1.0);
        let transform = Transform::from_translation(translation);
        commands
            .spawn()
            .insert_bundle((mesh.clone(), color, tex.clone(), transform));
    }
    let camera_transform = Transform::from_scale(Vec3::new(5.0, 5.0, 1.0));
    commands.spawn_bundle((camera_transform, Projection::default()));
}

pub fn wave_quad(
    frame_input: Res<FrameInput>,
    mut query: Query<(&mut Transform, &mut MeshColor), With<SimpleMesh>>,
) {
    let t = frame_input.time.time_in_seconds_since_start;
    query
        .iter_mut()
        .enumerate()
        .for_each(|(index, (mut transform, mut color))| {
            let t = t + index as f64;
            let t_sin = t.sin() as f32;
            transform.translation =
                Vec3::new(t.sin() as f32 * 3.0, (t * 3.0).cos() as f32 * 3.0, 1.0);
            let new_color = Color::hsl(t_sin.abs() * 360., 0.58, 0.25);
            color.0 = new_color;
        });
}

fn main() {
    App::default()
        .add_plugin(TransformPlugin)
        .add_plugin(MiniquadPlugin::default())
        .add_startup_system(load_square)
        .add_system_to_stage(CoreStage::Update, wave_quad)
        .run();
}
