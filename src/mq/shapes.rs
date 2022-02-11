// TODO(JryanH): I don't have the same pipeline as Macroquad, so I think instead I need to draw to a specific camera?
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct DebugShape2D;

pub enum DrawType {
    Solid { color: Color },
    Outline { thickness: f32, color: Color },
}

impl DebugShape2D {
    pub fn draw_circle(&mut self, _center: Vec2, _radius: f32, _draw_type: DrawType) {}
    pub fn draw_poly(
        &mut self,
        _center: Vec2,
        _sides: u8,
        _radius: f32,
        _rotation: f32,
        _draw_type: DrawType,
    ) {
    }
    pub fn draw_rectangle(&mut self, _center: Vec2, _extends: Vec2, _draw_type: DrawType) {}
    pub fn draw_triangle(&mut self, _v1: Vec2, _v2: Vec2, _v3: Vec2, _draw_type: DrawType) {}
    pub fn draw_line(&mut self, _start: Vec2, _end: Vec2, _thickness: f32, _color: Color) {}
}
