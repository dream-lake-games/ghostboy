use crate::prelude::*;

pub type Pos = IVec2;
pub type Zix = i32;
pub type Fzix = f32;
#[derive(Debug, Clone, Copy, Reflect, Default)]
pub struct Place {
    pub pos: Pos,
    pub zix: Zix,
}
impl Place {}

pub trait Queryable:
    Sized
    + Clone
    + Send
    + Sync
    + 'static
    + std::fmt::Debug
    + Reflect
    + FromReflect
    + TypePath
    + GetTypeRegistration
{
}
