use bevy_math::{Vec2, Vec3};

#[derive(Debug, PartialEq, Default)]
#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub uv: Vec2,
}

pub mod quad {
    use std::ops::Deref;

    use bevy_math::{Mat4, Vec4};
    use miniquad::*;

    use crate::color::Color;

    pub const VERTEX: &str = r#"#version 100
    attribute vec3 position;
    attribute vec2 uv;

    uniform vec4 InColor;
    uniform mat4 Model;
    uniform mat4 Projection;

    varying lowp vec4 color;
    varying lowp vec2 texcoord;

    void main() {
        gl_Position = Projection * Model * vec4(position, 1);
        color = InColor;
        texcoord = uv;
    }
    "#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord) * color;
    }
    "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("InColor", UniformType::Float4),
                    UniformDesc::new("Model", UniformType::Mat4),
                    UniformDesc::new("Projection", UniformType::Mat4),
                ],
            },
            images: vec!["tex".to_string()],
        }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct Uniform {
        pub color: Vec4,
        pub model: Mat4,
        pub projection: Mat4,
    }

    impl Default for Uniform {
        fn default() -> Self {
            Self {
                color: Color::ANTIQUE_WHITE.into(),
                model: Mat4::IDENTITY,
                projection: Mat4::IDENTITY,
            }
        }
    }

    fn build_shader(ctx: &mut miniquad::Context) -> Shader {
        Shader::new(ctx, VERTEX, FRAGMENT, meta()).unwrap()
    }

    pub struct QuadPipeline(pub Pipeline);
    impl Deref for QuadPipeline {
        type Target = miniquad::Pipeline;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    pub fn pipeline(ctx: &mut miniquad::Context) -> Pipeline {
        let shader = build_shader(ctx);
        Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("position", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        )
    }

    pub fn build(ctx: &mut miniquad::Context) -> QuadPipeline {
        QuadPipeline(pipeline(ctx))
    }
}
