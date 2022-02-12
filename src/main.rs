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

fn main() {
    App::default()
        .add_plugin(TransformPlugin)
        .add_plugin(MiniquadPlugin::default())
        .add_startup_system(load_square)
        .run();
}
