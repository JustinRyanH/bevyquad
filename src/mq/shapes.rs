// TODO(JryanH): I don't have the same pipeline as Macroquad, so I think instead I need to draw to a specific camera?
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct DebugShape2D;

pub enum DrawType {
    Solid { color: Color },
    Outline { thickness: f32, color: Color },
}

impl DebugShape2D {
    pub fn draw_circle(&mut self, center: Vec2, radius: f32, draw_type: DrawType) {}
    pub fn draw_poly(
        &mut self,
        center: Vec2,
        sides: u8,
        radius: f32,
        rotation: f32,
        draw_type: DrawType,
    ) {
    }
    pub fn draw_rectangle(&mut self, center: Vec2, extends: Vec2, draw_type: DrawType) {}
    pub fn draw_triangle(&mut self, v1: Vec2, v2: Vec2, v3: Vec2, draw_type: DrawType) {}
    pub fn draw_line(&mut self, start: Vec2, end: Vec2, thickness: f32, color: Color) {}
}
