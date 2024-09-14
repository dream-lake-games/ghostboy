use crate::prelude::*;

/// HBOX?????
#[derive(Clone, Debug, Reflect)]
pub struct Hbox {
    w: u32,
    h: u32,
}
impl Hbox {
    pub fn new(w: u32, h: u32) -> Self {
        Self { w, h }
    }
    pub fn as_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.w as f32,
            y: self.h as f32,
        }
    }
}
