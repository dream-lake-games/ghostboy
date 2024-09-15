use crate::prelude::*;

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

#[derive(Clone, Copy, Debug, Reflect, std::hash::Hash, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
