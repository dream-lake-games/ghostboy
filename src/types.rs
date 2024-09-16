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

#[derive(Component, Clone, Copy, Debug, Reflect, std::hash::Hash, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
}
impl Facing {
    pub fn from_dir4(dir: Dir4) -> Self {
        match dir {
            Dir4::Up | Dir4::Left => Self::Left,
            Dir4::Down | Dir4::Right => Self::Right,
        }
    }
    pub fn to_flip_x(&self) -> bool {
        match self {
            Self::Left => true,
            Self::Right => false,
        }
    }
    pub fn from_f32(x: f32) -> Self {
        if x >= 0.0 {
            Self::Right
        } else {
            Self::Left
        }
    }
    pub fn right(&self) -> bool {
        self == &Self::Right
    }
}

#[derive(Clone, Copy, Debug, Reflect, std::hash::Hash, PartialEq, Eq)]
pub enum Dir4 {
    Up,
    Down,
    Left,
    Right,
}
impl Dir4 {
    pub fn from_field_instance(fi: &FieldInstance) -> Self {
        let FieldValue::Enum(Some(string)) = &fi.value else {
            panic!("bad dir4 field instance");
        };
        match string.as_str() {
            "Up" => Self::Up,
            "Down" => Self::Down,
            "Left" => Self::Left,
            "Right" => Self::Right,
            _ => panic!("bad dir4 field isntance string"),
        }
    }
}
