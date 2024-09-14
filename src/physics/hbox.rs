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
}
