use crate::prelude::*;

pub struct Font {
    pub handle: usize,
}

impl Font {
    pub fn new(handle: usize) -> Self {
        Self { handle }
    }
}

pub struct TextParams {
    pub font: Font,
    pub font_size: u16,
    pub font_scale: f32,
    pub font_scale_aspect: f32,
    pub color: Color,
}

impl Default for TextParams {
    fn default() -> Self {
        Self {
            font: Font::new(0),
            font_size: 20,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: Color::WHITE,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DebugText;

impl DebugText {
    pub fn draw(_text: &str, _position: Vec2, _params: TextParams) {}
}
