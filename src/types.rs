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
    + Copy
    + Send
    + Sync
    + 'static
    + std::fmt::Debug
    + PartialEq
    + Eq
    + Reflect
    + FromReflect
    + TypePath
    + GetTypeRegistration
{
}
